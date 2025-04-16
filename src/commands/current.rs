use std::path::Path;

use crate::error::{ErrorKind, Result};
use crate::fs::FileSystem;
use crate::profile::ProfileManager;

pub fn run<F: FileSystem>(manager: &ProfileManager<F>) -> Result<()> {
    if !manager.is_initialized() {
        return Err(ErrorKind::Uninitialized.into());
    }

    let env_path = Path::new(".env");

    if !env_path.exists() {
        return Err(ErrorKind::NoActiveProfile.into());
    }

    if !manager.fs.is_symlink(env_path) {
        return Err(ErrorKind::NonLinkedEnv.into());
    }

    let target = manager.fs.read_link(env_path)?;
    let target = target.file_stem().unwrap();

    print!("{}\n", target.to_string_lossy());

    Ok(())
}
