//! Progress tracking in ~/.rustlearn/progress.json

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub exercises: HashMap<String, ExerciseProgress>,
    #[serde(default)]
    pub total_points: u32,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub text_scale: Option<f32>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub badges: Vec<String>,
    #[serde(default)]
    pub current_user_id: Option<i64>,
    #[serde(default)]
    pub current_username: Option<String>,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            exercises: HashMap::new(),
            total_points: 0,
            avatar: Some("🦀".to_string()),
            text_scale: Some(1.0),
            display_name: Some("Player".to_string()),
            theme: Some("Dark".to_string()),
            badges: Vec::new(),
            current_user_id: None,
            current_username: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseProgress {
    pub attempts: u32,
    pub completed: bool,
    #[serde(default)]
    pub quiz_completed: bool,
    #[serde(default)]
    pub first_started_at: Option<i64>,
    #[serde(default)]
    pub last_started_at: Option<i64>,
    #[serde(default)]
    pub completed_at: Option<i64>,
    #[serde(default)]
    pub best_duration_secs: Option<u64>,
    #[serde(default)]
    pub last_duration_secs: Option<u64>,
    #[serde(default)]
    pub points_earned: u32,
    #[serde(default)]
    pub feedback_helpful: Option<bool>,
}

impl Default for ExerciseProgress {
    fn default() -> Self { Self { attempts: 0, completed: false, quiz_completed: false, first_started_at: None, last_started_at: None, completed_at: None, best_duration_secs: None, last_duration_secs: None, points_earned: 0, feedback_helpful: None } }
}

pub fn progress_path() -> Result<PathBuf> {
    let dir = crate::util::data_dir()?;
    std::fs::create_dir_all(&dir).ok();
    Ok(dir.join("progress.json"))
}

pub fn load() -> Result<Progress> {
    let p = progress_path()?;
    if !p.exists() { return Ok(Progress::default()); }
    let data = fs::read_to_string(p)?;
    let prog: Progress = serde_json::from_str(&data).context("parse progress")?;
    Ok(prog)
}

pub fn save(progress: &Progress) -> Result<()> {
    let p = progress_path()?;
    let data = serde_json::to_string_pretty(progress)?;
    fs::write(p, data)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub avatar: Option<String>,
    pub lesson_id: String,
    pub points: u32,
    pub duration_secs: Option<u64>,
    pub timestamp: i64,
}

pub fn leaderboard_path() -> Result<PathBuf> {
    let dir = crate::util::data_dir()?;
    std::fs::create_dir_all(&dir).ok();
    Ok(dir.join("leaderboard.json"))
}

pub fn load_leaderboard() -> Result<Vec<LeaderboardEntry>> {
    let p = leaderboard_path()?;
    if !p.exists() { return Ok(Vec::new()); }
    let data = fs::read_to_string(p)?;
    let entries: Vec<LeaderboardEntry> = serde_json::from_str(&data).context("parse leaderboard")?;
    Ok(entries)
}

pub fn save_leaderboard(entries: &[LeaderboardEntry]) -> Result<()> {
    let p = leaderboard_path()?;
    let data = serde_json::to_string_pretty(entries)?;
    fs::write(p, data)?;
    Ok(())
}

pub fn add_leaderboard_entry(entry: LeaderboardEntry) -> Result<()> {
    let mut entries = load_leaderboard().unwrap_or_default();
    entries.push(entry);
    if entries.len() > 2000 { let drop_n = entries.len() - 2000; entries.drain(0..drop_n); }
    save_leaderboard(&entries)
}