use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const CONFIG_PATH: &str = "./.x-do/cargo-plot.toml";
const OUTPUT_PATH: &str = "./target/.x-do/code/";

/// 【ENG】 Creates default cargo-plot.toml if missing / 【POL】 Tworzy domyślny cargo-plot.toml jeśli brakuje
pub fn ensure_config() -> Result<()> {
    if Path::new(CONFIG_PATH).exists() {
        return Ok(());
    }
	if !Path::new(OUTPUT_PATH).exists() {
        fs::create_dir_all(OUTPUT_PATH).context("Failed to create logs directory")?;
    }

    // Używamy Twoich precyzyjnych ustawień dla Snapshot-Rust
    let template = r#"[[task]]
id = "p1"
name = "snapshot-rust"
description = "【ENG】 Standard Rust project snapshot / 【POL】 Standardowy zrzut projektu Rust"
input_folder = "."
output_folder = "./target/.x-do/code/"
patterns = [
    "./{.rustfmt,Cargo}.toml",
    "./{src,examples,tests}/**{/*.rs,/}",
    "./build.rs"
]
action = ["-m", "--save-archive"]
style = ["-b", "--lang", "en", "-v", "grid", "-u", "dec", "-s", "za-file-merge", "--no-emoji"]

[task.settings]
clean_before_run = true
silent = false
"#;

    fs::write(CONFIG_PATH, template)
        .with_context(|| format!("【ENG】 Init failed / 【POL】 Błąd inicjalizacji: {}", CONFIG_PATH))?;

    println!("✨ 【ENG】 Created / 【POL】 Utworzono: {}", CONFIG_PATH);
    Ok(())
}