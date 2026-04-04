use std::{
	fs::{self, File},
	process::{Command, Stdio},
};

use anyhow::{/*bail,*/ Context, Result};
use serde::Deserialize;

use crate::{
	cmd::rust_diag::{Task, push_tasks},
	shared::AbstractWorkflowModule,
};

pub mod init;

#[derive(Deserialize)]
struct ObsConfig {
	#[serde(rename = "cargo-fmt")]
	pub fmt: Option<Vec<Task>>,
	#[serde(rename = "cargo-check")]
	pub check: Option<Vec<Task>>,
	#[serde(rename = "cargo-clippy")]
	pub clippy: Option<Vec<Task>>,
	#[serde(rename = "cargo-audit")]
	pub audit: Option<Vec<Task>>,
	#[serde(rename = "cargo-doc")]
	pub doc: Option<Vec<Task>>,
	#[serde(rename = "cargo-build")]
	pub build: Option<Vec<Task>>,
	#[serde(rename = "cargo-test")]
	pub test: Option<Vec<Task>>,
	#[serde(rename = "cargo-package")]
	pub package: Option<Vec<Task>>,
	#[serde(rename = "cargo-verify-project")]
	pub verify: Option<Vec<Task>>,
}

pub struct ObservationModule;

impl AbstractWorkflowModule for ObservationModule {
	type Task = Task;

	const CONFIG_PATH: &'static str = "./.x-do/rust-diag-observation.toml";
	const OUTPUT_PATH: &'static str = "./target/.x-do/logs/err/";

	fn init_config(&self, force: bool) -> Result<()> {
		if force && std::path::Path::new(Self::CONFIG_PATH).exists() {
			fs::remove_file(Self::CONFIG_PATH).context("Failed to delete old config")?;
		}
		init::ensure_config(Self::CONFIG_PATH, Self::OUTPUT_PATH)?; // Wywołuje Twoją dotychczasową funkcję z init.rs
		Ok(())
	}

	fn load_tasks(&self) -> Result<Vec<Self::Task>> {
		let content = fs::read_to_string(Self::CONFIG_PATH)
			.with_context(|| format!("Failed to read {}", Self::CONFIG_PATH))?;
		let raw: ObsConfig = toml::from_str(&content).context("Failed to parse TOML")?;

		let mut tasks = Vec::new();
		push_tasks(&mut tasks, raw.fmt, "cargo", &["fmt", "--check"]);
		push_tasks(&mut tasks, raw.check, "cargo", &["check"]);
		push_tasks(&mut tasks, raw.clippy, "cargo", &["clippy"]);
		push_tasks(&mut tasks, raw.audit, "cargo", &["audit"]);
		push_tasks(&mut tasks, raw.doc, "cargo", &["doc"]);
		push_tasks(&mut tasks, raw.build, "cargo", &["build"]);
		push_tasks(&mut tasks, raw.test, "cargo", &["test"]);
		push_tasks(&mut tasks, raw.package, "cargo", &["package"]);
		push_tasks(&mut tasks, raw.verify, "cargo", &["verify-project"]);

		Ok(tasks)
	}

	fn print_list(&self, tasks: &[Self::Task]) {
		println!("📋 【ENG】 Observation Tasks / 【POL】 Zadania Obserwacji:");
		for t in tasks {
			println!("  - {:<4} | [{:02}] {}", t.id, t.order, t.name);
		}
	}

	fn execute_task(&self, task: &Self::Task) -> Result<()> {
		println!("🩺 Wykonywanie: {} ({})", task.name, task.id);

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
		if status.success() {
			println!("✅ OK.");
		} else {
			println!("⚠️  Problem wykryty w: {}", task.name);
		}
		Ok(())
	}
}

// Globalny punkt wejścia przekazuje wywołanie do trait-a
pub fn handle(arg: Option<&str>) -> Result<()> { ObservationModule.handle(arg) }
