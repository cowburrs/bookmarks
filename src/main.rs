use core::fmt;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use clap::Parser;
use colored::Colorize;
use directories::{BaseDirs, ProjectDirs};

mod cli;
use cli::Commands;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
};
use strsim::jaro_winkler;

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
            match dirs.get(&args.name.clone()) {
                None => {
                    dirs.insert(args.name.clone(), HomePath(cwd.clone()));
                }
                _ => {
                    eprintln!("bookmarks: key exists, first delete bookmark");
                    return;
                }
            }
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
                if jaro_winkler(name.as_str(), args.search.as_str()) > 0.8 || args.search.is_empty()
                {
                    match args.short {
                        true => {
                            println!("{}", name)
                        }
                        false => {
                            println!("{:<10} -> {}", name, path.display())
                        }
                    }
                }
            }
        }
        Commands::Delete(args) => {
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let mut dirs: HashMap<String, PathBuf> =
                serde_json::from_str(&json).expect("Could not deserialize");
            if dirs.remove(&args.name).is_none() {
                let sim = &dirs
                    .iter()
                    .filter(|(k, _)| jaro_winkler(k, &args.name) > 0.8)
                    .map(|(k, _)| k.as_str().purple().to_string())
                    .collect::<Vec<String>>();
                let likely = match sim.as_slice() {
                    [] => "".to_string(),
                    [x] => x.to_string(),
                    [x @ .., xs] => format!("{}, or {}", x.join(", "), xs),
                };
                eprintln!(
                    "bookmarks: \"{}\" not found, did you mean: {}?",
                    args.name, likely
                );
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
            let base_dirs = BaseDirs::new().expect("Could not find BaseDirs");
            let home = base_dirs.home_dir().join("");
            let json = std::fs::read_to_string(&config).unwrap_or("{}".to_string());
            let dirs: HashMap<String, PathBuf> =
                serde_json::from_str(&json).expect("Could not deserialize");
            let cwd = std::env::current_dir().expect("Couldn't get cwd");

            let path: PathBuf = if Path::new(&args.name).is_dir() {
                PathBuf::from(&args.name)
            } else {
                match dirs.get(&args.name) {
                    Some(thing) => thing.clone(),
                    None => {
                        eprintln!("bookmarks: no match found, using zoxide instead");
                        let output = std::process::Command::new("zoxide")
                            .arg("query")
                            .arg(&args.name)
                            .output();
                        match output {
                            Err(_) => {
                                eprintln!("bookmarks: zoxide not found.");
                                cwd.clone()
                            }
                            Ok(out) => {
                                let stderr = String::from_utf8_lossy(&out.stderr);
                                eprint!("{}", stderr.trim());
                                if !stderr.trim().is_empty() {
                                    eprintln!()
                                }
                                let mut stdout =
                                    String::from_utf8_lossy(&out.stdout).trim().to_string();
                                if stdout.is_empty() {
                                    stdout = cwd.to_string_lossy().to_string();
                                }
                                PathBuf::from(stdout)
                            }
                        }
                    }
                }
            };
            match path.strip_prefix("$HOME") {
                Ok(thing) => match args.raw {
                    true => {
                        println!("{}{}", home.display(), thing.display());
                    }
                    false => {
                        println!("cd {}{}", home.display(), thing.display());
                    }
                },
                Err(_) => match args.raw {
                    true => {
                        println!("{}", path.display());
                    }
                    false => {
                        println!("cd {}", path.display());
                    }
                },
            };
        }
    };
}
