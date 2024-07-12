use std::fs;
//use modelcards::merge::from_strings;

pub fn merge_modelcards(sources: Vec<String>, target: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let merged: String;
    if sources.is_empty() {
        return Err("No modelcards to merge".into());
    }
    if sources.len() < 2 {
        merged = fs::read_to_string(sources[0].clone())?;
    } else {
        let json_result = modelcards::merge::from_paths(sources)?;
        // let mut modelcards = Vec::new();
        // for source in sources {
        //     let modelcard = fs::read_to_string(source)?;
        //     modelcards.push(modelcard);
        // }
        // let json_result = modelcards::merge::from_strings(modelcards)?;
        if json_result.is_object() {
            merged = serde_json::to_string_pretty(&json_result)?;
        } else {
            merged = json_result.to_string();
        }
    }
    if target.is_some() {
        fs::write(target.clone().unwrap(), merged.clone())?;
    }
   Ok(merged)
}

/// Test if the merge_modelcards function works
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, remove_dir_all};
    use std::path::PathBuf;
    use std::env::temp_dir;
    use modelcards::utils::create_file;

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
    fn merge_modelcards_to_file() {
        let path = get_temp_dir("test_merge_modelcards_to_file", true);
        create_file(path.join("modelcard1.json").as_path(), r#"{"name": "model1"}"#).expect("Could not create modelcard1 data file");
        create_file(path.join("modelcard2.json").as_path(), r#"{"name": "model2"}"#).expect("Could not create modelcard2 data file");
        merge_modelcards(vec![path.join("modelcard1.json").to_str().unwrap().to_string(), path.join("modelcard2.json").to_str().unwrap().to_string()], Some(path.join("merged.json").to_str().unwrap().to_string())).expect("Could not merge modelcards");
        assert!(path.join("merged.json").exists());
    }

    #[test]
    fn merge_modelcards_to_stdout() {
        let path = get_temp_dir("test_merge_modelcards_to_stdout", true);
        create_file(path.join("modelcard1.json").as_path(), r#"{"name": "model1"}"#).expect("Could not create modelcard1 data file");
        create_file(path.join("modelcard2.json").as_path(), r#"{"name": "model2"}"#).expect("Could not create modelcard2 data file");
        let merged = merge_modelcards(vec![path.join("modelcard1.json").to_str().unwrap().to_string(), path.join("modelcard2.json").to_str().unwrap().to_string()], None).expect("Could not merge modelcards");
        println!("{}", merged);
    }

    #[test]
    fn merge_single_modelcard_to_file() {
        let path = get_temp_dir("test_merge_single_modelcard_to_file", true);
        create_file(path.join("modelcard.json").as_path(), r#"{"name": "single_model"}"#).expect("Could not create modelcard data file");
        merge_modelcards(vec![path.join("modelcard.json").to_str().unwrap().to_string()], Some(path.join("merged_single.json").to_str().unwrap().to_string())).expect("Could not merge single modelcard");
        assert!(path.join("merged_single.json").exists());
        let merged_content = fs::read_to_string(path.join("merged_single.json")).expect("Could not read merged file");
        assert_eq!(merged_content, r#"{"name": "single_model"}"#);
    }

    #[test]
    fn merge_modelcards_empty_sources() {
        let result = merge_modelcards(vec![], None);
        assert!(result.is_err());
    }

    #[test]
    fn merge_modelcards_invalid_path() {
        let path = get_temp_dir("test_merge_modelcards_invalid_path", true);
        let invalid_path = path.join("non_existent_modelcard.json").to_str().unwrap().to_string();
        let result = merge_modelcards(vec![invalid_path], None);
        assert!(result.is_err());
    }

    #[test]
    fn merge_modelcards_to_file_with_invalid_target() {
        let path = get_temp_dir("test_merge_modelcards_to_file_with_invalid_target", true);
        create_file(path.join("modelcard1.json").as_path(), r#"{"name": "model1"}"#).expect("Could not create modelcard1 data file");
        create_file(path.join("modelcard2.json").as_path(), r#"{"name": "model2"}"#).expect("Could not create modelcard2 data file");
        let invalid_target = path.join("/invalid/path/merged.json").to_str().unwrap().to_string();
        let result = merge_modelcards(vec![path.join("modelcard1.json").to_str().unwrap().to_string(), path.join("modelcard2.json").to_str().unwrap().to_string()], Some(invalid_target));
        assert!(result.is_err());
    }
}