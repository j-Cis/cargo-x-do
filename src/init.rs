use std::{fs, path::Path};

use anyhow::{Context, Result};

const CONFIG_DIR: &str = "./.x-do";
const GITKEEP_PATH: &str = "./.x-do/.gitkeep";

/// 【ENG】 Ensures the project workspace is initialized / 【POL】 Upewnia się, że przestrzeń projektowa jest zainicjowana
pub fn ensure_workspace() -> Result<()> {
	let dir = Path::new(CONFIG_DIR);
	let gitkeep = Path::new(GITKEEP_PATH);

	// 1. Sprawdzenie/Tworzenie folderu .x-do
	if !dir.exists() {
		println!(
			"📂 【ENG】 Creating config directory / 【POL】 Tworzenie folderu konfiguracji: {}",
			CONFIG_DIR
		);
		fs::create_dir_all(dir).with_context(|| {
			format!(
				"【ENG】 Failed to create directory / 【POL】 Nie udało się utworzyć folderu: {}",
				CONFIG_DIR
			)
		})?;
	}

	// 2. Sprawdzenie/Tworzenie pliku .gitkeep
	if !gitkeep.exists() {
		if !dir.exists() {
			// W razie gdyby coś poszło nie tak wcześniej
			fs::create_dir_all(dir)?;
		}

		fs::write(gitkeep, "").with_context(|| {
			format!("【ENG】 Failed to create / 【POL】 Nie udało się utworzyć: {}", GITKEEP_PATH)
		})?;

		println!(
			"✨ 【ENG】 Workspace initialized (.gitkeep created) / 【POL】 Inicjalizacja zakończona (utworzono .gitkeep)"
		);
	}

	Ok(())
}
