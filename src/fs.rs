//! Filesystem abstraction module for the envoke CLI tool.
//!
//! This module provides an abstract `FileSystem` trait and a concrete implementation
//! `EnvokeFileSystem` that wraps standard filesystem operations. This abstraction
//! enables easier testing and potential alternative implementations.

use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;
use std::fs;
use std::fs::File;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

/// Trait defining essential filesystem operations.
///
/// This trait abstracts filesystem interactions, allowing for different
/// implementations including mock implementations for testing purposes.
pub trait FileSystem {
    /// Checks if a path exists in the filesystem.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check.
    ///
    /// # Returns
    ///
    /// `true` if the path exists, `false` otherwise.
    fn path_exists(&self, path: &Path) -> bool;

    /// Creates a directory and all its parent directories if they don't exist.
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path to create.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an `Error` if directory creation fails
    fn create_dir(&self, path: &Path) -> Result<()>;

    /// Creates a new file, failing if the file already exists.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the file should be created.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an `Error` if file creation fails.
    fn create_file(&self, path: &Path) -> Result<File>;

    /// Reads the contents of a directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path to read.
    ///
    /// # Returns
    ///
    /// `Ok(ReadDir)` iterator on success, or an `Error` if reading fails.
    fn read_dir(&self, path: &Path) -> Result<ReadDir>;

    /// Opens a file with the specified options and returns a handle to it.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to open.
    /// * `options` - The options specifying how the file should be opened.
    ///
    /// # Returns
    ///
    /// `Ok(File)` handle on success, or an `Error` if opening fails.
    fn open_file(&self, path: &Path, options: &fs::OpenOptions) -> Result<File>;

    /// Checks if a path is a symbolic link.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check.
    ///
    /// # Returns
    ///
    /// `true` if the path is a symbolic link, `false` otherwise.
    fn is_symlink(&self, path: &Path) -> bool;

    /// Creates a new symbolic link at the specified path.
    ///
    /// # Arguments
    ///
    /// * `original` - The path that the symlink should point to.
    /// * `link` - The path where the symlink should be created.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an `Error` if symlink creation fails.
    fn create_symlink(&self, original: &Path, link: &Path) -> Result<()>;

    /// Reads the target of a symbolic link.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the symlink.
    ///
    /// # Returns
    ///
    /// `Ok(PathBuf)` containing the target path on success, or an `Error` if operation fails.
    fn read_link(&self, path: &Path) -> Result<PathBuf>;

    /// Removes a file or symlink at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file or symlink to remove.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an `Error` if removal fails.
    fn remove_file(&self, path: &Path) -> Result<()>;
}

/// Standard implementation of the `FileSystem` trait using the local filesystem.
///
/// This struct provides operations that directly interact with the local
/// filesystem through the standard library's `std::fs` module.
pub struct EnvokeFileSystem;

