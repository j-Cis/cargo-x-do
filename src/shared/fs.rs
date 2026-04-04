// src\shared\fs.rs

use std::collections::HashMap;
use std::fs;
// use serde::Deserialize;
use anyhow::{Context, Result};
use chrono::Local;
use std::io::Write;
use std::path::PathBuf;
use toml;
use toml::Value;

#[allow(dead_code)]
pub trait DefaultSettings {
	const FOLDER_OUT: &'static str = "target/";
	const FOLDER_ROOT: &'static str = "./";
	const FOLDER_MAIN: &'static str = ".x-do/";
	const FOLDER_SETTINGS: &'static str = "./.x-do/";
	const FOLDER_TASKSJOB: &'static str = "jobs/";
	const FOLDER_TEMPORAL: &'static str = "temp/";

	fn ensure_out_exist(name_out_folder: &str) {
		let mut path = PathBuf::from(Self::FOLDER_ROOT);
		path.push(Self::FOLDER_OUT);
		path.push(name_out_folder);

		if !path.exists() {
			fs::create_dir_all(&path).expect("Cannot create directory");
		}
	}

	fn build_path_set(file: &str) -> PathBuf {
		let mut path = PathBuf::from(Self::FOLDER_ROOT);
		path.push(Self::FOLDER_MAIN);
		path.push(Self::FOLDER_TASKSJOB);
		path.push(file);
		path
	}

	fn build_path_tmp(file: &str) -> PathBuf {
		let mut path = PathBuf::from(Self::FOLDER_ROOT);
		path.push(Self::FOLDER_MAIN);
		path.push(Self::FOLDER_TEMPORAL);
		// prefix_file [YYYYMMDDHHMMSSSSS_filename.ext]
		let timestamp = Local::now().format("%Y%m%d%H%M%S%3f"); // rok, miesiąc, dzień, godz, min, sek, milisekundy
		let prefix_file = format!("{}_{}", timestamp, file);

		path.push(prefix_file);
		path
	}

	fn init_if_not_exist(file: &str, template: &str) {
		let path = Self::build_path_set(file);
		println!("init_if_not_exist: {:?}", path);

		if !path.exists() {
			if let Some(parent) = path.parent() {
				fs::create_dir_all(parent).expect("Cannot create directories");
			}

			let mut f = fs::File::create(&path).expect("Cannot create file");
			f.write_all(template.as_bytes()).expect("Cannot write template");

			println!("created: {:?}", path);
		} else {
			println!("exists (skipped): {:?}", path);
		}
	}

