use crate::file::LoadedFile;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

pub fn gen_test_name(file: LoadedFile) -> String {
    let file_path = file.file_path;

    let split_path: Vec<&str> = file_path.split("/").collect();

    let file_name = split_path[split_path.len() - 1];

    let mut test_file_name = "test_".to_string();
    test_file_name.push_str(file_name);

    test_file_name
}

pub fn tests_to_file(test_string: &str, test_file_name: &str, out_path: &str) -> Result<(), Error> {
    let outfile_path = format!("{}/{}", out_path, test_file_name);

    let mut out_file = File::create(outfile_path)?;

    write!(out_file, "{}", test_string)?;

    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::file::LoadedFile;

    use super::gen_test_name;

    #[test]
    fn test_gen_test_name() {
        let lf = LoadedFile::new("./test-assets/functions.py");

        let actual_test_file_name = gen_test_name(lf);
        let expected_test_file_name = "test_functions.py".to_owned();

        assert_eq!(actual_test_file_name, expected_test_file_name)
    }
}
