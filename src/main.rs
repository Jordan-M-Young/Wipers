use async_openai::{types::CreateCompletionRequestArgs, Client};
use std::error::Error;
pub mod cli;
pub mod config;
pub mod file;
pub mod lock;
pub mod parse;
pub mod postprocess;
pub mod write;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let toml_filename = "./wipers.toml";
    let config = config::load_toml(toml_filename);
    println!("{:?}", config);

    let lock_set = lock::lock_array_to_set(config.lock.lockfiles);
    let out_path = "./test-outputs";

    let client = Client::new()
        .with_api_key(config.openai.key)
        .with_org_id(config.openai.org_id);

    let args: Vec<String> = env::args().collect();
    println!("Args {:?}", &args);
    let arg_config = match cli::argument_parse(args) {
        Ok(val) => val,
        Err(e) => {
            panic!("No File Argument provided. Try wipers -f '<MY_FILE>'")
        }
    };

    let file_path = arg_config.file_path;
    // let file_path = "./test-inputs/functions.py";

    match lock_set.get(&file_path) {
        Some(_val) => {
            println!("File Locked: {}", file_path)
        }
        None => {
            let lf = file::LoadedFile::new(&file_path);
            let parsed_file = parse::parse(&lf);

            let x: u16 = 1000;
            // single

            let mut tests: Vec<String> = vec![];
            for block in &parsed_file.blocks {
                println!("Block--------");

                let request = CreateCompletionRequestArgs::default()
                    .model("text-davinci-003")
                    .prompt(format!(
                        "Can you write some tests for the following {:?} code: {}",
                        parsed_file.file_type, block
                    ))
                    .max_tokens(x)
                    .build()?;

                let response = client.completions().create(request).await?;

                println!("\nResponse (single):\n");
                let choice = &response.choices[0].text;
                tests.push(choice.to_string());
            }

            let test_string = tests.join("\n");
            let test_file = postprocess::post_process(test_string, &parsed_file);
            let test_file_name = write::gen_test_name(lf);
            let write_result = write::tests_to_file(&test_file, &test_file_name, out_path);

            match write_result {
                Ok(_) => println!("File: {} Written Succesfully.", test_file),
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    };

    Ok(())
}
