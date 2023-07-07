use std::fs;
use std::string::FromUtf8Error;

use crate::parse::ParsedFile;

#[derive(Debug, PartialEq)]
pub struct LoadedFile {
    pub file_path: String,
    pub file_string: String,
    pub file_type: FileTypes,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileTypes {
    Python,
    Javascript,
    Rust,
    Default,
}

impl LoadedFile {
    pub fn new(file_path: &str) -> Self {
        let file_string = load_file_string(file_path).expect("couldnt load file");
        let file_type = get_file_type(file_path);
        let file_path = file_path.to_string();

        LoadedFile {
            file_path,
            file_string,
            file_type,
        }
    }
}

pub fn load_file_string(filepath: &str) -> Result<String, FromUtf8Error> {
    let file_bytes = fs::read(filepath).expect("couldn't load file");

    let file_string = String::from_utf8(file_bytes)?;
    Ok(file_string)
}

fn get_file_type(file_path: &str) -> FileTypes {
    if file_path.contains(".py") {
        FileTypes::Python
    } else if file_path.contains(".js") {
        FileTypes::Javascript
    } else if file_path.contains(".rs") {
        FileTypes::Rust
    } else {
        todo!("Haven't implmented a parser for your file type yet. ")
    }
}

pub fn write_tests_to_file(tests: Vec<String>, parsed_file: ParsedFile) {
    let mut test_file_string = parsed_file.imports;
}

#[cfg(test)]
mod tests {

    use super::{get_file_type, load_file_string, FileTypes, LoadedFile};

    #[test]
    fn test_get_file_type() {
        let python_file_name = "test.py";
        let expected_file_type = FileTypes::Python;
        let actual_file_type = get_file_type(python_file_name);

        assert_eq!(expected_file_type, actual_file_type);

        let js_file_name = "test.js";
        let expected_file_type = FileTypes::Javascript;
        let actual_file_type = get_file_type(js_file_name);

        assert_eq!(expected_file_type, actual_file_type);

        let rust_file_name = "test.rs";
        let expected_file_type = FileTypes::Rust;
        let actual_file_type = get_file_type(rust_file_name);

        assert_eq!(expected_file_type, actual_file_type)
    }

    #[test]
    fn test_load_file_string() {
        let file_name = "./test-assets/test_load_file.py";
        let actual_file_string = load_file_string(file_name).expect("didnt open file");

        let expected_file_string = r#"import os

def do_something():
    print("hello World")"#;

        assert_eq!(actual_file_string, expected_file_string)
    }

    #[test]
    #[should_panic(
        expected = "couldn't load file: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"
    )]
    fn test_load_file_string_fail() {
        let file_name = "./test-assets/no_exist.js";
        let _file_load_result = load_file_string(file_name);
    }

    #[test]
    fn test_loaded_file_new() {
        let expected_file_string = r#"import os

def do_something():
    print("hello World")"#;

        let actual_loaded_file = LoadedFile::new("./test-assets/test_load_file.py");
        let expected_loaded_file = LoadedFile {
            file_path: "./test-assets/test_load_file.py".to_string(),
            file_string: expected_file_string.to_string(),
            file_type: FileTypes::Python,
        };

        assert_eq!(actual_loaded_file, expected_loaded_file)
    }
}
