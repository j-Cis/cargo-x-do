use std::{
	io::{Result, Write, stdout},
	process::Command,
};

use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
	execute,
	style::Stylize,
	terminal,
};

pub fn handle_run(is_dry_run: bool) { handle_run_and_build(is_dry_run); }

pub fn handle_build(is_dry_run: bool) { handle_run_and_build(is_dry_run); }

pub fn handle_run_and_build(is_dry_run: bool) -> bool {
	let (action, is_clear) = match render_run_build_grid() {
		Ok(Some(res)) => res,
		_ => return false, // b lub Esc -> wracamy do poziomu 1
	};

	if is_clear {
		#[cfg(target_os = "windows")]
		let _ = Command::new("cmd").args(["/C", "cls"]).status();
		#[cfg(not(target_os = "windows"))]
		let _ = Command::new("clear").status();
	}

	match action {
		// --- RUN ---
		RunBuildAction::DeskDebug => run_cmd("cargo", &["run"], is_dry_run),
		RunBuildAction::DeskRelease => run_cmd("cargo", &["run", "--release"], is_dry_run),
		RunBuildAction::WasmHostDebug => {
			ensure_wasm_target(is_dry_run);
			run_cmd("trunk", &["serve", "--address", "127.0.0.1", "--port", "8080", "--open"], is_dry_run);
		}
		RunBuildAction::WasmHostRelease => {
			ensure_wasm_target(is_dry_run);
			run_cmd("trunk", &["serve", "--release", "--address", "127.0.0.1", "--port", "8080", "--open"], is_dry_run);
		}
		RunBuildAction::WasmWifiDebug => {
			ensure_wasm_target(is_dry_run);
			print_ip();
			run_cmd("trunk", &["serve", "--address", "0.0.0.0", "--port", "8080"], is_dry_run);
		}
		RunBuildAction::WasmWifiRelease => {
			ensure_wasm_target(is_dry_run);
			print_ip();
			run_cmd("trunk", &["serve", "--release", "--address", "0.0.0.0", "--port", "8080"], is_dry_run);
		}

		// --- BUILD ---
		RunBuildAction::DeskBuildDebug => run_cmd("cargo", &["build"], is_dry_run),
		RunBuildAction::DeskBuildRelease => run_cmd("cargo", &["build", "--release"], is_dry_run),
		RunBuildAction::WasmBuildDebug => {
			ensure_wasm_target(is_dry_run);
			run_cmd("trunk", &["build"], is_dry_run);
		}
		RunBuildAction::WasmBuildRelease => {
			ensure_wasm_target(is_dry_run);
			run_cmd("trunk", &["build", "--release"], is_dry_run);
		}

		// --- POMOCNICZE ---
		RunBuildAction::CheckIp => print_ip(),
		RunBuildAction::TargetAddWasm => run_cmd("rustup", &["target", "add", "wasm32-unknown-unknown"], is_dry_run),
	}

	true // Komenda wykonana
}

enum RunBuildAction {
	DeskDebug,
	DeskRelease,
	WasmHostDebug,
	WasmHostRelease,
	WasmWifiDebug,
	WasmWifiRelease,
	DeskBuildDebug,
	DeskBuildRelease,
	WasmBuildDebug,
	WasmBuildRelease,
	CheckIp,
	TargetAddWasm,
}

