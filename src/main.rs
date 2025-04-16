use std::process;

use clap::Parser;
use envoke::cli::{Cli, Command};
use envoke::commands::{create, current, init, list, remove, switch};
use envoke::config::Config;
use envoke::fs;
use envoke::profile::ProfileManager;

fn main() {
    let args = Cli::parse();
    let config = Config::default();
    let fs = fs::EnvokeFileSystem::new();
    let manager = ProfileManager::new(config, fs);

    let out = match args.command {
        Command::Init => init::run(&manager),
        Command::Create { profile } => create::run(&manager, profile),
        Command::Switch { profile, force } => switch::run(&manager, profile, force),
        Command::Remove { profile } => remove::run(&manager, profile),
        Command::List => list::run(&manager),
        Command::Current => current::run(&manager),
    };

    if let Err(e) = out {
        eprintln!("{}", e);
        process::exit(1);
    }
}
