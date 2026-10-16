
# 🚀 cargo-x-do / cargo x

[![Crates.io](https://img.shields.io/crates/v/cargo-x-do.svg)](https://crates.io/crates/cargo-x-do)
[![Downloads](https://img.shields.io/crates/d/cargo-x-do.svg)](https://crates.io/crates/cargo-x-do)
[![GitHub stars](https://img.shields.io/github/stars/j-Cis/cargo-x-do.svg?style=social&label=Star)](https://github.com/j-Cis/cargo-x-do)
[![Docs.rs](https://docs.rs/cargo-x-do/badge.svg)](https://docs.rs/cargo-x-do)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.94.1%2B-yellow.svg)](https://www.rust-lang.org)

**【ENG】** Instant single-key TUI developer task runner, live diagnostics with AI log dumps, WebAssembly testbed, and Git release orchestrator for Cargo projects.

**【POL】** Błyskawiczny menedżer zadań TUI sterowany pojedynczymi klawiszami, diagnostyka na żywo ze zrzutami logów pod AI, środowisko testowe WebAssembly oraz orkiestrator wydań Git dla projektów Cargo.

---

## 📦 Installation, Update & Uninstall / Instalacja, aktualizacja i usuwanie

### 📥 Install / Instalacja

```bash
# Z rejestru crates.io / From crates.io
cargo install cargo-x-do

# Ze źródeł lokalnych / From local repository
cargo install --path .

```

### 🔄 Update / Aktualizacja

**【ENG】** To update to the latest version, run the install command again with the `--force` flag.
**【POL】** Aby zaktualizować do najnowszej wersji, uruchom ponownie polecenie instalacji z flagą `--force`.

```bash
cargo install cargo-x-do --force

```

### 🗑️ Uninstall / Odinstalowywanie

```bash
cargo uninstall cargo-x-do

```

---

## ⚡ Usage & Alias / Używanie i skróty

**【ENG】** You can run the tool directly as a cargo subcommand. To make it even faster, add a custom alias to your global or project `.cargo/config.toml`.
**【POL】** Narzędzie uruchamia się bezpośrednio jako podkomendę cargo. Aby korzystać z niego jeszcze szybciej, dodaj alias do globalnego lub projektowego pliku `.cargo/config.toml`.

```bash
# Wywołanie standardowe / Standard invocation
cargo x-do

```

### Alias configuration / Konfiguracja aliasów (`.cargo/config.toml`)

**【ENG】** Add the following aliases to `./.cargo/config.toml` (or `~/.cargo/config.toml`) for shorthand invocations:

**【POL】** Dodaj poniższe aliasy do pliku `./.cargo/config.toml` (lub `~/.cargo/config.toml`), aby korzystać ze skróconych wywołań:

```toml
[alias]
x = "x-do"
xtask = "x-do"

```

Uruchomienie / Launch:

```bash
cargo x

```

---

## 🎮 Interface / Interfejs TUI

### Level 1: Main Grid / Poziom 1: Siatka główna

```text
🚀 x-do (xtask)

  [Enter] ♻️  last      [l] 📜 last list

  [i] ℹ️  info      [b] 🏗️  build      [g] 🌿 git
  [r] ▶️  run       [o] 🔓 dry: OFF    [c] 📦 cargo
  [s] ⚙️  setup

❌  [q] 🔚 exit / [Ctrl+C]

```

* **`[Enter]` / `[l]**`: Natychmiastowe ponowienie ostatniego polecenia lub nawigacja strzałkami po interaktywnej liście historii (`.cargo/.x-do-history.txt`).
* **`[o] 🔓 dry`**: Globalny przełącznik trybu piaskownicy (Dry Run). Żadna komenda modyfikująca pliki ani wywołująca zmiany w Git/Cargo nie zostanie fizycznie wykonana.
* **`[s] ⚙️ setup`**: Automatyczna inicjalizacja środowiska projektu. Generuje ustandaryzowany plik `.rustfmt.toml`, dodaje globalne aliasy `cargo x` oraz tworzy zaawansowany plik konfiguracyjny zrzutów kodu `.cargo/.x-do.toml`.
* **`[q]` / `[Ctrl+C]**`: Bezpieczne, natychmiastowe wyjście z programu (przywrócenie kursora i wyłączenie raw mode).
* **`[b]` / `[Esc]**`: Płynny powrót z dowolnego podmenu do poziomu 1 bez zamykania procesu.

---

## 🔍 Diagnostics & AI Logs / Diagnostyka i Logi dla AI (`[i]`)

**【ENG】** One screen for live checks and dump capturing. Lowercase keys run checks interactively; uppercase keys capture full `stdout` and `stderr` directly to `./dump/ai_logs/*.dump.txt` for prompt context.

**【POL】** Jeden ekran do podglądu na żywo i zrzutów. Małe litery uruchamiają weryfikację w konsoli; wielkie litery (`Shift`) przechwytują pełny strumień `stdout` oraz `stderr` do plików `./dump/ai_logs/*.dump.txt`, gotowych do wklejenia w prompt AI.

```text
🔍 diagnostyka & logi (ekran / dump)

  [e] 📚 check example   [E] 💾 check📚 -> dump
  [c] ✅ check           [C] 💾 check   -> dump
  [x] 🔬 clippy          [X] 💾 clippy  -> dump
  [f] 🧹 fmt             [F] 💾 fmt chk -> dump
  [t] 🧪 test            [T] 💾 test    -> dump
  [w] 🌐 wasm-check      [W] 💾 wasm    -> dump
  [a] 🚀 all             [A] 💾 all     -> dump
  [d] 👀 file/dir tree   [D] 💾 snapshot-> dump

  [z] 🧽 clear: OFF
❌  [Esc] / [b] 🔙 back    [q] 🔚 exit

```

* **`[z] 🧽 clear`**: Przełącznik czyszczenia konsoli systemowej (`cls`/`clear`) przed wykonaniem zadania.
* **`[d]`**: Podgląd drzewa projektu na żywo z wagami i czasem modyfikacji bez zapisu na dysk. Opiera się na silniku `querypath-fmt`.
* **`[D]`**: Pełny, potężny zrzut kodu i struktury projektu do jednego dokumentu Markdown za pomocą `querypath-snapshot`. Zawiera meta-dane, podwójne indeksowanie drzewa oraz filtruje logi zgodnie ze specyfikacją `.x-do.toml`.

---

## 🚀 Run & Build Matrix / Uruchamianie i Budowanie (`[r]` / `[b]`)

```text
🚀 run & build (uruchamianie i kompilacja)

  ▶️ RUN                                 🏗️ BUILD
  [d] 🖥️ desk debug                     [D] 🖥️ desk build debug
  [w] 🌍 wasm host (127.0.0.1)          [W] 🌍 wasm build debug
  [s] 📶 wasm wifi (0.0.0.0 + IP)
  [r] ⚡ desk release                   [R] ⚡ desk build release
  [e] 🚀 wasm host release              [E] 📦 wasm build release
  [f] 🔥 wasm wifi release (+ IP)

  [c] 📡 check IP                       [t] 🎯 target add wasm32
  [z] 🧽 clear: OFF
❌  [Esc] / [b] 🔙 back    [q] 🔚 exit

```

* **WASM WiFi (`[s]` / `[f]`)**: Automatycznie weryfikuje cel kompilacji `wasm32-unknown-unknown`, pobiera lokalne adresy IPv4 Twoich interfejsów sieciowych LAN i uruchamia serwer `trunk` na porcie `0.0.0.0:8080`, ułatwiając szybkie testy na urządzeniach mobilnych.
* **Host WASM (`[w]` / `[e]`)**: Uruchamia `trunk serve` na porcie `127.0.0.1:8080` z automatycznym otwarciem przeglądarki.

---

## 🌿 Project, Git & Releases / Zarządzanie Projektem (`[g]` / `[c]`)

```text
🌿 project (git & cargo)

  🏷️ RELEASE & GIT                     📦 CARGO & CHANGES
  [t] 🏷️  tag + push (vX.X-hash)       [f] 🔧 fix (--allow-dirty)
  [v] 🔢 bump version Cargo.toml       [m] 🧹 fmt (+nightly)
  [s] 📊 status (short)                [k] 🔄 update dependencies
  [l] 📜 log (10 graf)                 [c] 🗑️  clean target/
  [d] 🔍 diff roboczy                  [y] 🌲 dependency tree

  ⏪ ROLLBACK & STASH                  ⚠️ CLEANUP
  [x] ↩️  unstage (restore staged)     [X] ⚠️  clean untracked (-fd)
  [u] ⏪ undo commit (soft HEAD~1)     [p] 📥 stash (schowaj)
  [U] ⚠️  undo commit (hard HEAD~1)     [P] 📤 stash pop (przywróć)

  [z] 🧽 clear: OFF
❌  [Esc] / [b] 🔙 back    [q] 🔚 exit

```

* **`[t]` Tag & Push**: Odczytuje bieżącą wersję projektu bezpośrednio z `Cargo.toml`, pobiera krótki hash wskaźnika (`git rev-parse --short HEAD`), tworzy adnotowany tag systemowy w formacie `v<version>-<hash>` i natychmiast synchronizuje go na serwer (`git push origin --tags`).
* **`[v]` Bump Version**: Inteligentnie parsuje i wykrywa obecny schemat wersji w `Cargo.toml` (uwzględniając cykle produkcyjne typu `-dev.X`), generuje podpowiedź dla kolejnego numeru i bezpiecznie aktualizuje plik manifestu `Cargo.toml`.
* **Rollback & Safety**: Błyskawiczne wycofywanie modyfikacji ze strefy staging (`git restore --staged .`), bezproblemowe cofanie commitów (`git reset --soft` oraz destrukcyjne `git reset --hard`) a także bezpieczne eliminowanie nieśledzonych śmieci i plików (`git clean -fd`).

---

## ⚙️ Configuration / Konfiguracja (`.cargo/.x-do.toml`)

**【ENG】** Easily invoke `[s] setup` from the main menu, or manually customize snapshot generation patterns and output rules by creating a `.x-do.toml` file inside `./.cargo/` (or `./.xtask/` / project root):

**【POL】** Uruchom polecenie `[s] setup` w głównym menu, lub ręcznie dostosuj szczegółowe reguły zrzutów kodu i docelowe parametry wizualizacji drzewa umieszczając plik konfiguracyjny w `./.cargo/.x-do.toml` (lub `./.xtask/` / katalogu głównym):

```toml
[snapshot]
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

```

---

## 📜 License / Licencja

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

at your option.

---

**Jan Roman Cisowski „j-Cis”** · [GitHub](https://github.com/j-Cis/cargo-x-do) · [Crates.io](https://crates.io/crates/cargo-x-do)
