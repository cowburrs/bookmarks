use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use directories::ProjectDirs;

mod cli;
use cli::Commands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Dirs {
    dirs: HashMap<String, PathBuf>,
}

fn main() {
    let proj_dirs = ProjectDirs::from("com", "burrs", "bookmarks")
        .expect("Cannot find config directory for some reason??");
    let config = proj_dirs.config_dir().join("dirs");
    let args = cli::Args::parse();
    match args.command {
        Commands::Save(args) => { // TODO: Make it so that it saves /home/burrs into $HOME
            let cwd = std::env::current_dir().unwrap();
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let mut dirs: HashMap<String, PathBuf> = serde_json::from_str(&json).unwrap();
            dirs.insert(args.name.clone(), cwd.clone());
            let json = serde_json::to_string_pretty(&dirs).unwrap();
            println!("{}", json);
            std::fs::create_dir_all(config.parent().unwrap()).unwrap();
            std::fs::write(&config, json).unwrap();
            println!("Saved {} as {}", args.name, cwd.display())
        }
        Commands::List(args) => {
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let dirs: HashMap<String, PathBuf> = serde_json::from_str(&json).unwrap();
            for (name, path) in dirs {
                if name.contains(args.search.as_str()) {
                    println!("{}: {}", name, path.display())
                }
            }
        }
        Commands::Delete(args) => {
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let mut dirs: HashMap<String, PathBuf> = serde_json::from_str(&json).unwrap();
            if let None = dirs.remove(&args.name) {
                println!("{} not found!", args.name);
                return;
            }
            let json = serde_json::to_string_pretty(&dirs).unwrap();
            std::fs::create_dir_all(config.parent().unwrap()).unwrap();
            std::fs::write(&config, &json).unwrap();
            println!("Success: removed {}", args.name);
            println!("{}", json);
        }
        Commands::Go(args) => {
            let cwd = std::env::current_dir().unwrap();
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let dirs: HashMap<String, PathBuf> = serde_json::from_str(&json).unwrap();
            match dirs.get(&args.name.to_string()) {
                None => {
                    println!("{}", cwd.display());
                }
                Some(thing) => {
                    println!("{}", thing.display());
                }
            }
        }
    };
}
