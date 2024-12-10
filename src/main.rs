pub mod arguments;
pub mod link;
use std::fs;

use anyhow::anyhow;
use arguments::Cli;
use clap::Parser;
use link::FromTo;

pub fn main() -> anyhow::Result<()> {
    let error_handler = |e: anyhow::Error, c| {
        eprintln!("{e}");
        std::process::exit(c);
    };
    let error_handler_unwrap = |e| error_handler(e, 255);
    let args_input = Cli::parse();
    let from = args_input.source;
    let to = args_input.target;

    from.iter().for_each(|f| {
        if !f.exists() {
            error_handler(anyhow!("File {f:?} does not exist"), 2);
        }
    });
    // Checks if the file exist and prevents it from being overwritten.
    if to.exists() {
        return Err(anyhow!("Can not overwrite the file {to:?}"));
    }
    if from.len() > 1 && !to.exists() {
        fs::create_dir(&to)?;
    }
    if from.iter().filter(|f| f.is_dir()).count() > 0 {
        fs::create_dir(&to)?;
    }
    if from.len() == 1 {
        FromTo::new(&from[0], &to).create_link()?;
    }
    from.into_iter().for_each(|f| {
        FromTo::new(&f, &to)
            .create_link()
            .unwrap_or_else(error_handler_unwrap)
    });
    Ok(())
}
