//! SQLite-backed storage for users and attempts

use anyhow::{Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Attempt {
    pub id: i64,
    pub user_id: i64,
    pub lesson_id: String,
    pub passed: bool,
    pub duration_secs: Option<u64>,
    pub timestamp: i64,
}

fn db_path() -> Result<PathBuf> {
    let mut p = crate::persistence::progress_path()?;
    p.pop();
    p.push("app.db");
    Ok(p)
}

pub fn open() -> Result<Connection> {
    let path = db_path()?;
    if let Some(dir) = path.parent() { std::fs::create_dir_all(dir)?; }
    let conn = Connection::open(path)?;
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            display_name TEXT,
            avatar TEXT
        );
        CREATE TABLE IF NOT EXISTS attempts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            lesson_id TEXT NOT NULL,
            passed INTEGER NOT NULL,
            duration_secs INTEGER,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );
        "#,
    )?;
    Ok(conn)
}

pub fn register_user(username: &str, password: &str) -> Result<User> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    let conn = open()?;
    conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
        params![username, hash],
    )?;
    let id = conn.last_insert_rowid();
    Ok(User { id, username: username.to_string(), password_hash: String::from("<redacted>"), display_name: None, avatar: None })
}

pub fn authenticate(username: &str, password: &str) -> Result<Option<User>> {
    let conn = open()?;
    let row = conn
        .query_row(
            "SELECT id, username, password_hash, display_name, avatar FROM users WHERE username = ?1",
            params![username],
            |r| {
                Ok(User {
                    id: r.get(0)?,
                    username: r.get(1)?,
                    password_hash: r.get(2)?,
                    display_name: r.get(3).ok(),
                    avatar: r.get(4).ok(),
                })
            },
        )
        .optional()?;
    if let Some(mut u) = row {
        if bcrypt::verify(password, &u.password_hash).unwrap_or(false) {
            u.password_hash = String::from("<redacted>");
            return Ok(Some(u));
        }
    }
    Ok(None)
}

pub fn upsert_profile(user_id: i64, display_name: Option<String>, avatar: Option<String>) -> Result<()> {
    let conn = open()?;
    conn.execute(
        "UPDATE users SET display_name = ?1, avatar = ?2 WHERE id = ?3",
        params![display_name, avatar, user_id],
    )?;
    Ok(())
}

pub fn record_attempt(user_id: i64, lesson_id: &str, passed: bool, duration_secs: Option<u64>, timestamp: i64) -> Result<()> {
    let conn = open()?;
    conn.execute(
        "INSERT INTO attempts (user_id, lesson_id, passed, duration_secs, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![user_id, lesson_id, if passed {1} else {0}, duration_secs.map(|v| v as i64), timestamp],
    )?;
    Ok(())
}

pub fn attempts_for_user(user_id: i64) -> Result<Vec<Attempt>> {
    let conn = open()?;
    let mut stmt = conn.prepare("SELECT id, user_id, lesson_id, passed, duration_secs, timestamp FROM attempts WHERE user_id = ?1 ORDER BY timestamp DESC")?;
    let rows = stmt.query_map(params![user_id], |r| {
        Ok(Attempt {
            id: r.get(0)?,
            user_id: r.get(1)?,
            lesson_id: r.get(2)?,
            passed: {
                let v: i64 = r.get(3)?;
                v != 0
            },
            duration_secs: r.get::<_, Option<i64>>(4)?.map(|v| v as u64),
            timestamp: r.get(5)?,
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}

pub fn clear_attempts_for_user(user_id: i64) -> Result<()> {
    let conn = open()?;
    conn.execute("DELETE FROM attempts WHERE user_id = ?1", params![user_id])?;
    Ok(())
}