mod cmd;
mod init;

use anyhow::Result;
use crate::cmd::cargo_plot;

fn main() -> Result<()> {
	// 0. 【ENG】 Initial workspace check / 【POL】 Wstępne sprawdzenie przestrzeni roboczej
    init::ensure_workspace()?;
	
    let mut args: Vec<String> = std::env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("x-do") {
        args.remove(1);
    }

    match args.get(1).map(|s| s.as_str()) {
        Some("plot") => {
            cargo_plot::handle(args.get(2).map(|s| s.as_str()))?;
        }
        _ => {
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("🚀 cargo-x-do | 【ENG】 Workflow Orchestrator / 【POL】 Orchestrator Workflow");
    println!("\n【ENG】 Available modules: / 【POL】 Dostępne moduły:");
    println!("  plot      - 【ENG】 Snapshot & Visualization / 【POL】 Zrzuty i wizualizacja (cargo-plot)");
    
    println!("\n【ENG】 Usage: / 【POL】 Użycie:");
    println!("  cargo x-do plot [task_id]");
}