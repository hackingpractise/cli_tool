use anyhow::anyhow;
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::{self, Path, PathBuf};

fn create_slink<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> anyhow::Result<()> {
    let from = path::absolute(from)?;
    let to = to.as_ref();
    unix_fs::symlink(&from, to)?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct FromTo {
    from: PathBuf,
    to: PathBuf,
}
impl FromTo {
    // pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Self {
    //     Self {
    //         from: from.as_ref().to_path_buf(),
    //         to: to.as_ref().to_path_buf(),
    //     }
    // }
    pub fn new(from: PathBuf, to: PathBuf) -> Self {
        Self { from, to }
    }
    pub fn create_link(&self) -> anyhow::Result<()> {
        let from = self.from.as_path();
        let to = self.to.as_path();
        if !from.is_file() {
            Ok(self.dir_handler()?)
        } else if !to.try_exists()? {
            Ok(create_slink(from, to)?)
        } else if to.is_dir() {
            let to = to.join(from.iter().last().unwrap());
            Ok(create_slink(from, to)?)
        } else {
            Err(anyhow!("Unxpected error at {}", line!()))
        }
    }
    fn dir_handler(&self) -> anyhow::Result<()> {
        let from = self.from.as_path();
        let to = self.to.as_path();
        if !to.try_exists().expect("Error here") {
            fs::create_dir(to).expect("Error here");
        }
        for file in from.read_dir().expect("Error here") {
            let file = file.expect("Error here").path();
            let from = file.as_path();
            let to = to.join(from.iter().last().unwrap());
            create_slink(from, to).expect("Error here");
        }
        Ok(())
    }
    pub fn remove_link(&self) -> anyhow::Result<()> {
        // fs::remove_file(&self.to)?;
        todo!()
    }
}
