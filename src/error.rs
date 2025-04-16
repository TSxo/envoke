use std::path::PathBuf;
use std::result;
use std::{error, fmt};

/// A specialized [`result::Result`] type for Envoke operations.
pub type Result<T> = result::Result<T, Error>;

/// Represents possible errors associated with Envoke operations.
///
/// It is used in the [`Error`] type.
#[derive(Debug)]
pub enum ErrorKind {
    /// The directory has already been initialized.
    Initialized,

    /// The directory has not been initialized.
    Uninitialized,

    /// The profile does not exist.
    ProfileNotFound { profile: String },

    /// There is no active profile.
    NoActiveProfile,

    /// The file already exists.
    FileExists { file: PathBuf },

    /// Failed to open a file.
    OpenFile {
        file: PathBuf,
        source: std::io::Error,
    },

    /// Failed to create a file.
    CreateFile {
        file: PathBuf,
        source: std::io::Error,
    },

    /// Failed to remove a file.
    RemoveFile {
        file: PathBuf,
        source: std::io::Error,
    },

    /// Failed to write contents to a file.
    WriteFile {
        file: PathBuf,
        source: std::io::Error,
    },

    /// Failed to create a directory.
    CreateDir {
        file: PathBuf,
        source: std::io::Error,
    },

    /// Failed to read the directory.
    ReadDir {
        file: PathBuf,
        source: std::io::Error,
    },

    /// Failed to create a symlink.
    CreateSymlink {
        link: PathBuf,
        original: PathBuf,
        source: std::io::Error,
    },

    /// Failed to read a symlink.
    ReadLink {
        file: PathBuf,
        source: std::io::Error,
    },

    /// The .env is not a symlink.
    NonLinkedEnv,
}

impl ErrorKind {
    /// Returns the string representation of the `ErrorKind` variant.
    ///
    /// # Examples
    /// ```
    /// use envoke::error::ErrorKind;
    ///
    /// let expected = "This directory is already initialized.";
    /// assert_eq!(expected, ErrorKind::Initialized.as_string());
    pub fn as_string(&self) -> String {
        use ErrorKind::*;

        match self {
            Initialized => "This directory is already initialized.".into(),
            Uninitialized => "Directory has not been initialized - please run `envoke init`.".into(),
            ProfileNotFound { profile } => format!( "Profile `{}` does not exist. Run `envoke create {}` to create the profile.", profile, profile),
            NoActiveProfile  => "No active profile - activate a profile with: `envoke switch <profile>`.".into(),
            FileExists { file } => format!("The file `{}` already exists.", file.to_string_lossy()),
            OpenFile { file, .. } => format!("Failed to open file `{}`.", file.to_string_lossy()).into(),
            CreateFile { file, .. } => format!("Failed to create file `{}`.", file.to_string_lossy()).into(),
            RemoveFile { file, .. } => format!("Failed to remove file `{}`.", file.to_string_lossy()).into(),
            CreateDir { file, .. } => format!("Failed to create directory `{}`.", file.to_string_lossy()).into(),
            ReadDir { file, .. } => format!("Failed to read contents of directory `{}`.", file.to_string_lossy()) .into(),
            WriteFile { file, .. } => format!("Failed to write contents to file `{}`.", file.to_string_lossy()) .into(),
            CreateSymlink { link, original, .. } => format!("Failed to link `{}` to `{}`.", link.to_string_lossy(), original.to_string_lossy()) .into(),
            ReadLink { file, .. } => format!("Failed to read the link at `{}`.", file.to_string_lossy()),
            NonLinkedEnv => "The current `.env` is not managed by envoke. Backup your changes and delete the `.env`, or run `envoke switch <profile> --force`.".to_string(),
        }
    }
}

impl fmt::Display for ErrorKind {
    /// Shows a human-readable description of the `ErrorKind`.
    ///
    /// # Examples
    /// ```
    /// use envoke::error::ErrorKind;
    ///
    /// let expected = "This directory is already initialized.";
    /// assert_eq!(expected, ErrorKind::Initialized.to_string());
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.as_string()))
    }
}

