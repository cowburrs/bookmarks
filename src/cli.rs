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
    /// Save a bookmark
    Save(Save),
    /// List bookmarks using string similarity scores
    List(List),
    /// Go to a bookmark.
    Go(Go),
    /// Delete a bookmark
    Delete(Delete),
    /// shell completions (not implemented in nix)
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Parser, Debug)]
pub struct Save {
    /// The alias of the bookmark to save to
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct List {
    #[arg(default_value = "")]
    ///
    pub search: String,
    #[arg(short, long)]
    /// This will shorten the syntax e.g
    /// `nix        -> $HOME/nixos` will shorten down to `nix`
    /// You may find this useful if you need only the aliases.
    pub short: bool,
}

#[derive(Parser, Debug)]
pub struct Go {
    /// the alias to cd to.
    pub name: String,
    #[arg(short, long)]
    /// This flag will strip the cd at the start
    /// Useful if you have an alias and need to find the corresponding
    /// directory
    pub raw: bool,
}

#[derive(Parser, Debug)]
pub struct Delete {
    /// The alias to delete
    pub name: String,
}
