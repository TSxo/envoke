use std::path::Path;

use crate::error::{ErrorKind, Result};
use crate::fs::FileSystem;
use crate::profile::ProfileManager;

pub fn run<F, S>(manager: &ProfileManager<F>, profile: S) -> Result<()>
where
    F: FileSystem,
    S: AsRef<str>,
{
    let profile = profile.as_ref();

    if !manager.is_initialized() {
        return Err(ErrorKind::Uninitialized.into());
    }

    let profile_path = manager.profile_path(&profile);
    let env_path = Path::new(".env");

    if !profile_path.exists() {
        return Err(ErrorKind::ProfileNotFound {
            profile: profile.to_string(),
        }
        .into());
    }

    if env_path.exists() && env_path.is_symlink() {
        let target = manager.fs.read_link(env_path)?;
        let target = target.file_stem().unwrap();
        let profile = profile_path.file_stem().unwrap();

        if target == profile {
            println!("Unlinking .env");
            manager.fs.remove_file(&env_path)?;
        }
    }

    manager.fs.remove_file(&profile_path)?;

    println!("Profile {} removed.", profile);

    Ok(())
}
