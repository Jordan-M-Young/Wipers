use crate::file::LoadedFile;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

fn gen_test_name(file: LoadedFile) -> String {
    let file_path = file.file_path;

    let split_path: Vec<&str> = file_path.split("/").collect();

    let file_name = split_path[split_path.len()];

    let mut test_file_name = "test_".to_string();
    test_file_name.push_str(file_name);

    test_file_name
}

fn tests_to_file(test_string: &str, test_file_name: &str, out_path: &str) -> Result<(), Error> {
    let outfile_path = format!("{}/{}", out_path, test_file_name);

    let mut out_file = File::create(outfile_path)?;

    write!(out_file, "{}", test_string)?;

    Ok(())
}
