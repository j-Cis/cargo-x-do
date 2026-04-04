use anyhow::{Context, Result};

/// 【ENG】 Interface for tasks so the engine can sort and find them.
/// 【POL】 Interfejs zadania, aby silnik mógł je sortować i wyszukiwać.
pub trait AbstractExecutableTask {
	fn id(&self) -> &str;
	fn name(&self) -> &str;
	fn order(&self) -> i32;
}

/// 【ENG】 Core engine for all x-do modules.
/// 【POL】 Główny silnik dla wszystkich modułów x-do.
pub trait AbstractWorkflowModule {
	type Task: AbstractExecutableTask;

	/// Ścieżka do pliku TOML z konfiguracją modułu
	const CONFIG_PATH: &'static str;

	/// Główny katalog, do którego moduł zrzuca logi / artefakty
	const OUTPUT_PATH: &'static str;

	/// Inicjuje plik konfiguracyjny. `force=true` przy `--init` (wymusza nadpisanie).
	fn init_config(&self, force: bool) -> Result<()>;

	/// Wczytuje i spłaszcza listę zadań z pliku TOML.
	fn load_tasks(&self) -> Result<Vec<Self::Task>>;

	/// Wypisuje listę zadań (przy wywołaniu bez argumentów).
	fn print_list(&self, tasks: &[Self::Task]);

	/// Uruchamia konkretne zadanie.
	fn execute_task(&self, task: &Self::Task) -> Result<()>;

	/// 🧠 Wbudowany, inteligentny router komend.
	/// Domyślna implementacja - nie musisz jej pisać w modułach!
	fn handle(&self, arg: Option<&str>) -> Result<()> {
		// 1. Wymuszona inicjalizacja: `cargo x-do [nazwa] --init`
		if arg == Some("--init") {
			self.init_config(true)?;
			return Ok(());
		}

		// 2. Standardowe zapewnienie konfiguracji (nie nadpisuje)
		self.init_config(false)?;

		// 3. Pobranie zadań
		let mut tasks = self.load_tasks()?;

		// 4. Analiza argumentu
		match arg {
			None => {
				// Brak argumentu -> Wyświetl listę
				tasks.sort_by_key(AbstractExecutableTask::order);
				self.print_list(&tasks);
				Ok(())
			}
			Some("--all") => {
				// Flaga --all -> Wykonaj wszystkie po kolei według 'order'
				tasks.sort_by_key(AbstractExecutableTask::order);
				for task in tasks {
					self.execute_task(&task)?;
				}
				Ok(())
			}
			Some(targets_str) => {
				// Konkretny target lub lista "id1,id2,id3"
				let targets: Vec<&str> = targets_str.split(',').map(str::trim).collect();

				for target in targets {
					let task = tasks
						.iter()
						.find(|t| t.id() == target || t.name() == target)
						.with_context(|| {
							format!("❌ Nie znaleziono zadania: {target} w {}", Self::CONFIG_PATH)
						})?;

					self.execute_task(task)?;
				}
				Ok(())
			}
		}
	}
}
