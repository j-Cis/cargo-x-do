use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn ensure_config(config_path: &str, output_path: &str) -> Result<()> {
    if Path::new(config_path).exists() { return Ok(()); }
    
    if !Path::new(output_path).exists() {
        fs::create_dir_all(output_path).context("Failed to create plots directory")?;
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

	fs::write(config_path, template).context("Init failed: cargo-plot")?;
    println!("✨ 【ENG】 Created / 【POL】 Utworzono: {config_path}");
    Ok(())
}