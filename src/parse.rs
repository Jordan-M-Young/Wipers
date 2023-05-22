use crate::file::{FileTypes, LoadedFile};
use std::fs;
use std::string::FromUtf8Error;

enum BlockType {
    FunctionBlock,
    ClassBlock,
    OtherBlock,
}

struct BlockConstants {
    class_string: String,
    function_string: String,
    method_string: String,
}

impl BlockConstants {
    fn new(file_type: &FileTypes) -> Self {
        let constants = match file_type {
            FileTypes::Python => BlockConstants {
                class_string: "class ".to_string(),
                function_string: "def ".to_string(),
                method_string: "    def".to_string(),
            },
            FileTypes::Javascript => BlockConstants {
                class_string: "class".to_string(),
                function_string: "function ".to_string(),
                method_string: "    ".to_string(),
            },
            _ => todo!(),
        };
        constants
    }
}

pub fn parse(file: &LoadedFile) -> Vec<String> {
    let block_constants = BlockConstants::new(&file.file_type);

    match file.file_type {
        FileTypes::Python => parse_file(&file.file_string, block_constants),
        FileTypes::Javascript => parse_file(&file.file_string, block_constants),
        FileTypes::Rust => parse_rust(&file.file_string),
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

fn parse_file(file_string: &str, block_constants: BlockConstants) -> Vec<String> {
    let file_lines: Vec<&str> = file_string.split('\n').collect();
    let mut blocks: Vec<String> = vec![];
    let mut block_string = "".to_string();
    let mut block_type = BlockType::OtherBlock;
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

        block_string += line;
    }
    blocks.push(block_string);

    blocks
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
// }

fn parse_rust(file_string: &str) -> Vec<String> {
    vec!["".to_string()]
}
