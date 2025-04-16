use crate::error::{ErrorKind, Result};
use crate::fs::FileSystem;
use crate::profile::ProfileManager;

pub fn run<F: FileSystem>(manager: &ProfileManager<F>) -> Result<()> {
    if manager.is_initialized() {
        return Err(ErrorKind::Initialized.into());
    }

    manager.fs.create_dir(&manager.config.envoke_dir)?;

    println!("Successfully initialized!");

    Ok(())
}
