use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Stylize,
    terminal,
};
use std::fs;
use std::io::{stdout, Write, Result};
use std::path::Path;
use std::process::Command;

#[path = "../utils/dump_code_snapshot.rs"]
pub mod dump_code_snapshot;

pub fn handle_info(is_dry_run: bool) -> bool {
    let (action, is_clear) = match render_inspect_grid() {
        Ok(Some(res)) => res,
        _ => return false, // b / Esc -> wraca do menu głównego
    };

    if is_clear {
        // Twarde wyczyszczenie całego bufora konsoli systemowej (Windows/Unix)
        #[cfg(target_os = "windows")]
        let _ = Command::new("cmd").args(["/C", "cls"]).status();
        #[cfg(not(target_os = "windows"))]
        let _ = Command::new("clear").status();
    }

    match action {
        // --- EKRAN (małe litery) ---
        InspectAction::CheckExamples => run_cmd("cargo", &["check", "--examples"], is_dry_run),
        InspectAction::Check => run_cmd("cargo", &["check", "--all-targets"], is_dry_run),
        InspectAction::Clippy => run_cmd("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"], is_dry_run),
        InspectAction::Fmt => run_smart_fmt(false, is_dry_run),
        InspectAction::Test => run_cmd("cargo", &["test"], is_dry_run),
        InspectAction::WasmCheck => run_cmd("cargo", &["check", "--target", "wasm32-unknown-unknown"], is_dry_run),
        InspectAction::All => run_all_inspect(is_dry_run),
        InspectAction::SnapshotPreview => {
            if is_dry_run {
                println!("> [DRY RUN] 🔒 Wyświetliłbym podgląd struktury drzewa.\r\n");
            } else {
                dump_code_snapshot::preview_tree();
            }
        }

        // --- ZAPIS DO PLIKU / DUMP (wielkie litery) ---
        InspectAction::DumpCheckExamples => {
            run_cmd_and_save("cargo", &["check", "--examples"], "check_examples.dump.txt", is_dry_run)
        }
        InspectAction::DumpCheck => {
            run_cmd_and_save("cargo", &["check", "--all-targets"], "check.dump.txt", is_dry_run)
        }
        InspectAction::DumpClippy => {
            run_cmd_and_save("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"], "clippy.dump.txt", is_dry_run)
        }
        InspectAction::DumpFmtCheck => {
            run_cmd_and_save("cargo", &["fmt", "--all", "--", "--check"], "fmt_check.dump.txt", is_dry_run)
        }
        InspectAction::DumpTest => run_cmd_and_save("cargo", &["test"], "test.dump.txt", is_dry_run),
        InspectAction::DumpWasmCheck => {
            run_cmd_and_save("cargo", &["check", "--target", "wasm32-unknown-unknown"], "wasm_check.dump.txt", is_dry_run)
        }
        InspectAction::DumpAll => dump_all_logs(is_dry_run),
        InspectAction::SnapshotSave => {
            if is_dry_run {
                println!("> [DRY RUN] 🔒 Zapisałbym snapshot do pliku.\r\n");
            } else {
                dump_code_snapshot::generate();
            }
        }
    }
	true
}

pub fn handle_instant_snapshot(is_dry_run: bool) {
    if is_dry_run {
        println!("> [DRY RUN] 🔒 Zapisałbym snapshot do pliku.\r\n");
    } else {
        dump_code_snapshot::generate();
    }
}

enum InspectAction {
    CheckExamples,
    DumpCheckExamples,
    Check,
    DumpCheck,
    Clippy,
    DumpClippy,
    Fmt,
    DumpFmtCheck,
    Test,
    DumpTest,
    WasmCheck,
    DumpWasmCheck,
    All,
    DumpAll,
    SnapshotPreview,
    SnapshotSave,
}

