use std::{env, fs, path::Path};

use querypath::QueryPath;
use querypath_fmt::{
	CasePrecedence,
	CharClass,
	Column,
	DirWeightDisplay,
	GroupStrategy,
	NoExtPriority,
	NodeGroup,
	QueryPathBundle,
	QueryPathFmt,
	SameNamePriority,
	Sorting,
	StatsTemporal,
	StatsWeight,
	UnitSystem,
	WeightPrecision,
};
use querypath_snapshot::Snapshot;
use serde::Deserialize;

// --- Struktury Deserialize dla formatu TOML ---

#[derive(Deserialize, Debug)]
pub struct RootConfig {
	pub snapshot: SnapshotConfig,
}

#[derive(Deserialize, Debug)]
pub struct SnapshotConfig {
	pub title: String,
	pub title_sufix_date: bool,
	pub title_sufix_date_fmt: String,
	pub name: String,
	pub name_sufix_date: bool,
	pub name_sufix_date_fmt: String,
	pub output_dir: String,
	pub max_single_file_size: u64,
	pub add_footer_summary_setups: bool,
	pub add_header_customize_text: String,
	pub add_fotter_customize_text: String,

	pub scan: ScanConfig,
	pub sort: SortConfig,
	pub incl: InclConfig,
	pub tree: TreeConfig,
}

#[derive(Deserialize, Debug)]
pub struct ScanConfig {
	pub match_patterns: Vec<String>,
	pub keep_parent: bool,
}

#[derive(Deserialize, Debug)]
pub struct SortConfig {
	pub same_name_priority: String,
	pub no_ext_priority: String,
	pub ignore_leading_dot: bool,
	pub case_precedence: String,
}

#[derive(Deserialize, Debug)]
pub struct InclConfig {
	pub numerate_dirs: bool,
	pub numerate_binaries: bool,
	pub numerate_skip: Vec<String>,
	pub content_skip: Vec<String>,
	pub numerate_start_from: usize,
}

#[derive(Deserialize, Debug)]
pub struct TreeConfig {
	pub name_width: usize,
	pub path_width: usize,
	pub column_order_left: Vec<String>,
	pub column_order_right: Vec<String>,
	pub weight: WeightConfig,
	pub temporal: TemporalConfig,
}

#[derive(Deserialize, Debug)]
pub struct WeightConfig {
	pub stats_weight: bool,
	pub unit_system_binary: bool,
	pub dir_display: String,
	pub precision: String,
}

#[derive(Deserialize, Debug)]
pub struct TemporalConfig {
	pub stats_temporal: bool,
	pub pattern: String,
}

// --- Logika wczytywania ---

fn detect_project_name() -> String {
	if let Ok(content) = fs::read_to_string("Cargo.toml") {
		for line in content.lines() {
			let trimmed = line.trim();
			if trimmed.starts_with("name")
				&& let Some((_, val)) = trimmed.split_once('=')
			{
				let name = val.trim().trim_matches('"').trim_matches('\'');
				if !name.is_empty() {
					return name.to_string();
				}
			}
		}
	}
	env::current_dir()
		.ok()
		.and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
		.unwrap_or_else(|| "project".to_string())
}

fn load_config() -> SnapshotConfig {
	let candidate_paths = [
		Path::new("./.cargo/.x-do.toml"),
		Path::new("./.cargo/x-do.toml"),
		Path::new("./.xtask/.x-do.toml"),
		Path::new("./.xtask/x-do.toml"),
		Path::new("./.x-do.toml"),
	];

	for path in candidate_paths {
		if let Ok(content) = fs::read_to_string(path)
			&& let Ok(root) = toml::from_str::<RootConfig>(&content)
		{
			return root.snapshot;
		}
	}

	panic!("🚨 Nie znaleziono lub nie udało się sparsować .x-do.toml! Uruchom setup [s] w głównym menu.");
}

// --- ZASADA DRY: FABRYKI OBIEKTÓW FORMATUJĄCYCH ---

fn map_columns(cols: &[String]) -> Vec<Column> {
	cols.iter()
		.filter_map(|c| match c.as_str() {
			"Weight" => Some(Column::Weight),
			"Temporal" => Some(Column::Temporal),
			"Path" => Some(Column::Path),
			_ => None,
		})
		.collect()
}

fn build_sorting(cfg: &SnapshotConfig) -> Sorting {
	Sorting::new()
		.enabled(true)
		.group_strategy(GroupStrategy::Custom(vec![NodeGroup::TextFile, NodeGroup::Directory, NodeGroup::BinaryFile]))
		.same_name_priority(match cfg.sort.same_name_priority.as_str() {
			"DirectoryFirst" => SameNamePriority::DirectoryFirst,
			_ => SameNamePriority::FileFirst,
		})
		.no_ext_priority(match cfg.sort.no_ext_priority.as_str() {
			"Below" => NoExtPriority::Below,
			_ => NoExtPriority::Above,
		})
		.ignore_leading_dot(cfg.sort.ignore_leading_dot)
		.char_class_order([CharClass::Special, CharClass::Digit, CharClass::Letter])
		.case_precedence(match cfg.sort.case_precedence.as_str() {
			"UpperFirst" => CasePrecedence::UpperFirst,
			"LowerFirst" => CasePrecedence::LowerFirst,
			_ => CasePrecedence::Insensitive,
		})
}

