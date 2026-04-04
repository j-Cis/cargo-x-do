use std::{fs, path::Path};

use anyhow::{Context, Result};

#[allow(clippy::too_many_lines)]
pub fn ensure_config(config_path: &str, output_path: &str) -> Result<()> {
	if Path::new(config_path).exists() {
		return Ok(());
	}

	if !Path::new(output_path).exists() {
		fs::create_dir_all(output_path).context("Failed to create logs directory")?;
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
output = "target/.x-do/logs/fmt/clippy-fix.log"
git_commit = true
commit_message = "before: apply clippy suggestions"
description = "【ENG】 Apply clippy fixes / 【POL】 Zastosuj sugestie clippy"

[[cargo-fmt-nightly]]
args = ["--", "--check"]
id = "cC1"
name = "fmt-check"
order = 2
output = "target/.x-do/logs/fmt/fmt-check.log"
git_commit = true
commit_message = "before: format with nightly fmt"
description = "【ENG】 Nightly formatting / 【POL】 Formatowanie wersją nightly"

[[rustfmt]]
args = ["--edition", "2024"]
id = "cD1"
name = "rustfmt"
order = 3
output = "target/.x-do/logs/fmt/rustfmt.log"
git_commit = true
commit_message = "before: format with rustfmt"
description = "【ENG】 Direct rustfmt / 【POL】 Bezpośrednie rustfmt"
"#;

	fs::write(config_path, template).context("Init failed: corrections")?;
	println!("✨ 【ENG】 Created / 【POL】 Utworzono: {config_path}");
	Ok(())
}
