//! Compile-and-test grader with timeout.

use crate::exercise::Exercise;
use crate::util;
use anyhow::{Context, Result};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

pub struct GradeOutcome {
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
}

pub fn grade(ex: &Exercise, user_src: &Path, timeout_secs: u64) -> Result<GradeOutcome> {
    let proj = sandbox_dir_for(ex)?;

    if !proj.exists() {
        fs::create_dir_all(&proj)?;
        write_cargo_toml(&proj)?;
    }

    let src_dir = proj.join("src");
    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(proj.join("tests"))?;

    fs::copy(user_src, src_dir.join("lib.rs")).with_context(|| "copy user code into sandbox")?;

    let raw_tests = fs::read_to_string(&ex.tests_rs)?;
    let rewritten = raw_tests.replace("crate::", "exercise_sandbox::");
    fs::write(proj.join("tests").join("exercise.rs"), rewritten)?;

    let mut cmd = Command::new("cargo");
    cmd.arg("test")
        .arg("--quiet")
        .current_dir(proj)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().context("spawn cargo test")?;
    let status = match child
        .wait_timeout(Duration::from_secs(timeout_secs))
        .context("waiting for test with timeout")?
    {
        Some(status) => status,
        None => {
            child.kill().ok();
            child.wait().ok();
            return Ok(GradeOutcome { passed: false, stdout: String::new(), stderr: format!("Timed out after {}s", timeout_secs) });
        }
    };

    let output = child.wait_with_output().context("collect output")?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(GradeOutcome { passed: status.success(), stdout, stderr })
}

fn write_cargo_toml(root: &Path) -> Result<()> {
    let cargo = r#"[package]
name = "exercise_sandbox"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1"
tokio = { version = "1", features = ["rt-multi-thread","macros","sync"] }
"#;
    fs::write(root.join("Cargo.toml"), cargo)?;
    Ok(())
}

fn sandbox_dir_for(ex: &Exercise) -> Result<std::path::PathBuf> {
    let base = util::data_dir()?.join("sandboxes");
    fs::create_dir_all(&base).ok();
    Ok(base.join(ex.meta.id.replace('/', "_")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ex(tmp: &std::path::Path, code: &str, tests: &str) -> Exercise {
        let root = tmp.join("lesson");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("starter.rs"), code).unwrap();
        fs::write(root.join("tests.rs"), tests).unwrap();
        fs::write(root.join("exercise.toml"), r#"
title = "x"
difficulty = "beginner"
"#).unwrap();

        Exercise {
            meta: serde_json::from_value(serde_json::json!({
              "id": "x/y",
              "title": "x",
              "difficulty": "beginner",
              "hint": null,
              "timeout_secs": 2u64
            })).unwrap(),
            root: root.clone(),
            starter_rs: root.join("starter.rs"),
            tests_rs: root.join("tests.rs"),
            solution_rs: None,
            explanation_md: None,
        }
    }

    #[test]
    fn passes_and_fails() {
        let tmp = tempdir().unwrap();
        let ex = make_ex(tmp.path(), "pub fn add(a:i32,b:i32)->i32{a+b}\n",
            r#"
#[test] fn ok(){ assert_eq!(crate::add(1,2),3); }
#[test] fn no(){ assert_ne!(crate::add(1,2),4); }
"#);
        let user = tmp.path().join("user.rs");
        fs::write(&user, "pub fn add(a:i32,b:i32)->i32{a+b}\n").unwrap();

        let out = grade(&ex, &user, 10).unwrap();
        assert!(out.passed, "{}", out.stderr);

        fs::write(&user, "pub fn add(a:i32,b:i32)->i32{41}\n").unwrap();
        let out = grade(&ex, &user, 10).unwrap();
        assert!(!out.passed);
    }
}