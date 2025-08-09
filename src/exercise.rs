//! Exercise models and loader.

use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};
use anyhow::{Context, Result};
use walkdir::WalkDir;

#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseMeta {
    pub id: String,
    pub title: String,
    pub difficulty: String,
    pub hint: Option<String>,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Exercise {
    pub meta: ExerciseMeta,
    pub root: PathBuf,
    pub starter_rs: PathBuf,
    pub tests_rs: PathBuf,
    pub solution_rs: Option<PathBuf>,
    pub explanation_md: Option<PathBuf>,
}

impl Exercise {
    pub fn working_dir() -> Result<PathBuf> {
        let base = crate::util::data_dir()?.join("work");
        fs::create_dir_all(&base).ok();
        Ok(base)
    }

    pub fn working_file(&self) -> Result<PathBuf> {
        Ok(Self::working_dir()?.join(self.meta.id.replace('/', "_")).join("lib.rs"))
    }
}

pub fn load_all(lessons_root: &str) -> Result<Vec<Exercise>> {
    let mut out = Vec::new();

    for entry in WalkDir::new(lessons_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "exercise.toml")
    {
        let root = entry.path().parent().unwrap().to_path_buf();
        let meta_toml = fs::read_to_string(entry.path())
            .with_context(|| format!("Reading {:?}", entry.path()))?;
        #[derive(Deserialize)]
        struct PartialMeta { title: String, difficulty: String, hint: Option<String>, timeout_secs: Option<u64> }
        let pm: PartialMeta = toml::from_str(&meta_toml)
            .with_context(|| format!("Parsing {:?}", entry.path()))?;

        let rel_root = root.strip_prefix(Path::new(lessons_root)).unwrap_or(&root);
        let id = rel_root
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("/");

        let starter_rs = root.join("starter.rs");
        let tests_rs = root.join("tests.rs");
        let solution_rs = root.join("solution.rs");
        let explanation_md = root.join("explanation.md");
        if !starter_rs.exists() || !tests_rs.exists() {
            anyhow::bail!("Exercise {:?} missing starter.rs or tests.rs", root);
        }

        out.push(Exercise {
            meta: ExerciseMeta { id, title: pm.title, difficulty: pm.difficulty, hint: pm.hint, timeout_secs: pm.timeout_secs },
            root,
            starter_rs,
            tests_rs,
            solution_rs: if solution_rs.exists() { Some(solution_rs) } else { None },
            explanation_md: if explanation_md.exists() { Some(explanation_md) } else { None },
        });
    }

    out.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;

    #[test]
    fn loads_exercise() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("lessons").join("intro").join("vars");
        fs::create_dir_all(&root).unwrap();

        let mut meta = fs::File::create(root.join("exercise.toml")).unwrap();
        write!(meta, r#"
title = "Variables"
difficulty = "beginner"
hint = "Use let mut"
"#).unwrap();

        fs::write(root.join("starter.rs"), "pub fn answer()->i32{0}\n").unwrap();
        fs::write(root.join("tests.rs"), r#"
#[test] fn basic(){ assert_eq!(crate::answer(), 42); }
"#).unwrap();

        let xs = load_all(dir.path().join("lessons").to_str().unwrap()).unwrap();
        assert_eq!(xs.len(), 1);
        assert!(xs[0].starter_rs.exists());
        assert!(xs[0].tests_rs.exists());
        assert!(xs[0].meta.id.ends_with("intro/vars"));
    }
}