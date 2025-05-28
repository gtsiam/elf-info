use std::{env, fs, io, path::PathBuf};

use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;

#[allow(unused)]
#[path = "src/args.rs"]
mod args;

fn main() -> Result<(), io::Error> {
    println!("cargo::rerun-if-changed=src/args.c");
    println!("cargo:rerun-if-env-changed=COMPLETIONS_DIR");

    let Some(completions_dir) = env::var_os("COMPLETIONS_DIR")
        .map(PathBuf::from)
        .or_else(|| env::var_os("OUT_DIR").map(|out| PathBuf::from(out).join("completions")))
    else {
        return Ok(());
    };

    fs::create_dir_all(&completions_dir)?;

    let mut cmd = args::Options::command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut cmd, "elf", &completions_dir)?;
    }

    Ok(())
}
