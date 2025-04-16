use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// TestEnv provides a complete testing environment for Envoke commands.
pub struct TestEnv {
    /// The temporary directory for this test.
    pub temp_dir: TempDir,

    /// Path to the .envoke directory.
    pub envoke_dir: PathBuf,

    /// Path to the executable being tested (for integration tests).
    pub binary_path: PathBuf,
}

impl TestEnv {
    /// Creates a new test environment.
    pub fn new() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        let envoke_dir = temp_dir.path().join(".envoke");

        let mut binary_path = std::env::current_exe().unwrap();
        binary_path.pop(); // Remove the test binary name

        // In debug mode, the binary is in target/debug/
        if binary_path.ends_with("deps") {
            binary_path.pop(); // Remove "deps"
        }

        binary_path.push("envoke"); // Add the envoke binary name

        Self {
            temp_dir,
            envoke_dir,
            binary_path,
        }
    }

    /// Path to the root of the test directory.
    pub fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Path to the envoke directory.
    pub fn envoke_path(&self, name: &str) -> PathBuf {
        self.envoke_dir.join(format!("{}.env", name))
    }

    /// Run a command in the test directory and return its output.
    pub fn run_command(&self, args: &[&str]) -> std::process::Output {
        Command::new(&self.binary_path)
            .args(args)
            .current_dir(self.temp_path())
            .output()
            .unwrap()
    }
}
