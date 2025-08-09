use anyhow::Result;
use eframe::egui;
use std::sync::{Arc, Mutex};
use webbrowser;
use egui::WidgetText;

#[derive(Default, Clone)]
struct AppState {
    lessons_root: String,
    selected_id: Option<String>,
    exercises: Vec<(String, String, String)>,
    last_output: String,
    filter_text: String,
    show_only_incomplete: bool,
    show_output_tab: bool,
    text_scale: f32,
    auth_username: String,
    auth_password: String,
    auth_show_password: bool,
}

pub struct GuiApp {
    state: Arc<Mutex<AppState>>,
}

impl GuiApp {
    fn is_unlocked(state: &AppState, id: &str) -> bool {
        if let Ok(list) = rust_game::exercise::load_all(&state.lessons_root) {
            if let Some(ex) = list.iter().find(|e| e.meta.id == id) {
                let diff = ex.meta.difficulty.clone();
                let mut same: Vec<&rust_game::exercise::Exercise> = list.iter().filter(|e| e.meta.difficulty == diff).collect();
                same.sort_by(|a,b| a.meta.id.cmp(&b.meta.id));
                if let Some(pos) = same.iter().position(|e| e.meta.id == id) {
                    if pos == 0 { return true; }
                    let prev_id = &same[pos - 1].meta.id;
                    if let Ok(p) = rust_game::persistence::load() {
                        return p.exercises.get(prev_id).map(|e| e.completed).unwrap_or(false);
                    }
                }
            }
        }
        false
    }

