use crate::file::{FileTypes, LoadedFile};
use std::fs;
use std::string::FromUtf8Error;

enum BlockType {
    FunctionBlock,
    ClassBlock,
    OtherBlock,
}

pub struct BlockConstants {
    pub class_string: String,
    pub function_string: String,
    pub method_string: String,
    pub import_strings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedFile {
    pub blocks: Vec<String>,
    pub imports: String,
    pub file_type: FileTypes,
}

impl BlockConstants {
    pub fn new(file_type: &FileTypes) -> Self {
        let constants = match file_type {
            FileTypes::Python => BlockConstants {
                class_string: "class ".to_string(),
                function_string: "def ".to_string(),
                method_string: "    def".to_string(),
                import_strings: vec!["import".to_string()],
            },
            FileTypes::Javascript => BlockConstants {
                class_string: "class".to_string(),
                function_string: "function ".to_string(),
                method_string: "    ".to_string(),
                import_strings: vec!["import".to_string(), "require".to_string()],
            },
            _ => todo!(),
        };
        constants
    }
}

pub fn parse(file: &LoadedFile) -> ParsedFile {
    match file.file_type {
        FileTypes::Python => parse_file(&file),
        FileTypes::Javascript => parse_file(&file),
        FileTypes::Rust => parse_file(&file),
        FileTypes::Default => {
            todo!("How'd you get here")
        }
    }
}

pub fn load_file_string(filepath: &str) -> Result<String, FromUtf8Error> {
    let file_bytes = fs::read(filepath).expect("couldn't load file");

    let file_string = String::from_utf8(file_bytes)?;
    Ok(file_string)
}

fn parse_file(file: &LoadedFile) -> ParsedFile {
    let block_constants: BlockConstants = BlockConstants::new(&file.file_type);
    let file_string = &file.file_string;
    let file_type = file.file_type.clone();

    let file_lines: Vec<&str> = file_string.split('\n').collect();
    let mut blocks: Vec<String> = vec![];
    let mut block_string = "".to_string();
    let mut block_type = BlockType::OtherBlock;
    let mut import_string = "".to_string();
    for line in file_lines {
        if line.contains(&block_constants.method_string) {
            block_string += line;
            continue;
        }

        if line.contains(&block_constants.function_string) {
            match block_type {
                BlockType::ClassBlock | BlockType::FunctionBlock => blocks.push(block_string),
                _ => {}
            }

            block_type = BlockType::FunctionBlock;
            block_string = line.to_owned();
            continue;
        }

        if line.contains(&block_constants.class_string) {
            match block_type {
                BlockType::ClassBlock | BlockType::FunctionBlock => blocks.push(block_string),
                _ => {}
            }

            block_type = BlockType::ClassBlock;
            block_string = line.to_owned();
            continue;
        }

        for statement in &block_constants.import_strings {
            if line.contains(statement) {
                import_string += line;
                import_string += "\n";
                break;
            }
        }

        block_string += line;
    }
    blocks.push(block_string);

    ParsedFile {
        blocks,
        imports: import_string,
        file_type,
    }
}

// fn parse_javascript(file_string: &str) -> Vec<String> {
//     let file_lines: Vec<&str> = file_string.split('\n').collect();
//     let mut blocks: Vec<String> = vec![];
//     let mut block_string = "".to_string();
//     let mut block_type = BlockType::OtherBlock;
//     for line in file_lines {

//         if line.contains("  ") {
//             block_string += line;
//             continue
//         }

//         if line.contains("function ") {

//             match block_type {
//                 BlockType::ClassBlock | BlockType::FunctionBlock => {
//                     blocks.push(block_string)
//                 }
//                 _ => {}
//             }

//             block_type = BlockType::FunctionBlock;
//             block_string = line.to_owned();
//             continue
//         }

//         if line.contains("class ") {
//             match block_type {
//                 BlockType::ClassBlock | BlockType::FunctionBlock => {
//                     blocks.push(block_string)
//                 }
//                 _ => {}
//             }

//             block_type = BlockType::ClassBlock;
//             block_string = line.to_owned();
//             continue
//         }

//         block_string += line;

//     }
//     blocks.push(block_string);

//     blocks
