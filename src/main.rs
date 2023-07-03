use async_openai::{types::CreateCompletionRequestArgs, Client};
use std::error::Error;
pub mod file;
pub mod parse;
pub mod postprocess;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let file_path = "./test-inputs/functions.py";

    let lf = file::LoadedFile::new(file_path);
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

    // println!("{}", test_file);

    Ok(())
}
