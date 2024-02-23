use std::{
    fs::File,
    io::{Read, Write},
    path::Path
};

use anyhow::{Context, Result};

// canonicalize(path) function on windows system returns a path with UNC.
// Example: \\?\C:\Users\VssAdministrator\AppData\Local\Temp\new_project
// More details on Universal Naming Convention (UNC):
// https://en.wikipedia.org/wiki/Path_(computing)#Uniform_Naming_Convention
// So the following const will be used to remove the network part of the UNC to display users a more common
// path on windows systems.
// This is a workaround until this issue https://github.com/rust-lang/rust/issues/42869 was fixed.
const LOCAL_UNC: &str = "\\\\?\\";

// Remove the unc part of a windows path
pub fn strip_unc(path: &Path) -> String {
    let path_to_refine = path.to_str().unwrap();
    path_to_refine.trim_start_matches(LOCAL_UNC).to_string()
}

pub fn create_file(path: &Path, content: &str) -> Result<()> {
    let mut file = File::create(path).with_context(|| format!("Failed to create File {}", path.display()))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn load_json_file(file_path: &Path) -> serde_json::Value {
    let mut file = File::open(file_path).unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    serde_json::from_str(&file_string).unwrap()
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