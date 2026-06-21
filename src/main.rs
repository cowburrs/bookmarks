use core::fmt;
use std::{collections::HashMap, path::PathBuf};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use directories::{BaseDirs, ProjectDirs};

mod cli;
use cli::Commands;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
};

struct HomePath(PathBuf);

impl HomePath {}

impl Serialize for HomePath {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let base_dirs = BaseDirs::new().expect("Could not find BaseDirs");
        let home = base_dirs.home_dir();
        match self.0.as_path().strip_prefix(home) {
            Ok(thing) => {
                let portable = format!("$HOME/{}", thing.display());
                serializer.serialize_str(&portable)
            }
            Err(_) => self.0.as_path().serialize(serializer),
        }
    }
}

struct HomePathVisitor;
impl<'de> Visitor<'de> for HomePathVisitor {
    type Value = PathBuf;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("path string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let base_dirs = BaseDirs::new().expect("Could not find BaseDirs");
        let home = base_dirs.home_dir().join("");
        let path: PathBuf = From::from(v);
        let x = match path.strip_prefix("$HOME") {
            Ok(thing) => {
                let portable = format!("{}{}", home.display(), thing.display());
                From::from(portable)
            }
            Err(_) => path,
        };
        Ok(x)
    }
}

impl<'de> Deserialize<'de> for HomePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(HomePath(
            deserializer
                .deserialize_string(HomePathVisitor)
                .expect("Could not deserialize"),
        ))
    }
}

fn main() {
    let proj_dirs = ProjectDirs::from("com", "burrs", "bookmarks")
        .expect("Cannot find config directory for some reason??");
    let config = proj_dirs.config_dir().join("dirs");
    let args = cli::Args::parse();
    match args.command {
        Commands::Save(args) => {
            let cwd = std::env::current_dir().expect("Could not find current working directory");
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let mut dirs: HashMap<String, HomePath> =
                serde_json::from_str(&json).expect("Could not deserialize");
            dirs.insert(args.name.clone(), HomePath(cwd.clone()));
            let json = serde_json::to_string_pretty(&dirs).expect("Could not serialize");
            if let Some(thing) = config.parent() {
                std::fs::create_dir_all(thing).expect("Could not create conf directory");
            }
            std::fs::write(&config, json).expect("Could not write to file");
            println!("bookmarks: Saved {} as {}", args.name, cwd.display())
        }
        Commands::List(args) => {
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let dirs: HashMap<String, PathBuf> =
                serde_json::from_str(&json).expect("Could not deserialize");
            for (name, path) in dirs {
                if name.contains(args.search.as_str()) {
                    println!("{}: {}", name, path.display())
                }
            }
        }
        Commands::Delete(args) => {
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let mut dirs: HashMap<String, PathBuf> =
                serde_json::from_str(&json).expect("Could not deserialize");
            if dirs.remove(&args.name).is_none() {
                eprintln!("bookmarks: {} not found", args.name);
                return;
            }
            let json = serde_json::to_string_pretty(&dirs).unwrap();
            if let Some(thing) = config.parent() {
                std::fs::create_dir_all(thing).expect("Could not create conf directory");
            }
            std::fs::write(&config, json).expect("Could not write to file");
            println!("bookmarks: removed {}", args.name);
        }
        Commands::Go(args) => {
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let dirs: HashMap<String, PathBuf> =
                serde_json::from_str(&json).expect("Could not deserialize");
            let path = match dirs.get(&args.name.to_string()) {
                None => {
                    eprintln!("bookmarks: no match found, using zoxide instead");
                    println!("cd \"$(zoxide query {})\"", args.name);
                    return;
                }
                Some(thing) => thing,
            };
            let base_dirs = BaseDirs::new().expect("Could not find BaseDirs");
            let home = base_dirs.home_dir().join("");
            match path.strip_prefix("$HOME") {
                Ok(thing) => {
                    println!("cd {}{}", home.display(), thing.display());
                }
                Err(_) => {
                    println!("cd {}", path.display());
                }
            };
        }
        Commands::Completions { shell } => {
            let mut cmd = cli::Args::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
    };
}
