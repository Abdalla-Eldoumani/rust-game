//! Utilities: data dir resolution, editor launching, formatting.

use anyhow::{Context, Result};
use std::{env, path::{Path, PathBuf}, process::Command};

pub fn data_dir() -> Result<PathBuf> {
    let base = dirs::home_dir().context("home dir not found")?;
    Ok(base.join(".rustlearn"))
}

pub fn open_in_editor(path: &PathBuf) -> Result<()> {
    if let Ok(editor) = env::var("EDITOR") {
        let mut parts = editor.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
        Command::new(cmd)
            .args(args)
            .arg(path)
            .status()
            .with_context(|| format!("launch editor {:?}", editor))?;
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").arg("/C").arg("start").arg(path).status()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).status()?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(path).status()?;
    }
    Ok(())
}

pub fn reveal_in_file_manager(path: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let arg = format!("/select,{}", path.display());
        Command::new("explorer").arg(arg).status()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg("-R").arg(path).status()?;
    }
    #[cfg(target_os = "linux")]
    {
        let dir = if path.is_file() { path.parent().unwrap_or(Path::new(".")) } else { path };
        Command::new("xdg-open").arg(dir).status()?;
    }
    Ok(())
}