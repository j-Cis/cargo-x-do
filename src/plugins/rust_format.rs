// src\plugins\rust_checks.rs

use crate::shared::fs::DefaultSettings;
use anyhow::Result;
use anyhow::{Context, bail};
use std::fs::File;
use std::process::{Command, Stdio};

struct Settings;

// implementujemy trait
impl DefaultSettings for Settings {}

// stałe wewnątrz struct
impl Settings {
	pub const CONFIG_INIT: &'static str = r#" 
git_commit = "per-task" # "single", "none"
when_commit = "before" # "after", "combo"
message_before = "before: apply automatic fix/fmt"
message_after = "after: apply automatic fix/fmt"

[[trim-whitespace]]
args = []
id = "cA1"
name = "trim-whitespace"
order = 0
git_commit = true
commit_message = "before: apply automatic trim trailing whitespace"
description = "【ENG】 Trim Trailing Whitespace / 【POL】 Usuwanie końcowych spacji"

[[cargo-clippy-fix]]
args = ["--allow-dirty"]
id = "cB1"
name = "clippy-fix"
order = 1
output = "target/.x-do/rust/fmt/clippy-fix.log"
git_commit = true
commit_message = "before: apply clippy suggestions"
description = "【ENG】 Apply clippy fixes / 【POL】 Zastosuj sugestie clippy"

# [[cargo-fmt-nightly]]
# args = ["--", "--check"]
# id = "cC1"
# name = "fmt-check"
# order = 2
# output = "target/.x-do/rust/fmt/fmt-check.log"
# git_commit = true
# commit_message = "before: format with nightly fmt"
# description = "【ENG】 Nightly formatting / 【POL】 Formatowanie wersją nightly"

[[rustfmt]]
args = []
id = "cD1"
name = "rustfmt"
order = 3
output = "target/.x-do/rust/fmt/rustfmt.log"
git_commit = true
commit_message = "before: format with rustfmt"
description = "【ENG】 Direct rustfmt / 【POL】 Bezpośrednie rustfmt"
"#;

