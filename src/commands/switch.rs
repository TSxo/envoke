use std::path::Path;

use crate::error::{ErrorKind, Result};
use crate::fs::FileSystem;
use crate::profile::ProfileManager;

pub fn run<F, S>(manager: &ProfileManager<F>, profile: S, force: bool) -> Result<()>
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

    if env_path.exists() {
        if force || manager.fs.is_symlink(env_path) {
            manager.fs.remove_file(&env_path)?;
        } else {
            return Err(ErrorKind::NonLinkedEnv.into());
        }
    }

    manager.fs.create_symlink(&profile_path, env_path)?;

    println!("Profile `{}` linked to .env", profile);

    Ok(())
}
