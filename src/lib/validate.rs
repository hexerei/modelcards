use std::{
    path::Path
};

use crate::utils::load_json_file;
use valico::json_schema::scope;

pub fn check_against_schema(path: &Path, modelcard: &Path) -> bool {
    let schema_v7 = load_json_file(&path.join("schema/modelcard.schema.json"));
    let modelcard = load_json_file(&modelcard);
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assets::schema;
    use crate::utils::create_file;

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
        assert!(check_against_schema(&dir, &dir.join("sample.json")));
    }
}