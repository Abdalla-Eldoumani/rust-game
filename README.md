## Rust Game – Learn Rust by Solving Exercises

An interactive Rust learning environment with a desktop GUI. Work through curated lessons from beginner to advanced, edit your solution locally, and let the built‑in grader run tests and track your progress.

### Status
- This is a beta build and a work in progress.
- The current focus is the GUI application; some CLI features are not yet included.
- Lessons, ordering, and features (quizzes, badges, gating) may change.
- Lessons are NOT fully correct yet.

### Highlights
- **Curated lessons** organized by difficulty: `intro`, `intermediate`, `advanced`
- **Auto‑grading** in a sandboxed Cargo project with timeouts
- **Progress tracking** and simple scoring/badges stored locally
- **Desktop GUI** built with `eframe`/`egui`
- **Local first**: no network required to solve lessons

---

## Getting Started

### Prerequisites
- Rust toolchain with Cargo (install via `https://rustup.rs`)

### Run the GUI
```bash
cargo run --bin rust-game-gui
```

Use the left panel to browse lessons. Select a lesson, then use the Actions area to Start/Open/Check. The GUI also provides Fresh Start (Clear All) to wipe local progress and sandboxes.

Notes:
- You can change the lessons path via the top bar input. Default is `lessons/` in this repository.
- To attempt exercises, use the GUI’s Account menu to Register/Login. Accounts are stored locally (SQLite) and not sent anywhere.

---

## Using the GUI
1) Launch the app and optionally set the lessons path in the top bar.
2) Register or log in from the Account menu (required to Start/Open/Check).
3) Select a lesson in the left panel.
4) Click “Start” to create your working copy, then “Open” to open it in your editor.
5) Implement your solution in the working file and click “Check” to run tests.
6) Use Next ▶ to move to the next unlocked lesson.

---

## How It Works

### Repository layout
- `src/`
  - `engine.rs`: Orchestrates lesson loading, grading, progress, and commands
  - `grader.rs`: Creates a temporary Cargo project and runs tests with a timeout
  - `exercise.rs`: Lesson metadata and loader
  - `persistence.rs`: Local JSON progress and leaderboard storage
  - `storage.rs`: Optional SQLite for user accounts and attempt history
  - `quiz.rs`: Optional per‑lesson multiple‑choice quiz loader (`quiz.toml`)
  - `util.rs`: Data dir resolution, editor/file‑manager helpers
- `src/bin/gui.rs`: Desktop GUI built with `eframe`/`egui`
- `lessons/`: All lesson content organized by difficulty and topic

### Lesson structure
Each lesson directory contains at least:
- `exercise.toml` (metadata)
- `starter.rs` (your starting point)
- `tests.rs` (the grader runs these)

Optional files:
- `solution.rs` (official solution preview)
- `explanation.md` (short write‑up)
- `quiz.toml` (multiple‑choice questions)

Example `exercise.toml`:
```toml
title = "Intro: Variables"
difficulty = "beginner"      # beginner | intermediate | advanced
hint = "Use `let` and return 42."
timeout_secs = 15            # optional per‑exercise test timeout
```

### Your working copy
When you Start a lesson in the GUI, the project creates a personal working directory under your home folder:
- Windows: `C:\Users\<you>\ .rustlearn\work\<lesson_id>\lib.rs`
- macOS/Linux: `/home/<you>/.rustlearn/work/<lesson_id>/lib.rs`

Edit that `lib.rs` file; the grader runs tests against it. You can always re‑initialize with Reset in the GUI.

### Grading
`grader.rs` builds a tiny sandbox Cargo project per lesson:
- Copies your working `lib.rs` into the sandbox
- Rewrites `tests.rs` to import the sandbox crate and runs `cargo test`
- Enforces a per‑exercise timeout (default 15s) to avoid hangs

### Progress, points, and badges
- Local JSON file: `~/.rustlearn/progress.json` tracks attempts, completions, points, durations, and simple badges
- Local JSON: `~/.rustlearn/leaderboard.json` stores recent completions
- Optional SQLite database: `~/.rustlearn/app.db` stores users and attempt history when you log in via the GUI

Privacy/security:
- Passwords are stored as bcrypt hashes in the local SQLite DB
- No data is sent anywhere by default; everything is local to your machine

### Lesson ordering and locks
Lessons are gated within each difficulty tier by a curated order. Complete the preceding lesson in that tier to unlock the next.

---

## Creating New Lessons
1) Create a directory under `lessons/<tier>/<slug>/` (e.g. `lessons/intro/variables/`)
2) Add required files:
   - `exercise.toml` (see example above)
   - `starter.rs`: Provide the function signature(s) students should implement
   - `tests.rs`: Use `#[test]` functions; import functions using `crate::...`
3) Optional: `solution.rs`, `explanation.md`, `quiz.toml`

Grading tips:
- Keep tests deterministic and fast
- For async/concurrency lessons, consider timeouts and flakiness
- Prefer clear failure messages that teach

---

## Development
- Build: `cargo build`
- Test (unit and grader tests): `cargo test`
- Run GUI: `cargo run --bin rust-game-gui`

Key dependencies (selection):
- Engine/Content: `serde`, `serde_json`, `toml`, `walkdir`
- Grader: `wait-timeout`
- GUI: `eframe`, `egui`, `webbrowser`
- Persistence: `dirs`, `rusqlite` (bundled), `bcrypt`

Windows notes:
- Editor detection prefers `$EDITOR`. If not set, the app attempts OS‑specific fallbacks (e.g., `cmd /C start` / `explorer`).

---

## Troubleshooting
- “Working file not found”: Click Start in the GUI before Open or Check
- Long‑running tests: Review your code for deadlocks/infinite loops; tests time out by default
- Editor didn’t open: Set `$EDITOR` (e.g., `export EDITOR="code -w"`) or open the printed path manually
- Validation issues: Ensure each lesson has `exercise.toml`, `starter.rs`, and `tests.rs`

---

## License
This project is open source and available under the [MIT License](LICENSE).