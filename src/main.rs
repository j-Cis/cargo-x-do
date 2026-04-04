mod cmd;
mod init;
mod shared;

use anyhow::Result;
use crate::cmd::cargo_plot;
use crate::cmd::rust_diag;

fn main() -> Result<()> {
    init::ensure_workspace()?;	
    let mut args: Vec<String> = std::env::args().collect();
	if args.get(1).map(String::as_str) == Some("x-do") { args.remove(1); }
    
    match args.get(1).map(String::as_str) {
        Some("plot") => cargo_plot::handle(args.get(2).map(String::as_str))?,
        Some("err") => rust_diag::rust_observation::handle(args.get(2).map(String::as_str))?,
        Some("fmt") => rust_diag::rust_corrections::handle(args.get(2).map(String::as_str))?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    println!("Usage/Użycie: cargo x-do [plot|err|fix] <id>");
    println!("【ENG】 plot: snapshots | err: observation | fix: corrections");
    println!("【POL】 plot: zrzuty    | err: obserwacja  | fix: poprawki");
}