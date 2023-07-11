#[derive(Debug, PartialEq)]
pub struct ArgumentConfig {
    pub file_path: String,
}

pub fn argument_parse(args: Vec<String>) -> Option<ArgumentConfig> {
    let mut file_index: usize = 0;
    let args_size = args.len();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-f" {
            file_index = i + 1;
            break;
        }
    }

    if file_index == 0 {
        None
    } else if file_index >= args_size {
        None
    } else {
        let file_path = &args[file_index];

        Some(ArgumentConfig {
            file_path: file_path.to_owned(),
        })
    }
}

#[cfg(test)]

mod tests {
    use super::{argument_parse, ArgumentConfig};

    #[test]
    fn test_argument_parse() {
        let test_filepath = "./some/file/path.js".to_string();

        let input_args = vec![
            "wipers".to_string(),
            "-f".to_string(),
            test_filepath.clone(),
        ];

        let expected_config_result = Some(ArgumentConfig {
            file_path: test_filepath,
        });

        let actual_config_result = argument_parse(input_args);

        assert_eq!(expected_config_result, actual_config_result)
    }

    #[test]
    fn test_argument_parse_fail_no_file() {
        let input_args = vec!["wipers".to_string(), "-f".to_string()];

        let actual_config_result = argument_parse(input_args);

        assert!(actual_config_result.is_none())
    }

    #[test]
    fn test_argument_parse_fail_no_file_flag() {
        let test_filepath = "./some/file/path.js".to_string();

        let input_args = vec!["wipers".to_string(), test_filepath];

        let actual_config_result = argument_parse(input_args);

        assert!(actual_config_result.is_none())
    }

    #[test]
    fn test_argument_parse_fail_no_args() {
        let test_filepath = "./some/file/path.js".to_string();

        let input_args = vec!["wipers".to_string()];

        let actual_config_result = argument_parse(input_args);

        assert!(actual_config_result.is_none())
    }
}
