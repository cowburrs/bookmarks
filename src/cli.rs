pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    Save(Save),
    List(List),
    Go(Go),
    Delete(Delete),
}

#[derive(Parser, Debug)]
pub struct Save {
    #[arg(default_value = "")]
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct List {
    #[arg(default_value = "")]
    pub search: String,
}

#[derive(Parser, Debug)]
pub struct Go {
    #[arg(default_value = "default")]
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct Delete {
    #[arg(default_value = "default")]
    pub name: String,
}
