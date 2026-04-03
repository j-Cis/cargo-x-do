use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const CONFIG_PATH: &str = "./.x-do/rust-diag-corrections.toml";
const OUTPUT_PATH: &str = "./target/.x-do/logs/";

pub fn ensure_config() -> Result<()> {
    if Path::new(CONFIG_PATH).exists() { return Ok(()); }

    // Zapewniamy istnienie folderu na logi
    if !Path::new(OUTPUT_PATH).exists() {
        fs::create_dir_all(OUTPUT_PATH).context("Failed to create logs directory")?;
    }

    let template = r#"# Corrections (Writing & Committing)
git_commit = "per-task" # "single", "none"
commit_message = "before: apply automatic fix/fmt"

[[trim-whitespace]]
args = []
id = "cA1"
name = "trim-whitespace"
order = 0
git_commit = true
commit_message = "before: apply automatic trim trailing whitespace"
description = "【ENG】 Trim Trailing Whitespace / 【POL】 Usuwanie końcowych spacji"

[[cargo-clippy-fix]]
args = ["--allow-dirty"]
id = "cB1"
name = "clippy-fix"
order = 1
output = "target/.x-do/logs/clippy-fix.log"
git_commit = true
commit_message = "before: apply clippy suggestions"
description = "【ENG】 Apply clippy fixes / 【POL】 Zastosuj sugestie clippy"

[[cargo-fmt-nightly]]
args = ["--all"]
id = "cC1"
name = "fmt-all"
order = 2
output = "target/.x-do/logs/fmt-all.log"
git_commit = true
commit_message = "before: format with nightly fmt"
description = "【ENG】 Nightly formatting / 【POL】 Formatowanie wersją nightly"

[[rustfmt]]
args = ["--edition", "2024"]
id = "cD1"
name = "rustfmt"
order = 3
output = "target/.x-do/logs/rustfmt.log"
git_commit = true
commit_message = "before: format with rustfmt"
description = "【ENG】 Direct rustfmt / 【POL】 Bezpośrednie rustfmt"
"#;

    fs::write(CONFIG_PATH, template).context("Init failed: corrections")?;
    println!("✨ 【ENG】 Created / 【POL】 Utworzono: {}", CONFIG_PATH);
    Ok(())
}