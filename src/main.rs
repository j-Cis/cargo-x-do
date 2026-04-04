mod init;
mod plugins;
mod shared;

use anyhow::Result;

fn main() -> Result<()> {
	init::ensure_workspace()?;
	// 1. Zbieramy argumenty
	let mut args: Vec<String> = std::env::args().collect();

	// 2. Obsługa subkomendy Cargo
	// Jeśli wywołano `cargo x-do plot`, args = ["ścieżka/do/bin", "x-do", "plot"]
	// Usuwamy "x-do", żeby na indeksie 1 zawsze była główna komenda.
	if args.get(1).map(String::as_str) == Some("x-do") {
		args.remove(1);
	}

	// Teraz:
	// args[0] to nazwa binarki
	// args[1] to komenda (np. "plot", "check", "err")
	// args[2] to argument dla pluginu (np. "--all", "p1", "--init")
	let command = args.get(1).map(String::as_str);
	let plugin_arg = args.get(2).map(String::as_str);

	// 3. Główny Router
	match command {
		Some("plot") | Some("print") | Some("p") => {
			plugins::cargo_plot::main_of_plugin(args.get(2).map(String::as_str))?
		}
		Some("check") | Some("c") | Some("err") | Some("e") => {
			plugins::rust_checks::main_of_plugin(plugin_arg)?
		}
		Some("format") | Some("fmt") | Some("fix") | Some("f") => {
			plugins::rust_format::main_of_plugin(plugin_arg)?
		}
		_ => print_help(),
	}
	Ok(())
}

fn print_help() {
	println!("Usage/Użycie: cargo x-do [plot|err|fix] <id|id,id|--all|--init>");
	println!("---");
	println!("【ENG】 plot: snapshots | err: observation | fix: corrections");
	println!("【POL】 plot: zrzuty    | err: obserwacja  | fix: poprawki");
	println!("---");
	println!("【ENG】 --init: force re-init | --all: run all | id,id: run multiple");
	println!("【POL】 --init: wymuś init   | --all: wszystko | id,id: wywołaj wiele");
	println!("---");
	println!("【ENG】 (empty): lists tasks | (id): runs specific task by ID or Name");
	println!("【POL】 (puste): lista zadań | (id): zadanie po ID lub nazwie");
}
