use querypath::QueryPath;
use querypath_fmt::{
    CasePrecedence, CharClass, Column, DirWeightDisplay, GroupStrategy, NoExtPriority, NodeGroup,
    QueryPathBundle, QueryPathFmt, SameNamePriority, Sorting, StatsTemporal, StatsWeight,
    UnitSystem, WeightPrecision,
};
use querypath_snapshot::Snapshot;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct SnapshotConfig {
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_max_size")]
    pub max_single_file_size: u64,
    #[serde(default = "default_matches")]
    pub match_patterns: Vec<String>,
    #[serde(default = "default_skip_content")]
    pub content_skip: Vec<String>,
}

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

fn default_title() -> String {
    format!("{} (CODE SNAPSHOT) v:", detect_project_name().to_uppercase())
}

fn default_name() -> String {
    format!("{}_", detect_project_name())
}

fn default_output_dir() -> String {
    "./dump".to_string()
}

fn default_max_size() -> u64 {
    524288
}

fn default_matches() -> Vec<String> {
    vec![
        "./{examples|tests|src|scripts|.github|.xtask|.cargo}/**".to_string(),
        "./{Cargo|Trunk}.toml".to_string(),
        "./index.html".to_string(),
        "./assets/**".to_string(),
        "./{.gitattributes|.gitignore|.rustfmt.toml}".to_string(),
        "!./.xtask/target/**".to_string(),
        "!./.xtask/Cargo.lock".to_string(),
        "!./Cargo.lock".to_string(),
        "!./{target|.git|dist|dump}/**".to_string(),
    ]
}

fn default_skip_content() -> Vec<String> {
    vec!["./assets/**".to_string(), "*.svg".to_string()]
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            title: default_title(),
            name: default_name(),
            output_dir: default_output_dir(),
            max_single_file_size: default_max_size(),
            match_patterns: default_matches(),
            content_skip: default_skip_content(),
        }
    }
}

/// [d] Podgląd struktury plików w terminalu
pub fn preview_tree() {
    let config = load_config();
    println!("🔍 Skanowanie struktury plików...");

    let results = QueryPath::new()
        .scan_at(["./"])
        .keep_parent(true)
        .match_pattern(&config.match_patterns)
        .run()
        .expect("Błąd skanowania plików");

    let sorting = Sorting::new()
        .enabled(true)
        .group_strategy(GroupStrategy::Custom(vec![
            NodeGroup::TextFile,
            NodeGroup::Directory,
            NodeGroup::BinaryFile,
        ]))
        .same_name_priority(SameNamePriority::FileFirst)
        .no_ext_priority(NoExtPriority::Above)
        .ignore_leading_dot(true)
        .char_class_order([CharClass::Special, CharClass::Digit, CharClass::Letter])
        .case_precedence(CasePrecedence::Insensitive);

    let idn = QueryPathBundle::new()
        .queueing_before_numerate(sorting)
        .numerate_dirs(false)
        .numerate_binaries(false)
        .numerate_skip(&config.content_skip)
        .numerate_start_from(1)
        .apply_to(&results);

    let fmt = QueryPathFmt::new()
        .name_width(25)
        .path_width(35)
        .column_order_left(vec![])
        .column_order_right(vec![Column::Weight, Column::Temporal, Column::Path])
        .stats_weight(
            StatsWeight::new()
                .enabled(true)
                .unit_system(UnitSystem::Binary)
                .dir_display(DirWeightDisplay::MatchedOnly)
                .precision(WeightPrecision::Tenths),
        )
        .stats_temporal(StatsTemporal::new().enabled(true).pattern("WYYY-WW-D hh:mm:ss"));

    println!("\r\n--- STRUKTURA PLIKÓW PROJEKTU ---");
    println!("{}", fmt.format(&idn));
    println!("---------------------------------\r\n");
}

/// [D] lub [s] w menu głównym – pełny zapis do pliku Markdown
pub fn generate() {
    let config = load_config();
    println!("🔍 Skanowanie i zrzut kodu...");

    let results = QueryPath::new()
        .scan_at(["./"])
        .keep_parent(true)
        .match_pattern(&config.match_patterns)
        .run()
        .expect("Błąd skanowania plików");

    let sorting = Sorting::new()
        .enabled(true)
        .group_strategy(GroupStrategy::Custom(vec![
            NodeGroup::TextFile,
            NodeGroup::Directory,
            NodeGroup::BinaryFile,
        ]))
        .same_name_priority(SameNamePriority::FileFirst)
        .no_ext_priority(NoExtPriority::Above)
        .ignore_leading_dot(true)
        .char_class_order([CharClass::Special, CharClass::Digit, CharClass::Letter])
        .case_precedence(CasePrecedence::Insensitive);

    let idn = QueryPathBundle::new()
        .queueing_before_numerate(sorting)
        .numerate_dirs(false)
        .numerate_binaries(false)
        .numerate_skip(&config.content_skip)
        .numerate_start_from(1)
        .apply_to(&results);

    let fmt = QueryPathFmt::new()
        .name_width(25)
        .path_width(35)
        .column_order_left(vec![])
        .column_order_right(vec![Column::Weight, Column::Temporal, Column::Path])
        .stats_weight(
            StatsWeight::new()
                .enabled(true)
                .unit_system(UnitSystem::Binary)
                .dir_display(DirWeightDisplay::MatchedOnly)
                .precision(WeightPrecision::Tenths),
        )
        .stats_temporal(StatsTemporal::new().enabled(true).pattern("WYYY-WW-D hh:mm:ss"));

    let snapshot = Snapshot::new()
        .title(&config.title)
        .title_sufix_date_fmt("WYYY-WW D::hh:mm:ss:tt:qq")
        .name(&config.name)
        .name_sufix_date_fmt("WYYYWW-Dhhmm-ssttqq")
        .output_dir(&config.output_dir)
        .max_single_file_size(config.max_single_file_size)
        .content_skip(&config.content_skip)
        .add_footer_summary_setups(true)
        .add_header_customize_text("\nGenerated via cargo-x-do\n")
        .add_fotter_customize_text(
            "\n---\n*Generated automatically via querypath & querypath-snapshot*\n",
        );

    match snapshot.generate_and_save(&results, &idn, &fmt) {
        Ok(path) => println!("✅ Zapisano snapshot do: {}\r\n", path.display()),
        Err(e) => eprintln!("❌ Błąd tworzenia migawki: {}\r\n", e),
    }
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
            && let Ok(cfg) = toml::from_str::<SnapshotConfig>(&content)
        {
            return cfg;
        }
    }

    SnapshotConfig::default()
}