use std::fs;
use std::string::FromUtf8Error;

use crate::parse::ParsedFile;

#[derive(Debug)]
pub struct LoadedFile {
    pub file_path: String,
    pub file_string: String,
    pub file_type: FileTypes,
}

#[derive(Debug, Clone, Copy)]
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
