use async_openai::Client;
use std::error::Error;
pub mod cli;
pub mod config;
pub mod file;
pub mod lock;
pub mod openai;
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
        Err(_e) => {
            panic!("No File Argument provided. Try wipers -f '<MY_FILE>' -o '<MY_TEST_DIR>'")
        }
    };

    let file_path = arg_config.file_path;

    match lock_set.get(&file_path) {
        Some(_val) => {
            println!("File Locked: {}", file_path)
        }
        None => {
            let lf = file::LoadedFile::new(&file_path);
            let parsed_file = parse::parse(&lf);

            let mut tests: Vec<String> = vec![];
            for block in &parsed_file.blocks {
                println!("Block--------");
                let prompt_text = openai::format_prompt(block, parsed_file.file_type);
                let params = openai::RequestParams {
                    max_tokens: 1000,
                    prompt: prompt_text,
                };

                match openai::make_openai_request(&client, params).await {
                    Ok(response) => {
                        println!("\nResponse (single):\n");
                        let choice = &response.choices[0].text;
                        tests.push(choice.to_string());
                    }
                    Err(err) => {
                        println!("OpenAI Error {}", err);
                    }
                }
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
