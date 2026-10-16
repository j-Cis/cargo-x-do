use std::{fs, process::Command};

use inquire::Select;

const HISTORY_FILE: &str = ".cargo/.x-do-history.txt";

pub fn add_to_history(cmd: &str, args: &[&str]) {
	let _ = fs::create_dir_all(".cargo");
	let full_cmd = format!("{} {}", cmd, args.join(" "));

	let mut history = read_history();
	// Przenieś na górę, jeśli już istnieje
	history.retain(|x| x != &full_cmd);
	history.insert(0, full_cmd);

	// Zatrzymaj maksymalnie 20 ostatnich komend
	history.truncate(20);

	let _ = fs::write(HISTORY_FILE, history.join("\n"));
}

pub fn read_history() -> Vec<String> {
	if let Ok(content) = fs::read_to_string(HISTORY_FILE) {
		content.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
	} else {
		Vec::new()
	}
}

pub fn has_history() -> bool { !read_history().is_empty() }

pub fn handle_last(is_dry_run: bool) -> bool {
	let history = read_history();
	if let Some(last) = history.first() {
		execute_from_str(last, is_dry_run);
	} else {
		println!("🚫 Brak historii poleceń.");
	}
	true
}

pub fn handle_list(is_dry_run: bool) -> bool {
	let history = read_history();
	if history.is_empty() {
		println!("🚫 Brak historii poleceń.");
		return true;
	}

	match Select::new("📜 Wybierz polecenie do ponownego wykonania:", history).prompt() {
		Ok(choice) => {
			execute_from_str(&choice, is_dry_run);
		}
		Err(_) => println!("Anulowano."),
	}
	true
}

fn execute_from_str(cmd_str: &str, is_dry_run: bool) {
	let mut parts = cmd_str.split_whitespace();
	if let Some(prog) = parts.next() {
		let args: Vec<&str> = parts.collect();
		if is_dry_run {
			println!("> [DRY RUN] 🔒 Uruchomiłbym: {} {}", prog, args.join(" "));
		} else {
			println!("> Wykonuję: {} {}\r\n", prog, args.join(" "));
			let _ = Command::new(prog).args(&args).status();
			// Zaktualizuj historię
			add_to_history(prog, &args);
		}
	}
}
