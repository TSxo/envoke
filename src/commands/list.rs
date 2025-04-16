use crate::error::{ErrorKind, Result};
use crate::fs::FileSystem;
use crate::profile::ProfileManager;

pub fn run<F: FileSystem>(manager: &ProfileManager<F>) -> Result<()> {
    if !manager.is_initialized() {
        return Err(ErrorKind::Uninitialized.into());
    }

    let list = manager.profiles()?;
    if list.is_empty() {
        println!("No profiles found. Run `envoke create <profile>` to get started!")
    } else {
        for profile in list {
            println!("{}", profile);
        }
    }

    Ok(())
}
