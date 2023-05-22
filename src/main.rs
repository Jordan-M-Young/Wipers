use async_openai::{types::CreateCompletionRequestArgs, Client};
use std::error::Error;
pub mod file;
pub mod parse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let file_path = "./test-inputs/functions.js";

    let lf = file::LoadedFile::new(file_path);
    let file_type = &lf.file_type;
    let blocks = parse::parse(&lf);

    let x: u16 = 1000;
    // single

    for block in blocks {
        let request = CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .prompt(format!(
                "Can you write some tests for the following {:?} code: {}",
                file_type, block
            ))
            .max_tokens(x)
            .build()?;

        let response = client.completions().create(request).await?;

        println!("\nResponse (single):\n");
        for choice in response.choices {
            println!("{}", choice.text);
        }
    }

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(format!(
            "Can you write some tests for the followin python code: {}",
            "def add(x,y):\n
        \treturn x + y"
        ))
        .max_tokens(x)
        .build()?;

    let response = client.completions().create(request).await?;

    println!("\nResponse (single):\n");
    for choice in response.choices {
        println!("{}", choice.text);
    }

    Ok(())
}
