use serde::Deserialize;

pub mod rust_observation;
pub mod rust_corrections;

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