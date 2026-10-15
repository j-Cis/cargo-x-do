#[path = "feature/git_and_cargo.rs"]
pub mod git_and_cargo;
#[path = "feature/info_and_save.rs"]
pub mod info_and_save;
#[path = "feature/last_and_history.rs"]
pub mod last_and_history;
#[path = "feature/run_and_build.rs"]
pub mod run_and_build;

use std::io::{stdout, Write, Result};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Stylize,
    terminal,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Last,
    LastList,
    Info,
    Run,
    Build,
    Git,
    Cargo,
}

fn main() {
    let has_history = last_and_history::has_history();

    // Główna pętla programu – powrót z podmenu wraca tutaj
    loop {
        match run_level_1(has_history) {
            Ok(Some((action, is_dry_run))) => {
                let executed = match action {
                    Action::Last => last_and_history::handle_last(is_dry_run),
                    Action::LastList => last_and_history::handle_list(is_dry_run),
                    Action::Info => info_and_save::handle_info(is_dry_run),
                    Action::Run | Action::Build => run_and_build::handle_run_and_build(is_dry_run),
                    Action::Git => git_and_cargo::handle_git(is_dry_run),
                    Action::Cargo => git_and_cargo::handle_cargo(is_dry_run),
                };

                // Jeśli wykonano rzeczywistą komendę -> kończymy proces.
                // Jeśli w podmenu wciśnięto powrót (b/Esc) -> pętla wyświetla Level 1 ponownie.
                if executed {
                    break;
                }
            }
            // Wyjście z poziomu 1 (Esc / q / Ctrl+C)
            Ok(None) => break,
            Err(e) => {
                eprintln!("🚨 Błąd terminala: {}\r\n", e);
                break;
            }
        }
    }
}

pub fn run_level_1(has_history: bool) -> Result<Option<(Action, bool)>> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;
    execute!(out, cursor::Hide)?;

    let mut is_dry_run = false;
    let mut first_draw = true;
    let mut final_result = None;

    'main_loop: loop {
        // Zamiast spamować terminal, przy re-drawingu (po wciśnięciu 'o')
        // cofamy kursor o 9 linii w górę i czyścimy ekran w dół.
        if !first_draw {
            execute!(
                out,
                cursor::MoveUp(9),
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )?;
        }
        first_draw = false;

        // 1. Nagłówek (3 linie w dół)
        print!("\r\n🚀 x-do (xtask)\r\n\r\n");

        // 2. Historia (2 linie w dół)
        if has_history {
            print!(
                "  {} ♻️  last      {} 📜 last list\r\n\r\n",
                "[Enter]".yellow().bold(),
                "[l]".yellow().bold()
            );
        } else {
            print!(
                "  {} ♻️  last      {} 📜 last list  {}\r\n\r\n",
                "[Enter]".dark_grey(),
                "[l]".dark_grey(),
                "🚫".dark_grey()
            );
        }

        // 3. Główna siatka (3 linie w dół)
        let dry_key = if is_dry_run { "[o]".red().bold() } else { "[o]".yellow().bold() };
        let dry_lbl = if is_dry_run { "🔒 dry: ON " } else { "🔓 dry: OFF" };

        print!(
            "  {} ℹ️  info      {} 🏗️  build      {} 🌿 git\r\n",
            "[i]".yellow().bold(),
            "[b]".yellow().bold(),
            "[g]".yellow().bold()
        );
        print!(
            "  {} ▶️  run       {} {}   {} 📦 cargo\r\n\r\n",
            "[r]".yellow().bold(),
            dry_key,
            dry_lbl,
            "[c]".yellow().bold()
        );

        // 4. Stopka wyjścia (1 linia w dół = Razem 9 linii wydruku)
        print!("❌  {}\r\n", "[Esc] / [q] / [Ctrl+C]".dark_grey());
        
        // Wymuszamy wypisanie bufora w trybie surowym
        out.flush()?;

        // 5. Oczekiwanie na klawisz
        loop {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Obsługa wyjścia
                if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
                    || key.code == KeyCode::Esc
                    || key.code == KeyCode::Char('q')
                {
                    break 'main_loop; // Przerywa całkowicie, zwracając None
                }

                match key.code {
                    // Przełącznik Dry Run
                    KeyCode::Char('o') => {
                        is_dry_run = !is_dry_run;
                        break; // Przerywa tylko wewnętrzną pętlę i przerysowuje menu!
                    }

                    // Akcje właściwe
                    KeyCode::Enter if has_history => {
                        final_result = Some((Action::Last, is_dry_run));
                        break 'main_loop;
                    }
                    KeyCode::Char('l') if has_history => {
                        final_result = Some((Action::LastList, is_dry_run));
                        break 'main_loop;
                    }
                    KeyCode::Char('i') => {
                        final_result = Some((Action::Info, is_dry_run));
                        break 'main_loop;
                    }
                    KeyCode::Char('r') => {
                        final_result = Some((Action::Run, is_dry_run));
                        break 'main_loop;
                    }
                    KeyCode::Char('b') => {
                        final_result = Some((Action::Build, is_dry_run));
                        break 'main_loop;
                    }
                    KeyCode::Char('g') => {
                        final_result = Some((Action::Git, is_dry_run));
                        break 'main_loop;
                    }
                    KeyCode::Char('c') => {
                        final_result = Some((Action::Cargo, is_dry_run));
                        break 'main_loop;
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