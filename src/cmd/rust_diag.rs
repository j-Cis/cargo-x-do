use serde::Deserialize;

use crate::shared::AbstractExecutableTask;

pub mod rust_corrections;
pub mod rust_observation;

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
	pub id: String,
	pub name: String,
	pub order: i32,
	#[serde(default)]
	pub args: Vec<String>,
	pub output: Option<String>,
	#[allow(dead_code)]
	pub description: Option<String>,

	// Pola specyficzne dla poprawek
	pub git_commit: Option<bool>,
	pub commit_message: Option<String>,

	// Pola wewnętrzne silnika (nie ma ich w TOML)
	#[serde(skip)]
	pub binary: String,
	#[serde(skip)]
	pub base_args: Vec<String>,
}

impl AbstractExecutableTask for Task {
	fn id(&self) -> &str { &self.id }
	fn name(&self) -> &str { &self.name }
	fn order(&self) -> i32 { self.order }
}

// Funkcja pomocnicza do budowania płaskiej listy zadań
pub fn push_tasks(all: &mut Vec<Task>, list: Option<Vec<Task>>, bin: &str, base: &[&str]) {
	if let Some(tasks) = list {
		for mut t in tasks {
			t.binary = bin.to_string();
			t.base_args = base.iter().map(|s| s.to_string()).collect();
			all.push(t);
		}
	}
}
