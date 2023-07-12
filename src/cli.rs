use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ArgumentConfig {
    pub file_path: String,
    pub output_path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgError {
    bad_arg: String,
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument Not Supplied")
    }
}

pub fn check_arg(
    flag_index: usize,
    flag_str: &str,
    args: &Vec<String>,
) -> Result<String, ArgError> {
    let args_size = args.len();

    if flag_index == 0 || flag_index >= args_size {
        Err(ArgError {
            bad_arg: flag_str.to_string(),
        })
    } else {
        Ok(args[flag_index].clone())
    }
}

pub fn argument_parse(args: Vec<String>) -> Result<ArgumentConfig, ArgError> {
    let mut file_index: usize = 0;
    let mut output_index: usize = 0;
    let file_flag = "-f";
    let output_flag = "-o";

    for (i, arg) in args.iter().enumerate() {
        if arg == file_flag {
            file_index = i + 1;
            continue;
        }

        if arg == output_flag {
            output_index = i + 1;
            continue;
        }
    }

    // try to get the values of required flags!
    let file_path = check_arg(file_index, file_flag, &args)?;
    let output_path = check_arg(output_index, output_flag, &args)?;

    Ok(ArgumentConfig {
        file_path,
        output_path,
    })
}

#[cfg(test)]

mod tests {
    use super::{argument_parse, check_arg, ArgumentConfig};

    #[test]
    fn test_argument_parse() {
        let test_filepath = "./some/file/path.js".to_string();
        let test_output_path = "./test-outputs".to_string();

        let input_args = vec![
            "wipers".to_string(),
            "-f".to_string(),
            test_filepath.clone(),
            "-o".to_string(),
            test_output_path.clone(),
        ];

        let expected_config_result = Ok(ArgumentConfig {
            file_path: test_filepath,
            output_path: test_output_path,
        });

        let actual_config_result = argument_parse(input_args);

        assert_eq!(expected_config_result, actual_config_result)
    }

    #[test]
    fn test_argument_parse_fail_no_file() {
        let input_args = vec!["wipers".to_string(), "-f".to_string()];

        let actual_config_result = argument_parse(input_args);

        assert!(actual_config_result.is_err())
    }

    #[test]
    fn test_argument_parse_fail_no_file_flag() {
        let test_filepath = "./some/file/path.js".to_string();

        let input_args = vec!["wipers".to_string(), test_filepath];

        let actual_config_result = argument_parse(input_args);

        assert!(actual_config_result.is_err())
    }

    #[test]
    fn test_argument_parse_fail_no_args() {
        let input_args = vec!["wipers".to_string()];

        let actual_config_result = argument_parse(input_args);

        assert!(actual_config_result.is_err())
    }

    #[test]
    fn test_check_arg() {
        let args: Vec<String> = vec![
            "wipers".to_string(),
            "-f".to_string(),
            "./my_file.js".to_string(),
        ];
        let expected_arg_string = "./my_file.js".to_string();
        let flag_index = 2;
        let flag_str = "-f";

        let actual_arg_string = check_arg(flag_index, flag_str, &args);

        assert!(actual_arg_string.is_ok());

        assert_eq!(expected_arg_string, actual_arg_string.unwrap());

        let args: Vec<String> = vec![
            "wipers".to_string(),
            "-f".to_string(),
            "./my_file.js".to_string(),
            "-o".to_string(),
            "./tests".to_string(),
        ];
        let expected_arg_string = "./tests".to_string();
        let flag_index = 4;
        let flag_str = "-o";

        let actual_arg_string = check_arg(flag_index, flag_str, &args);

        assert!(actual_arg_string.is_ok());

        assert_eq!(expected_arg_string, actual_arg_string.unwrap())
    }
}
