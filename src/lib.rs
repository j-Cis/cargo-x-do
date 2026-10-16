// ./src/lib.rs

#[path = "feature/git_and_cargo.rs"]
pub mod git_and_cargo;
#[path = "feature/info_and_save.rs"]
pub mod info_and_save;
#[path = "feature/last_and_history.rs"]
pub mod last_and_history;
#[path = "feature/run_and_build.rs"]
pub mod run_and_build;

use std::{
	fs,
	io::{Result, Write, stdout},
};

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
	Setup,
}

pub fn run_tui() {
	let has_history = last_and_history::has_history();

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
					Action::Setup => handle_setup(is_dry_run),
				};

				if executed {
					break;
				}
			}
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
		if !first_draw {
			execute!(
				out,
				cursor::MoveUp(10), // Podniesione do 10, bo doszła linia setup
				terminal::Clear(terminal::ClearType::FromCursorDown)
			)?;
		}
		first_draw = false;

		print!("\r\n🚀 x-do (xtask)\r\n\r\n");

		if has_history {
			print!("  {} ♻️  last      {} 📜 last list\r\n\r\n", "[Enter]".yellow().bold(), "[l]".yellow().bold());
		} else {
			print!(
				"  {} ♻️  last      {} 📜 last list  {}\r\n\r\n",
				"[Enter]".dark_grey(),
				"[l]".dark_grey(),
				"🚫".dark_grey()
			);
		}

		let dry_key = if is_dry_run { "[o]".red().bold() } else { "[o]".yellow().bold() };
		let dry_lbl = if is_dry_run { "🔒 dry: ON " } else { "🔓 dry: OFF" };

		print!(
			"  {} ℹ️  info      {} 🏗️  build      {} 🌿 git\r\n",
			"[i]".yellow().bold(),
			"[b]".yellow().bold(),
			"[g]".yellow().bold()
		);
		print!(
			"  {} ▶️  run       {} {}   {} 📦 cargo\r\n",
			"[r]".yellow().bold(),
			dry_key,
			dry_lbl,
			"[c]".yellow().bold()
		);
		print!("  {} ⚙️  setup\r\n\r\n", "[s]".yellow().bold());

		print!("❌  {}\r\n", "[q] 🔚 exit / [Ctrl+C]".dark_grey());

		out.flush()?;

		loop {
			if let Event::Key(key) = event::read()? {
				if key.kind != KeyEventKind::Press {
					continue;
				}

				if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
					|| key.code == KeyCode::Esc
					|| key.code == KeyCode::Char('q')
				{
					break 'main_loop;
				}

				match key.code {
					KeyCode::Char('o') => {
						is_dry_run = !is_dry_run;
						break;
					}
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
					KeyCode::Char('s') => {
						final_result = Some((Action::Setup, is_dry_run));
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

fn handle_setup(is_dry_run: bool) -> bool {
	if is_dry_run {
		println!(
			"> [DRY RUN] 🔒 Wykonałbym inicjalizację: tworzenie .rustfmt.toml, .cargo/config.toml oraz .cargo/.x-do.toml\r\n"
		);
		return true;
	}

	println!("⚙️ Inicjalizacja środowiska...\r\n");

	// 1. .rustfmt.toml - PEŁNA KONFIGURACJA
	let rustfmt_cfg = r#"# =========================
# 🧠 RUST EDITION / CORE
# =========================

# 01【A01】 Wersja edycji Rust (wpływa na składnię i reguły języka)
edition = "2024"

# 02【A02】 Włącza niestabilne funkcje rustfmt (eksperymentalne opcje formatowania)
unstable_features = true


# =========================
# 📏 FORMATOWANIE OGÓLNE (LINIE / UKŁAD)
# =========================

# 03【B01】 Maksymalna szerokość linii kodu
max_width = 120

# 04【B02】 Styl końca linii (Auto = zachowuje LF/CRLF)
newline_style = "Auto"

# 05【B03】 Heurystyka formatowania (Max = minimalne łamanie linii)
use_small_heuristics = "Max"

# 06【B04】 Styl wcięć (blokowy)
indent_style = "Block"

# 07【B05】 Używa TABów zamiast spacji
hard_tabs = true

# 08【B06】 Maksymalna liczba pustych linii
blank_lines_upper_bound = 1


# =========================
# 🔧 STRUKTURA KODU (BLOCKS / CONTROL FLOW)
# =========================

# 09【C01】 Preferuje klamry w tej samej linii co nagłówek
brace_style = "PreferSameLine"

# 10【C02】 Klamry w if/for/while zawsze w tej samej linii
control_brace_style = "AlwaysSameLine"

# 11【C03】 Where clause może być w jednej linii
where_single_line = true

# 12【C04】 Funkcje mogą być jednoliniowe
fn_single_line = true

# 13【C05】 Max długość if/else w jednej linii
single_line_if_else_max_width = 100

# 14【C06】 Elastyczne formatowanie wyrażeń delimitowanych
overflow_delimited_expr = true

# 15【C07】 Łączenie wyrażeń kontrolnych w bardziej kompaktową formę
combine_control_expr = true


# =========================
# 🧩 FUNKCJE / WYWOŁANIA / STRUKTURY
# =========================

# 16【D01】 Skrócona inicjalizacja structów
use_field_init_shorthand = true

# 17【D02】 Struct literal w jednej linii jeśli się mieści
struct_lit_single_line = true

# 18【D03】 Maksymalna szerokość wywołań funkcji
fn_call_width = 100

# 19【D04】 Maksymalna szerokość tablic
array_width = 100

# 20【D05】 Maksymalna szerokość atrybutów typu derive
attr_fn_like_width = 100


# =========================
# 📦 IMPORTY I MODUŁY
# =========================

# 21【E01】 Automatyczne sortowanie importów
reorder_imports = true

# 22【E02】 Granularność importów (Crate-level grouping)
imports_granularity = "Crate"

# 23【E03】 Grupowanie importów: std → external → local
group_imports = "StdExternalCrate"

# 24【E04】 Styl układu importów (horizontal / vertical fallback)
imports_layout = "HorizontalVertical"


# =========================
# 🧾 DOKUMENTACJA I KOMENTARZE
# =========================

# 25【F01】 Formatowanie kodu w doc comments
format_code_in_doc_comments = true

# 26【F02】 Normalizacja atrybutów dokumentacji
normalize_doc_attributes = true

# 27【F03】 Automatyczne zawijanie komentarzy
wrap_comments = true

# 28【F04】 Normalizacja stylu komentarzy
normalize_comments = true


# =========================
# 🧠 MACRA / META / ABSTRAKCJE
# =========================

# 29【G01】 Formatowanie matcherów w macro_rules!
format_macro_matchers = true

# 30【G02】 Sortowanie elementów w impl
# reorder_impl_items = true # IRYTUJĄCE


# =========================
# 🧮 LITERALS / STYLE DETALE
# =========================

# 31【H01】 HEX literały w uppercase
hex_literal_case = "Upper"

# 32【H02】 Skrócony try operator (? zamiast pełnych form)
use_try_shorthand = true
"#;

	let _ = fs::write(".rustfmt.toml", rustfmt_cfg);
	println!("✅ Utworzono pełny .rustfmt.toml");

	// 2. .cargo/config.toml
	let _ = fs::create_dir_all(".cargo");
	let mut config = fs::read_to_string(".cargo/config.toml").unwrap_or_default();
	if !config.contains("[alias]") {
		config.push_str("\n[alias]\nx = \"x-do\"\nxtask = \"x-do\"\n");
		let _ = fs::write(".cargo/config.toml", config);
		println!("✅ Dodano aliasy do .cargo/config.toml");
	} else {
		println!("⚠️ Sekcja [alias] już istnieje w .cargo/config.toml");
	}

	// 3. .cargo/.x-do.toml
	let xdo_cfg = r#"[snapshot]
title = "PROJECT (CODE SNAPSHOT) v:"
title_sufix_date = true
title_sufix_date_fmt = "WYYY-WW-D::hh:mm:ss:tt:qq"
name = "project_"
name_sufix_date = true
name_sufix_date_fmt = "WYYYWW-Dhhmm-ssttqq"
output_dir = "./dump"
max_single_file_size = 524288
add_footer_summary_setups = true
add_header_customize_text = "\nGenerated via cargo-x-do\n"
add_fotter_customize_text = "\n---\n*Generated automatically via querypath & querypath-snapshot*\n"

[snapshot.scan]
match_patterns = [
	"./{examples|tests|src|scripts|.github|.xtask|.cargo}/**",
	"./{Cargo|Trunk}.toml",
	"./index.html",
	"./assets/**",
	"./{.gitattributes|.gitignore|.rustfmt.toml}",
	"!./.xtask/target/**",
	"!./.xtask/Cargo.lock",
	"!./Cargo.lock",
	"!./{target|.git|dist|dump}/**"
]
keep_parent = true

[snapshot.sort]
same_name_priority = "FileFirst"
no_ext_priority = "Above"
ignore_leading_dot = true
case_precedence = "Insensitive"

[snapshot.incl]
numerate_dirs = false
numerate_binaries = false
numerate_skip = [
	"*.svg"
]
content_skip = [
	"./assets/**",
	"*.svg"
]
numerate_start_from = 1

[snapshot.tree]
name_width = 25
path_width = 25
column_order_left = []
column_order_right = ["Weight", "Temporal", "Path"]

[snapshot.tree.weight]
stats_weight = true
unit_system_binary = true
dir_display = "MatchedOnly"
precision = "0.1"

[snapshot.tree.temporal]
stats_temporal = true
pattern = "WYYY-WW-D:hh:mm:ss"
"#;
	let _ = fs::write(".cargo/.x-do.toml", xdo_cfg);
	println!("✅ Utworzono standardową konfigurację .cargo/.x-do.toml\r\n");

	true
}
