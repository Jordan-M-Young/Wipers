pub struct ArgumentConfig {
    pub file_path: String,
}

pub fn argument_parse(args: Vec<String>) -> Option<ArgumentConfig> {
    let mut file_index: usize = 0;

    for (i, arg) in args.iter().enumerate() {
        if arg == "-f" {
            file_index = i + 1;
            break;
        }
    }

    if file_index == 0 {
        None
    } else {
        let file_path = &args[file_index];

        Some(ArgumentConfig {
            file_path: file_path.to_owned(),
        })
    }
}
