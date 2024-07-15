//! # `utils` Module
//!
//! Provides utility functions for file operations and path manipulations.
//!
//! ## Functions
//!
//! - `strip_unc`: Removes the UNC prefix from a Windows path.
//! - `create_file`: Creates a file with the specified content.
//! - `load_json_file`: Loads and deserializes a JSON file into a [`serde_json::Value`].
//! - `is_directory_empty`: Check if a directory is empty.
//!
//! ## Notes
//!
//! - The `strip_unc` function is specific to Windows systems and deals with paths starting with `\\?\`.
//! - The `create_file` and `load_json_file` functions use [`anyhow::Result`] for error handling, allowing for simple and flexible error management.

use std::{
    fs::File,
    io::{Read, Write},
    path::Path
};

use anyhow::{bail, Context, Result};

/// const will be used to remove the network part of the UNC to display users a more common path on windows systems.
const LOCAL_UNC: &str = "\\\\?\\";

/// Removes the UNC prefix from a Windows path.
/// 
/// The `strip_unc` function is specific to Windows systems and deals with paths starting with `\\?\`.
/// Canonicalize(path) function on windows system returns a path with UNC.
/// UNC sample: `\\?\C:\Users\VssAdministrator\AppData\Local\Temp\new_project`
/// More details on Universal Naming Convention (UNC) can be found [here](https://en.wikipedia.org/wiki/Path_(computing)#Uniform_Naming_Convention).
/// 
/// <div class="alert alert-warning">
/// This is a workaround until this issue https://github.com/rust-lang/rust/issues/42869 was fixed.
/// </div>
/// 
/// ## Example
/// 
/// ```rust
/// use std::path::Path;
/// use crate::utils::strip_unc;
///
/// let path = Path::new(r"\\?\C:\Path\to\File");
/// let cleaned_path = strip_unc(path);
/// ```
/// 
pub fn strip_unc(path: &Path) -> String {
    let path_to_refine = path.to_str().unwrap();
    path_to_refine.trim_start_matches(LOCAL_UNC).to_string()
}