fn render_inspect_grid() -> Result<Option<(InspectAction, bool)>> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;
    execute!(out, cursor::Hide)?;

    let mut is_clear = false;
    let mut first_draw = true;
    let mut final_result = None;

    'inspect_loop: loop {
        if !first_draw {
            execute!(
                out,
                cursor::MoveUp(14), // Dopasowane do ilości wypisywanych niżej wierszy
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )?;
        }
        first_draw = false;

        print!("\r\n🔍 diagnostyka & logi (ekran / dump)\r\n\r\n");

        print!(
            "  {} 📚 check example {} 💾 check📚 -> dump\r\n",
            "[e]".yellow().bold(),
            "[E]".cyan().bold()
        );
        print!(
            "  {} ✅ check         {} 💾 check   -> dump\r\n",
            "[c]".yellow().bold(),
            "[C]".cyan().bold()
        );
        print!(
            "  {} 🔬 clippy        {} 💾 clippy  -> dump\r\n",
            "[x]".yellow().bold(),
            "[X]".cyan().bold()
        );
        print!(
            "  {} 🧹 fmt           {} 💾 fmt chk -> dump\r\n",
            "[f]".yellow().bold(),
            "[F]".cyan().bold()
        );
        print!(
            "  {} 🧪 test          {} 💾 test    -> dump\r\n",
            "[t]".yellow().bold(),
            "[T]".cyan().bold()
        );
        print!(
            "  {} 🌐 wasm-check    {} 💾 wasm    -> dump\r\n",
            "[w]".yellow().bold(),
            "[W]".cyan().bold()
        );
        print!(
            "  {} 🚀 all           {} 💾 all     -> dump\r\n",
            "[a]".yellow().bold(),
            "[A]".cyan().bold()
        );
        print!(
            "  {} 👀 file/dir tree {} 💾 snapshot-> dump\r\n\r\n",
            "[d]".yellow().bold(),
            "[D]".cyan().bold()
        );

        let clear_lbl = if is_clear { "🧽 clear: ON " } else { "🧽 clear: OFF" };
        let clear_key = if is_clear { "[z]".red().bold() } else { "[z]".yellow().bold() };
        
        print!("  {} {}\r\n", clear_key, clear_lbl);
        print!("❌  {}\r\n", "[Esc] / [b] back".dark_grey());

        out.flush()?;

        loop {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
                    || key.code == KeyCode::Esc
                    || key.code == KeyCode::Char('q')
                    || key.code == KeyCode::Char('b')
                {
                    break 'inspect_loop;
                }

                match key.code {
                    KeyCode::Char('z') => {
                        is_clear = !is_clear;
                        break;
                    }
                    KeyCode::Char('e') => { final_result = Some((InspectAction::CheckExamples, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('c') => { final_result = Some((InspectAction::Check, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('x') => { final_result = Some((InspectAction::Clippy, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('f') => { final_result = Some((InspectAction::Fmt, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('t') => { final_result = Some((InspectAction::Test, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('w') => { final_result = Some((InspectAction::WasmCheck, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('a') => { final_result = Some((InspectAction::All, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('d') => { final_result = Some((InspectAction::SnapshotPreview, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('E') => { final_result = Some((InspectAction::DumpCheckExamples, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('C') => { final_result = Some((InspectAction::DumpCheck, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('X') => { final_result = Some((InspectAction::DumpClippy, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('F') => { final_result = Some((InspectAction::DumpFmtCheck, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('T') => { final_result = Some((InspectAction::DumpTest, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('W') => { final_result = Some((InspectAction::DumpWasmCheck, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('A') => { final_result = Some((InspectAction::DumpAll, is_clear)); break 'inspect_loop; }
                    KeyCode::Char('D') => { final_result = Some((InspectAction::SnapshotSave, is_clear)); break 'inspect_loop; }
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
}

fn run_smart_fmt(check_only: bool, is_dry_run: bool) {
    let mut args = vec!["+nightly", "fmt", "--all"];
    if check_only {
        args.extend(&["--", "--check"]);
    }
    
    if is_dry_run {
        println!("> [DRY RUN] 🔒 Uruchomiłbym: cargo {}\r\n", args.join(" "));
        return;
    }

    println!("> Uruchamiam formatting (+nightly)...");
    let status = Command::new("cargo").args(&args).status();

    if status.is_err() || !status.unwrap().success() {
        println!("⚠️ Nightly fmt niedostępne, używam standardowego `cargo fmt`...");
        let mut fallback = vec!["fmt", "--all"];
        if check_only {
            fallback.extend(&["--", "--check"]);
        }
        let _ = Command::new("cargo").args(&fallback).status();
    }
}

fn run_all_inspect(is_dry_run: bool) {
    run_smart_fmt(true, is_dry_run);
    run_cmd("cargo", &["check", "--all-targets"], is_dry_run);
    run_cmd("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"], is_dry_run);
    run_cmd("cargo", &["test"], is_dry_run);
}

fn dump_all_logs(is_dry_run: bool) {
    run_cmd_and_save("cargo", &["check", "--all-targets"], "all_check.dump.txt", is_dry_run);
    run_cmd_and_save("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"], "all_clippy.dump.txt", is_dry_run);
    run_cmd_and_save("cargo", &["test"], "all_test.dump.txt", is_dry_run);
    if !is_dry_run {
        println!("✅ Komplet logów zapisany do ./dump/ai_logs/\r\n");
    }
}

fn run_cmd_and_save(program: &str, args: &[&str], filename: &str, is_dry_run: bool) {
    if is_dry_run {
        println!("> [DRY RUN] 🔒 Przechwyciłbym do pliku: {} {} -> ./dump/ai_logs/{}\r\n", program, args.join(" "), filename);
        return;
    }
    
    println!("> Przechwytuję do pliku: {} {}\r\n", program, args.join(" "));

    let dump_dir = Path::new("./dump/ai_logs");
    if !dump_dir.exists() {
        let _ = fs::create_dir_all(dump_dir);
    }

    let output = match Command::new(program).args(args).output() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("🚨 Błąd procesu: {}\r\n", e);
            return;
        }
    };

    let file_path = dump_dir.join(filename);
    let mut log_content = format!("COMMAND: {} {}\n\n", program, args.join(" "));

    if !output.stdout.is_empty() {
        log_content.push_str("--- STDOUT ---\n");
        log_content.push_str(&String::from_utf8_lossy(&output.stdout));
        log_content.push('\n');
    }

    if !output.stderr.is_empty() {
        log_content.push_str("--- STDERR ---\n");
        log_content.push_str(&String::from_utf8_lossy(&output.stderr));
        log_content.push('\n');
    }

    match fs::write(&file_path, log_content) {
        Ok(_) => println!("✅ Zapisano log: {}\r\n", file_path.display()),
        Err(e) => eprintln!("🚨 Błąd zapisu: {}\r\n", e),
    }
}