    fn next_unlocked_id(state: &AppState, current_id: &str) -> Option<String> {
        if let Ok(list) = rust_game::exercise::load_all(&state.lessons_root) {
            if let Some((idx, _)) = list.iter().enumerate().find(|(_, e)| e.meta.id == current_id) {
                for j in (idx + 1)..list.len() {
                    let nid = &list[j].meta.id;
                    if Self::is_unlocked(state, nid) { return Some(nid.clone()); }
                }
            }
        }
        None
    }
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = 8.0.into();
        visuals.panel_fill = egui::Color32::from_rgb(22, 24, 28);
        cc.egui_ctx.set_visuals(visuals);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        cc.egui_ctx.set_style(style);
        Self {
            state: Arc::new(Mutex::new(AppState {
                lessons_root: "lessons".to_string(),
                filter_text: String::new(),
                show_only_incomplete: false,
                show_output_tab: false,
                text_scale: 1.0,
                auth_username: String::new(),
                auth_password: String::new(),
                auth_show_password: true,
                ..Default::default()
            })),
        }
    }

    fn refresh_list(state: &mut AppState) {
        match rust_game::exercise::load_all(&state.lessons_root) {
            Ok(list) => {
                state.exercises = list
                    .into_iter()
                    .map(|e| (e.meta.id, e.meta.title, e.meta.difficulty))
                    .collect();
            }
            Err(e) => {
                state.last_output = format!("Error loading lessons: {e}");
            }
        }
    }

    fn run_engine_check(state: &mut AppState) {
        if let Some(id) = state.selected_id.clone() {
            let out = (|| -> Result<String> {
                // Load exercise
                let ex = rust_game::exercise::load_all(&state.lessons_root)?
                    .into_iter()
                    .find(|e| e.meta.id == id)
                    .ok_or_else(|| anyhow::anyhow!("Exercise not found"))?;
                let file = ex.working_file()?;
                if !file.exists() {
                    return Ok("Working file not found. Use Start first.".to_string());
                }
                let timeout = ex.meta.timeout_secs.unwrap_or(15);
                let outcome = rust_game::grader::grade(&ex, &file, timeout)?;
                // Update progress similar to CLI engine
                let mut prog = rust_game::persistence::load().unwrap_or_default();
                let entry = prog.exercises.entry(ex.meta.id.clone()).or_default();
                entry.attempts += 1;
                if outcome.passed {
                    entry.completed = true;
                }
                rust_game::persistence::save(&prog)?;
                if outcome.passed {
                    Ok("All tests passed 🎉".to_string())
                } else {
                    let mut s = String::new();
                    if !outcome.stdout.is_empty() { s.push_str("stdout:\n"); s.push_str(&outcome.stdout); }
                    if !outcome.stderr.is_empty() { s.push_str("\nstderr:\n"); s.push_str(&outcome.stderr); }
                    if s.is_empty() { s = "Some tests failed".to_string(); }
                    Ok(s)
                }
            })();
            let msg = match out { Ok(s) => s, Err(e) => format!("{e}") };
            // On success, auto-select next lesson if available
            if msg.starts_with("All tests passed") {
                if let Ok(list) = rust_game::exercise::load_all(&state.lessons_root) {
                    if let Some((idx, _)) = list.iter().enumerate().find(|(_, e)| Some(&e.meta.id) == state.selected_id.as_ref()) {
                        if idx + 1 < list.len() {
                            state.selected_id = Some(list[idx + 1].meta.id.clone());
                            state.last_output = format!("{}\n→ Next unlocked: {}", msg, list[idx + 1].meta.id);
                            return;
                        }
                    }
                }
            }
            state.last_output = msg;
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();
        // Keyboard shortcut: F5 refreshes lesson list
        if ctx.input(|i| i.key_pressed(egui::Key::F5)) {
            Self::refresh_list(&mut state);
            state.last_output = "Lessons refreshed".to_string();
        }

        egui::TopBottomPanel::top("top").frame(
            egui::Frame::default().fill(egui::Color32::from_rgb(30, 34, 40)).inner_margin(egui::Margin::symmetric(12.0, 8.0))
        ).show(ctx, |ui| {
            ui.columns(3, |columns| {
                columns[0].horizontal(|ui| {
                    ui.heading("RustLearn");
                    ui.add_space(16.0);
                    ui.label(egui::RichText::new("Beautiful. Fast. Safe.").italics().weak());
                });
                columns[1].with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("Resources", |ui| {
                            if ui.button("Rust Book").clicked() { let _ = webbrowser::open("https://doc.rust-lang.org/book/"); ui.close_menu(); }
                            if ui.button("Rust by Example").clicked() { let _ = webbrowser::open("https://doc.rust-lang.org/rust-by-example/"); ui.close_menu(); }
                            if ui.button("Rustlings").clicked() { let _ = webbrowser::open("https://github.com/rust-lang/rustlings"); ui.close_menu(); }
                            if ui.button("Exercism – Rust").clicked() { let _ = webbrowser::open("https://exercism.org/tracks/rust"); ui.close_menu(); }
                            if ui.button("Cheats.rs").clicked() { let _ = webbrowser::open("https://cheats.rs/"); ui.close_menu(); }
                            if ui.button("Rust Design Patterns").clicked() { let _ = webbrowser::open("https://rust-unofficial.github.io/patterns/"); ui.close_menu(); }
                            if ui.button("Awesome Rust").clicked() { let _ = webbrowser::open("https://github.com/rust-unofficial/awesome-rust"); ui.close_menu(); }
                            if ui.button("Tokio – Async").clicked() { let _ = webbrowser::open("https://tokio.rs/"); ui.close_menu(); }
                            if ui.button("Bevy – Game Engine").clicked() { let _ = webbrowser::open("https://bevyengine.org/"); ui.close_menu(); }
                            if ui.button("Rust Course").clicked() { let _ = webbrowser::open("https://www.youtube.com/watch?v=rQ_J9WH6CGk"); ui.close_menu(); }
                        });
                    });
                    egui::ComboBox::from_id_source("theme_selector")
                        .selected_text("Theme")
                        .show_ui(ui, |ui| {
                            if ui.selectable_label(false, "Dark").clicked() { let mut v = egui::Visuals::dark(); v.window_rounding = 8.0.into(); v.panel_fill = egui::Color32::from_rgb(22,24,28); ctx.set_visuals(v); }
                            if ui.selectable_label(false, "Light").clicked() { let mut v = egui::Visuals::light(); v.window_rounding = 8.0.into(); v.panel_fill = egui::Color32::from_rgb(245,246,248); ctx.set_visuals(v); }
                            if ui.selectable_label(false, "Ocean").clicked() { let mut v = egui::Visuals::dark(); v.override_text_color = Some(egui::Color32::from_rgb(208, 230, 255)); v.panel_fill = egui::Color32::from_rgb(10, 25, 47); ctx.set_visuals(v); }
                        });
                    if ui.button("Refresh").on_hover_text("Reload lessons (F5)").clicked() { Self::refresh_list(&mut state); }
                });
                columns[2].with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::menu::bar(ui, |ui| {
                        let p = rust_game::persistence::load().ok();
                        let (logged_in, label) = match &p {
                            Some(pp) if pp.current_user_id.is_some() => {
                                let name = pp.current_username.clone().unwrap_or_else(|| "User".into());
                                let avatar = pp.avatar.clone().unwrap_or_else(|| "🦀".into());
                                (true, format!("{} {}", avatar, name))
                            }
                            _ => (false, "Log in".to_string()),
                        };
                        ui.menu_button(WidgetText::from(label), |ui| {
                            if logged_in {
                                ui.label("Status: Logged in");
                                if ui.button("Fresh Start (Clear All)").clicked() {
                                    let _ = rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::ClearAll);
                                    state.last_output = "All local progress and attempts cleared".to_string();
                                }
                                if ui.button("Logout").clicked() {
                                    if let Ok(mut pp) = rust_game::persistence::load() {
                                        pp.current_user_id = None; pp.current_username = None; let _ = rust_game::persistence::save(&pp);
                                    }
                                    state.last_output = "Logged out".to_string();
                                    state.auth_password.clear();
                                    ui.close_menu();
                                }
                            } else {
                                ui.label("Status: Not logged in");
                                ui.horizontal(|ui| { ui.label("Username"); ui.text_edit_singleline(&mut state.auth_username); });
                                ui.horizontal(|ui| {
                                    ui.label("Password");
                                    let show_now = state.auth_show_password;
                                    let mut te = egui::TextEdit::singleline(&mut state.auth_password);
                                    if !show_now { te = te.password(true); }
                                    ui.add(te);
                                    ui.checkbox(&mut state.auth_show_password, "Show");
                                });
                                if ui.button("Login").clicked() {
                                    match rust_game::storage::authenticate(&state.auth_username, &state.auth_password) {
                                        Ok(Some(u)) => {
                                            let mut pp = rust_game::persistence::load().unwrap_or_default();
                                            pp.current_user_id = Some(u.id);
                                            pp.current_username = Some(u.username);
                                            let _ = rust_game::persistence::save(&pp);
                                            state.last_output = format!("Logged in as {}", pp.current_username.clone().unwrap_or_default());
                                            state.auth_password.clear();
                                            ui.close_menu();
                                        }
                                        Ok(None) => { state.last_output = "Invalid username or password".to_string(); }
                                        Err(e) => { state.last_output = format!("Login error: {e}"); }
                                    }
                                }
                                if ui.button("Register").clicked() {
                                    if !state.auth_username.is_empty() && !state.auth_password.is_empty() {
                                        let _ = rust_game::storage::register_user(&state.auth_username, &state.auth_password);
                                        if let Ok(Some(u)) = rust_game::storage::authenticate(&state.auth_username, &state.auth_password) {
                                            let mut pp = rust_game::persistence::load().unwrap_or_default();
                                            pp.current_user_id = Some(u.id);
                                            pp.current_username = Some(u.username);
                                            let _ = rust_game::persistence::save(&pp);
                                            state.last_output = format!("Registered and logged in as {}", pp.current_username.clone().unwrap_or_default());
                                            state.auth_password.clear();
                                            ui.close_menu();
                                        }
                                    }
                                }
                            }
                        });
                    });
                    ui.add_space(8.0);
                    let mut lr = state.lessons_root.clone();
                    ui.add(egui::TextEdit::singleline(&mut lr).hint_text("lessons path"));
                    if lr != state.lessons_root { state.lessons_root = lr; }
                    if let Ok(p) = rust_game::persistence::load() {
                        ui.separator();
                        ui.label(egui::RichText::new(p.avatar.clone().unwrap_or_else(|| "🦀".to_string())).size(18.0));
                        ui.label(egui::RichText::new(format!("{} ({})", p.display_name.clone().unwrap_or_else(|| "Player".into()), p.total_points)).monospace());
                        ui.menu_button("Profile", |ui| {
                            let mut prog = p.clone();
                            let mut name = prog.display_name.clone().unwrap_or_else(|| "Player".to_string());
                            ui.label("Display name");
                            ui.text_edit_singleline(&mut name);
                            prog.display_name = Some(name);
                            ui.label("Avatar");
                            let mut av = prog.avatar.clone().unwrap_or_else(|| "🦀".to_string());
                            ui.text_edit_singleline(&mut av);
                            prog.avatar = Some(av);
                            ui.label("Theme");
                            let mut theme = prog.theme.clone().unwrap_or_else(|| "Dark".to_string());
                            egui::ComboBox::from_id_source("theme_profile").selected_text(theme.clone()).show_ui(ui, |ui| {
                                for t in ["Dark","Light","Ocean"] { ui.selectable_value(&mut theme, t.to_string(), t); }
                            });
                            prog.theme = Some(theme);
                            if ui.button("Save").clicked() { let _ = rust_game::persistence::save(&prog); ui.close_menu(); }
                        });
                    }
                });
            });
        });

        egui::SidePanel::left("left").resizable(true).min_width(280.0).frame(
            egui::Frame::default().fill(egui::Color32::from_rgb(24, 26, 31)).inner_margin(egui::Margin::same(10.0))
        ).show(ctx, |ui| {
            ui.heading("Exercises");
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut state.filter_text).hint_text("Search lessons..."));
                ui.toggle_value(&mut state.show_only_incomplete, egui::RichText::new("Incomplete").small());
            });
            ui.add_space(6.0);
            let progress = rust_game::persistence::load().ok();
            let filtered: Vec<(usize, (String, String, String))> = state
                .exercises
                .iter().cloned().enumerate()
                .filter(|(_, (id, title, _))| {
                    let q = state.filter_text.to_lowercase();
                    if q.is_empty() { true } else { id.to_lowercase().contains(&q) || title.to_lowercase().contains(&q) }
                })
                .collect();
            egui::ScrollArea::vertical().auto_shrink([false;2]).show(ui, |ui| {
                let mut items = filtered;
                fn diff_order(d: &str) -> u8 { match d { "beginner" => 0, "intermediate" => 1, _ => 2 } }
                fn slug<'a>(id: &'a str) -> &'a str { id.rsplit('/').next().unwrap_or(id) }
                fn slug_rank(diff: &str, slug: &str) -> u32 {
                    let (beginner, intermediate, advanced): (&[&str], &[&str], &[&str]) = (
                        &[
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
                        ],
                        &[
                            "lifetimes_longest",
                            "generics_asref_maxlen",
                            "result_map_errors",
                            "iterators_combinators",
                            "iterators_ownership",
                            "rc_refcell",
                            "thiserror",
                        ],
                        &[
                            "arc_mutex_counter",
                            "arc_rwlock_counter",
                            "channels_mpsc",
                            "tokio_mpsc",
                            "tokio_timeout_race",
                            "traits_dyn_dispatch",
                        ],
                    );
                    let list: &[&str] = match diff { "beginner" => beginner, "intermediate" => intermediate, _ => advanced };
                    list.iter().position(|s| s == &slug).map(|i| i as u32).unwrap_or(999)
                }
                items.sort_by(|a,b| {
                    let da = diff_order(&a.1 .2); let db = diff_order(&b.1 .2);
                    if da != db { return da.cmp(&db); }
                    let ra = slug_rank(&a.1 .2, slug(&a.1 .0));
                    let rb = slug_rank(&b.1 .2, slug(&b.1 .0));
                    if ra != rb { return ra.cmp(&rb); }
                    a.1 .0.cmp(&b.1 .0)
                });
                let mut current: Option<String> = None;
                for (index, (id, title, diff)) in items.into_iter().map(|(i, v)| (i, v)) {
                    if current.as_deref() != Some(&diff) {
                        current = Some(diff.clone());
                        ui.separator();
                        ui.label(egui::RichText::new(current.clone().unwrap()).small().strong());
                    }
                    let completed = progress.as_ref().and_then(|p| p.exercises.get(&id)).map(|ep| ep.completed).unwrap_or(false);
                    if state.show_only_incomplete && completed { continue; }
                    let locked = !Self::is_unlocked(&state, &id);
                    let check = if completed { "✓" } else if locked { "🔒" } else { "•" };
                    let is_selected = state.selected_id.as_ref() == Some(&id);
                    let row = ui.add_sized([ui.available_width(), 28.0], egui::SelectableLabel::new(is_selected, format!("{}  {}", check, title)));
                    if row.clicked() {
                        if !locked { state.selected_id = Some(id.clone()); }
                        let logged_in = rust_game::persistence::load().ok().and_then(|p| p.current_user_id).is_some();
                        if !logged_in {
                            state.last_output = "User must be logged in to attempt this".to_string();
                            state.show_output_tab = true;
                        }
                    }
                    ui.horizontal(|ui| {
                        let badge = match diff.as_str() {
                            "beginner" => egui::RichText::new("BEGINNER").color(egui::Color32::from_rgb(80, 200, 120)),
                            "intermediate" => egui::RichText::new("INTERMEDIATE").color(egui::Color32::from_rgb(255, 170, 0)),
                            _ => egui::RichText::new("ADVANCED").color(egui::Color32::from_rgb(255, 90, 90)),
                        };
                        ui.label(badge);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(id).small().weak());
                        });
                    });
                    ui.separator();
                }
            });
        });

        egui::CentralPanel::default().frame(egui::Frame::default().fill(egui::Color32::from_rgb(26, 29, 35)).inner_margin(egui::Margin::same(12.0))).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(!state.show_output_tab, "Details").clicked() { state.show_output_tab = false; }
                if ui.selectable_label(state.show_output_tab, "Output").clicked() { state.show_output_tab = true; }
                ui.separator();
                ui.label("Text size");
                let mut scale = state.text_scale;
                if ui.add(egui::Slider::new(&mut scale, 0.8..=1.6).show_value(false)).changed() {
                    state.text_scale = scale;
                    let mut style = (*ctx.style()).clone();
                    style.text_styles.iter_mut().for_each(|(_, f)| f.size *= scale);
                    ctx.set_style(style);
                    if let Ok(mut p) = rust_game::persistence::load() {
                        p.text_scale = Some(scale);
                        let _ = rust_game::persistence::save(&p);
                    }
                }
            });
            ui.separator();
            if !state.show_output_tab {
                egui::ScrollArea::vertical().auto_shrink([false;2]).show(ui, |ui| {
                    ui.heading("Details");
                    if let Some(sel) = &state.selected_id {
                        if let Ok(list) = rust_game::exercise::load_all(&state.lessons_root) {
                            if let Some(ex) = list.into_iter().find(|e| &e.meta.id == sel) {
                                ui.label(format!("Title: {}", ex.meta.title));
                                ui.label(format!("Difficulty: {}", ex.meta.difficulty));
                                if let Some(h) = ex.meta.hint.as_deref() { ui.label(format!("Hint: {h}")); }
                                let can_view = {
                                    if let Ok(p) = rust_game::persistence::load() {
                                        p.exercises
                                            .get(&ex.meta.id)
                                            .map(|ep| ep.attempts > 0)
                                            .unwrap_or(false)
                                    } else {
                                        false
                                    }
                                };
                                if let Some(sol) = ex.solution_rs.clone() {
                                    ui.separator();
                                    if ui.add_enabled(can_view, egui::Button::new("View Solution")).clicked() {
                                        let preview = std::fs::read_to_string(&sol)
                                            .map(|s| {
                                                let limited: String = s.lines().take(60).collect::<Vec<_>>().join("\n");
                                                format!("Solution: {}\n{}{}", sol.display(), limited, if s.lines().count() > 60 { "\n..." } else { "" })
                                            })
                                            .unwrap_or_else(|e| format!("Failed to load solution: {e}"));
                                        state.last_output = preview;
                                        state.show_output_tab = true;
                                    }
                                    ui.label(egui::RichText::new(format!("Solution available: {}", sol.display())).small().weak());
                                }
                                if let Some(exp) = ex.explanation_md.clone() {
                                    ui.separator();
                                    if ui.add_enabled(can_view, egui::Button::new("Open Explanation in Output")).clicked() {
                                        state.last_output = std::fs::read_to_string(&exp).unwrap_or_else(|_| "Failed to load explanation".to_string());
                                        state.show_output_tab = true;
                                    }
                                    ui.label(egui::RichText::new(format!("Source: {}", exp.display())).small().weak());
                                }
                                if let Ok(p) = rust_game::persistence::load() {
                                    if let Some(ep) = p.exercises.get(&ex.meta.id) {
                                        ui.label(format!("Attempts: {}", ep.attempts));
                                        ui.label(format!("Completed: {}", ep.completed));
                                    }
                                }
                            }
                        }
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    ui.heading("Output");
                    let has_sel = state.selected_id.is_some();
                    let logged_in = rust_game::persistence::load().ok().and_then(|p| p.current_user_id).is_some();
                    let locked = if let Some(sel) = &state.selected_id { !Self::is_unlocked(&state, sel) } else { true };
                    if ui.add_enabled(has_sel && !locked && logged_in, egui::Button::new("Start")).clicked() {
                        if let Some(id) = state.selected_id.clone() {
                            let res = (|| -> Result<()> {
                                rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::Start { id: id.clone() })?;
                                rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::Open { id })?;
                                Ok(())
                            })();
                            if let Err(e) = res { state.last_output = format!("{e}"); }
                        }
                    }
                    if ui.add_enabled(has_sel && logged_in, egui::Button::new("Open")).clicked() {
                        if let Some(id) = state.selected_id.clone() {
                            let _ = rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::Open { id });
                        }
                    }
                    if ui.add_enabled(has_sel, egui::Button::new("Next ▶")).clicked() {
                        if let Some(cur) = state.selected_id.clone() {
                            if let Some(next_id) = Self::next_unlocked_id(&state, &cur) {
                                state.selected_id = Some(next_id);
                            } else {
                                state.last_output = "No further unlocked lesson".to_string();
                            }
                        }
                    }
                });
                egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut state.last_output).font(egui::TextStyle::Monospace).desired_rows(18).desired_width(f32::INFINITY));
                });
            }
            if let Some(sel) = &state.selected_id {
                if let Ok(list) = rust_game::exercise::load_all(&state.lessons_root) {
                    if let Some(ex) = list.into_iter().find(|e| &e.meta.id == sel) {
                        ui.label(format!("Title: {}", ex.meta.title));
                        ui.label(format!("Difficulty: {}", ex.meta.difficulty));
                        if let Some(h) = ex.meta.hint.as_deref() { ui.label(format!("Hint: {h}")); }
                        if let Ok(Some(qz)) = rust_game::quiz::load_quiz(&ex.root) {
                            ui.separator();
                            ui.label(egui::RichText::new(qz.title).strong());
                            for (qi, q) in qz.questions.iter().enumerate() {
                                ui.label(format!("Q{}: {}", qi + 1, q.prompt));
                                let mut selected: Option<usize> = None;
                                for (i, opt) in q.options.iter().enumerate() {
                                    if ui.selectable_label(false, opt).clicked() { selected = Some(i); }
                                }
                                if let Some(i) = selected {
                                    let correct = i == q.answer_index;
                                    let msg = if correct { "✅ Correct!" } else { "❌ Try again." };
                                    ui.label(msg);
                                    if correct {
                                        if let Ok(mut p) = rust_game::persistence::load() {
                                            let e = p.exercises.entry(ex.meta.id.clone()).or_default();
                                            e.quiz_completed = true;
                                            let _ = rust_game::persistence::save(&p);
                                        }
                                    }
                                }
                                ui.add_space(8.0);
                            }
                        }
                        if let Ok(p) = rust_game::persistence::load() {
                            if let Some(ep) = p.exercises.get(&ex.meta.id) {
                                ui.label(format!("Attempts: {}", ep.attempts));
                                ui.label(format!("Completed: {}", ep.completed));
                                if let Some(b) = ep.best_duration_secs { ui.label(format!("Best time: {}s", b)); }
                                if let Some(l) = ep.last_duration_secs { ui.label(format!("Last time: {}s", l)); }
                                if ep.points_earned > 0 { ui.label(format!("Points: {}", ep.points_earned)); }
                            }
                            if !p.badges.is_empty() {
                                ui.separator();
                                ui.label("Badges:");
                                for b in &p.badges { ui.label(format!("🏅 {}", b)); }
                            }
                        }
                        if let Ok(path) = ex.working_file() {
                            ui.separator();
                            ui.label("Edit your solution here:");
                            ui.monospace(path.display().to_string());
                        }
                    }
                }
            }
            ui.separator();
            ui.heading("Actions");
            ui.horizontal(|ui| {
                let has_sel = state.selected_id.is_some();
                let logged_in = rust_game::persistence::load().ok().and_then(|p| p.current_user_id).is_some();
                let locked = if let Some(sel) = &state.selected_id { !Self::is_unlocked(&state, sel) } else { true };
                if ui.add_enabled(has_sel && !locked && logged_in, egui::Button::new("Start")).clicked() {
                    if let Some(id) = state.selected_id.clone() {
                        let res = (|| -> Result<()> {
                            let ex = rust_game::engine::Command::Start { id };
                            rust_game::engine::run(state.lessons_root.clone(), ex)?;
                            let id = state.selected_id.clone().unwrap();
                            rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::Open { id })?;
                            Ok(())
                        })();
                        if let Err(e) = res { state.last_output = format!("{e}"); }
                    }
                }
                if ui.add_enabled(has_sel && logged_in, egui::Button::new("Open")).clicked() {
                    if let Some(id) = state.selected_id.clone() {
                        let res = (|| -> Result<()> {
                            let ex = rust_game::engine::Command::Open { id };
                            rust_game::engine::run(state.lessons_root.clone(), ex)?;
                            Ok(())
                        })();
                        if let Err(e) = res { state.last_output = format!("{e}"); }
                    }
                }
                if ui.add_enabled(has_sel && logged_in, egui::Button::new("Reset Solution")).clicked() {
                    if let Some(id) = state.selected_id.clone() {
                        let res = (|| -> Result<()> {
                            rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::Reset { id: id.clone() })?;
                            rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::Start { id: id.clone() })?;
                            rust_game::engine::run(state.lessons_root.clone(), rust_game::engine::Command::Open { id })?;
                            Ok(())
                        })();
                        if let Err(e) = res { state.last_output = format!("{e}"); }
                    }
                }
                if ui.add_enabled(has_sel && logged_in, egui::Button::new("Reveal File")).clicked() {
                    if let Some(id) = state.selected_id.clone() {
                        let out = (|| -> Result<String> {
                            let ex = rust_game::exercise::load_all(&state.lessons_root)?
                                .into_iter().find(|e| e.meta.id == id)
                                .ok_or_else(|| anyhow::anyhow!("Exercise not found"))?;
                            let path = ex.working_file()?;
                            let _ = rust_game::util::reveal_in_file_manager(&path);
                            Ok(format!("Working file: {}", path.display()))
                        })();
                        state.last_output = match out { Ok(s) => s, Err(e) => format!("{e}") };
                    }
                }
                if ui.add_enabled(has_sel && logged_in, egui::Button::new("Check")).clicked() {
                    Self::run_engine_check(&mut state);
                }
                if ui.add_enabled(has_sel && logged_in, egui::Button::new("Reset")).clicked() {
                    if let Some(id) = state.selected_id.clone() {
                        let res = (|| -> Result<()> {
                            let ex = rust_game::engine::Command::Reset { id };
                            rust_game::engine::run(state.lessons_root.clone(), ex)?;
                            Ok(())
                        })();
                        if let Err(e) = res { state.last_output = format!("{e}"); }
                    }
                }
                if ui.add_enabled(has_sel, egui::Button::new("Hint")).clicked() {
                    if let Some(id) = state.selected_id.clone() {
                        let out = (|| -> Result<String> {
                            let ex = rust_game::exercise::load_all(&state.lessons_root)?
                                .into_iter().find(|e| e.meta.id == id)
                                .ok_or_else(|| anyhow::anyhow!("Exercise not found"))?;
                            Ok(ex.meta.hint.unwrap_or_else(|| "No hint available.".to_string()))
                        })();
                        state.last_output = match out { Ok(s) => s, Err(e) => format!("{e}") };
                    }
                }
                if ui.add_enabled(has_sel, egui::Button::new("Next ▶")).clicked() {
                    if let Some(cur) = state.selected_id.clone() {
                        if let Some(next_id) = Self::next_unlocked_id(&state, &cur) {
                            state.selected_id = Some(next_id);
                        } else {
                            state.last_output = "No further unlocked lesson".to_string();
                        }
                    }
                }
            });
        });
        egui::TopBottomPanel::bottom("bottom").frame(
            egui::Frame::default().fill(egui::Color32::from_rgb(30, 34, 40)).inner_margin(egui::Margin::symmetric(12.0, 8.0))
        ).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.small("© RustLearn");
                ui.add_space(12.0);
                if let Ok(list) = rust_game::exercise::load_all(&state.lessons_root) {
                    let total = list.len();
                    if let Ok(p) = rust_game::persistence::load() {
                        let done = p.exercises.values().filter(|e| e.completed).count();
                        let badge = match (done * 100) / (total.max(1)) {
                            0..=24 => "Novice",
                            25..=49 => "Bronze",
                            50..=74 => "Silver",
                            75..=99 => "Gold",
                            _ => "Platinum",
                        };
                        ui.small(format!("Progress: {done}/{total} – Badge: {badge}"));
                    }
                }
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Rust Learn GUI",
        options,
        Box::new(|cc| Box::new(GuiApp::new(cc))),
    ) {
        eprintln!("GUI error: {e}");
    }
}