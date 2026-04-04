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
[[cargo-fmt]] 
args = []
id = "oA1"
name = "check-fmt"
order = 0
output = "target/.x-do/rust/fmt-check.log"
description = "【ENG】 Check code formatting / 【POL】 Sprawdzenie formatowania kodu"

[[cargo-check]]
args = []
id = "oB1"
name = "check"
order = 1
output = "target/.x-do/rust/check.log"
description = "【ENG】 Compile check / 【POL】 Sprawdzenie kompilacji"

[[cargo-check]]
args = ["--examples"]
id = "oB2"
name = "check-examples"
order = 2
output = "target/.x-do/rust/check-examples.log"
description = "【ENG】 Check examples / 【POL】 Sprawdzenie przykładów"

[[cargo-check]]
args = ["--tests"]
id = "oB3"
name = "check-tests"
order = 3
output = "target/.x-do/rust/check-tests.log"
description = "【ENG】 Check tests / 【POL】 Sprawdzenie testów"

[[cargo-clippy]]
args = []
id = "oC1"
name = "clippy-basic"
order = 4
output = "target/.x-do/rust/clippy.log"
description = "【ENG】 Basic lints / 【POL】 Podstawowa analiza clippy"

[[cargo-clippy]]
args = ["--all-targets", "--all-features", "--", "-D", "warnings"]
id = "oC2"
name = "clippy-strict"
order = 5
output = "target/.x-do/rust/clippy-strict.log"
description = "【ENG】 Strict lints (deny warnings) / 【POL】 Rygorystyczne clippy (błędy na ostrzeżeniach)"

[[cargo-audit]]
args = []
id = "oF1"
name = "audit"
order = 12
output = "target/.x-do/rust/audit.log"
description = "【ENG】 Security audit / 【POL】 Audyt bezpieczeństwa"

[[cargo-doc]]
args = ["--no-deps",
	"--document-private-items",
	"--all-features"]
id = "oD1"
name = "doc"
order = 13
output = "target/.x-do/rust/doc.log"
description = "【ENG】 Generate documentation / 【POL】 Generowanie dokumentacji"

[[cargo-build]]
args = ["--release"]
id = "oG1"
name = "build-release"
order = 7
output = "target/.x-do/rust/build-release.log"
description = "【ENG】 Release build / 【POL】 Budowa wersji release"

[[cargo-build]]
args = []
id = "oG2"
name = "build-debug"
order = 6
output = "target/.x-do/rust/build-debug.log"
description = "【ENG】 Debug build / 【POL】 Budowa wersji debug"

[[cargo-test]]
args = ["--no-run"]
id = "oH1"
name = "test-no-run"
order = 8
output = "target/.x-do/rust/test-compile.log"
description = "【ENG】 Compile tests only / 【POL】 Kompilacja testów (bez uruchamiania)"

[[cargo-test]]
args = ["--verbose"]
id = "oH2"
name = "test-verbose"
order = 9
output = "target/.x-do/rust/test-verbose.log"
description = "【ENG】 Run tests (verbose) / 【POL】 Uruchomienie testów (szczegółowo)"

[[cargo-test]]
args = ["--doc"]
id = "oH3"
name = "test-doc"
order = 10
output = "target/.x-do/rust/test-doc.log"
description = "【ENG】 Test documentation examples / 【POL】 Testowanie przykładów w dokumentacji"

[[cargo-package]]
args = ["--list"]
id = "oI1"
name = "test-doc"
order = 11
output = "target/.x-do/rust/package-list.log"
description = "【ENG】 List package contents / 【POL】 Lista zawartości paczki"


[[cargo-verify-project]]
args = [ ]
id = "oJ1"
name = "verify"
order = 14
output = "target/.x-do/rust/verify.log"
description = "【ENG】 Verify Cargo.toml / 【POL】 Weryfikacja Cargo.toml"
"#;

	pub const CONFIG_FILE: &'static str = "rust-checks.toml";
	pub const DEFAULT_OUT: &'static str = "rust/";
	pub const MENU_HELPER: &'static str = r#"
▶️ cargo x-do (check || c)              
▶️ cargo x-do (check || c) --help              
▶️ cargo x-do (check || c) --init       
▶️ cargo x-do (check || c) --init-force 
▶️ cargo x-do (check || c) [id] 
▶️ cargo x-do (check || c) [name]        
▶️ cargo x-do (check || c) --all
"#;
	pub const ANCHORS: [&'static str; 9] = [
		"[[cargo-fmt]]",
		"[[cargo-check]]",
		"[[cargo-clippy]]",
		"[[cargo-audit]]",
		"[[cargo-doc]]",
		"[[cargo-build]]",
		"[[cargo-test]]",
		"[[cargo-package]]",
		"[[cargo-verify-project]]",
	];
	pub const KEYS_BY_ANACHOR_LIST: [&'static str; 4] = ["id", "name", "description", "order"];
}

pub fn main_of_plugin(arg: Option<&str>) -> anyhow::Result<()> {
	Settings::init_if_not_exist(Settings::CONFIG_FILE, Settings::CONFIG_INIT);
	Settings::ensure_out_exist(Settings::DEFAULT_OUT);

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

			handle_engine_for_all(all_tasks)?;
		}
		Some(target) => {
			// Używamy wyłącznie poprawnej metody: get_one_by_anchor_and_key
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
					// Znaleźliśmy! Dodajemy informację o pochodzeniu (anchor) i przerywamy szukanie
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
				Some(task) => handle_engine_for_one(task)?,
				None => bail!("❌ Nie znaleziono zadania (ID/Name): {}", target),
			}
		}
	}

	Ok(())
}

/// Funkcja sortująca i wykonująca wszystkie przekazane zadania
fn handle_engine_for_all(mut tasks: Vec<toml::Value>) -> Result<()> {
	// 1. Sortujemy zadania rosnąco według pola "order"
	tasks.sort_by_key(|t| t.get("order").and_then(|v| v.as_integer()).unwrap_or(999));

	// 2. Wykonujemy po kolei
	for task in tasks {
		// Jeśli zadanie rzuci błędem, bail! w handle_engine_for_one przerwie pętlę.
		handle_engine_for_one(task)?;
	}

	Ok(())
}

/// Funkcja wykonująca konkretne zadanie na podstawie wczytanego z TOML-a wycinka
fn handle_engine_for_one(task: toml::Value) -> Result<()> {
	let name = task.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
	let id = task.get("id").and_then(|v| v.as_str()).unwrap_or("??");
	let output = task.get("output").and_then(|v| v.as_str());

	// Odczytujemy anchor (wstrzyknięty wcześniej) by ustalić komendę (np. "cargo-clippy" -> "clippy")
	let anchor = task.get("__anchor").and_then(|v| v.as_str()).unwrap_or("cargo-check");
	let subcommand = anchor.replace("cargo-", "");

	println!("📈 Wykonywanie: {} ({})", name, id);

	// Budujemy argumenty dla Cargo
	let mut cmd_args = vec![subcommand];

	if let Some(args_arr) = task.get("args").and_then(|v| v.as_array()) {
		for a in args_arr {
			if let Some(s) = a.as_str() {
				cmd_args.push(s.to_string());
			}
		}
	}

	// Czyste, bezpośrednie wywołanie bez magicznych environment variables
	let mut cmd = Command::new("cargo");
	cmd.args(&cmd_args);

	// Przekierowanie logów do pliku określonego w `output`
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

	println!("✅ OK.");
	Ok(())
}