	fn init_clear_safest(file: &str, template: &str) {
		let path = Self::build_path_set(file);
		let temp = Self::build_path_tmp(file);
		println!("init_clear_safest: {:?}", path);

		if path.exists() {
			if let Some(parent) = temp.parent() {
				fs::create_dir_all(parent).expect("Cannot create temp directories");
			}
			fs::rename(&path, &temp).expect("Cannot move existing file to temp");
			println!("moved existing file to temp: {:?}", temp);
		}

		// teraz tworzymy nowy plik w path
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent).expect("Cannot create directories");
		}

		let mut f = fs::File::create(&path).expect("Cannot create file");
		f.write_all(template.as_bytes()).expect("Cannot write template");
		println!("fresh init: {:?}", path);
	}

	fn init_clear_forced(file: &str, template: &str) {
		let path = Self::build_path_set(file);
		println!("init_clear_forced: {:?}", path);

		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent).expect("Cannot create directories");
		}

		let mut f = fs::File::create(&path).expect("Cannot overwrite file");
		f.write_all(template.as_bytes()).expect("Cannot write template");

		println!("overwritten: {:?}", path);
	}

	// 🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀🔀

	/// Pobiera pojedynczą wartość z TOML po zwykłym kluczu
	fn get_one_by_key(file: &str, key: &str) -> Result<Value> {
		// budujemy ścieżkę do pliku
		let path = Self::build_path_set(file);

		// wczytanie całego pliku
		let content =
			fs::read_to_string(&path).with_context(|| format!("Cannot read file {:?}", path))?;

		// parsowanie TOML
		let value: Value = toml::from_str(&content)
			.with_context(|| format!("Cannot parse TOML from {:?}", path))?;

		// pobranie klucza
		if let Some(val) = value.get(key) {
			Ok(val.clone())
		} else {
			anyhow::bail!("Key '{}' not found in {:?}", key, path);
		}
	}

	/// Pobiera wiele zwykłych kluczy z TOML i zwraca ich wartości jako Vec<(String, Value)>
	fn get_all_by_all_key(file: &str, keys: &[&str]) -> Result<Vec<(String, Value)>> {
		let path = Self::build_path_set(file);

		// wczytanie całego pliku
		let content =
			fs::read_to_string(&path).with_context(|| format!("Cannot read file {:?}", path))?;

		// parsowanie TOML
		let value: Value = toml::from_str(&content)
			.with_context(|| format!("Cannot parse TOML from {:?}", path))?;

		let mut results = Vec::new();

		for &key in keys {
			if let Some(val) = value.get(key) {
				results.push((key.to_string(), val.clone()));
			} else {
				anyhow::bail!("Key '{}' not found in {:?}", key, path);
			}
		}

		Ok(results)
	}

	/// Wyszukuje fragment TOML po anchorze i podanym kluczu (id lub name)
	fn get_one_by_anchor_and_key(
		file: &str,
		anchor: &str,
		key_value: &str, // wartość, którą szukamy
		key_is_id: bool, // true = szukamy w polu "id", false = w polu "name"
	) -> Result<Value> {
		let path = Self::build_path_set(file);
		println!("get_one_get_one_get_one_by_anchor_and_key: {:?}", path);

		// wczytaj cały plik
		let content =
			fs::read_to_string(&path).with_context(|| format!("Cannot read file {:?}", path))?;

		// sparsuj TOML
		let value: Value = toml::from_str(&content)
			.with_context(|| format!("Cannot parse TOML from {:?}", path))?;

		// przetwórz anchor: np. "[[task]]" → "task"
		let anchor_key = anchor.trim_matches(|c| c == '[' || c == ']');

		if let Some(items) = value.get(anchor_key) {
			if let Value::Array(arr) = items {
				// wybieramy pole do wyszukania
				let field_to_search = if key_is_id { "id" } else { "name" };

				for item in arr {
					if let Value::Table(table) = item {
						if let Some(Value::String(s)) = table.get(field_to_search) {
							if s == key_value {
								// jeśli znaleziono dopasowanie, zwracamy cały fragment
								return Ok(item.clone());
							}
						}
					}
				}
			}
		}

		anyhow::bail!(
			"No anchor {} with {}={} found in {:?}",
			anchor,
			if key_is_id { "id" } else { "name" },
			key_value,
			path
		);
	}

	/// Zwraca wszystkie elementy danego anchoru jako Vec<Value>
	fn get_all_by_anchor(file: &str, anchor: &str) -> Result<Vec<Value>> {
		let path = Self::build_path_set(file);
		println!("get_all_by_anchor: {:?}", path);

		// wczytaj plik
		let content =
			fs::read_to_string(&path).with_context(|| format!("Cannot read file {:?}", path))?;

		// parsuj TOML
		let value: Value = toml::from_str(&content)
			.with_context(|| format!("Cannot parse TOML from {:?}", path))?;

		// "[[task]]" → "task"
		let anchor_key = anchor.trim_matches(|c| c == '[' || c == ']');

		if let Some(items) = value.get(anchor_key) {
			if let Value::Array(arr) = items {
				// zwracamy wszystkie elementy jako Vec<Value>
				return Ok(arr.clone());
			}
		}

		anyhow::bail!("No anchor {} found in {:?}", anchor, path);
	}

	/// Zwraca wszystkie elementy dla wielu anchorów jako jeden Vec<Value>
	fn get_all_by_all_anchor(file: &str, anchors: &[&str]) -> Result<Vec<Value>> {
		let path = Self::build_path_set(file);
		println!("get_all_by_all_anchor: {:?}", path);

		let content =
			fs::read_to_string(&path).with_context(|| format!("Cannot read file {:?}", path))?;

		let value: Value = toml::from_str(&content)
			.with_context(|| format!("Cannot parse TOML from {:?}", path))?;

		let mut results = Vec::new();

		for &anchor in anchors {
			let anchor_key = anchor.trim_matches(|c| c == '[' || c == ']');

			if let Some(items) = value.get(anchor_key) {
				if let Value::Array(arr) = items {
					results.extend(arr.clone()); // ← kluczowe
				}
			}
		}

		if results.is_empty() {
			anyhow::bail!("No anchors {:?} found in {:?}", anchors, path);
		}

		Ok(results)
	}

	fn load_anchors_list(
		file: &str,
		anchors: &[&str],
		keys_by_anchors: &[&str],
	) -> Result<Vec<HashMap<String, String>>> {
		let path = Self::build_path_set(file);
		println!("load_anchors_list: {:?}", path);

		// wczytanie całego pliku
		let content =
			fs::read_to_string(&path).with_context(|| format!("Cannot read file {:?}", path))?;

		// sparsowanie do toml::Value
		let value: toml::Value = toml::from_str(&content)
			.with_context(|| format!("Cannot parse TOML from {:?}", path))?;

		let mut results = Vec::new();

		for &anchor in anchors {
			// anchor = np. "[[task]]" → usuwamy [] i dostajemy "task"
			let key = anchor.trim_matches(|c| c == '[' || c == ']');

			if let Some(items) = value.get(key) {
				if let toml::Value::Array(arr) = items {
					for item in arr {
						let mut map = HashMap::new();
						if let toml::Value::Table(table) = item {
							for &k in keys_by_anchors {
								if let Some(v) = table.get(k) {
									// konwertujemy wszystko na String
									let s = match v {
										toml::Value::String(s) => s.clone(),
										toml::Value::Integer(i) => i.to_string(),
										toml::Value::Float(f) => f.to_string(),
										toml::Value::Boolean(b) => b.to_string(),
										_ => continue, // ignorujemy tablice, inne struktury
									};
									map.insert(k.to_string(), s);
								}
							}
						}
						results.push(map);
					}
				}
			}
		}

		Ok(results)
	}

	/// Wersja "plot", która wypisuje od razu każdy anchor i jego klucze
	fn plot_anchors_list(file: &str, anchors: &[&str], keys_by_anchors: &[&str]) -> Result<()> {
		let path = Self::build_path_set(file);
		println!("plot_anchors_list: {:?}", path);

		// wczytaj plik TOML
		let content =
			fs::read_to_string(&path).with_context(|| format!("Cannot read file {:?}", path))?;

		let value: Value = toml::from_str(&content)
			.with_context(|| format!("Cannot parse TOML from {:?}", path))?;

		// iterujemy po anchorach
		for &anchor in anchors {
			let key = anchor.trim_matches(|c| c == '[' || c == ']');

			if let Some(items) = value.get(key) {
				if let Value::Array(arr) = items {
					for item in arr {
						if let Value::Table(table) = item {
							// tworzę tymczasową mapę tylko do wypisania
							let mut map = HashMap::new();
							for &k in keys_by_anchors {
								if let Some(v) = table.get(k) {
									let s = match v {
										Value::String(s) => s.clone(),
										Value::Integer(i) => i.to_string(),
										Value::Float(f) => f.to_string(),
										Value::Boolean(b) => b.to_string(),
										_ => continue,
									};
									map.insert(k.to_string(), s);
								}
							}
							println!("{:?}", map); // wypisujemy od razu
						}
					}
				}
			}
		}

		Ok(())
	}
}