fn build_bundle(cfg: &SnapshotConfig, sorting: Sorting) -> QueryPathBundle {
	QueryPathBundle::new()
		.queueing_before_numerate(sorting)
		.numerate_dirs(cfg.incl.numerate_dirs)
		.numerate_binaries(cfg.incl.numerate_binaries)
		.numerate_skip(&cfg.incl.numerate_skip)
		.numerate_start_from(cfg.incl.numerate_start_from)
}

fn build_fmt(cfg: &SnapshotConfig) -> QueryPathFmt {
	let weight_display = match cfg.tree.weight.dir_display.as_str() {
		"None" => DirWeightDisplay::None,
		"RealOnly" => DirWeightDisplay::RealOnly,
		"Both" => DirWeightDisplay::Both,
		_ => DirWeightDisplay::MatchedOnly,
	};

	let weight_precision = match cfg.tree.weight.precision.as_str() {
		"0.01" | "Hundredths" => WeightPrecision::Hundredths,
		_ => WeightPrecision::Tenths,
	};

	let unit_system = if cfg.tree.weight.unit_system_binary { UnitSystem::Binary } else { UnitSystem::Decimal };

	QueryPathFmt::new()
		.name_width(cfg.tree.name_width)
		.path_width(cfg.tree.path_width)
		.column_order_left(map_columns(&cfg.tree.column_order_left))
		.column_order_right(map_columns(&cfg.tree.column_order_right))
		.stats_weight(
			StatsWeight::new()
				.enabled(cfg.tree.weight.stats_weight)
				.unit_system(unit_system)
				.dir_display(weight_display)
				.precision(weight_precision),
		)
		.stats_temporal(
			StatsTemporal::new().enabled(cfg.tree.temporal.stats_temporal).pattern(&cfg.tree.temporal.pattern),
		)
}

// --- AKCJE TUI ---

pub fn preview_tree() {
	let cfg = load_config();
	println!("🔍 Skanowanie struktury plików...");

	let results = QueryPath::new()
		.scan_at(["./"])
		.keep_parent(cfg.scan.keep_parent)
		.match_pattern(&cfg.scan.match_patterns)
		.run()
		.expect("Błąd skanowania plików");

	// Użycie wydzielonych fabryk (DRY)
	let sorting = build_sorting(&cfg);
	let idn = build_bundle(&cfg, sorting).apply_to(&results);
	let fmt = build_fmt(&cfg);

	println!("\r\n--- STRUKTURA PLIKÓW PROJEKTU ---");
	println!("{}", fmt.format(&idn));
	println!("---------------------------------\r\n");
}

pub fn generate() {
	let cfg = load_config();
	println!("🔍 Skanowanie i zrzut kodu...");

	let results = QueryPath::new()
		.scan_at(["./"])
		.keep_parent(cfg.scan.keep_parent)
		.match_pattern(&cfg.scan.match_patterns)
		.run()
		.expect("Błąd skanowania plików");

	// Użycie wydzielonych fabryk (DRY)
	let sorting = build_sorting(&cfg);
	let idn = build_bundle(&cfg, sorting).apply_to(&results);
	let fmt = build_fmt(&cfg);

	let mut final_title = cfg.title.clone();
	if final_title.contains("PROJECT") {
		final_title = final_title.replace("PROJECT", &detect_project_name().to_uppercase());
	}

	let mut final_name = cfg.name.clone();
	if final_name.contains("project") {
		final_name = final_name.replace("project", &detect_project_name());
	}

	let mut snapshot = Snapshot::new()
		.title(&final_title)
		.name(&final_name)
		.output_dir(&cfg.output_dir)
		.max_single_file_size(cfg.max_single_file_size)
		.content_skip(&cfg.incl.content_skip)
		.add_footer_summary_setups(cfg.add_footer_summary_setups)
		.add_header_customize_text(&cfg.add_header_customize_text)
		.add_footer_customize_text(&cfg.add_fotter_customize_text);

	if cfg.title_sufix_date {
		snapshot = snapshot.title_suffix_date_fmt(&cfg.title_sufix_date_fmt);
	}

	if cfg.name_sufix_date {
		snapshot = snapshot.name_suffix_date_fmt(&cfg.name_sufix_date_fmt);
	}

	match snapshot.generate_and_save(&results, &idn, &fmt) {
		Ok(path) => println!("✅ Zapisano snapshot do: {}\r\n", path.display()),
		Err(e) => eprintln!("❌ Błąd tworzenia migawki: {}\r\n", e),
	}
}