	pub const CONFIG_FILE: &'static str = "rust-format.toml";
	pub const DEFAULT_OUT: &'static str = "rust/";
	pub const MENU_HELPER: &'static str = r#"
▶️ cargo x-do (format || f)              
▶️ cargo x-do (format || f) --help              
▶️ cargo x-do (format || f) --init       
▶️ cargo x-do (format || f) --init-force 
▶️ cargo x-do (format || f) [id] 
▶️ cargo x-do (format || f) [name]        
▶️ cargo x-do (format || f) --all
"#;
	pub const ANCHORS: [&'static str; 3] =
		["[[trim-whitespace]]", "[[cargo-clippy-fix]]", "[[rustfmt]]"];
	pub const KEYS_BY_ANACHOR_LIST: [&'static str; 4] = ["id", "name", "description", "order"];
	pub const RESTORE_POINT: [&'static str; 4] =
		["git_commit", "when_commit", "message_before", "message_after"];
}

pub fn main_of_plugin(arg: Option<&str>) -> anyhow::Result<()> {
	Settings::init_if_not_exist(Settings::CONFIG_FILE, Settings::CONFIG_INIT);
	Settings::ensure_out_exist(Settings::DEFAULT_OUT);
	let restore_points =
		Settings::get_all_by_all_key(Settings::CONFIG_FILE, &Settings::RESTORE_POINT)?;

	match arg {
		None => {
			Settings::plot_anchors_list(
				Settings::CONFIG_FILE,
				&Settings::ANCHORS,
				&Settings::KEYS_BY_ANACHOR_LIST,
			)?;
		}
		Some("--help") | Some("-h") => {
			println!("{}", Settings::MENU_HELPER);
		}
		Some("--init") => {
			Settings::init_clear_safest(Settings::CONFIG_FILE, Settings::CONFIG_INIT);
		}
		Some("--init-force") => {
			Settings::init_clear_forced(Settings::CONFIG_FILE, Settings::CONFIG_INIT);
		}
		Some("--all") | Some("-a") => {
			let mut all_tasks = Vec::new();
			for &anchor in &Settings::ANCHORS {
				if let Ok(tasks) = Settings::get_all_by_anchor(Settings::CONFIG_FILE, anchor) {
					let clean_key = anchor.trim_matches(|c| c == '[' || c == ']');
					for mut task in tasks {
						if let toml::Value::Table(ref mut t) = task {
							t.insert(
								"__anchor".to_string(),
								toml::Value::String(clean_key.to_string()),
							);
						}
						all_tasks.push(task);
					}
				}
			}
			// Przekazujemy globalne opcje jako DRUGI ARGUMENT
			handle_engine_for_all(all_tasks, &restore_points)?;
		}
		Some(target) => {
			let mut found_task = None;
			for &anchor in &Settings::ANCHORS {
				if let Ok(mut val) =
					Settings::get_one_by_anchor_and_key(Settings::CONFIG_FILE, anchor, target, true)
						.or_else(|_| {
							Settings::get_one_by_anchor_and_key(
								Settings::CONFIG_FILE,
								anchor,
								target,
								false,
							)
						}) {
					if let toml::Value::Table(ref mut t) = val {
						let clean_key = anchor.trim_matches(|c| c == '[' || c == ']');
						t.insert(
							"__anchor".to_string(),
							toml::Value::String(clean_key.to_string()),
						);
					}
					found_task = Some(val);
					break;
				}
			}
			match found_task {
				Some(task) => handle_engine_for_one(task, &restore_points)?, // Drugi argument!
				None => bail!("❌ Nie znaleziono zadania (ID/Name): {}", target),
			}
		}
	}
	Ok(())
}

/// Pomocnicza funkcja do wywoływania Gita
fn run_git_commit(msg: &str) -> Result<()> {
	println!("📦 Committing: {}", msg);
	Command::new("git").args(["add", "."]).status()?;
	Command::new("git").args(["commit", "-m", msg]).status()?;
	Ok(())
}

/// Funkcja sortująca, zarządzająca Gitem dla trybu 'single' i odpalająca zadania
fn handle_engine_for_all(
	mut tasks: Vec<toml::Value>,
	options: &Vec<(String, toml::Value)>,
) -> Result<()> {
	tasks.sort_by_key(|t| t.get("order").and_then(|v| v.as_integer()).unwrap_or(999));

	let global_git = options
		.iter()
		.find(|(k, _)| k == "git_commit")
		.and_then(|(_, v)| v.as_str())
		.unwrap_or("none");
	let when = options
		.iter()
		.find(|(k, _)| k == "when_commit")
		.and_then(|(_, v)| v.as_str())
		.unwrap_or("before");

	// PRZYPADKI A, B, C (Warunki czasowe)
	let do_before = when == "before" || when == "combo";
	let do_after = when == "after" || when == "combo";

	// PRZYPADEK 2: single (Grupa - BEFORE)
	// Zabrania wykonania w zadaniach, działa tylko wokół grupy.
	if global_git == "single" && do_before {
		let msg = options
			.iter()
			.find(|(k, _)| k == "message_before")
			.and_then(|(_, v)| v.as_str())
			.unwrap_or("before: apply automatic fix/fmt");
		run_git_commit(msg)?;
	}

	// Wykonanie wszystkich zadań
	for task in tasks {
		handle_engine_for_one(task, options)?;
	}

	// PRZYPADEK 2: single (Grupa - AFTER)
	if global_git == "single" && do_after {
		let msg = options
			.iter()
			.find(|(k, _)| k == "message_after")
			.and_then(|(_, v)| v.as_str())
			.unwrap_or("after: apply automatic fix/fmt");
		run_git_commit(msg)?;
	}

	Ok(())
}

/// Funkcja wykonująca konkretne zadanie i zarządzająca Gitem TYLKO dla trybu 'per-task'
fn handle_engine_for_one(task: toml::Value, options: &Vec<(String, toml::Value)>) -> Result<()> {
	let name = task.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
	let id = task.get("id").and_then(|v| v.as_str()).unwrap_or("??");
	let output = task.get("output").and_then(|v| v.as_str());
	let anchor = task.get("__anchor").and_then(|v| v.as_str()).unwrap_or("");

	// Odczyt ustawień globalnych
	let global_git = options
		.iter()
		.find(|(k, _)| k == "git_commit")
		.and_then(|(_, v)| v.as_str())
		.unwrap_or("none");
	let when = options
		.iter()
		.find(|(k, _)| k == "when_commit")
		.and_then(|(_, v)| v.as_str())
		.unwrap_or("before");

	// Odczyt opcji konkretnego zadania
	let task_commit = task.get("git_commit").and_then(|v| v.as_bool()).unwrap_or(false);

	// PRZYPADKI A, B, C (Warunki czasowe)
	let do_before = when == "before" || when == "combo";
	let do_after = when == "after" || when == "combo";

	println!("📈 Wykonywanie: {} ({})", name, id);

	// PRZYPADEK 3: per-task (Zadanie - BEFORE)
	// Pozwala wykonać commit o ile task_commit nie jest false.
	if global_git == "per-task" && task_commit && do_before {
		let default_msg = options
			.iter()
			.find(|(k, _)| k == "message_before")
			.and_then(|(_, v)| v.as_str())
			.unwrap_or("before: task execution");
		let msg = task.get("commit_message").and_then(|v| v.as_str()).unwrap_or(default_msg);
		run_git_commit(msg)?;
	}

	// --- CZYSTE WYKONANIE ZADANIA ---
	if anchor == "trim-whitespace" {
		println!("✂️  Przycinanie końcowych spacji (Internal engine)...");
		// Tu logika trim
	} else {
		let mut binary = "cargo";
		let mut cmd_args = Vec::new();

		match anchor {
			"cargo-clippy-fix" => {
				cmd_args.push("clippy".to_string());
				cmd_args.push("--fix".to_string());
			}
			"rustfmt" => binary = "rustfmt",
			_ => cmd_args.push(anchor.replace("cargo-", "")),
		}

		if let Some(args_arr) = task.get("args").and_then(|v| v.as_array()) {
			for a in args_arr {
				if let Some(s) = a.as_str() {
					cmd_args.push(s.to_string());
				}
			}
		}

		let mut cmd = Command::new(binary);
		cmd.args(&cmd_args);

		if let Some(out_path) = output {
			if let Some(parent) = std::path::Path::new(out_path).parent() {
				std::fs::create_dir_all(parent)?;
			}
			let file = File::create(out_path)?;
			cmd.stdout(Stdio::from(file.try_clone()?)).stderr(Stdio::from(file));
		}

		let status =
			cmd.status().with_context(|| format!("❌ Błąd uruchamiania dla zadania {}", id))?;
		if !status.success() {
			bail!("❌ Zakończono błędem: {} ({})", name, id);
		}
	}

	// PRZYPADEK 3: per-task (Zadanie - AFTER)
	if global_git == "per-task" && task_commit && do_after {
		let default_msg = options
			.iter()
			.find(|(k, _)| k == "message_after")
			.and_then(|(_, v)| v.as_str())
			.unwrap_or("after: task execution");
		let msg = task.get("commit_message").and_then(|v| v.as_str()).unwrap_or(default_msg);
		run_git_commit(msg)?;
	}

	println!("✅ OK.");
	Ok(())
}
