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
            let mut vec: Vec<PathBuf> = fs::read_dir(&from)?.map(|f| f.unwrap().path()).collect();
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
