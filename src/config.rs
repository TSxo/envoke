//! Configuration module for the envoke CLI tool.
//!
//! This module provides the `Config` struct which stores all essential paths
//! and settings used throughout the application.

use std::path::PathBuf;

/// Stores configuration settings and paths for the envoke CLI tool.
///
/// `Config` centralizes all essential paths and settings, providing a single point
/// of configuration for the application. It includes default values for standard
/// setups while allowing customization when needed.
///
/// # Examples
///
/// ```
/// use envoke::config::Config;
/// use std::path::PathBuf;
///
/// // Using default configuration
/// let config = Config::default();
/// assert_eq!(config.envoke_dir, PathBuf::from(".envoke"));
///
/// // Custom configuration
/// let custom_config = Config::new(
///     PathBuf::from("/custom/path/.envoke"),
/// );
/// ```
#[derive(Debug)]
pub struct Config {
    /// Root directory for storing environment profiles and metadata.
    pub envoke_dir: PathBuf,
}

impl Config {
    /// Creates a new `Config` with custom paths.
    ///
    /// # Arguments
    ///
    /// * `envoke_dir` - Directory path for storing environment profiles and metadata.
    /// * `current_file` - File path for tracking the currently active profile.
    ///
    /// # Returns
    ///
    /// A new `Config` instance with the specified paths.
    pub fn new(envoke_dir: PathBuf) -> Self {
        Config { envoke_dir }
    }
}

impl Default for Config {
    /// Creates a default `Config` instance with standard paths.
    ///
    /// The default configuration uses:
    /// - `.envoke` for the root directory.
    /// - `.envoke/current` for the current profile file.
    ///
    /// # Returns
    ///
    /// A `Config` instance with default paths.
    fn default() -> Self {
        let envoke_dir = PathBuf::from(".envoke");
        Config { envoke_dir }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.envoke_dir, PathBuf::from(".envoke"));
    }

    #[test]
    fn test_custom_config() {
        let config = Config::new(PathBuf::from("/custom/.envoke"));
        assert_eq!(config.envoke_dir, PathBuf::from("/custom/.envoke"));
    }
}
