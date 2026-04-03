use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::fs::{self, File};
use std::process::{Command, Stdio};
use crate::cmd::rust_diag::{push_tasks, Task};

pub mod init;

const CONFIG_PATH: &str = "./.x-do/rust-diag-corrections.toml";

#[derive(Deserialize)]
struct CorrConfig {
	#[allow(dead_code)]
    git_commit: Option<String>, 
    #[allow(dead_code)]
    commit_message: Option<String>,  
    
    #[serde(rename = "trim-whitespace")] trim: Option<Vec<Task>>,
    #[serde(rename = "cargo-clippy-fix")] clippy_fix: Option<Vec<Task>>,
    #[serde(rename = "cargo-fmt-nightly")] fmt_nightly: Option<Vec<Task>>,
    #[serde(rename = "rustfmt")] rustfmt: Option<Vec<Task>>,
}

pub fn handle(arg: Option<&str>) -> Result<()> {
    init::ensure_config()?;
    let content = fs::read_to_string(CONFIG_PATH)?;
    let raw: CorrConfig = toml::from_str(&content)?;

    let mut tasks = Vec::new();
    push_tasks(&mut tasks, raw.trim, "internal-trim", &[]);
    push_tasks(&mut tasks, raw.clippy_fix, "cargo", &["clippy"]);
    push_tasks(&mut tasks, raw.fmt_nightly, "cargo", &["+nightly", "fmt"]);
    push_tasks(&mut tasks, raw.rustfmt, "rustfmt", &[]);

    tasks.sort_by_key(|t| t.order);

    match arg {
        None => {
            list_tasks(&tasks);
            Ok(())
        }
        Some("--all") => {
            for task in tasks { run_task(&task)?; }
            Ok(())
        }
        Some(target) => {
            let task = tasks.iter().find(|t| t.id == target || t.name == target)
                .context("Task not found")?;
            run_task(task)
        }
    }
}

fn list_tasks(tasks: &[Task]) {
    println!("🛠️  【ENG】 Correction Tasks / 【POL】 Zadania Poprawek:");
    for t in tasks {
        println!("  - {:<4} | [{:02}] {}", t.id, t.order, t.name);
    }
}

fn run_task(task: &Task) -> Result<()> {
    println!("🚀 Wykonywanie: {} ({})", task.name, task.id);

    if task.binary == "internal-trim" {
        run_internal_trim()?;
    } else {
        let mut final_args = task.base_args.clone();
        final_args.extend(task.args.clone());

        let mut cmd = Command::new(&task.binary);
        cmd.args(&final_args);

        if let Some(out_path) = &task.output {
            if let Some(p) = std::path::Path::new(out_path).parent() { fs::create_dir_all(p)?; }
            let f = File::create(out_path)?;
            cmd.stdout(Stdio::from(f.try_clone()?)).stderr(Stdio::from(f));
        }

        let status = cmd.status()?;
        if !status.success() { bail!("❌ Zakończono błędem: {}", task.name); }
    }

    // Obsługa gita (per-task)
    if task.git_commit.unwrap_or(false) {
        let msg = task.commit_message.as_deref().unwrap_or("chore: auto correction applied");
        println!("📦 Committing: {}", msg);
        Command::new("git").args(["add", "."]).status()?;
        Command::new("git").args(["commit", "-m", msg]).status()?;
    }

    println!("✅ OK.");
    Ok(())
}

// Prosta, wbudowana funkcja do przycinania białych znaków
fn run_internal_trim() -> Result<()> {
    // W ramach MVP, symulujemy działanie lub wywołujemy skrypt
    // W prawdziwej implementacji Rusta można tu przejść po plikach w src/**/*.rs
    println!("✂️  Przycinanie końcowych spacji (Internal engine)...");
    Ok(())
}