use crate::parse::BlockConstants;
use crate::parse::ParsedFile;
use std::collections::HashSet;

struct ProcessedTest {
    test_code: String,
    imports: HashSet<String>,
}

// fn import_set_union(imports: Vec<HashSet<String>>) -> HashSet<String> {

//     if imports.len() == 0 {
//         return HashSet::new()
//     }

//     if imports.len() == 1 {
//         return imports[0]

//     }

//     let mut main_set = HashSet::new();

//     for set in imports {
//         main_set = main_set.union(&set)
//     }

//     main_set

// }

fn set_to_string(import_set: HashSet<String>) -> String {
    let mut import_statements = "".to_string();
    for statement in import_set {
        import_statements += &statement
    }
    import_statements
}

pub fn post_process(test: String, parsed_file: &ParsedFile) -> String {
    let base_test_lines = test.split("\n");


    let mut import_set: HashSet<String> = HashSet::new();

    let block_constants = BlockConstants::new(&parsed_file.file_type);

    let mut processed_test_string = "".to_string();

    for line in base_test_lines {
        for statement in &block_constants.import_strings {
            if line.contains(statement) {
                import_set.insert(line.to_string());
                break;
            }

            processed_test_string += line
        }
    }




    println!("{:?}",&import_set);
    let import_statements = set_to_string(import_set);
    println!("{}",&import_statements);
    
    let import_statements = vec![parsed_file.imports.clone(),import_statements].join("\n");


    let final_test_string = vec![import_statements, processed_test_string].join("\n");
    final_test_string
}