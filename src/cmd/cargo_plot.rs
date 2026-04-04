pub mod init;

use std::{fs, path::Path, process::Command};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

const REQUIRED_PLOT_VERSION: &str = "0.2.0";
const PLOT_CONFIG_PATH: &str = "./.x-do/cargo-plot.toml";

#[derive(Debug, Deserialize)]
struct PlotConfig {
	task: Vec<Task>,
}

#[derive(Debug, Deserialize)]
struct Task {
	name: String,
	id: String,
	description: String,
	input_folder: String,
	output_folder: String,
	patterns: Vec<String>,
	action: Vec<String>,
	style: Vec<String>,
	#[serde(default)]
	settings: Settings,
}

#[derive(Debug, Deserialize, Default)]
struct Settings {
	clean_before_run: bool,
	#[serde(default)]
	silent: bool,
}

/// 【ENG】 Main entry point for 'plot' module / 【POL】 Główny punkt wejścia dla modułu 'plot'
pub fn handle(task_id: Option<&str>) -> Result<()> {
	init::ensure_config()?;
	check_config()?;

	match task_id {
		Some(id) => run_task(id),
		None => list_tasks(),
	}
}

fn check_config() -> Result<()> {
	if !Path::new(PLOT_CONFIG_PATH).exists() {
		bail!(
			"❌ 【ENG】 Config file not found: {} / 【POL】 Nie znaleziono pliku konfiguracji: {}\n\
             💡 【ENG】 Create it to define your snapshot tasks / 【POL】 Utwórz go, aby zdefiniować zadania zrzutów.",
			PLOT_CONFIG_PATH,
			PLOT_CONFIG_PATH
		);
	}
	Ok(())
}

fn check_runner() -> Result<()> {
	let output = Command::new("cargo").args(["plot", "--version"]).output();

	match output {
		Ok(out) if out.status.success() => {
			let version_str = String::from_utf8_lossy(&out.stdout);
			if !version_str.contains("0.2") {
				println!(
					"⚠️ 【ENG】 Warning: cargo-plot version mismatch (expected v{}) / 【POL】 Ostrzeżenie: Niezgodność wersji cargo-plot (wymagana v{})",
					REQUIRED_PLOT_VERSION, REQUIRED_PLOT_VERSION
				);
			}
			Ok(())
		}
		_ => bail!(
			"❌ 【ENG】 Executor 'cargo-plot' not found / 【POL】 Nie znaleziono wykonawcy 'cargo-plot'\n\
             💡 【ENG】 Install it: / 【POL】 Zainstaluj go: cargo install cargo-plot --version {}",
			REQUIRED_PLOT_VERSION
		),
	}
}

fn list_tasks() -> Result<()> {
	let content = fs::read_to_string(PLOT_CONFIG_PATH)?;
	let config: PlotConfig = toml::from_str(&content)?;

	println!(
		"✅ 【ENG】 Config found / 【POL】 Konfiguracja znaleziona: {}",
		PLOT_CONFIG_PATH
	);
	println!("📋 【ENG】 Available PLOT tasks: / 【POL】 Dostępne zadania PLOT:");

	for t in config.task {
		println!("  - {:<5} | {}", t.id, t.name);
	}

	println!("\n👉 【ENG】 Use: / 【POL】 Użyj: cargo x-do plot [ID]");
	Ok(())
}

fn run_task(target: &str) -> Result<()> {
	check_runner()?;

	let content = fs::read_to_string(PLOT_CONFIG_PATH)?;
	let config: PlotConfig = toml::from_str(&content).with_context(|| {
		format!("【ENG】 Error parsing / 【POL】 Błąd parsowania: {}", PLOT_CONFIG_PATH)
	})?;

	let task =
		config.task.iter().find(|t| t.id == target || t.name == target).with_context(|| {
			format!(
				"【ENG】 Task '{}' not found / 【POL】 Zadanie '{}' nie istnieje",
				target, target
			)
		})?;

	println!(
		"🎨 【ENG】 Running task / 【POL】 Uruchamiam zadanie: {} | {}",
		task.id, task.name
	);
	if !task.description.is_empty() {
		println!("📖 {}", task.description);
	}

	let out_path = Path::new(&task.output_folder);
	if task.settings.clean_before_run && out_path.exists() {
		if !task.settings.silent {
			println!(
				"🧹 【ENG】 Cleaning output / 【POL】 Czyszczenie folderu wynikowego: {}",
				task.output_folder
			);
		}
		fs::remove_dir_all(out_path)?;
	}
	fs::create_dir_all(out_path)?;

	let mut cmd_args = vec!["plot".to_string()];
	cmd_args.extend(task.style.iter().cloned());
	cmd_args.extend(task.action.iter().cloned());
	cmd_args.push("-o".to_string());
	cmd_args.push(task.output_folder.clone());

	for p in &task.patterns {
		cmd_args.push("-p".to_string());
		cmd_args.push(p.clone());
	}

	if !task.settings.silent {
		println!("⚙️  【ENG】 Exec: / 【POL】 Wykonuję: cargo {}", cmd_args.join(" "));
	}

	let status = Command::new("cargo").args(&cmd_args).current_dir(&task.input_folder).status()?;

	if !status.success() {
		bail!("❌ 【ENG】 cargo-plot failed / 【POL】 cargo-plot zakończył się niepowodzeniem.");
	}

	println!("✨ 【ENG】 Done! / 【POL】 Gotowe!");
	Ok(())
}
