use anyhow::Error;
use serde_json::Value;

use crate::pushover::notify;

pub async fn process_notifications(
    result: Value,
    config: &crate::config::Config,
) -> Result<u32, Error> {
    let mut latest_id = config.latest_notification_id;

    for notification in result["data"]["page"]["notifications"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
    {
        let id = notification["id"].as_u64().expect("expected a notification id") as u32;

        if latest_id >= id {
            continue;
        }

        latest_id = id;

        match notification["type"].as_str().unwrap_or("None") {
            "AIRING" => {
                let title = notification["media"]["title"]["userPreferred"]
                    .as_str()
                    .unwrap_or("Unknown");
                let message = format!(
                    "{}{}{}{}{}",
                    notification["contexts"][0].as_str().unwrap_or("? "),
                    notification["episode"].as_u64().unwrap_or(0),
                    notification["contexts"][1].as_str().unwrap_or(" ? "),
                    notification["media"]["title"]["userPreferred"]
                        .as_str()
                        .unwrap_or("Unknown"),
                    notification["contexts"][2].as_str().unwrap_or(" ?")
                );
                notify(
                    title,
                    &message,
                    notification["media"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "FOLLOWING" => {
                let title = "New Follower";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, None, &config.app_token, &config.user_token).await?;
            }
            "ACTIVITY_MESSAGE" => {
                let title = "New Message";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["message"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "ACTIVITY_MENTION" => {
                let title = "New Mention";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["activity"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "ACTIVITY_REPLY" | "ACTIVITY_REPLY_SUBSCRIBED" => {
                let title = "New Reply";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["activity"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "ACTIVITY_LIKE" | "ACTIVITY_REPLY_LIKE" => {
                let title = "New Like";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["activity"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "THREAD_COMMENT_MENTION" => {
                let title = "New Mention";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["comment"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "THREAD_COMMENT_REPLY" | "THREAD_COMMENT_SUBSCRIBED" => {
                let title = "New Reply";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["comment"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "THREAD_COMMENT_LIKE" | "THREAD_LIKE" => {
                let title = "New Like";
                let message = format!(
                    "{}{}",
                    notification["user"]["name"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["comment"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "RELATED_MEDIA_ADDITION" => {
                let title = "New Related Media";
                let message = format!(
                    "{}{}",
                    notification["media"]["title"]["userPreferred"]
                        .as_str()
                        .unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["media"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "MEDIA_DATA_CHANGE" => {
                let title = "New Data Change";
                let message = format!(
                    "{}{}",
                    notification["media"]["title"]["userPreferred"]
                        .as_str()
                        .unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["media"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "MEDIA_MERGE" => {
                let title = "New Media Merge";
                let message = format!(
                    "{}{}",
                    notification["media"]["title"]["userPreferred"]
                        .as_str()
                        .unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["media"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            "MEDIA_DELETION" => {
                let title = "New Media Deletion";
                let message = format!(
                    "{}{}",
                    notification["deletedMediaTitle"].as_str().unwrap_or("Unknown"),
                    notification["context"].as_str().unwrap_or("?")
                );
                notify(
                    title,
                    &message,
                    notification["media"]["siteUrl"].as_str(),
                    &config.app_token,
                    &config.user_token,
                )
                .await?;
            }
            _ => {}
        }
    }

    Ok(latest_id)
} 