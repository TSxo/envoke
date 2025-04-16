//! Profile management functionality for the envoke CLI tool.
//!
//! This module provides `ProfileManager` struct which handles operations related
//! to environment profiles, including listing available profiles, checking profile
//! existence, and managing profile paths.

use crate::{config::Config, error::Result, fs::FileSystem};

use std::path::PathBuf;

/// Manages environment profiles for the envoke CLI tool.
///
/// `ProfileManager` provides a layer of abstraction between the filesystem
/// operations and configuration settings, handling business logic related to
/// profile management.
pub struct ProfileManager<F: FileSystem> {
    /// Configuration settings for paths.
    pub config: Config,

    /// Filesystem implementation for I/O operations.
    pub fs: F,
}

impl<F: FileSystem> ProfileManager<F> {
    /// Creates a new `ProfileManager` with the specified configuration and
    /// filesystem.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration containing path information.
    /// * `fs` - Implementation of the `FileSystem` trait for I/O operations.
    ///
    /// # Returns
    ///
    /// A new `ProfileManager` instance.
    pub fn new(config: Config, fs: F) -> Self {
        Self { config, fs }
    }

    /// Checks whether the environment directory has been initialized.
    ///
    /// # Returns
    ///
    /// `true` if the directory exists, `false` otherwise.
    pub fn is_initialized(&self) -> bool {
        self.fs.path_exists(&self.config.envoke_dir)
    }

    /// Gets the full path for a profile with the given name.
    ///
    /// Automatically appends ".env" extension if not already present.
    ///
    /// # Arguments
    ///
    /// * `profile` - The name of the profile.
    ///
    /// # Returns
    ///
    /// The full path to the profile file.
    pub fn profile_path<S: AsRef<str>>(&self, profile: S) -> PathBuf {
        let mut profile = profile.as_ref().to_string();
        if !profile.ends_with(".env") {
            profile += ".env"
        }

        self.config.envoke_dir.join(profile)
    }

    /// Lists all available profiles.
    ///
    /// Reads the envoke directory and returns the names of all valid profiles,
    /// excluding the file extension.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of profile names on success, or an error
    /// if the directory cannot be read.
    pub fn profiles(&self) -> Result<Vec<String>> {
        let entries = self.fs.read_dir(&self.config.envoke_dir)?;

        let profiles = entries
            .filter_map(|entry_result| {
                // Skip entries with errors.
                let entry = entry_result.ok()?;
                let path = entry.path();

                // Only include .env files.
                if !path.is_file() || path.extension().map_or(true, |ext| ext != "env") {
                    return None;
                }

                // Extract and convert the filename stem.
                path.file_stem().and_then(|s| s.to_str()).map(String::from)
            })
            .collect();

        Ok(profiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::EnvokeFileSystem;
    use tempfile::TempDir;

    /// Helper function to create a ProfileManager with a temporary directory.
    fn profile_manager() -> ProfileManager<EnvokeFileSystem> {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let envoke_dir = temp_path.join(".envoke");
        let config = Config::new(envoke_dir);

        let fs = EnvokeFileSystem::new();

        ProfileManager::new(config, fs)
    }

    #[test]
    fn test_is_initialized() {
        let manager = profile_manager();

        assert!(!manager.is_initialized());

        manager.fs.create_dir(&manager.config.envoke_dir).unwrap();

        assert!(manager.is_initialized());
    }

    #[test]
    fn test_profile_path() {
        let manager = profile_manager();

        // Test without .env extension.
        let path = manager.profile_path("dev");
        assert_eq!(path.file_name().unwrap(), "dev.env");

        // Test with .env extension.
        let path = manager.profile_path("prod.env");
        assert_eq!(path.file_name().unwrap(), "prod.env");
    }

    #[test]
    fn test_profiles_with_files() {
        let manager = profile_manager();

        // Create the directory and some profile files.
        manager.fs.create_dir(&manager.config.envoke_dir).unwrap();

        // Create some valid profile files.
        let dev = manager.profile_path("dev");
        let prod = manager.profile_path("prod");

        manager.fs.create_file(&dev).unwrap();
        manager.fs.create_file(&prod).unwrap();

        // Create some non-profile files.
        let readme = manager.config.envoke_dir.join("README.md");
        let subdir = manager.config.envoke_dir.join("subdir");
        manager.fs.create_file(&readme).unwrap();
        manager.fs.create_dir(&subdir).unwrap();

        // Test listing profiles.
        let profiles = manager.profiles().unwrap();

        // Should only include the valid .env files, without extension.
        assert_eq!(profiles.len(), 2);
        assert!(profiles.contains(&"dev".to_string()));
        assert!(profiles.contains(&"prod".to_string()));
        assert!(!profiles.contains(&"README".to_string()));
        assert!(!profiles.contains(&"subdir".to_string()));
    }

    #[test]
    fn test_profiles_with_empty_directory() {
        let manager = profile_manager();

        // Create the directory but no files.
        manager.fs.create_dir(&manager.config.envoke_dir).unwrap();

        // Test listing profiles - should be empty.
        let profiles = manager.profiles().unwrap();
        assert!(profiles.is_empty());
    }

    #[test]
    fn test_profiles_with_nonexistent_directory() {
        let manager = profile_manager();

        // Test listing profiles - should error.
        let result = manager.profiles();
        assert!(result.is_err());
    }
}
