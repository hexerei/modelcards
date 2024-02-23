use std::{
    fs::{canonicalize, create_dir},
    path::Path
};

use modelcards::{
    assets::{schema, templates},
    utils::{strip_unc, create_file, is_directory_empty}
};

use anyhow::{bail, Result};

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

// fn is_directory_empty(path: &Path, allow_hidden: bool) -> Result<bool> {
//     if path.is_dir() {
//         let mut entries = match path.read_dir() {
//             Ok(entries) => entries,
//             Err(e) => bail!("Could not read '{}' because of error: {}", path.to_string_lossy().to_string(), e),
//         };
//         if entries.any(|x| match x {
//             Ok(file) => {
//                 if allow_hidden {
//                     !file.file_name().to_str().expect("Could not convert filename to &str").starts_with('.')
//                 } else {
//                     true
//                 }

//             },
//             Err(_) => true,
//         }) {
//             return Ok(false);
//         }
//         return Ok(true);
//     }
//     Ok(false)
// }


fn populate(path: &Path, config: &str) -> Result<()> {
    if !path.exists() {
        create_dir(path)?;
    }
    create_file(&path.join("config.toml"), config)?;
    create_file(&path.join("sample.json"), &schema::get_sample())?;
    create_dir(path.join("schema"))?;
    create_file(&path.join("schema/modelcard.schema.json"), &schema::get_schema())?;
    create_dir(path.join("templates"))?;
    create_file(&path.join("templates/modelcard.md.jinja"), &templates::get_md())?;
    create_file(&path.join("templates/modelcard.html.jinja"), &templates::get_html())?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir, remove_dir_all},
        path::PathBuf
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

    fn check_modelcards_dir(path: &Path) {
        assert!(path.exists());
        assert!(path.join("config.toml").exists());
        assert!(path.join("sample.json").exists());
        assert!(path.join("schema").exists());
        assert!(path.join("schema/modelcard.schema.json").exists());
        assert!(path.join("templates").exists());
        assert!(path.join("templates/modelcard.md.jinja").exists());
        assert!(path.join("templates/modelcard.html.jinja").exists());
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
        check_modelcards_dir(&dir);
        remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn populate_non_existing_directory() {
        let dir = get_temp_dir("test_non_existing_dir", false);
        populate(&dir, "").expect("Could not populate modelcards directories");
        check_modelcards_dir(&dir);
        remove_dir_all(&dir).unwrap();
    }

}