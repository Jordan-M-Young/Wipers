use async_openai::{
    types::{CreateCompletionRequestArgs, CreateCompletionResponse},
    Client,
};

use crate::file::FileTypes;

pub struct RequestParams {
    pub max_tokens: u16,
    pub file_type: FileTypes,
    pub text_block: String,
}

pub async fn make_openai_request(
    client: &Client,
    params: RequestParams,
) -> Result<CreateCompletionResponse, async_openai::error::OpenAIError> {
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(format!(
            "Can you write some tests for the following {:?} code: {}",
            params.file_type, params.text_block
        ))
        .max_tokens(params.max_tokens)
        .build()?;

    client.completions().create(request).await
}
