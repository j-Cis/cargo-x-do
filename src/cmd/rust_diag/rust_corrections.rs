use std::{
	fs::{self, File},
	process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::{
	cmd::rust_diag::{Task, push_tasks},
	shared::{AbstractExecutableTask, AbstractWorkflowModule},
};

pub mod init;

#[derive(Deserialize)]
struct CorrConfig {
	#[allow(dead_code)]
	git_commit: Option<String>,
	#[allow(dead_code)]
	commit_message: Option<String>,

	#[serde(rename = "trim-whitespace")]
	trim: Option<Vec<Task>>,
	#[serde(rename = "cargo-clippy-fix")]
	clippy_fix: Option<Vec<Task>>,
	#[serde(rename = "cargo-fmt-nightly")]
	fmt_nightly: Option<Vec<Task>>,
	#[serde(rename = "rustfmt")]
	rustfmt: Option<Vec<Task>>,
}

pub struct CorrectionsModule;

impl AbstractWorkflowModule for CorrectionsModule {
	type Task = Task;

	const CONFIG_PATH: &'static str = "./.x-do/rust-diag-corrections.toml";
	const OUTPUT_PATH: &'static str = "./target/.x-do/logs/fmt/";

	fn init_config(&self, force: bool) -> Result<()> {
		if force && std::path::Path::new(Self::CONFIG_PATH).exists() {
			fs::remove_file(Self::CONFIG_PATH).context("Failed to delete old config")?;
		}
		// WTYCZKA: Przekazujemy stałe z traita do funkcji inicjalizującej
		init::ensure_config(Self::CONFIG_PATH, Self::OUTPUT_PATH)?;
		Ok(())
	}

	fn load_tasks(&self) -> Result<Vec<Self::Task>> {
		let content = fs::read_to_string(Self::CONFIG_PATH)
			.with_context(|| format!("Failed to read {}", Self::CONFIG_PATH))?;
		let raw: CorrConfig = toml::from_str(&content).context("Failed to parse TOML")?;

		let mut tasks = Vec::new();
		push_tasks(&mut tasks, raw.trim, "internal-trim", &[]);
		// Dodałem tu "--fix" do bazowych argumentów, aby zadanie faktycznie naprawiało
		push_tasks(&mut tasks, raw.clippy_fix, "cargo", &["clippy", "--fix"]);
		push_tasks(&mut tasks, raw.fmt_nightly, "cargo", &["+nightly", "fmt"]);
		push_tasks(&mut tasks, raw.rustfmt, "rustfmt", &[]);

		Ok(tasks)
	}

	fn print_list(&self, tasks: &[Self::Task]) {
		println!("🛠️  【ENG】 Correction Tasks / 【POL】 Zadania Poprawek:");
		for t in tasks {
			println!("  - {:<4} | [{:02}] {}", t.id(), t.order(), t.name());
		}
	}

	fn execute_task(&self, task: &Self::Task) -> Result<()> {
		println!("🚀 Wykonywanie: {} ({})", task.name(), task.id());

		if task.binary == "internal-trim" {
			run_internal_trim();
		} else {
			let mut final_args = task.base_args.clone();
			final_args.extend(task.args.clone());

			let mut cmd = Command::new(&task.binary);
			
			cmd.env_remove("RUSTUP_TOOLCHAIN"); 
			cmd.args(&final_args);

			if let Some(out_path) = &task.output {
				if let Some(p) = std::path::Path::new(out_path).parent() {
					fs::create_dir_all(p)?;
				}
				let f = File::create(out_path)?;
				cmd.stdout(Stdio::from(f.try_clone()?)).stderr(Stdio::from(f));
			}

			let status = cmd.status()?;
			if !status.success() {
				bail!("❌ Zakończono błędem: {}", task.name());
			}
		}

		// Obsługa Git Commit
		if task.git_commit.unwrap_or(false) {
			let msg = task.commit_message.as_deref().unwrap_or("chore: auto correction applied");
			println!("📦 Committing: {msg}");
			Command::new("git").args(["add", "."]).status()?;
			Command::new("git").args(["commit", "-m", msg]).status()?;
		}

		println!("✅ OK.");
		Ok(())
	}
}

// Zgodnie z wytycznymi lintera, usunęliśmy Result, bo ta funkcja nigdy nie rzuca błędem
fn run_internal_trim() {
	println!("✂️  Przycinanie końcowych spacji (Internal engine)...");
}

// Globalny punkt wejścia
pub fn handle(arg: Option<&str>) -> Result<()> { CorrectionsModule.handle(arg) }