/// Creates a file with the specified content.
/// 
/// ## Panics
/// 
/// This function uses [`anyhow::Result`] for error handling, allowing for simple and flexible error management.
/// Errors can occur when creating the file or writing the content. Errors will be of type [`anyhow::Error`].
/// 
/// ## Example
/// 
/// ```rust
/// use std::path::Path;
/// use crate::utils::create_file;
///
/// let path = Path::new("your_path.txt");
/// create_file(path, "File content").expect("Failed to create file");
/// ```
/// 
pub fn create_file(path: &Path, content: &str) -> Result<()> {
    let mut file = File::create(path).with_context(|| format!("Failed to create File {}", path.display()))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Load and deserializes a JSON file into a `serde_json::Value`.
/// 
/// ## Panics
/// 
/// This function uses `anyhow::Result` for error handling, allowing for simple and flexible error management.
/// Errors can occur when reading the file (io error) or deserializing the content (serde error). Errors will be of type `anyhow::Error`.
/// 
/// ## Example
///
/// ```rust
/// use std::path::Path;
/// use crate::utils::load_json_file;
///
/// let path = Path::new("your_path.json");
/// let json_value = load_json_file(path).expect("Failed to load JSON");
/// ```
///
pub fn load_json_file(file_path: &Path) -> Result<serde_json::Value> {
    let mut file = File::open(file_path).with_context(|| format!("Failed to open file {}", file_path.display()))?;
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;
        Ok(serde_json::from_str(&file_string)?)
}

/// Check if a directory is empty.
/// 
/// Utility function that checks if a directory is empty.
/// 
/// ## Panics
/// 
/// This function uses `anyhow::Result` for error handling, allowing for simple and flexible error management.
/// Errors can occur when reading the directory (io error). Errors will be of type `anyhow::Error`.
/// 
/// ## Example
///
/// ```rust
/// use std::path::Path;
/// use crate::utils::is_directory_empty;
///
/// let path = Path::new("your_directory");
/// if is_directory_empty(path).expect("Failed to read directory") {
///    println!("Directory is empty!");
/// }
/// ```
/// 
pub fn is_directory_empty(path: &Path, allow_hidden: bool) -> Result<bool> {
    if path.is_dir() {
        let mut entries = match path.read_dir() {
            Ok(entries) => entries,
            Err(e) => bail!("Could not read '{}' because of error: {}", path.to_string_lossy().to_string(), e),
        };
        if entries.any(|x| match x {
            Ok(file) => {
                if allow_hidden {
                    !file.file_name().to_str().expect("Could not convert filename to &str").starts_with('.')
                } else {
                    true
                }

            },
            Err(_) => true,
        }) {
            return Ok(false);
        }
        return Ok(true);
    }
    Ok(false)
}

/// Console utilities
/// 
/// Provides utility functions for logging messages to the console.
/// Additionally provides a function to exit the program with a success or error message and appropriate exit code.
/// Currently this module uses the [`log`] crate for logging messages to the console and is a wrapper to this,
/// which will allow implementing different logging mechanisms (console, files, port) in the future.
/// 
/// ## Functions
/// 
/// - `error`: Logs an error message to the console.
/// - `error_exit`: Logs an error message to the console and exits the program with an error code.
/// - `warn`: Logs a warning message to the console.
/// - `info`: Logs an information message to the console.
/// - `success_exit`: Logs a success message to the console and exits the program with a success code.
/// - `debug`: Logs a debug message to the console. Only available in debug builds.
/// 
/// ## Notes
/// 
/// - The `debug` function is only available in debug builds and will not be compiled in release builds.
/// - All functions use the env_logger crate for logging messages to the console. Therefore visibility of messages can be controlled by setting the `RUST_LOG` environment variable.
/// - The `error_exit` and `success_exit` functions will exit the program with an appropriate exit code (1 for error, 0 for success).
/// 
pub mod console {
    //use std::io::Write;

    /// Logs an error message to the console.
    /// 
    /// The `error` function logs an error message to the console using the [`log`] crate.
    /// Additionally, if an error is provided, it will be logged as well.
    /// Visibility of the message can be controlled by setting the `RUST_LOG` environment variable.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use crate::utils::console::error;
    /// error("An error occurred", None);
    /// ```
    /// 
    pub fn error(msg: &str, e: Option<impl std::fmt::Debug>) {
        log::error!("{}", msg);
        //eprintln!("Error: {}", msg);
        if let Some(e) = e {
            log::error!("{:?}", e);
            //eprintln!("{:?}", e);
        }
    }

    pub fn error_exit(msg: &str, e: Option<impl std::fmt::Debug>) {
        error(msg, e);
        std::process::exit(1);
    }

    /// Logs a warning message to the console.
    /// 
    /// The `warn` function logs a warning message to the console using the [`log`] crate.
    /// Visibility of the message can be controlled by setting the `RUST_LOG` environment variable.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use crate::utils::console::warn;
    /// warn("You should be careful with this!");
    /// ```
    /// 
    pub fn warn(msg: &str) {
        log::warn!("{}", msg);
        //eprintln!("Warning: {}", msg);
    }

    /// Logs an information message to the console.
    /// 
    /// The `info` function logs an information message to the console using the [`log`] crate.
    /// Visibility of the message can be controlled by setting the `RUST_LOG` environment variable.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use crate::utils::console::info;
    /// info("Everything is fine!");
    /// ```
    /// 
    pub fn info(msg: &str) {
        log::info!("{}", msg);
        //println!("{}", msg);
    }

    pub fn success_exit(msg: &str) {
        info(msg);
        std::process::exit(0);
    }

    /// Logs a debug message to the console.
    /// 
    /// The `debug` function logs a debug message to the console using the [`log`] crate.
    /// Visibility of the message can be controlled by setting the `RUST_LOG` environment variable.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use crate::utils::console::debug;
    /// debug("You should be careful with this!");
    /// ```
    /// 
    #[cfg(debug_assertions)]
    pub fn debug(msg: &str) {
        log::debug!("{}", msg);
        //println!("Debug: {}", msg);
    }

    /// Disables logging of debug messages to the console.
    /// 
    /// If debug_assertions are disabled, the `debug` function will be compiled out and not perform any action.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use crate::utils::console::debug;
    /// debug("You should be careful with this!");
    /// ```
    /// 
    #[cfg(not(debug_assertions))]
    pub fn debug(_msg: &str) {}
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env::temp_dir,
        fs::{canonicalize, create_dir, remove_dir_all},
        path::{Path, PathBuf}
    };

    fn get_temp_dir(path: &str, create: bool) -> PathBuf {
        let mut dir = temp_dir();
        dir.push(path);
        if dir.exists() {
            remove_dir_all(&dir).expect("Could not free test directory");
        }
        if create {
            create_dir(&dir).expect("Could not create test directory");
        }
        dir
    }

    #[test]
    fn strip_unc_test() {
        let dir = get_temp_dir("test_strip_unc", true);
        if cfg!(target_os = "windows") {
            let stripped_path = strip_unc(&canonicalize(Path::new(&dir)).unwrap());
            assert!(same_file::is_same_file(Path::new(&stripped_path), &dir).unwrap());
            assert!(!stripped_path.starts_with(LOCAL_UNC), "The path was not stripped.");
        } else {
            assert_eq!(
                strip_unc(&canonicalize(Path::new(&dir)).unwrap()),
                canonicalize(Path::new(&dir)).unwrap().to_str().unwrap().to_string()
            );
        }

        remove_dir_all(&dir).unwrap();
    }

    // If the following test fails it means that the canonicalize function is fixed and strip_unc
    // function/workaround is not anymore required.
    // See issue https://github.com/rust-lang/rust/issues/42869 as a reference.
    #[test]
    #[cfg(target_os = "windows")]
    fn strip_unc_required_test() {
        let dir = get_temp_dir("test_strip_unc_required", true);
        let canonicalized_path = canonicalize(Path::new(&dir)).unwrap();
        assert!(same_file::is_same_file(Path::new(&canonicalized_path), &dir).unwrap());
        assert!(canonicalized_path.to_str().unwrap().starts_with(LOCAL_UNC));

        remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn create_file_test() {
        let dir = get_temp_dir("test_create_file", true);
        let file_path = dir.join("test_file.txt");
        create_file(&file_path, "test content").expect("Could not create file");
        assert!(file_path.exists());
        remove_dir_all(&dir).unwrap();
    }
}