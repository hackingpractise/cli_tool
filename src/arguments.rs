use anyhow::anyhow;
use anyhow::Result;
use clap::value_parser;
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::{ABOUT, VERSION};

#[derive(Parser, Clone, Debug)]
#[command(name = "cli tool")]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, num_args = 1.., value_parser = value_parser!(PathBuf))]
    pub source: Vec<PathBuf>,
    #[arg(long, short, value_parser = clap::value_parser!(PathBuf), required = true)]
    pub target: PathBuf,
    #[arg(long, short)]
    pub remove: bool,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub source: Vec<PathBuf>,
    pub target: PathBuf,
    pub remove: bool,
}

impl Arguments {
    pub fn parse() -> Self {
        Self::try_parse().unwrap()
    }
    pub fn try_parse() -> Result<Self> {
        let source_arg = Arg::new("source")
            .short('s')
            .long("source")
            .value_parser(value_parser!(PathBuf))
            .help("Files that will be linked.")
            .default_value(".")
            .value_delimiter(' ')
            .num_args(1..);
        let target_arg = Arg::new("target")
            .short('t')
            .long("target")
            .value_parser(value_parser!(PathBuf))
            .help("Directory or File where it will be placed")
            .required(true)
            .num_args(1);
        let remove_arg = Arg::new("remove")
            .short('r')
            .long("remove")
            .help("If option is provided the symlink is removed.")
            .action(ArgAction::SetTrue);
        let arg = Command::new("File link manager")
            .version(VERSION)
            .about(ABOUT)
            .args(&[source_arg, target_arg, remove_arg])
            .get_matches();
        let source: Vec<PathBuf> = arg
            .try_get_many::<PathBuf>("source")?
            .ok_or(anyhow!("Not enough arguments."))?
            .cloned()
            .collect();
        let target: PathBuf = arg
            .try_get_one::<PathBuf>("target")?
            .ok_or(anyhow!("Not enough arguments"))?
            .to_path_buf();
        let remove: bool = *arg
            .try_get_one::<bool>("remove")?
            .ok_or(anyhow!("Not enough arguments."))?;
        Ok(Self {
            source,
            target,
            remove,
        })
    }
    pub fn parse_from<T, I>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from::<T, I>(itr).unwrap()
    }
    pub fn try_parse_from<T, I>(itr: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let source_arg = Arg::new("source")
            .short('s')
            .long("source")
            .value_parser(value_parser!(PathBuf))
            .help("Files that will be linked.")
            .default_value(".")
            .value_delimiter(' ')
            .num_args(1..);
        let target_arg = Arg::new("target")
            .short('t')
            .long("target")
            .value_parser(value_parser!(PathBuf))
            .help("Directory or File where it will be placed")
            .required(true)
            .num_args(1);
        let remove_arg = Arg::new("remove")
            .short('r')
            .long("remove")
            .help("If option is provided the symlink is removed.")
            .action(ArgAction::SetTrue);
        let arg = Command::new("File link manager")
            .args(&[source_arg, target_arg, remove_arg])
            .try_get_matches_from(itr)?;
        let source: Vec<PathBuf> = arg
            .try_get_many::<PathBuf>("source")?
            .ok_or(anyhow!("Not enough arguments."))?
            .cloned()
            .collect();
        let target: PathBuf = arg
            .try_get_one::<PathBuf>("target")?
            .ok_or(anyhow!("Not enough arguments"))?
            .to_path_buf();
        let remove: bool = *arg
            .try_get_one::<bool>("remove")?
            .ok_or(anyhow!("Not enough arguments."))?;
        Ok(Self {
            source,
            target,
            remove,
        })
    }
}
