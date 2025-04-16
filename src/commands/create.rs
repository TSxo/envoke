use std::io::Write;

use crate::error::{Error, ErrorKind, Result};
use crate::fs::FileSystem;
use crate::profile::ProfileManager;

const PROFILE_HEADER: &str = "\
# ------------------------------------------------------------------------------
# Profile: ";

pub fn run<F, S>(manager: &ProfileManager<F>, profile: S) -> Result<()>
where
    F: FileSystem,
    S: AsRef<str>,
{
    let profile = profile.as_ref();

    if !manager.is_initialized() {
        return Err(ErrorKind::Uninitialized.into());
    }

    let path = manager.profile_path(&profile);

    if path.exists() {
        return Err(ErrorKind::FileExists { file: path }.into());
    }

    let mut file = manager.fs.create_file(&path)?;
    writeln!(file, "{}{}", PROFILE_HEADER, profile).map_err(|e| {
        Error::new(ErrorKind::WriteFile {
            file: path.to_path_buf(),
            source: e,
        })
    })?;

    println!("Profile {} created at {}", profile, path.to_string_lossy());

    Ok(())
}
