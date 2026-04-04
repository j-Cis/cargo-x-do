use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::process::Command;

use crate::cmd::rust_diag::{push_tasks, Task};
use crate::shared::abstracts::AbstractWorkflowModule;

pub mod init;

#[derive(Deserialize)]
struct PlotConfig {
    #[serde(rename = "cargo-plot")] plot: Option<Vec<Task>>,
}

pub struct PlotModule;

impl AbstractWorkflowModule for PlotModule {
    type Task = Task;

    const CONFIG_PATH: &'static str = "./.x-do/cargo-plot.toml";
    const OUTPUT_PATH: &'static str = "./target/.x-do/code/";

    fn init_config(&self, force: bool) -> Result<()> {
        if force && std::path::Path::new(Self::CONFIG_PATH).exists() {
            fs::remove_file(Self::CONFIG_PATH).context("Failed to delete old config")?;
        }
        init::ensure_config(Self::CONFIG_PATH, Self::OUTPUT_PATH)?;
        Ok(())
    }

    fn load_tasks(&self) -> Result<Vec<Self::Task>> {
        let content = fs::read_to_string(Self::CONFIG_PATH)
            .with_context(|| format!("Failed to read {}", Self::CONFIG_PATH))?;
        let raw: PlotConfig = toml::from_str(&content).context("Failed to parse TOML")?;

        let mut tasks = Vec::new();
        // Tutaj 'cargo-plot' to nazwa binarki, nie subkomenda cargo
        push_tasks(&mut tasks, raw.plot, "cargo-plot", &[]);
        Ok(tasks)
    }

    fn print_list(&self, tasks: &[Self::Task]) {
        println!("📊 【ENG】 Plotting Tasks / 【POL】 Zadania Wykresów:");
        for t in tasks {
            use crate::shared::abstracts::AbstractExecutableTask;
            println!("  - {:<4} | [{:02}] {}", t.id(), t.order(), t.name());
        }
    }

    fn execute_task(&self, task: &Self::Task) -> Result<()> {
        println!("📈 Generowanie: {} ({})", task.name, task.id);
        
        let mut final_args = task.base_args.clone();
        final_args.extend(task.args.clone());

        let mut cmd = Command::new(&task.binary);
        // 🚨 Odcięcie pępowiny - usuwamy wpływ nadrzędnego Cargo na toolchain
        cmd.env_remove("RUSTUP_TOOLCHAIN"); 
        cmd.args(&final_args);

        let status = cmd.status()?;
        if status.success() {
            println!("✅ OK.");
        } else {
            println!("⚠️  Problem przy generowaniu: {}", task.name);
        }
        Ok(())
    }
}

pub fn handle(arg: Option<&str>) -> Result<()> {
    PlotModule.handle(arg)
}