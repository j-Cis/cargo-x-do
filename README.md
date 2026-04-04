# 🚀 cargo-x-do

**【ENG】** A modular, cross-platform task runner and workflow orchestrator for Rust projects. Automates git versioning, diagnostics, and project snapshots via local TOML configurations.

**【POL】** Modułowy, wieloplatformowy menedżer zadań i orkiestrator przepływów pracy dla projektów Rust. Automatyzuje wersjonowanie git, diagnostykę i zrzuty projektu za pomocą lokalnych konfiguracji TOML.

---

## 🛠️ Installation / Instalacja

### 1. From Crates.io (Remote) / Z Crates.io (Zdalnie)

**【ENG】** Once the crate is published, install it directly from the registry:

**【POL】** Gdy paczka zostanie opublikowana, zainstaluj ją bezpośrednio z rejestru:

```bash
cargo install cargo-x-do
```

### 2. From GitHub (Remote) / Z GitHub (Zdalnie)

**【ENG】** Install the latest development version directly from the source:

**【POL】** Zainstaluj najnowszą wersję rozwojową bezpośrednio ze źródła:

```bash
cargo install --git https://github.com/j-Cis/cargo-x-do
```

### 3. Local Installation / Instalacja Lokalna

**【ENG】** If you are developing the tool itself, install it from the local directory:

**【POL】** Jeśli rozwijasz samo narzędzie, zainstaluj je z lokalnego katalogu:

```bash
# Inside cargo-x-do folder / Wewnątrz folderu cargo-x-do
cargo install --path .
```

---

## 🗑️ Uninstallation / Odinstalowanie

**【ENG】** To remove the tool from your system, simply run:

**【POL】** Aby usunąć narzędzie z systemu, po prostu wpisz:

```bash
cargo uninstall cargo-x-do
```

## 【ENG】 Features / 【POL】 Funkcje

* **【ENG】 TOML-based Configuration 【POL】 Konfiguracja oparta na TOML**: 
  【ENG】 Auto-generates and manages local settings in the `.x-do/` directory. 
  【POL】 Automatycznie generuje i zarządza lokalnymi ustawieniami w katalogu `.x-do/`.
* **【ENG】 Modular Plugins 【POL】 Modułowe Wtyczki**: 
    【ENG】 Divided into dedicated runners for formatting, checking, and snapshotting 
    【POL】 Podzielone na dedykowane moduły do formatowania, sprawdzania błędów i tworzenia zrzutów kodu.
* **【ENG】 Git Integration 【POL】 Integracja z Gitem**: 
  【ENG】 Automatically creates commits before or after running formatting tasks (e.g., `clippy-fix`, `rustfmt`). 
  【POL】 Automatycznie tworzy commity przed lub po wykonaniu zadań formatujących (np. `clippy-fix`, `rustfmt`).
* **【ENG】 Targeted Execution 【POL】 Wykonywanie Celowane**: 
  【ENG】 Run all tasks (`--all`) or specific ones by their `id` or `name`. 
  【POL】 Uruchamiaj wszystkie zadania (`--all`) lub wybrane pojedyncze przy pomocy ich `id` lub `name`.

---

## 【ENG】 Installation / 【POL】 Instalacja

【ENG】 To install `cargo-x-do` from crates.io (requires Rust 1.94.1+):

【POL】 Aby zainstalować `cargo-x-do` z crates.io (wymaga Rust 1.94.1+):

