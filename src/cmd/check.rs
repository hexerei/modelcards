use std::{
    fs::File,
    io::Read,
    path::Path
};

use valico::json_schema::scope;

pub fn check_project() -> bool {
    check_against_schema(Path::new("."))
}

fn check_against_schema(path: &Path) -> bool {
    let schema_v7 = load_json_file(&path.join("schema/modelcard.schema.json"));
    let modelcard = load_json_file(&path.join("sample.json"));
    let mut scope = scope::Scope::new();
    //let schema = scope.compile_and_return(schema_v7, true).ok().unwrap();
    match scope.compile_and_return(schema_v7, true) {
        Ok(s) => {
            let vs = s.validate(&modelcard);
            if !vs.is_valid() {
                eprintln!("Validation failed: {:?}", vs);
                return false;
            }
            true
        },
        Err(e) => {
            eprintln!("Could not compile schema: {:?}", e);
            false
        }
    }
}

fn load_json_file(file_path: &Path) -> serde_json::Value {
    let mut file = File::open(file_path).unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    serde_json::from_str(&file_string).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::create_file;
    use crate::cmd::init::schema;

    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir_all},
        path::{PathBuf, Path}
    };
    use anyhow::Result;

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

    fn populate_modelcards_dir(path: &Path) -> Result<()> {
        create_file(&path.join("sample.json"), &schema::get_sample())?;
        create_dir(path.join("schema"))?;
        create_file(&path.join("schema/modelcard.schema.json"), &schema::get_schema())?;
        Ok(())
    }

    #[test]
    fn check_valid_against_schema() {
        let dir = get_temp_dir("test_check_against_schema", true);
        populate_modelcards_dir(&dir).expect("Could not populate modelcards directory");
        assert!(check_against_schema(&dir));
    }
}