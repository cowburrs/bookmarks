pub use clap::Parser;
use clap_complete::Shell;

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
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Parser, Debug)]
pub struct Save {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct List {
    #[arg(default_value = "")]
    pub search: String,
    #[arg(short, long)]
    pub short: bool,
}

#[derive(Parser, Debug)]
pub struct Go {
    pub name: String,
    #[arg(short = 's', long = "short")]
    pub raw: bool,
}

#[derive(Parser, Debug)]
pub struct Delete {
    pub name: String,
}
