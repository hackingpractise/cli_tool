pub mod arguments;
use std::fs;

use anyhow::anyhow;
use arguments::Cli;
use clap::Parser;
use link::FromTo;

const ABOUT: &str = "cli tools to create symbolic links from one directory to another";
const VERSION: &str = "0.1.0";

pub fn main() -> anyhow::Result<()> {
    let error_handler = |e: anyhow::Error, c| {
        eprintln!("{e}");
        std::process::exit(c);
    };
    let error_handler_unwrap = |e| {
        eprintln!("{e}");
        std::process::exit(255);
    };
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

pub mod link {
    use crate::NOW;
    use anyhow::{anyhow, Result};
    use std::fs;
    use std::os::unix::fs as unix_fs;
    use std::path::{Path, PathBuf};

    #[derive(Clone, Debug)]
    pub struct FromTo {
        from: PathBuf,
        to: PathBuf,
    }
    impl FromTo {
        pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Self {
            Self {
                from: from.as_ref().to_path_buf(),
                to: to.as_ref().to_path_buf(),
            }
        }
        pub fn create_link(&self) -> anyhow::Result<()> {
            if !self.from.is_file() {
                Ok(self.aux_dir_traveler(20)?)
            } else if !self.to.exists() {
                Ok(unix_fs::symlink(&self.from, &self.to)?)
            } else if self.to.is_dir() {
                let to = self.to.join(self.from.iter().last().unwrap());
                Ok(unix_fs::symlink(&self.from, &to)?)
            } else {
                Err(anyhow!("Unxpected error at {}", line!()))
            }
        }
        fn aux_dir_traveler(&self, depth: usize) -> Result<()> {
            if depth < 1 {
                return Err(anyhow!("Too deep at {}", line!()));
            }
            let from = self.from.clone();
            let to = self.to.clone();
            if from.is_dir() {
                let mut vec: Vec<PathBuf> =
                    fs::read_dir(&from)?.map(|f| f.unwrap().path()).collect();
                vec.sort();
                for entry in vec.into_iter() {
                    let path = entry;
                    if path.is_dir() {
                        let new = path.iter().last().unwrap();
                        let new_to = to.join(new);
                        if new_to.exists() {
                            return Err(anyhow!("The to path is not empty: {new_to:?}"));
                        }
                        fs::create_dir(&new_to)?;
                        let new_self = Self {
                            from: path,
                            to: new_to,
                        };
                        Self::aux_dir_traveler(&new_self, depth - 1)?;
                    } else {
                        let n = path.iter().last().unwrap();
                        let new_to = to.join(n);
                        unix_fs::symlink(&from, &new_to)?;
                    }
                }
                Ok(())
            } else {
                Ok(unix_fs::symlink(&from, &to)?)
            }
        }
        pub fn remove_link(&self) -> anyhow::Result<()> {
            // fs::remove_file(&self.to)?;
            todo!()
        }
    }
}