fn render_run_build_grid() -> Result<Option<(RunBuildAction, bool)>> {
	let mut out = stdout();
	terminal::enable_raw_mode()?;
	execute!(out, cursor::Hide)?;

	let mut is_clear = false;
	let mut first_draw = true;
	let mut final_result = None;

	'menu_loop: loop {
		if !first_draw {
			execute!(out, cursor::MoveUp(14), terminal::Clear(terminal::ClearType::FromCursorDown))?;
		}
		first_draw = false;

		print!("🚀 run & build (uruchamianie i kompilacja)\r\n\r\n");

		print!(" {}                                {}\r\n\r\n", "▶️ RUN".bold(), "🏗️ BUILD".bold());
		print!(
			"  {} 🖥️ desk debug                     {} 🖥️ desk build debug\r\n",
			"[d]".yellow().bold(),
			"[D]".cyan().bold()
		);
		print!(
			"  {} 🌍 wasm host (127.0.0.1)         {} 🌍 wasm build debug\r\n",
			"[w]".yellow().bold(),
			"[W]".cyan().bold()
		);
		print!("  {} 📶 wasm wifi (0.0.0.0 + IP)\r\n", "[s]".yellow().bold());
		print!(
			"  {} ⚡ desk release                  {} ⚡ desk build release\r\n",
			"[r]".yellow().bold(),
			"[R]".cyan().bold()
		);
		print!(
			"  {} 🚀 wasm host release             {} 📦 wasm build release\r\n",
			"[e]".yellow().bold(),
			"[E]".cyan().bold()
		);
		print!("  {} 🔥 wasm wifi release (+ IP)\r\n\r\n", "[f]".yellow().bold());

		print!(
			"  {} 📡 check IP                      {} 🎯 target add wasm32\r\n",
			"[c]".yellow().bold(),
			"[t]".cyan().bold()
		);

		let clear_lbl = if is_clear { "🧽 clear: ON " } else { "🧽 clear: OFF" };
		let clear_key = if is_clear { "[z]".red().bold() } else { "[z]".yellow().bold() };
		print!("  {} {}\r\n", clear_key, clear_lbl);
		print!("❌  {}\r\n", "[Esc] / [b] 🔙 back    [q] 🔚 exit".dark_grey());

		out.flush()?;

		loop {
			if let Event::Key(key) = event::read()? {
				if key.kind != KeyEventKind::Press {
					continue;
				}

				// Całkowite wyjście z programu (q lub Ctrl+C)
				if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
					|| key.code == KeyCode::Char('q')
				{
					let _ = terminal::disable_raw_mode();
					let _ = execute!(stdout(), cursor::Show);
					std::process::exit(0);
				}

				// Powrót do poziomu 1 (Esc lub b)
				if key.code == KeyCode::Esc || key.code == KeyCode::Char('b') {
					break 'menu_loop;
				}

				match key.code {
					// Toggle clear
					KeyCode::Char('z') => {
						is_clear = !is_clear;
						break;
					}

					// RUN (lewa kolumna)
					KeyCode::Char('d') => {
						final_result = Some((RunBuildAction::DeskDebug, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('w') => {
						final_result = Some((RunBuildAction::WasmHostDebug, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('s') => {
						final_result = Some((RunBuildAction::WasmWifiDebug, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('r') => {
						final_result = Some((RunBuildAction::DeskRelease, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('e') => {
						final_result = Some((RunBuildAction::WasmHostRelease, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('f') => {
						final_result = Some((RunBuildAction::WasmWifiRelease, is_clear));
						break 'menu_loop;
					}

					// BUILD (prawa kolumna)
					KeyCode::Char('D') => {
						final_result = Some((RunBuildAction::DeskBuildDebug, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('W') => {
						final_result = Some((RunBuildAction::WasmBuildDebug, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('R') => {
						final_result = Some((RunBuildAction::DeskBuildRelease, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('E') => {
						final_result = Some((RunBuildAction::WasmBuildRelease, is_clear));
						break 'menu_loop;
					}

					// Dolne opcje
					KeyCode::Char('c') => {
						final_result = Some((RunBuildAction::CheckIp, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('t') => {
						final_result = Some((RunBuildAction::TargetAddWasm, is_clear));
						break 'menu_loop;
					}

					_ => {}
				}
			}
		}
	}

	terminal::disable_raw_mode()?;
	execute!(out, cursor::Show)?;
	print!("\r\n");

	Ok(final_result)
}

fn run_cmd(program: &str, args: &[&str], is_dry_run: bool) {
	if is_dry_run {
		println!("> [DRY RUN] 🔒 Uruchomiłbym: {} {}\r\n", program, args.join(" "));
		return;
	}
	println!("> Wykonuję: {} {}\r\n", program, args.join(" "));
	let _ = Command::new(program).args(args).status();
	crate::last_and_history::add_to_history(program, args);
}

fn ensure_wasm_target(is_dry_run: bool) {
	if is_dry_run {
		println!("> [DRY RUN] 🔒 Sprawdziłbym/dodał target wasm32-unknown-unknown");
		return;
	}
	let _ = Command::new("rustup").args(["target", "add", "wasm32-unknown-unknown"]).status();
}

fn print_ip() {
	println!("📡 Adresy sieciowe (LAN):");
	#[cfg(target_os = "windows")]
	{
		let out = Command::new("cmd").args(["/C", "ipconfig | findstr IPv"]).output();
		if let Ok(res) = out {
			print!("{}", String::from_utf8_lossy(&res.stdout));
		}
	}
	#[cfg(not(target_os = "windows"))]
	{
		let out = Command::new("sh").args(["-c", "hostname -I || ip route get 1"]).output();
		if let Ok(res) = out {
			print!("{}", String::from_utf8_lossy(&res.stdout));
		}
	}
	println!("👉 Wpisz w przeglądarce telefonu: http://<TWOJ_IP>:8080\r\n");
}