```bash
cargo install cargo-x-do
````

---

## 【ENG】 Usage / 【POL】 Użycie

【ENG】 The CLI supports three main subcommands, each representing a specific workflow plugin.

【POL】 Interfejs wiersza poleceń obsługuje trzy główne podkomendy, z których każda odpowiada za konkretną wtyczkę.

### 【ENG】 1. Diagnostics (check) / 【POL】 1. Diagnostyka (check)

【ENG】 Runs various `cargo check`, `cargo clippy`, `cargo test`, and `cargo audit` commands.

【POL】 Uruchamia różne komendy typu `cargo check`, `cargo clippy`, `cargo test` oraz `cargo audit`.

```bash
# 【ENG】 Aliases / 【POL】 Aliasy: check, c, err, e
cargo x-do check --init       # 【ENG】 Initialize config / 【POL】 Inicjalizacja konfiguracji
cargo x-do check --all        # 【ENG】 Run all tasks / 【POL】 Uruchom wszystkie zadania
cargo x-do check clippy-basic # 【ENG】 Run specific task by name / 【POL】 Uruchom konkretne zadanie po nazwie
cargo x-do check oC1          # 【ENG】 Run specific task by ID / 【POL】 Uruchom konkretne zadanie po ID
```

### 【ENG】 2. Formatting & Corrections (format) / 【POL】 2. Formatowanie i Poprawki (format)

【ENG】 Applies code fixes, trims whitespaces, and runs `rustfmt`. Can be configured to auto-commit changes via git.

【POL】 Aplikuje poprawki w kodzie, usuwa zbędne spacje i uruchamia `rustfmt`. Może być skonfigurowane do automatycznego tworzenia commitów w gicie.

```bash
# 【ENG】 Aliases / 【POL】 Aliasy: format, fmt, fix, f
cargo x-do format --init      # 【ENG】 Initialize config / 【POL】 Inicjalizacja konfiguracji
cargo x-do format --all       # 【ENG】 Run all formatters / 【POL】 Uruchom wszystkie formattery
cargo x-do format rustfmt     # 【ENG】 Run specific task / 【POL】 Uruchom konkretne zadanie
```

### 【ENG】 3. Project Snapshots (plot) / 【POL】 3. Zrzuty Projektu (plot)

【ENG】 Uses `cargo-plot` to generate text/markdown archives of your project for LLM context or backups.

【POL】 Używa narzędzia `cargo-plot` do generowania tekstowych archiwów projektu (markdown) jako kontekst dla LLM lub kopie zapasowe.

```bash
# 【ENG】 Aliases / 【POL】 Aliasy: plot, print, p
cargo x-do plot --init        # 【ENG】 Initialize config / 【POL】 Inicjalizacja konfiguracji
cargo x-do plot p1            # 【ENG】 Run snapshot task by ID / 【POL】 Uruchom zadanie zrzutu po ID
```

---

## 【ENG】 Configuration / 【POL】 Konfiguracja

【ENG】 Running any `--init` command will generate a `.x-do` folder in your project root with the following TOML files:

【POL】 Uruchomienie jakiejkolwiek komendy z flagą `--init` wygeneruje folder `.x-do` w głównym katalogu projektu, zawierający następujące pliki TOML:

* `.x-do/rust-checks.toml`
* `.x-do/rust-format.toml`
* `.x-do/cargo-plot.toml`

【ENG】 You can customize commands, arguments (`args`), outputs (`output`), and execution order (`order`) directly in these files.

【POL】 Możesz dostosować komendy, argumenty (`args`), pliki wyjściowe (`output`) oraz kolejność wykonywania (`order`) bezpośrednio w tych plikach.

### 【ENG】 Git Auto-commits (in `rust-format.toml`) / 【POL】 Automatyczne commity (w `rust-format.toml`)

【ENG】 The formatting engine supports global automatic commits:

【POL】 Silnik formatujący obsługuje globalne automatyczne commity:

```toml
git_commit = "per-task" # 【ENG】 options: "single", "per-task", "none" / 【POL】 opcje: "single", "per-task", "none"
when_commit = "before"  # 【ENG】 options: "before", "after", "combo" / 【POL】 opcje: "before", "after", "combo"
message_before = "before: apply automatic fix/fmt"
```

---

## 【ENG】 General Command Help / 【POL】 Ogólna Pomoc Zadań

```text
Usage/Użycie: cargo x-do [plot|err|fix] <id|id,id|--all|--init>
---
【ENG】 plot: snapshots | err: observation | fix: corrections
【POL】 plot: zrzuty    | err: obserwacja  | fix: poprawki
---
【ENG】 --init: force re-init | --all: run all | id,id: run multiple
【POL】 --init: wymuś init   | --all: wszystko | id,id: wywołaj wiele
---
【ENG】 (empty): lists tasks | (id): runs specific task by ID or Name
【POL】 (puste): lista zadań | (id): zadanie po ID lub nazwie
```

---

## 【ENG】 License / 【POL】 Licencja

【ENG】 Licensed under either of [MIT](https://www.google.com/search?q=LICENSE-MIT) or [Apache-2.0](https://www.google.com/search?q=LICENSE-APACHE) at your option.

【POL】 Udostępniane na licencji [MIT](https://www.google.com/search?q=LICENSE-MIT) lub [Apache-2.0](https://www.google.com/search?q=LICENSE-APACHE) (do wyboru).

---

**【ENG】** Built with ❤️ by **Jan Roman Cisowski „j-Cis”**.  
\n**【POL】** Stworzone ❤️ przez **Jana Romana Cisowskiego „j-Cis”**.
