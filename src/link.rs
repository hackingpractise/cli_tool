use anyhow::{anyhow, Result};
// use std::fs;
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
            Ok(self.dir_handler()?)
        } else if !self.to.exists() {
            Ok(unix_fs::symlink(&self.from, &self.to)?)
        } else if self.to.is_dir() {
            let to = self.to.join(self.from.iter().last().unwrap());
            Ok(unix_fs::symlink(&self.from, &to)?)
        } else {
            Err(anyhow!("Unxpected error at {}", line!()))
        }
    }
    fn dir_handler(&self) -> Result<()> {
        let from = &self.from;
        let to = &self.to;
        if from.is_dir() {
            for entry in from.read_dir()? {
                let from = entry?.path();
                let to = to.join(from.iter().last().unwrap());
                if to.exists() {
                    return Err(anyhow!("The to path is not empty: {to:?}"));
                }
                unix_fs::symlink(from, to)?;
            }
            Ok(())
        } else {
            if to.exists() {
                return Err(anyhow!("The to path is not empty: {to:?}"));
            }
            Ok(unix_fs::symlink(&from, &to)?)
        }
    }
    pub fn remove_link(&self) -> anyhow::Result<()> {
        // fs::remove_file(&self.to)?;
        todo!()
    }
}
