use std::{
	fs,
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
use inquire::{Confirm, Text};

pub fn handle_git(is_dry_run: bool) -> bool { handle_project(is_dry_run) }

pub fn handle_cargo(is_dry_run: bool) -> bool { handle_project(is_dry_run) }

pub fn handle_project(is_dry_run: bool) -> bool {
	// DODANA LINIA:
	eprintln!(">>> WESZŁO DO HANDLE PROJECT");

	let (action, is_clear) = match render_project_grid() {
		Ok(Some(res)) => res,
		_ => {
			// DODANA LINIA:
			eprintln!(">>> RENDER GRID ZWRÓCIŁ BŁĄD LUB NONE!");
			return false;
		}
	};

	if is_clear {
		#[cfg(target_os = "windows")]
		let _ = Command::new("cmd").args(["/C", "cls"]).status();
		#[cfg(not(target_os = "windows"))]
		let _ = Command::new("clear").status();
	}

	match action {
		// --- RELEASE & TAGS ---
		ProjectAction::TagAndPush => tag_release_with_hash(is_dry_run),
		ProjectAction::BumpVersion => bump_cargo_version(is_dry_run),

		// --- GIT STATUS & INSPEKCJA ---
		ProjectAction::GitStatus => run_cmd("git", &["status", "-s"], is_dry_run),
		ProjectAction::GitLog => run_cmd("git", &["log", "--oneline", "--graph", "--decorate", "-n", "10"], is_dry_run),
		ProjectAction::GitDiff => run_cmd("git", &["diff"], is_dry_run),

		// --- BEZPIECZEŃSTWO I COFANIE ZMIAN ---
		ProjectAction::UnstageAll => run_cmd("git", &["restore", "--staged", "."], is_dry_run),
		ProjectAction::UndoCommitSoft => {
			if is_dry_run {
				println!("> [DRY RUN] 🔒 git reset --soft HEAD~1\r\n");
			} else if confirm("Cofnąć ostatni commit z zachowaniem plików w roboczym katalogu?") {
				run_cmd("git", &["reset", "--soft", "HEAD~1"], false);
			}
		}
		ProjectAction::UndoCommitHard => {
			if is_dry_run {
				println!("> [DRY RUN] 🔒 git reset --hard HEAD~1\r\n");
			} else if confirm("⚠️ BEZPOWROTNIE usunąć ostatni commit i wszystkie zmiany w plikach?") {
				run_cmd("git", &["reset", "--hard", "HEAD~1"], false);
			}
		}
		ProjectAction::GitStash => run_cmd("git", &["stash"], is_dry_run),
		ProjectAction::GitStashPop => run_cmd("git", &["stash", "pop"], is_dry_run),
		ProjectAction::CleanUntracked => {
			if is_dry_run {
				println!("> [DRY RUN] 🔒 git clean -fd\r\n");
			} else if confirm("⚠️ Usunąć wszystkie nieśledzone pliki i katalogi (git clean -fd)?") {
				run_cmd("git", &["clean", "-fd"], false);
			}
		}

		// --- CARGO OPS ---
		ProjectAction::CargoFix => {
			run_cmd("cargo", &["fix", "--allow-dirty", "--allow-no-vcs"], is_dry_run);
		}
		ProjectAction::CargoFmt => run_smart_fmt(is_dry_run),
		ProjectAction::CargoUpdate => run_cmd("cargo", &["update"], is_dry_run),
		ProjectAction::CargoClean => {
			if is_dry_run {
				println!("> [DRY RUN] 🔒 cargo clean\r\n");
			} else if confirm("⚠️ Usunąć wszystkie artefakty kompilacji (target/)?") {
				run_cmd("cargo", &["clean"], false);
			}
		}
		ProjectAction::CargoTree => run_cmd("cargo", &["tree", "--depth", "1"], is_dry_run),
	}

	true
}

enum ProjectAction {
	TagAndPush,
	BumpVersion,
	GitStatus,
	GitLog,
	GitDiff,
	UnstageAll,
	UndoCommitSoft,
	UndoCommitHard,
	GitStash,
	GitStashPop,
	CleanUntracked,
	CargoFix,
	CargoFmt,
	CargoUpdate,
	CargoClean,
	CargoTree,
}

fn render_project_grid() -> Result<Option<(ProjectAction, bool)>> {
	let mut out = stdout();
	terminal::enable_raw_mode()?;
	execute!(out, cursor::Hide)?;

	let mut is_clear = false;
	let mut first_draw = true;
	let mut final_result = None;

	'menu_loop: loop {
		if !first_draw {
			execute!(out, cursor::MoveUp(16), terminal::Clear(terminal::ClearType::FromCursorDown))?;
		}
		first_draw = false;

		print!("\r\n🌿 project (git & cargo)\r\n\r\n");

		print!("  {}                           {}\r\n", "🏷️ RELEASE & GIT".bold(), "📦 CARGO & CHANGES".bold());
		print!(
			"  {} 🏷️  tag + push (vX.X-hash)    {} 🔧 fix (--allow-dirty)\r\n",
			"[t]".yellow().bold(),
			"[f]".cyan().bold()
		);
		print!(
			"  {} 🔢 bump version Cargo.toml    {} 🧹 fmt (+nightly)\r\n",
			"[v]".yellow().bold(),
			"[m]".cyan().bold()
		);
		print!(
			"  {} 📊 status (short)             {} 🔄 update dependencies\r\n",
			"[s]".yellow().bold(),
			"[k]".cyan().bold()
		);
		print!(
			"  {} 📜 log (10 graf)              {} 🗑️  clean target/\r\n",
			"[l]".yellow().bold(),
			"[c]".cyan().bold()
		);
		print!(
			"  {} 🔍 diff roboczy               {} 🌲 dependency tree\r\n\r\n",
			"[d]".yellow().bold(),
			"[y]".cyan().bold()
		);

		print!("  {}                               {}\r\n", "⏪ ROLLBACK & STASH".bold(), "⚠️ CLEANUP".bold());
		print!(
			"  {} ↩️  unstage (restore staged)  {} ⚠️  clean untracked (-fd)\r\n",
			"[x]".yellow().bold(),
			"[X]".red().bold()
		);
		print!(
			"  {} ⏪ undo commit (soft HEAD~1)  {} 📥 stash (schowaj)\r\n",
			"[u]".yellow().bold(),
			"[p]".cyan().bold()
		);
		print!(
			"  {} ⚠️  undo commit (hard HEAD~1)  {} 📤 stash pop (przywróć)\r\n\r\n",
			"[U]".red().bold(),
			"[P]".cyan().bold()
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

				if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
					|| key.code == KeyCode::Char('q')
				{
					let _ = terminal::disable_raw_mode();
					let _ = execute!(stdout(), cursor::Show);
					std::process::exit(0);
				}

				if key.code == KeyCode::Esc || key.code == KeyCode::Char('b') {
					break 'menu_loop;
				}

				match key.code {
					// Toggle clear
					KeyCode::Char('z') => {
						is_clear = !is_clear;
						break;
					}

					// Release & Tags
					KeyCode::Char('t') => {
						final_result = Some((ProjectAction::TagAndPush, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('v') => {
						final_result = Some((ProjectAction::BumpVersion, is_clear));
						break 'menu_loop;
					}

					// Git inspection
					KeyCode::Char('s') => {
						final_result = Some((ProjectAction::GitStatus, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('l') => {
						final_result = Some((ProjectAction::GitLog, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('d') => {
						final_result = Some((ProjectAction::GitDiff, is_clear));
						break 'menu_loop;
					}

					// Rollback & Stash
					KeyCode::Char('x') => {
						final_result = Some((ProjectAction::UnstageAll, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('u') => {
						final_result = Some((ProjectAction::UndoCommitSoft, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('U') => {
						final_result = Some((ProjectAction::UndoCommitHard, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('X') => {
						final_result = Some((ProjectAction::CleanUntracked, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('p') => {
						final_result = Some((ProjectAction::GitStash, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('P') => {
						final_result = Some((ProjectAction::GitStashPop, is_clear));
						break 'menu_loop;
					}

					// Cargo ops
					KeyCode::Char('f') => {
						final_result = Some((ProjectAction::CargoFix, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('m') => {
						final_result = Some((ProjectAction::CargoFmt, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('k') => {
						final_result = Some((ProjectAction::CargoUpdate, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('c') => {
						final_result = Some((ProjectAction::CargoClean, is_clear));
						break 'menu_loop;
					}
					KeyCode::Char('y') => {
						final_result = Some((ProjectAction::CargoTree, is_clear));
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

// ==========================================
// LOGIKA SPECJALNA: TAGOWANIE I WERSJONOWANIE
// ==========================================

fn tag_release_with_hash(is_dry_run: bool) {
    let version = match read_cargo_version() {
        Some(v) => v,
        None => {
            eprintln!("🚨 Nie udało się odczytać wersji z Cargo.toml");
            return;
        }
    };

    // 1. WYŁĄCZAMY RAW MODE - przywracamy normalny terminal przed pobraniem tekstu
    let _ = terminal::disable_raw_mode();
    let _ = execute!(std::io::stdout(), cursor::Show);

    println!("\r\n📝 Wprowadź/wklej wiadomość commita (naciśnij Enter w pustej linii, aby zakończyć):");
    let mut lines = Vec::new();
    let stdin = std::io::stdin();

    loop {
        let mut buffer = String::new();
        if stdin.read_line(&mut buffer).is_err() {
            break;
        }
        let trimmed = buffer.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break; // Pusta linia (podwójny Enter) kończy wklejanie
        }
        lines.push(trimmed.to_string());
    }

    let commit_msg = lines.join("\n");

    if commit_msg.trim().is_empty() {
        println!("Wiadomość pusta, anulowano proces wydania.\r\n");
        return;
    }

    if is_dry_run {
        println!("> [DRY RUN] 🔒 Wykonałbym pełen łańcuch wydania:");
        println!("  git add .");
        println!("  git commit -m \"{}\"", commit_msg);
        println!("  $hash = git rev-parse --short HEAD");
        println!("  git tag -a \"v{}-$hash\" -m \"Release v{} hash: $hash\"", version, version);
        println!("  git push origin main --follow-tags");
        println!("  git push origin --tags\r\n");
        return;
    }

    println!("📥 Dodawanie plików (git add .)...");
    let _ = Command::new("git").args(["add", "."]).status();

    println!("📝 Zapisywanie commita (git commit)...");
    let commit_status = Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .status();

    if commit_status.is_err() || !commit_status.unwrap().success() {
        eprintln!("⚠️ Nie udało się utworzyć commita (być może nie ma żadnych zmian). Próbuję kontynuować tagowanie...");
    }

    let hash_out = Command::new("git").args(["rev-parse", "--short", "HEAD"]).output();
    let hash = match hash_out {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => {
            eprintln!("🚨 Błąd pobierania hasha z git rev-parse");
            return;
        }
    };

    let tag_name = format!("v{}-{}", version, hash);
    let tag_msg = format!("Release v{} hash: {}", version, hash);

    println!("🏷️ Tworzenie taga: {}", tag_name);
    let status_tag = Command::new("git")
        .args(["tag", "-a", &tag_name, "-m", &tag_msg])
        .status();

    if let Ok(st) = status_tag && st.success() {
        println!("🚀 Wysyłanie gałęzi głównej wraz z powiązanymi tagami (git push origin main --follow-tags)...");
        let _ = Command::new("git").args(["push", "origin", "main", "--follow-tags"]).status();

        println!("🚀 Weryfikacja wysłania wszystkich tagów (git push origin --tags)...");
        let _ = Command::new("git").args(["push", "origin", "--tags"]).status();

        crate::last_and_history::add_to_history("git", &["push", "origin", "main", "--follow-tags"]);
        crate::last_and_history::add_to_history("git", &["push", "origin", "--tags"]);
    } else {
        eprintln!("🚨 Nie udało się utworzyć taga (być może taki tag już istnieje).");
    }
}

fn bump_cargo_version(is_dry_run: bool) {
	let manifest_path = "Cargo.toml";
	let content = match fs::read_to_string(manifest_path) {
		Ok(c) => c,
		Err(_) => {
			eprintln!("🚨 Nie znaleziono pliku Cargo.toml");
			return;
		}
	};

	let current_version = match read_cargo_version() {
		Some(v) => v,
		None => {
			eprintln!("🚨 Nie znaleziono pola `version` w Cargo.toml");
			return;
		}
	};

	let suggested_version = suggest_next_version(&current_version);

	let new_version = match Text::new("Wprowadź nową wersję:").with_default(&suggested_version).prompt() {
		Ok(v) => v.trim().to_string(),
		Err(_) => {
			println!("Anulowano podbijanie wersji.");
			return;
		}
	};

	if new_version.is_empty() || new_version == current_version {
		println!("Wersja bez zmian.");
		return;
	}

	if is_dry_run {
		println!("> [DRY RUN] 🔒 Podmieniłbym w Cargo.toml: `{}` -> `{}`\r\n", current_version, new_version);
		return;
	}

	// Bezpieczna podmiana wyłącznie linii w sekcji [package]
	let mut replaced = false;
	let mut new_lines = Vec::new();

	for line in content.lines() {
		if !replaced
			&& line.trim().starts_with("version")
			&& let Some((k, _)) = line.split_once('=')
		{
			new_lines.push(format!("{}= \"{}\"", k, new_version));
			replaced = true;
		} else {
			new_lines.push(line.to_string());
		}
	}

	let mut output = new_lines.join("\n");
	if content.ends_with('\n') {
		output.push('\n');
	}

	match fs::write(manifest_path, output) {
		Ok(_) => println!("✅ Zaktualizowano Cargo.toml do wersji: {}\r\n", new_version),
		Err(e) => eprintln!("🚨 Błąd zapisu do Cargo.toml: {}\r\n", e),
	}
}

fn read_cargo_version() -> Option<String> {
	let content = fs::read_to_string("Cargo.toml").ok()?;
	for line in content.lines() {
		let trimmed = line.trim();
		if trimmed.starts_with("version")
			&& let Some((_, val)) = trimmed.split_once('=')
		{
			let clean = val.trim().trim_matches('"').trim_matches('\'');
			if !clean.is_empty() {
				return Some(clean.to_string());
			}
		}
	}
	None
}

fn suggest_next_version(ver: &str) -> String {
	// 1. Sprawdzamy czy to wersja deweloperska np. 0.1.0-dev.4
	if let Some((base, dev_num_str)) = ver.rsplit_once("-dev.")
		&& let Ok(num) = dev_num_str.parse::<u64>()
	{
		return format!("{}-dev.{}", base, num + 1);
	}

	// 2. Jeśli to standardowe SemVer X.Y.Z -> podbijamy patch
	if let Some((major_minor, patch_str)) = ver.rsplit_once('.')
		&& let Ok(patch) = patch_str.parse::<u64>()
	{
		return format!("{}.{}", major_minor, patch + 1);
	}

	// Fallback
	format!("{}-dev.1", ver)
}

// ==========================================
// NARZĘDZIA POMOCNICZE
// ==========================================

fn run_cmd(program: &str, args: &[&str], is_dry_run: bool) {
	if is_dry_run {
		println!("> [DRY RUN] 🔒 Uruchomiłbym: {} {}\r\n", program, args.join(" "));
		return;
	}
	println!("> Wykonuję: {} {}\r\n", program, args.join(" "));
	let _ = Command::new(program).args(args).status();
	crate::last_and_history::add_to_history(program, args);
}

fn run_smart_fmt(is_dry_run: bool) {
	if is_dry_run {
		println!("> [DRY RUN] 🔒 Uruchomiłbym: cargo +nightly fmt --all\r\n");
		return;
	}
	println!("> Uruchamiam formatting (+nightly)...");
	let status = Command::new("cargo").args(["+nightly", "fmt", "--all"]).status();
	if status.is_err() || !status.unwrap().success() {
		println!("⚠️ Nightly fmt niedostępne, używam standardowego `cargo fmt`...");
		let _ = Command::new("cargo").args(["fmt", "--all"]).status();
		crate::last_and_history::add_to_history("cargo", &["fmt", "--all"]);
	} else {
		crate::last_and_history::add_to_history("cargo", &["+nightly", "fmt", "--all"]);
	}
}

fn confirm(prompt: &str) -> bool { Confirm::new(prompt).with_default(false).prompt().unwrap_or(false) }
