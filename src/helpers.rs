use anyhow::Result;

pub async fn send_chat_completion(
    request: async_openai::types::CreateChatCompletionRequest,
    client: &async_openai::Client<async_openai::config::OpenAIConfig>,
) -> Result<String> {
    let response = client
        .chat()
        .create(request)
        .await?
        .choices
        .pop()
        .expect("chat completions should have choices")
        .message
        .content
        .expect("messages should have content");

    Ok(response)
}
