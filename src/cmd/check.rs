use std::path::Path;
use modelcards::validate::check_against_schema;
use anyhow::Result;

pub fn check_project(path: &Path, modelcard: Option<String>) -> Result<bool> {
    let modelcard = modelcard.unwrap_or_else(|| "sample.json".to_string());
    let modelcard = Path::new(&modelcard);
    check_against_schema(path, modelcard)
}

#[cfg(test)]
mod tests {

    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir_all},
        path::{PathBuf, Path}
    };

    use super::*;
    use modelcards::utils::create_file;
    use modelcards::assets::schema;

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
        assert!(check_against_schema(&dir, &dir.join("sample.json")).is_ok());
    }
}