impl EnvokeFileSystem {
    /// Creates a new `EnvokeFileSystem` instance.
    ///
    /// # Returns
    ///
    /// A new `EnvokeFileSystem` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for EnvokeFileSystem {
    fn path_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(|e| {
            Error::new(ErrorKind::CreateDir {
                file: path.to_path_buf(),
                source: e,
            })
        })
    }

    fn create_file(&self, path: &Path) -> Result<File> {
        fs::File::create_new(path).map_err(|e| {
            Error::new(ErrorKind::CreateFile {
                file: path.to_path_buf(),
                source: e,
            })
        })
    }

    fn read_dir(&self, path: &Path) -> Result<ReadDir> {
        fs::read_dir(path).map_err(|e| {
            Error::new(ErrorKind::ReadDir {
                file: path.to_path_buf(),
                source: e,
            })
        })
    }

    fn open_file(&self, path: &Path, options: &fs::OpenOptions) -> Result<File> {
        options.open(path).map_err(|e| {
            Error::new(ErrorKind::OpenFile {
                file: path.to_path_buf(),
                source: e,
            })
        })
    }

    fn is_symlink(&self, path: &Path) -> bool {
        path.exists()
            && fs::symlink_metadata(path)
                .map(|meta| meta.file_type().is_symlink())
                .unwrap_or(false)
    }

    fn create_symlink(&self, original: &Path, link: &Path) -> Result<()> {
        std::os::unix::fs::symlink(original, link).map_err(|e| {
            Error::new(ErrorKind::CreateSymlink {
                link: link.to_path_buf(),
                original: original.to_path_buf(),
                source: e,
            })
        })
    }

    fn read_link(&self, path: &Path) -> Result<std::path::PathBuf> {
        fs::read_link(path).map_err(|e| {
            Error::new(ErrorKind::ReadLink {
                file: path.to_path_buf(),
                source: e,
            })
        })
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path).map_err(|e| {
            Error::new(ErrorKind::RemoveFile {
                file: path.to_path_buf(),
                source: e,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use tempfile::tempdir;

    fn setup() -> (EnvokeFileSystem, tempfile::TempDir) {
        let fs_impl = EnvokeFileSystem::new();
        let temp_dir = tempdir().unwrap();
        (fs_impl, temp_dir)
    }

    #[test]
    fn test_path_exists() {
        let (fs_impl, temp_dir) = setup();
        let path = temp_dir.path();

        assert!(fs_impl.path_exists(path));

        let non_existent_path = path.join("non_existent");
        assert!(!fs_impl.path_exists(&non_existent_path));
    }

    #[test]
    fn test_create_dir() {
        let (fs_impl, temp_dir) = setup();
        let new_dir = temp_dir.path().join("new_directory");

        let result = fs_impl.create_dir(&new_dir);
        assert!(result.is_ok());
        assert!(new_dir.exists());

        let nested_dir = new_dir.join("nested/deeply/path");
        let result = fs_impl.create_dir(&nested_dir);
        assert!(result.is_ok());
        assert!(nested_dir.exists());
    }

    #[test]
    fn test_create_file() {
        let (fs_impl, temp_dir) = setup();
        let file_path = temp_dir.path().join("test_file.txt");

        let result = fs_impl.create_file(&file_path);
        assert!(result.is_ok());
        assert!(file_path.exists());

        let result = fs_impl.create_file(&file_path);
        assert!(result.is_err());
        match result.unwrap_err().kind {
            ErrorKind::CreateFile { .. } => (),
            _ => panic!("Expected CreateFile error"),
        }
    }

    #[test]
    fn test_read_dir() {
        let (fs_impl, temp_dir) = setup();

        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs_impl.create_file(&file1).unwrap();
        fs_impl.create_file(&file2).unwrap();

        let result = fs_impl.read_dir(temp_dir.path());
        assert!(result.is_ok());

        let entries: Vec<_> = result
            .unwrap()
            .collect::<std::io::Result<Vec<_>>>()
            .unwrap();
        assert_eq!(entries.len(), 2);

        let non_existent_dir = temp_dir.path().join("non_existent_dir");
        let result = fs_impl.read_dir(&non_existent_dir);
        assert!(result.is_err());
        match result.unwrap_err().kind {
            ErrorKind::ReadDir { .. } => (),
            _ => panic!("Expected ReadDir error"),
        }
    }

    #[test]
    fn test_create_file_in_non_existent_dir() {
        let (fs_impl, temp_dir) = setup();

        let non_existent_dir = temp_dir.path().join("non_existent_dir");
        let file_path = non_existent_dir.join("test_file.txt");

        let result = fs_impl.create_file(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_file_with_invalid_permissions() {
        let (fs_impl, temp_dir) = setup();

        let readonly_dir = temp_dir.path().join("readonly_dir");
        fs_impl.create_dir(&readonly_dir).unwrap();

        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&readonly_dir, perms).unwrap();

        let file_path = readonly_dir.join("test_file.txt");
        let result = fs_impl.create_file(&file_path);
        assert!(result.is_err());

        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&readonly_dir, perms).unwrap();
    }

    #[test]
    fn test_open_file() {
        let (fs_impl, temp_dir) = setup();

        let file_path = temp_dir.path().join("test_file.txt");

        let mut file = fs_impl.create_file(&file_path).unwrap();
        file.write_all(b"Hello, world!").unwrap();

        let mut options = fs::OpenOptions::new();
        options.read(true);

        let result = fs_impl.open_file(&file_path, &options);
        assert!(result.is_ok());

        let mut file = result.unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "Hello, world!");

        let non_existent_file = temp_dir.path().join("non_existent_file.txt");
        let result = fs_impl.open_file(&non_existent_file, &options);
        assert!(result.is_err());
        match result.unwrap_err().kind {
            ErrorKind::OpenFile { .. } => (),
            _ => panic!("Expected OpenFile error"),
        }
    }

    #[test]
    fn test_symlink_operations() {
        let (fs_impl, temp_dir) = setup();

        // Create a file to link to.
        let original_path = temp_dir.path().join("target_file.txt");
        let mut file = fs_impl.create_file(&original_path).unwrap();
        file.write_all(b"Target content").unwrap();

        // Create a symlink.
        let link_path = temp_dir.path().join("symlink_file.txt");
        let result = fs_impl.create_symlink(&original_path, &link_path);
        assert!(result.is_ok());

        // Check it is a symlink.
        assert!(fs_impl.is_symlink(&link_path));
        assert!(!fs_impl.is_symlink(&original_path));

        // Read the link.
        let read_target = fs_impl.read_link(&link_path).unwrap();
        assert_eq!(read_target, original_path);

        // Test reading content through the symlink.
        let mut options = fs::OpenOptions::new();
        options.read(true);
        let mut link_file = fs_impl.open_file(&link_path, &options).unwrap();
        let mut content = String::new();
        link_file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "Target content");
    }

    #[test]
    fn test_remove_symlink() {
        let (fs_impl, temp_dir) = setup();

        // Create a file to link to.
        let original_path = temp_dir.path().join("target_file.txt");
        let mut file = fs_impl.create_file(&original_path).unwrap();
        file.write_all(b"Target content").unwrap();

        // Create a symlink.
        let link_path = temp_dir.path().join("symlink_file.txt");
        fs_impl.create_symlink(&original_path, &link_path).unwrap();

        // Verify it exists.
        assert!(fs_impl.path_exists(&link_path));

        // Remove the symlink.
        let result = fs_impl.remove_file(&link_path);
        assert!(result.is_ok());

        // Verify the symlink is gone but the original file remains.
        assert!(!fs_impl.path_exists(&link_path));
        assert!(fs_impl.path_exists(&original_path));
    }
}
