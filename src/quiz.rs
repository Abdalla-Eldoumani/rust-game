//! Simple lesson quiz loader

use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct QuizConfig {
    pub title: String,
    #[serde(default)]
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Question {
    pub prompt: String,
    pub options: Vec<String>,
    pub answer_index: usize,
}

pub fn load_quiz(dir: &Path) -> Result<Option<QuizConfig>> {
    let p = dir.join("quiz.toml");
    if !p.exists() { return Ok(None); }
    let data = fs::read_to_string(&p).with_context(|| format!("read quiz at {}", p.display()))?;
    let q: QuizConfig = toml::from_str(&data).context("parse quiz toml")?;
    Ok(Some(q))
}