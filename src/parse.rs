use crate::file::{FileTypes, LoadedFile};

enum BlockType {
    Function,
    Class,
    Other,
}

#[derive(Debug, PartialEq)]
pub struct BlockConstants {
    pub class_string: String,
    pub function_string: String,
    pub method_string: String,
    pub import_strings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedFile {
    pub blocks: Vec<String>,
    pub imports: String,
    pub file_type: FileTypes,
}

impl BlockConstants {
    pub fn new(file_type: &FileTypes) -> Self {
        match file_type {
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
        }
    }
}

pub fn parse(file: &LoadedFile) -> ParsedFile {
    match file.file_type {
        FileTypes::Python => parse_file(file),
        FileTypes::Javascript => parse_file(file),
        FileTypes::Rust => parse_file(file),
        FileTypes::Default => {
            todo!("How'd you get here")
        }
    }
}

fn parse_file(file: &LoadedFile) -> ParsedFile {
    let block_constants: BlockConstants = BlockConstants::new(&file.file_type);
    let file_string = &file.file_string;
    let file_type = file.file_type;

    let file_lines: Vec<&str> = file_string.split('\n').collect();
    let mut blocks: Vec<String> = vec![];
    let mut block_string = "".to_string();
    let mut block_type = BlockType::Other;
    let mut import_string = "".to_string();
    for line in file_lines {
        if line.contains(&block_constants.method_string) {
            block_string += line;
            block_string += "\n";
            continue;
        }

        if line.contains(&block_constants.function_string) {
            match block_type {
                BlockType::Class | BlockType::Function => blocks.push(block_string),
                _ => {}
            }

            block_type = BlockType::Function;
            block_string = line.to_owned();
            block_string += "\n";
            continue;
        }

        if line.contains(&block_constants.class_string) {
            match block_type {
                BlockType::Class | BlockType::Function => blocks.push(block_string),
                _ => {}
            }

            block_type = BlockType::Class;
            block_string = line.to_owned();
            block_string += "\n";
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
        block_string += "\n"
    }
    blocks.push(block_string);

    ParsedFile {
        blocks,
        imports: import_string,
        file_type,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, BlockConstants, ParsedFile};
    use crate::file::{FileTypes, LoadedFile};

    #[test]
    fn test_parse() {
        // dummy file a minimal python file #1
        let file = LoadedFile::new("./test-assets/test_load_file.py");

        let actual_parsed_file = parse(&file);

        let expected_block = r#"def do_something():
    print("hello World")
"#
        .to_string();
        let expected_blocks: Vec<String> = vec![expected_block];
        let expected_imports = "import os\n";
        let expected_parsed_file = ParsedFile {
            blocks: expected_blocks,
            imports: expected_imports.to_string(),
            file_type: FileTypes::Python,
        };

        assert_eq!(expected_parsed_file, actual_parsed_file);

        // dummy file #2 a larger python file

        let file = LoadedFile::new("./test-assets/functions.py");

        let actual_parsed_file = parse(&file);

        let expected_block_a = r#"def add(x,y):
    return x + y

"#
        .to_string();
        let expected_block_b = r#"class myClass():
    def __init__(self):
        self.some_val = 0
        self.other_val = True

    def get_some_val(self):
        return self.some_val

"#
        .to_string();
        let expected_block_c = r#"def subtract(x,y):
    return x-y
"#
        .to_string();

        let expected_blocks: Vec<String> =
            vec![expected_block_a, expected_block_b, expected_block_c];
        let expected_imports = "import os\nimport sys\n";
        let expected_parsed_file = ParsedFile {
            blocks: expected_blocks,
            imports: expected_imports.to_string(),
            file_type: FileTypes::Python,
        };

        assert_eq!(expected_parsed_file, actual_parsed_file)

        // javascript dummy file....
    }

    #[test]
    fn test_block_constants_new() {
        // python file type
        let file_type = FileTypes::Python;
        let actual_block_constants = BlockConstants::new(&file_type);
        let expected_block_constants = BlockConstants {
            class_string: "class ".to_string(),
            function_string: "def ".to_string(),
            method_string: "    def".to_string(),
            import_strings: vec!["import".to_string()],
        };

        assert_eq!(actual_block_constants, expected_block_constants);

        // javascript file type

        let file_type = FileTypes::Javascript;
        let actual_block_constants = BlockConstants::new(&file_type);
        let expected_block_constants = BlockConstants {
            class_string: "class".to_string(),
            function_string: "function ".to_string(),
            method_string: "    ".to_string(),
            import_strings: vec!["import".to_string(), "require".to_string()],
        };

        assert_eq!(actual_block_constants, expected_block_constants);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn block_constants_new_fail() {
        let file_type = FileTypes::Rust;
        let _actual_block_constants = BlockConstants::new(&file_type);
    }
}
