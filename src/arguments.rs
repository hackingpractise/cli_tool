use clap::value_parser;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(name = "cli tool")]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, num_args = 1.., value_parser = value_parser!(PathBuf), required = true)]
    pub source: Vec<PathBuf>,
    #[arg(long, short, value_parser = value_parser!(PathBuf), required = true)]
    pub target: PathBuf,
    #[arg(long, short)]
    pub remove: bool,
}
