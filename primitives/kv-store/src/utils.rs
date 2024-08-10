use std::{
  fs::File,
  path::{Path, PathBuf},
};
use crate::result::*;

pub fn syncDir(path: &impl AsRef<Path>) -> Result<()> {
  File::open(path.as_ref())
    .map_err(|_| Error::FileOpen)?
    .sync_all()
    .map_err(|_| Error::FileSync)
}
