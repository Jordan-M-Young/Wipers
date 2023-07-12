use async_openai::{
    types::{CreateCompletionRequestArgs, CreateCompletionResponse},
    Client,
};

use crate::file::FileTypes;

pub struct RequestParams {
    pub max_tokens: u16,
    pub prompt: String,
}

pub fn format_prompt(text_block: &str, file_type: FileTypes) -> String {
    format!(
        "Can you write some tests for the following {:?} code: {}",
        file_type, text_block
    )
}

pub async fn make_openai_request(
    client: &Client,
    params: RequestParams,
) -> Result<CreateCompletionResponse, async_openai::error::OpenAIError> {
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(params.prompt)
        .max_tokens(params.max_tokens)
        .build()?;

    client.completions().create(request).await
}
