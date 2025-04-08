use anyhow::Error;
use reqwest::Client;
use std::collections::HashMap;

pub async fn notify(
    title: &str,
    message: &str,
    url: Option<&str>,
    app_token: &str,
    user_token: &str,
) -> Result<(), Error> {
    let mut params = HashMap::new();
    params.insert("token".to_string(), app_token.to_string());
    params.insert("user".to_string(), user_token.to_string());
    params.insert("message".to_string(), message.to_string());
    params.insert("title".to_string(), title.to_string());
    params.insert("priority".to_string(), 0.to_string());
    if let Some(url) = url {
        params.insert("url".to_string(), url.to_string());
        params.insert("url_title".to_string(), "View on AniList".to_string());
    }

    let client = Client::new();
    client
        .post("https://api.pushover.net/1/messages.json")
        .form(&params)
        .send()
        .await?;

    Ok(())
} 