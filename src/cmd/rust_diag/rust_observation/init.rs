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

	let template = r#"# Observation (Read-only)
[[cargo-fmt]] # --check - domyślne bo jesteśmy w Observation
args = []
id = "oA1"
name = "check-fmt"
order = 0
output = "target/.x-do/logs/err/fmt-check.log"
description = "【ENG】 Check code formatting / 【POL】 Sprawdzenie formatowania kodu"

[[cargo-check]]
args = []
id = "oB1"
name = "check"
order = 1
output = "target/.x-do/logs/err/check.log"
description = "【ENG】 Compile check / 【POL】 Sprawdzenie kompilacji"

[[cargo-check]]
args = ["--examples"]
id = "oB2"
name = "check-examples"
order = 2
output = "target/.x-do/logs/err/check-examples.log"
description = "【ENG】 Check examples / 【POL】 Sprawdzenie przykładów"

[[cargo-check]]
args = ["--tests"]
id = "oB3"
name = "check-tests"
order = 3
output = "target/.x-do/logs/err/check-tests.log"
description = "【ENG】 Check tests / 【POL】 Sprawdzenie testów"

[[cargo-clippy]]
args = []
id = "oC1"
name = "clippy-basic"
order = 4
output = "target/.x-do/logs/err/clippy.log"
description = "【ENG】 Basic lints / 【POL】 Podstawowa analiza clippy"

[[cargo-clippy]]
args = ["--all-targets", "--all-features", "--", "-D", "warnings"]
id = "oC2"
name = "clippy-strict"
order = 5
output = "target/.x-do/logs/err/clippy-strict.log"
description = "【ENG】 Strict lints (deny warnings) / 【POL】 Rygorystyczne clippy (błędy na ostrzeżeniach)"

[[cargo-audit]]
args = []
id = "oF1"
name = "audit"
order = 12
output = "target/.x-do/logs/err/audit.log"
description = "【ENG】 Security audit / 【POL】 Audyt bezpieczeństwa"

[[cargo-doc]]
args = ["--no-deps",
	"--document-private-items",
	"--all-features"]
id = "oD1"
name = "doc"
order = 13
output = "target/.x-do/logs/err/doc.log"
description = "【ENG】 Generate documentation / 【POL】 Generowanie dokumentacji"

[[cargo-build]]
args = ["--release"]
id = "oG1"
name = "build-release"
order = 7
output = "target/.x-do/logs/err/build-release.log"
description = "【ENG】 Release build / 【POL】 Budowa wersji release"

[[cargo-build]]
args = []
id = "oG2"
name = "build-debug"
order = 6
output = "target/.x-do/logs/err/build-debug.log"
description = "【ENG】 Debug build / 【POL】 Budowa wersji debug"

[[cargo-test]]
args = ["--no-run"]
id = "oH1"
name = "test-no-run"
order = 8
output = "target/.x-do/logs/err/test-compile.log"
description = "【ENG】 Compile tests only / 【POL】 Kompilacja testów (bez uruchamiania)"

[[cargo-test]]
args = ["--verbose"]
id = "oH2"
name = "test-verbose"
order = 9
output = "target/.x-do/logs/err/test-verbose.log"
description = "【ENG】 Run tests (verbose) / 【POL】 Uruchomienie testów (szczegółowo)"

[[cargo-test]]
args = ["--doc"]
id = "oH3"
name = "test-doc"
order = 10
output = "target/.x-do/logs/err/test-doc.log"
description = "【ENG】 Test documentation examples / 【POL】 Testowanie przykładów w dokumentacji"

[[cargo-package]]
args = ["--list"]
id = "oI1"
name = "test-doc"
order = 11
output = "target/.x-do/logs/err/package-list.log"
description = "【ENG】 List package contents / 【POL】 Lista zawartości paczki"


[[cargo-verify-project]]
args = [ ]
id = "oJ1"
name = "verify"
order = 14
output = "target/.x-do/logs/err/verify.log"
description = "【ENG】 Verify Cargo.toml / 【POL】 Weryfikacja Cargo.toml"
"#;

	fs::write(config_path, template).context("Init failed: observation")?;
	println!("✨ 【ENG】 Created / 【POL】 Utworzono: {config_path}");
	Ok(())
}
