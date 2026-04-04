// src\plugins\cargo_plot.rs

use std::{fs, path::Path, process::Command};

use anyhow::{Context, Result, bail};
use toml;

use crate::shared::fs::DefaultSettings;
struct Settings;

// implementujemy trait
impl DefaultSettings for Settings {}

// stałe wewnątrz struct
impl Settings {
	pub const CONFIG_INIT: &'static str = r#"
[[task]]
id = "p1"
name = "snapshot-rust"
description = "【ENG】 Standard Rust project snapshot / 【POL】 Standardowy zrzut projektu Rust"
input_folder = "."
output_folder = "./target/.x-do/code/"
patterns = [
    "./{.rustfmt,Cargo}.toml",
    "./{src,examples,tests}/**{/*.rs,/}",
    "./build.rs"
]
action = ["-m", "--save-archive"]
style = ["-b", "--lang", "en", "-v", "grid", "-u", "dec", "-s", "za-file-merge", "--no-emoji"]

[task.settings]
clean_before_run = true
silent = false
"#;
	pub const CONFIG_FILE: &'static str = "cargo-plot.toml";
	pub const DEFAULT_OUT: &'static str = "code/";
	pub const MENU_HELPER: &'static str = r#"
▶️ cargo x-do plot
▶️ cargo x-do plot --help
▶️ cargo x-do plot --init
▶️ cargo x-do plot --init-force
▶️ cargo x-do plot [id]
▶️ cargo x-do plot [name]
"#;

	pub const ANCHORS: [&'static str; 1] = ["[[task]]"];
	pub const KEYS_BY_ANACHOR_LIST: [&'static str; 3] = ["id", "name", "description"];
}

pub fn main_of_plugin(arg: Option<&str>) -> anyhow::Result<()> {
	Settings::init_if_not_exist(Settings::CONFIG_FILE, Settings::CONFIG_INIT);
	Settings::ensure_out_exist(Settings::DEFAULT_OUT);

	match arg {
		None => {
			// `cargo x-do plot`
			Settings::plot_anchors_list(
				Settings::CONFIG_FILE,
				&Settings::ANCHORS,
				&Settings::KEYS_BY_ANACHOR_LIST,
			)?;
		}
		Some("--help") | Some("-h") => {
			// `cargo x-do plot --help`
			println!("{}", Settings::MENU_HELPER);
		}
		Some("--init") => {
			// `cargo x-do plot --init`
			Settings::init_clear_safest(Settings::CONFIG_FILE, Settings::CONFIG_INIT);
		}
		Some("--init-force") => {
			// `cargo x-do plot --init-force`
			Settings::init_clear_forced(Settings::CONFIG_FILE, Settings::CONFIG_INIT);
		}
		Some(target) => {
			// `cargo x-do plot [id]` lub `cargo x-do plot [name]`
			// Najpierw próbujemy znaleźć po ID (true), jak nie znajdzie, szukamy po name (false)
			let task_value = match Settings::get_one_by_anchor_and_key(
				Settings::CONFIG_FILE,
				"[[task]]",
				target,
				true,
			) {
				Ok(val) => val,
				Err(_) => Settings::get_one_by_anchor_and_key(
					Settings::CONFIG_FILE,
					"[[task]]",
					target,
					false,
				)?,
			};

			// Przekazujemy surowy fragment TOML (toml::Value) do funkcji wykonawczej
			handle_engine(task_value)?;
		}
	}

	Ok(())
}

/// Funkcja wykonująca konkretne zadanie na podstawie wczytanego z TOML-a wycinka
fn handle_engine(task: toml::Value) -> Result<()> {
	// Wyciągamy podstawowe dane
	let id = task.get("id").and_then(|v| v.as_str()).unwrap_or("??");
	let name = task.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
	let description = task.get("description").and_then(|v| v.as_str()).unwrap_or("");

	// --- NOWA OBSŁUGA IO ---
	let io = task
		.get("io")
		.and_then(|v| v.as_array())
		.map(|arr| {
			arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>()
		})
		.unwrap_or_default();

	// Pobieramy indeks 0 jako input (domyślnie "."), indeks 1 jako output
	let input_folder = io.first().map(|s| s.as_str()).unwrap_or(".");
	let output_folder = io.get(1).map(|s| s.as_str()).unwrap_or("./target/.x-do/code/");
	// -----------------------

	let patterns = task
		.get("patterns")
		.and_then(|v| v.as_array())
		.map(|arr| {
			arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>()
		})
		.unwrap_or_default();

	let action = task
		.get("action")
		.and_then(|v| v.as_array())
		.map(|arr| {
			arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>()
		})
		.unwrap_or_default();

	let style = task
		.get("style")
		.and_then(|v| v.as_array())
		.map(|arr| {
			arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>()
		})
		.unwrap_or_default();

	let settings = task.get("settings").and_then(|v| v.as_table());

	let clean_before_run =
		settings.and_then(|t| t.get("clean_before_run")).and_then(|v| v.as_bool()).unwrap_or(true);

	let silent = settings.and_then(|t| t.get("silent")).and_then(|v| v.as_bool()).unwrap_or(false);

	if !silent {
		println!("📈 Generating task: {} ({})", name, id);
		if !description.is_empty() {
			println!("📖 {}", description);
		}
	}

	let out_path = Path::new(output_folder);
	if clean_before_run && out_path.exists() {
		if !silent {
			println!("🧹 Cleaning output folder: {}", output_folder);
		}
		fs::remove_dir_all(out_path)?;
	}
	fs::create_dir_all(out_path)?;

	// Budujemy argumenty dla cargo-plot
	let mut cmd_args = vec!["plot".to_string()];
	cmd_args.extend(style);
	cmd_args.extend(action);
	cmd_args.push("-o".to_string());
	cmd_args.push(output_folder.to_string());

	for p in patterns {
		cmd_args.push("-p".to_string());
		cmd_args.push(p);
	}

	if !silent {
		println!("⚙️  Executing: cargo {}", cmd_args.join(" "));
	}

	// Czyste, bezpośrednie wywołanie bez mieszania w środowisku
	let status = Command::new("cargo")
		.args(&cmd_args)
		.current_dir(input_folder)
		.status()
		.with_context(|| format!("Failed to execute cargo-plot for task {}", id))?;

	if !status.success() {
		bail!("❌ cargo-plot failed for task {} ({})", id, name);
	}

	if !silent {
		println!("✨ Done: {} ({})", name, id);
	}

	Ok(())
}
