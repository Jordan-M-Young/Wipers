use std::fs;
use std::string::FromUtf8Error;

// struct BaseParser {
//     filepath: String,
//     file_string: String
// }

#[derive(Debug)]
pub enum ParserTypes {
    Python,
    Javascript,
    Rust,
    Default,
}

#[derive(Debug)]
pub struct Parser {
    filepath: String,
    file_string: String,
    parser_type: ParserTypes,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            filepath: "".to_string(),
            file_string: "".to_string(),
            parser_type: ParserTypes::Default,
        }
    }
}

impl Parser {
    pub fn new(filepath: &str) -> Self {
        if filepath.contains(".py") {
            Parser {
                filepath: filepath.to_string(),
                file_string: Default::default(),
                parser_type: ParserTypes::Python,
            }
        } else if filepath.contains(".js") {
            Parser {
                filepath: filepath.to_string(),
                file_string: Default::default(),
                parser_type: ParserTypes::Javascript,
            }
        } else if filepath.contains(".rs") {
            Parser {
                filepath: filepath.to_string(),
                file_string: Default::default(),
                parser_type: ParserTypes::Rust,
            }
        } else {
            todo!("Haven't implmented a parser for your file type yet. ")
        }
    }

    pub fn set_file_string(&mut self) -> Result<(), FromUtf8Error> {
        let file_bytes = fs::read(&self.filepath).expect("couldn't load file");

        let file_string = String::from_utf8(file_bytes)?;
        self.file_string = file_string.clone();

        Ok(())
    }

    pub fn get_file_string(&mut self) -> Result<String, FromUtf8Error> {
        let file_bytes = fs::read(&self.filepath).expect("couldn't load file");

        let file_string = String::from_utf8(file_bytes)?;
        Ok(file_string)
    }
}
