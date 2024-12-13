use clap::value_parser;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(name = "cli tool")]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, num_args = 1.., value_parser = value_parser!(PathBuf), required = true, help = "Files that will be linked.")]
    pub source: Vec<PathBuf>,
    #[arg(long, short, value_parser = value_parser!(PathBuf), required = true, help = "Directory or File where it will be placed")]
    pub target: PathBuf,
    #[arg(long, short, help = "If option is provided the symlink is removed.")]
    pub remove: bool,
}