impl error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ErrorKind::OpenFile { source, .. } => Some(source),
            ErrorKind::CreateFile { source, .. } => Some(source),
            ErrorKind::CreateDir { source, .. } => Some(source),
            ErrorKind::ReadDir { source, .. } => Some(source),
            ErrorKind::WriteFile { source, .. } => Some(source),
            ErrorKind::CreateSymlink { source, .. } => Some(source),
            ErrorKind::ReadLink { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<ErrorKind> for Error {
    /// Converts an [`ErrorKind`] into an [`Error`].
    ///
    /// This allows implicit conversion from `ErrorKind` to `Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use envoke::error::{Error, ErrorKind};
    ///
    /// let adready_init = ErrorKind::Initialized;
    /// let error = Error::from(adready_init);
    /// let expected = "This directory is already initialized.";
    /// assert_eq!(expected, format!("{error}"));
    /// ```
    fn from(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

/// The error type for Envoke operations.
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
}

impl Error {
    /// Creates a new `Error` from an [`ErrorKind`].
    ///
    /// # Examples
    ///
    /// ```
    /// use envoke::error::{Error, ErrorKind};
    ///
    /// let error = Error::new(ErrorKind::Initialized);
    /// let expected = "This directory is already initialized.";
    /// assert_eq!(expected, error.to_string());
    /// ```
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

impl fmt::Display for Error {
    /// Shows a human-readable description of the `Error`.
    ///
    /// # Examples
    /// ```
    /// use envoke::error::{Error, ErrorKind};
    ///
    /// let error = Error::new(ErrorKind::Initialized);
    /// let expected = "This directory is already initialized.";
    /// assert_eq!(expected, format!("{error}"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.as_string(),)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;
    use std::path::PathBuf;

    #[test]
    fn test_error_kind_as_string() {
        let original = PathBuf::from("/test/original.txt");
        let link = PathBuf::from("/test/new_link.txt");

        // Test a selection of variants.
        assert_eq!(
            "This directory is already initialized.",
            ErrorKind::Initialized.as_string()
        );

        assert_eq!(
            "Directory has not been initialized - please run `envoke init`.",
            ErrorKind::Uninitialized.as_string()
        );

        assert_eq!(
            "Failed to link `/test/new_link.txt` to `/test/original.txt`.",
            ErrorKind::CreateSymlink {
                link: link.clone(),
                original: original.clone(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidInput, "Symlink failed")
            }
            .as_string()
        );

        assert_eq!(
            "Failed to read the link at `/test/new_link.txt`.",
            ErrorKind::ReadLink {
                file: link.clone(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidInput, "Read link failed")
            }
            .as_string()
        );
    }

    #[test]
    fn test_error_kind_display() {
        let error_kind = ErrorKind::Initialized;
        assert_eq!(
            "This directory is already initialized.",
            error_kind.to_string()
        );

        let path = PathBuf::from("/test/file.txt");
        let file_exists = ErrorKind::FileExists { file: path };
        assert_eq!(
            "The file `/test/file.txt` already exists.",
            file_exists.to_string()
        );
    }

    #[test]
    fn test_error_kind_source() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let path = PathBuf::from("/test/file.txt");

        let open_file = ErrorKind::OpenFile {
            file: path,
            source: io_error,
        };
        assert!(open_file.source().is_some());

        let initialized = ErrorKind::Initialized;
        assert!(initialized.source().is_none());
    }

    #[test]
    fn test_error_kind_to_error_conversion() {
        let error_kind = ErrorKind::Initialized;
        let error: Error = error_kind.into();

        assert_eq!("This directory is already initialized.", error.to_string());
    }

    #[test]
    fn test_error_new() {
        let error = Error::new(ErrorKind::Initialized);
        assert_eq!("This directory is already initialized.", error.to_string());

        let path = PathBuf::from("/test/file.txt");
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let error = Error::new(ErrorKind::OpenFile {
            file: path,
            source: io_error,
        });
        assert_eq!("Failed to open file `/test/file.txt`.", error.to_string());
    }

    #[test]
    fn test_error_display() {
        let error = Error::new(ErrorKind::Initialized);
        assert_eq!(
            "This directory is already initialized.",
            format!("{}", error)
        );
    }

    #[test]
    fn test_error_source() {
        let error = Error::new(ErrorKind::Initialized);
        assert!(error.source().is_some());

        // Testing that the error source is the ErrorKind
        let source = error.source().unwrap();
        assert_eq!("This directory is already initialized.", source.to_string());
    }

    #[test]
    fn test_result_type() {
        // Test the success case
        let result: Result<i32> = Ok(42);
        assert_eq!(42, result.unwrap());

        // Test the error case
        let error = Error::new(ErrorKind::Initialized);
        let result: Result<i32> = Err(error);
        assert!(result.is_err());
        assert_eq!(
            "This directory is already initialized.",
            result.unwrap_err().to_string()
        );
    }
}
