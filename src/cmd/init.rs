use std::{
    fs::{canonicalize, create_dir, File},
    io::Write,
    path::Path
};

use anyhow::{bail, Context, Result};

// canonicalize(path) function on windows system returns a path with UNC.
// Example: \\?\C:\Users\VssAdministrator\AppData\Local\Temp\new_project
// More details on Universal Naming Convention (UNC):
// https://en.wikipedia.org/wiki/Path_(computing)#Uniform_Naming_Convention
// So the following const will be used to remove the network part of the UNC to display users a more common
// path on windows systems.
// This is a workaround until this issue https://github.com/rust-lang/rust/issues/42869 was fixed.
const LOCAL_UNC: &str = "\\\\?\\";

pub fn create_new_project(name: &str, force: bool) -> Result<()> {
    let path = Path::new(name);
    if path.exists() && !is_directory_empty(path, true)? && !force {
        if name == "." {
            bail!("Can not deploy to current directory.");
        } else {
            bail!("Can not deploy to {}", path.to_string_lossy().to_string());
        }
    }
    println!("Welcome to modelcards!");

    let config ="# empty config file";
    populate(path, config)?;
    println!();
    println!("Done! Your project was created in {}", strip_unc(&canonicalize(path).unwrap()));
    println!();
    println!("Get started by moving into the directory and building your card: modelcads build");
    Ok(())
}

fn is_directory_empty(path: &Path, allow_hidden: bool) -> Result<bool> {
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

// Remove the unc part of a windows path
fn strip_unc(path: &Path) -> String {
    let path_to_refine = path.to_str().unwrap();
    path_to_refine.trim_start_matches(LOCAL_UNC).to_string()
}


fn populate(path: &Path, config: &str) -> Result<()> {
    if !path.exists() {
        create_dir(path)?;
    }
    create_file(&path.join("config.toml"), config)?;
    create_dir(path.join("content"))?;
    create_dir(path.join("templates"))?;

    Ok(())
}

fn create_file(path: &Path, content: &str) -> Result<()> {
    let mut file = File::create(path).with_context(|| format!("Failed to create File {}", path.display()))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir, remove_dir_all},
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
    fn init_empty_directory() {
        let dir = get_temp_dir("test_empty_dir", true);
        let allowed = is_directory_empty(&dir, false)
            .expect("An error happened reading the directory's contents");
        remove_dir(&dir).unwrap();
        assert!(allowed);
    }

    #[test]
    fn init_non_empty_directory() {
        let dir = get_temp_dir("test_non_empty_dir", true);
        let mut content = dir.clone();
        content.push("content");
        create_dir(&content).unwrap();
        let allowed = is_directory_empty(&dir, false)
            .expect("An error happened reading the directory's contents");
        remove_dir(&content).unwrap();
        remove_dir(&dir).unwrap();
        assert!(!allowed);
    }

    #[test]
    fn init_almost_empty_directory() {
        let dir = get_temp_dir("test_dir_with_hidden", true);
        let mut git = dir.clone();
        git.push(".git");
        create_dir(&git).unwrap();
        let allowed = is_directory_empty(&dir, true)
            .expect("An error happened reading the directory's contents");
        remove_dir(&git).unwrap();
        remove_dir(&dir).unwrap();
        assert!(allowed);
    }

    #[test]
    fn populate_existing_directory() {
        let dir = get_temp_dir("test_populate_existing_dir", true);
        populate(&dir, "").expect("Could not populate modelcards directories");

        assert!(dir.join("config.toml").exists());
        assert!(dir.join("content").exists());
        assert!(dir.join("templates").exists());

        remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn populate_non_existing_directory() {
        let dir = get_temp_dir("test_non_existing_dir", false);
        populate(&dir, "").expect("Could not populate modelcards directories");

        assert!(dir.exists());
        assert!(dir.join("config.toml").exists());
        assert!(dir.join("content").exists());
        assert!(dir.join("templates").exists());

        remove_dir_all(&dir).unwrap();
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
    
}