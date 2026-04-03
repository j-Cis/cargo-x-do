# 🚀 cargo-x-do

**【ENG】** A modular, cross-platform task runner and workflow orchestrator for Rust projects. Stop fighting `Makefile` syntax and embrace the power of Rust-driven automation with simple TOML configurations.

**【POL】** Modułowy, wieloplatformowy system automatyzacji zadań i orchestrator workflow dla projektów Rust. Przestań walczyć ze składnią `Makefile` i wykorzystaj potęgę automatyzacji opartej na Ruście z prostą konfiguracją TOML.

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

---

## 📋 Quick Start / Szybki Start

**【ENG】**

1. Create a `./.x-do/` folder in your project root.
2. Add your modular configuration files (e.g., `cargo-plot.toml`, `rust-dev.toml`).
3. Run your tasks using the cargo subcommand:
**【POL】**
4. Utwórz folder `./.x-do/` w głównym katalogu swojego projektu.
5. Dodaj modułowe pliki konfiguracyjne (np. `cargo-plot.toml`, `rust-dev.toml`).
6. Uruchamiaj zadania za pomocą subkomendy cargo:

```bash
cargo x-do plot pA2
```

---

## ⚙️ Compatibility / Kompatybilność

* **cargo-plot**: `>= 0.2.0`
* **Rust Edition**: `2024`

---

**【ENG】** Built with ❤️ by **Jan Roman Cisowski „j-Cis”**.  
**【POL】** Stworzone ❤️ przez **Jana Romana Cisowskiego „j-Cis”**.
