//! Orchestrates commands and UX.

use crate::{
    exercise::{self, Exercise},
    grader,
    persistence::{self, ExerciseProgress},
    util,
};
use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::{fs, path::PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run(lessons_root: String, cmd: Command) -> Result<()> {
    match cmd {
        Command::List => cmd_list(&lessons_root),
        Command::Start { id } => cmd_start(&lessons_root, &id),
        Command::Open { id } => cmd_open(&id),
        Command::Check { id, timeout } => cmd_check(&lessons_root, &id, timeout),
        Command::Hint { id } => cmd_hint(&lessons_root, &id),
        Command::Progress => cmd_progress(),
        Command::Reset { id } => cmd_reset(&id),
        Command::CheckAll => cmd_check_all(&lessons_root),
        Command::Validate => cmd_validate(&lessons_root),
        Command::Solution { id } => cmd_solution(&lessons_root, &id),
        Command::ClearAll => cmd_clear_all(),
    }
}

#[derive(Debug)]
pub enum Command {
    List,
    Start { id: String },
    Open { id: String },
    Check { id: String, timeout: Option<u64> },
    Hint { id: String },
    Progress,
    Reset { id: String },
    CheckAll,
    Validate,
    Solution { id: String },
    ClearAll,
}

fn load_by_id(lessons_root: &str, id: &str) -> Result<Exercise> {
    let all = exercise::load_all(lessons_root)?;
    all.into_iter()
        .find(|e| e.meta.id == id)
        .with_context(|| format!("Exercise '{}' not found", id))
}

fn cmd_list(lessons_root: &str) -> Result<()> {
    let xs = exercise::load_all(lessons_root)?;
    if xs.is_empty() {
        println!("No exercises found under '{}'", lessons_root);
        return Ok(());
    }
    for x in xs {
        println!(
            "{}  {}  [{}]",
            x.meta.id.bold(),
            x.meta.title,
            x.meta.difficulty
        );
    }
    Ok(())
}

fn cmd_start(lessons_root: &str, id: &str) -> Result<()> {
    let ex = load_by_id(lessons_root, id)?;
    if !start_unlocked(lessons_root, id)? {
        anyhow::bail!("Exercise '{}' is locked. Complete the previous lesson first (or set RUST_GAME_FORCE=1).", id);
    }
    let work_dir = ex
        .working_file()?
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap();
    fs::create_dir_all(&work_dir)?;
    let work_file = ex.working_file()?;
    if work_file.exists() {
        println!("Working copy already exists at {:?}", work_file);
    } else {
        fs::copy(&ex.starter_rs, &work_file)?;
        println!("Initialized working copy at {:?}", work_file);
    }
    let mut prog = persistence::load().unwrap_or_default();
    let entry = prog.exercises.entry(ex.meta.id.clone()).or_insert(ExerciseProgress::default());
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    if entry.first_started_at.is_none() { entry.first_started_at = Some(now); }
    entry.last_started_at = Some(now);
    persistence::save(&prog)?;
    Ok(())
}

fn cmd_open(id: &str) -> Result<()> {
    let dummy_meta = crate::exercise::ExerciseMeta {
        id: id.to_string(),
        title: "".into(),
        difficulty: "beginner".into(),
        hint: None,
        timeout_secs: None,
    };
    let ex = Exercise { meta: dummy_meta, root: PathBuf::new(), starter_rs: PathBuf::new(), tests_rs: PathBuf::new(), solution_rs: None, explanation_md: None };
    let file = ex.working_file()?;
    if !file.exists() {
        anyhow::bail!("Working file not found: {:?}. Run `start` first to create it, then edit this file.", file);
    }
    util::open_in_editor(&file)?;
    let _ = util::reveal_in_file_manager(&file);
    Ok(())
}

fn cmd_check(lessons_root: &str, id: &str, timeout: Option<u64>) -> Result<()> {
    let ex = load_by_id(lessons_root, id)?;
    let file = ex.working_file()?;
    if !file.exists() {
        anyhow::bail!("Working file not found: {:?}. Run `start` first.", file);
    }

    let timeout = timeout.or(ex.meta.timeout_secs).unwrap_or(15);

    let bar = ProgressBar::new_spinner();
    bar.set_style(ProgressStyle::with_template("{spinner} Running tests... {msg}").unwrap());
    bar.enable_steady_tick(std::time::Duration::from_millis(100));

    let outcome = grader::grade(&ex, &file, timeout)?;
    bar.finish_and_clear();

    if outcome.passed {
        println!("{}", "All tests passed 🎉".green().bold());
        let mut prog = persistence::load().unwrap_or_default();
        let lesson_id = ex.meta.id.clone();
        let (now, points, should_award, last_duration_secs) = {
            let entry = prog.exercises.entry(lesson_id.clone()).or_insert(ExerciseProgress::default());
            entry.completed = true;
            entry.attempts += 1;
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            entry.completed_at = Some(now);
            if let Some(start) = entry.last_started_at {
                let dur = (now - start).max(0) as u64;
                entry.last_duration_secs = Some(dur);
                entry.best_duration_secs = Some(entry.best_duration_secs.map(|b| b.min(dur)).unwrap_or(dur));
            }
            let points = match ex.meta.difficulty.as_str() { "beginner" => 10, "intermediate" => 25, _ => 50 };
            let should_award = entry.points_earned == 0;
            let last_duration_secs = entry.last_duration_secs;
            (now, points, should_award, last_duration_secs)
        };
        if should_award {
            prog.total_points = prog.total_points.saturating_add(points as u32);
            if points >= 50 && !prog.badges.contains(&"First Advanced".to_string()) {
                prog.badges.push("First Advanced".to_string());
            }
            if prog.exercises.values().filter(|e| e.completed).count() as u32 >= 5 && !prog.badges.contains(&"Getting Serious".to_string()) {
                prog.badges.push("Getting Serious".to_string());
            }
            if prog.total_points >= 100 && !prog.badges.contains(&"Century".to_string()) {
                prog.badges.push("Century".to_string());
            }
            let name = prog.display_name.clone().unwrap_or_else(|| "Player".to_string());
            let avatar = prog.avatar.clone();
            let lb = persistence::LeaderboardEntry { name, avatar, lesson_id: lesson_id.clone(), points: points as u32, duration_secs: last_duration_secs, timestamp: now };
            let _ = persistence::add_leaderboard_entry(lb);
            if let Some(ent) = prog.exercises.get_mut(&lesson_id) { ent.points_earned = points; }
        }
        persistence::save(&prog)?;
        if let Some(uid) = prog.current_user_id {
            let _ = crate::storage::record_attempt(uid, &lesson_id, true, last_duration_secs, now);
        }
    } else {
        println!("{}", "Some tests failed".red().bold());
        if !outcome.stdout.is_empty() { println!("stdout:\n{}", outcome.stdout); }
        if !outcome.stderr.is_empty() {
            println!("stderr:\n{}", outcome.stderr);
            let codes = extract_error_codes(&outcome.stderr);
            if !codes.is_empty() {
                println!("Help:");
                for c in codes { println!("  - https://doc.rust-lang.org/error_codes/{}.html", c); }
            }
        }
        let mut prog = persistence::load().unwrap_or_default();
        let entry = prog.exercises.entry(ex.meta.id.clone()).or_insert(ExerciseProgress::default());
        entry.attempts += 1;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let lesson_id = ex.meta.id.clone();
        persistence::save(&prog)?;
        if let Some(uid) = prog.current_user_id {
            let _ = crate::storage::record_attempt(uid, &lesson_id, false, None, now);
        }
    }
    Ok(())
}

fn start_unlocked(lessons_root: &str, id: &str) -> Result<bool> {
    if env::var("RUST_GAME_FORCE").ok().as_deref() == Some("1") { return Ok(true); }
    let all = exercise::load_all(lessons_root)?;
    let ex = match all.iter().find(|e| e.meta.id == id) { Some(e) => e, None => return Ok(false) };
    let diff = ex.meta.difficulty.clone();
    fn slug(id: &str) -> &str { id.rsplit('/').next().unwrap_or(id) }
    fn rank(diff: &str, slug: &str) -> u32 {
        let beginner: &[&str] = &[
            "variables",
            "control_flow_loops",
            "arrays_slices_basics",
            "functions",
            "ownership",
            "strings_utf8",
            "pattern_matching_ergonomics",
            "match_enums",
            "hashmap",
            "iterators",
            "result_error",
            "traits",
        ];
        let intermediate: &[&str] = &[
            "lifetimes_longest",
            "generics_asref_maxlen",
            "result_map_errors",
            "iterators_combinators",
            "iterators_ownership",
            "rc_refcell",
            "thiserror",
        ];
        let advanced: &[&str] = &[
            "arc_mutex_counter",
            "arc_rwlock_counter",
            "channels_mpsc",
            "tokio_mpsc",
            "tokio_timeout_race",
            "traits_dyn_dispatch",
        ];
        let list = match diff { "beginner" => beginner, "intermediate" => intermediate, _ => advanced };
        list.iter().position(|s| s == &slug).map(|i| i as u32).unwrap_or(999)
    }
    let mut same: Vec<&exercise::Exercise> = all.iter().filter(|e| e.meta.difficulty == diff).collect();
    same.sort_by(|a,b| {
        let ra = rank(&a.meta.difficulty, slug(&a.meta.id));
        let rb = rank(&b.meta.difficulty, slug(&b.meta.id));
        if ra != rb { return ra.cmp(&rb); }
        a.meta.id.cmp(&b.meta.id)
    });
    let pos = same.iter().position(|e| e.meta.id == id).unwrap_or(0);
    if pos == 0 { return Ok(true); }
    let prev_id = &same[pos - 1].meta.id;
    let prog = persistence::load().unwrap_or_default();
    let completed = prog.exercises.get(prev_id).map(|e| e.completed).unwrap_or(false);
    Ok(completed)
}

fn cmd_hint(lessons_root: &str, id: &str) -> Result<()> {
    let ex = load_by_id(lessons_root, id)?;
    match ex.meta.hint.as_deref() { Some(h) => println!("Hint: {}", h), None => println!("No hint available."), }
    Ok(())
}

fn extract_error_codes(stderr: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = stderr.as_bytes();
    let mut i = 0;
    while i + 5 <= bytes.len() {
        if bytes[i] == b'E' && bytes[i + 1].is_ascii_digit() && bytes[i + 2].is_ascii_digit()
            && bytes[i + 3].is_ascii_digit() && bytes[i + 4].is_ascii_digit()
        {
            let code = &stderr[i..i + 5];
            if !out.contains(&code.to_string()) {
                out.push(code.to_string());
            }
            i += 5;
            continue;
        }
        i += 1;
    }
    out
}

fn cmd_progress() -> Result<()> {
    let prog = persistence::load().unwrap_or_default();
    if prog.exercises.is_empty() {
        println!("No progress yet. Start an exercise!");
        return Ok(());
    }
    for (id, e) in prog.exercises { let status = if e.completed { "completed" } else { "in progress" }; println!("{} - {} (attempts: {})", id, status, e.attempts); }
    Ok(())
}

fn cmd_reset(id: &str) -> Result<()> {
    let dummy_meta = crate::exercise::ExerciseMeta { id: id.to_string(), title: "".into(), difficulty: "beginner".into(), hint: None, timeout_secs: None };
    let ex = Exercise { meta: dummy_meta, root: PathBuf::new(), starter_rs: PathBuf::new(), tests_rs: PathBuf::new(), solution_rs: None, explanation_md: None };
    let dir = ex.working_file()?.parent().unwrap().to_path_buf();
    if dir.exists() { fs::remove_dir_all(&dir)?; println!("Reset working dir {:?}", dir); } else { println!("No working dir for {}", id); }
    let sandbox = crate::util::data_dir()?.join("sandboxes").join(id.replace('/', "_"));
    if sandbox.exists() { fs::remove_dir_all(&sandbox)?; println!("Reset sandbox dir {:?}", sandbox); }
    Ok(())
}

fn cmd_check_all(lessons_root: &str) -> Result<()> {
    let all = exercise::load_all(lessons_root)?;
    if all.is_empty() { println!("No lessons found"); return Ok(()); }
    let mut total = 0usize;
    let mut passed = 0usize;
    for ex in all {
        total += 1;
        let file = ex.working_file()?;
        let source = if file.exists() { file.clone() } else { ex.starter_rs.clone() };
        let timeout = ex.meta.timeout_secs.unwrap_or(15);
        let out = grader::grade(&ex, &source, timeout)?;
        let mark = if out.passed { passed += 1; "✓" } else { "✗" };
        println!("{} {} - {}", mark, ex.meta.id, if out.passed { "pass" } else { "fail" });
        if !out.passed {
            if !out.stdout.is_empty() { println!("stdout:\n{}", out.stdout); }
            if !out.stderr.is_empty() { println!("stderr:\n{}", out.stderr); }
        }
    }
    println!("Summary: {passed}/{total} passed");
    Ok(())
}

fn cmd_validate(lessons_root: &str) -> Result<()> {
    let all = exercise::load_all(lessons_root)?;
    let mut ok = true;
    for ex in &all {
        if !ex.starter_rs.exists() { ok = false; println!("Missing starter.rs for {}", ex.meta.id); }
        if !ex.tests_rs.exists() { ok = false; println!("Missing tests.rs for {}", ex.meta.id); }
        if ex.meta.title.trim().is_empty() { ok = false; println!("Empty title for {}", ex.meta.id); }
        if !matches!(ex.meta.difficulty.as_str(), "beginner"|"intermediate"|"advanced") {
            ok = false; println!("Invalid difficulty for {}: {}", ex.meta.id, ex.meta.difficulty);
        }
    }
    if ok { println!("All lesson metadata OK ({} lessons)", all.len()); }
    Ok(())
}

fn cmd_solution(lessons_root: &str, id: &str) -> Result<()> {
    let ex = load_by_id(lessons_root, id)?;
    let sol = ex.root.join("solution.rs");
    if sol.exists() {
        println!("Solution: {}", sol.display());
        if let Ok(data) = std::fs::read_to_string(&sol) {
            let preview: String = data.lines().take(40).collect::<Vec<_>>().join("\n");
            println!("{}\n...", preview);
        }
    } else {
        println!("No official solution for {}", id);
    }
    Ok(())
}

fn cmd_clear_all() -> Result<()> {
    let uid = persistence::load().ok().and_then(|p| p.current_user_id);
    if let Ok(p) = persistence::progress_path() { let _ = std::fs::remove_file(p); }
    if let Ok(lp) = persistence::leaderboard_path() { let _ = std::fs::remove_file(lp); }
    let work = crate::exercise::Exercise::working_dir()?;
    if work.exists() { let _ = std::fs::remove_dir_all(&work); }
    let sand = crate::util::data_dir()?.join("sandboxes");
    if sand.exists() { let _ = std::fs::remove_dir_all(&sand); }
    if let Some(id) = uid { let _ = crate::storage::clear_attempts_for_user(id); }
    println!("Reset complete. Fresh start ready.");
    Ok(())
}