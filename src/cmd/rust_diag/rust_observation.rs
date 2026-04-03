use anyhow::{/*bail,*/ Context, Result};
use serde::Deserialize;
use std::fs::{self, File};
use std::process::{Command, Stdio};
use crate::cmd::rust_diag::{push_tasks, Task};

pub mod init;

const CONFIG_PATH: &str = "./.x-do/rust-diag-observation.toml";

#[derive(Deserialize)]
struct ObsConfig {
    #[serde(rename = "cargo-fmt")] cargo_fmt: Option<Vec<Task>>,
    #[serde(rename = "cargo-check")] cargo_check: Option<Vec<Task>>,
    #[serde(rename = "cargo-clippy")] cargo_clippy: Option<Vec<Task>>,
    #[serde(rename = "cargo-audit")] cargo_audit: Option<Vec<Task>>,
    #[serde(rename = "cargo-doc")] cargo_doc: Option<Vec<Task>>,
    #[serde(rename = "cargo-build")] cargo_build: Option<Vec<Task>>,
    #[serde(rename = "cargo-test")] cargo_test: Option<Vec<Task>>,
    #[serde(rename = "cargo-package")] cargo_package: Option<Vec<Task>>,
    #[serde(rename = "cargo-verify-project")] cargo_verify: Option<Vec<Task>>,
}

pub fn handle(arg: Option<&str>) -> Result<()> {
    init::ensure_config()?;
    let content = fs::read_to_string(CONFIG_PATH).context("Failed to read config")?;
    let raw: ObsConfig = toml::from_str(&content).context("Failed to parse TOML")?;

    let mut tasks = Vec::new();
    // Mapowanie sekcji TOML na bazowe komendy systemowe
    push_tasks(&mut tasks, raw.cargo_fmt, "cargo", &["fmt", "--check"]);
    push_tasks(&mut tasks, raw.cargo_check, "cargo", &["check"]);
    push_tasks(&mut tasks, raw.cargo_clippy, "cargo", &["clippy"]);
    push_tasks(&mut tasks, raw.cargo_audit, "cargo", &["audit"]);
    push_tasks(&mut tasks, raw.cargo_doc, "cargo", &["doc"]);
    push_tasks(&mut tasks, raw.cargo_build, "cargo", &["build"]);
    push_tasks(&mut tasks, raw.cargo_test, "cargo", &["test"]);
    push_tasks(&mut tasks, raw.cargo_package, "cargo", &["package"]);
    push_tasks(&mut tasks, raw.cargo_verify, "cargo", &["verify-project"]);

    // Sortowanie według pola `order`
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
                .context("Task not found in configuration")?;
            run_task(task)
        }
    }
}

fn list_tasks(tasks: &[Task]) {
    println!("📋 【ENG】 Observation Tasks / 【POL】 Zadania Obserwacji:");
    for t in tasks {
        println!("  - {:<4} | [{:02}] {}", t.id, t.order, t.name);
    }
}

fn run_task(task: &Task) -> Result<()> {
    println!("🩺 Wykonywanie: {} ({})", task.name, task.id);
    
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
    if !status.success() {
        println!("⚠️  Problem wykryty w: {}", task.name);
    } else {
        println!("✅ OK.");
    }
    Ok(())
}