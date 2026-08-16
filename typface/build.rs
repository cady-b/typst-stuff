use clap::{CommandFactory, ValueEnum};
use clap_complete::{Shell, generate_to};
use std::env;

include!("src/args.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=src/args.rs");

    if env::var_os("OUT_DIR").is_none() {
        return Ok(());
    }

    let outdir = "target/completions";
    std::fs::create_dir_all(outdir);

    let mut cmd = Cli::command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "typface", &outdir)?;
    }

    Ok(())
}
