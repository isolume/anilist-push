use dotenvy::dotenv;
use std::{env, fs};

use reqwest::Client;

use std::collections::HashMap;
use anyhow::Error;
use serde_json::{from_str, json};

async fn notify(title: &str, message: &str, url: Option<&str>) -> Result<(), Error> {
    let mut params = HashMap::new();
    params.insert("token".to_string(), env::var("APPTOKEN")?);
    params.insert("user".to_string(), env::var("USERTOKEN")?);
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

fn store_latest(latest: u32) -> Result<(), Error> {
    fs::write("latest", latest.to_string())?;
    Ok(())
}

fn retrieve_latest() -> u32 {
    if let Ok(contents) = fs::read_to_string("latest") {
        if let Ok(value) = contents.trim().parse::<u32>() {
            return value;
        }
    }
    0
}

const QUERY: &str = "
    query {
        page: Page {
            notifications: notifications {
                ... on AiringNotification {
                    id
                    type
                    episode
                    contexts
                    media: media {
                        title { userPreferred }
                        siteUrl
                    }
                }
                ... on FollowingNotification {
                    id
                    type
                    context
                    user: user {
                        name
                    }
                }
                ... on ActivityMessageNotification {
                    id
                    type
                    context
                    user: user {
                        name
                    }
                    message: message {
                        siteUrl
                    }
                }
                ... on ActivityMentionNotification {
                    id
                    type
                    context
                    user: user {
                        name
                    }
                    activity: activity {
                        ... on TextActivity {
                            siteUrl
                        }
                        ... on ListActivity {
                            siteUrl
                        }
                        ... on MessageActivity {
                            siteUrl
                        }
                    }
                }
                ... on ActivityReplyNotification {
                    id
                    type
                    context
                    user: user {
                        name
                    }
                    activity: activity {
                        ... on TextActivity {
                            siteUrl
                        }
                        ... on ListActivity {
                            siteUrl
                        }
                        ... on MessageActivity {
                            siteUrl
                        }
                    }
                }
                ... on ActivityReplySubscribedNotification {
                    id
                    type
                    context
                    user: user {
                        name
                    }
                    activity: activity {
                        ... on TextActivity {
                            siteUrl
                        }
                        ... on ListActivity {
                            siteUrl
                        }
                        ... on MessageActivity {
                            siteUrl
                        }
                    }
                }
                ... on ActivityLikeNotification {
                    id
                    type
                    context
                    user: user  {
                        name
                    }
                    activity: activity {
                        ... on TextActivity {
                            siteUrl
                        }
                        ... on ListActivity {
                            siteUrl
                        }
                        ... on MessageActivity {
                            siteUrl
                        }
                    }
                }
                ... on ActivityReplyLikeNotification {
                    id
                    type
                    context
                    user: user  {
                        name
                    }
                    activity: activity {
                        ... on TextActivity {
                            siteUrl
                        }
                        ... on ListActivity {
                            siteUrl
                        }
                        ... on MessageActivity {
                            siteUrl
                        }
                    }
                }
                ... on ThreadCommentMentionNotification {
                    id
                    type
                    context
                    user: user  {
                        name
                    }
                    comment: comment {
                        siteUrl
                    }
                }
                ... on ThreadCommentReplyNotification {
                    id
                    type
                    context
                    user: user  {
                        name
                    }
                    comment: comment {
                        siteUrl
                    }
                }
                ... on ThreadCommentSubscribedNotification {
                    id
                    type
                    context
                    user: user  {
                        name
                    }
                    comment: comment {
                        siteUrl
                    }
                }
                ... on ThreadCommentLikeNotification {
                    id
                    type
                    context
                    user: user  {
                        name
                    }
                    comment: comment {
                        siteUrl
                    }
                }
                ... on ThreadLikeNotification {
                    id
                    type
                    context
                    user: user  {
                        name
                    }
                    comment: comment {
                        siteUrl
                    }
                }
                ... on RelatedMediaAdditionNotification {
                    id
                    type
                    context
                    media: media {
                        title { userPreferred }
                        siteUrl
                    }
                }
                ... on MediaDataChangeNotification {
                    id
                    type
                    context
                    media: media {
                        title { userPreferred }
                        siteUrl
                    }
                }
                ... on MediaMergeNotification {
                    id
                    type
                    context
                    media: media {
                        title { userPreferred }
                        siteUrl
                    }
                }
                ... on MediaDeletionNotification {
                    id
                    type
                    context
                    deletedMediaTitle
                }
            }
        }
    }
";



#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut latest: u32 = retrieve_latest();
    dotenv().expect(".env file not found");
    
    let al_token = env::var("al_token").expect("al_token not found");

    let json = json!({"query": QUERY});

    let client = Client::new();

    let resp = client.post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", "Bearer ".to_owned() + &al_token)
        .body(json.to_string())
        .send()
        .await?
        .text()
        .await?;

    let result: serde_json::Value = from_str(&resp)?;
    
    for notification in result["data"]["page"]["notifications"].as_array().unwrap().iter().rev() {
        let id = notification["id"].as_u64().expect("expected a notification id") as u32;

        if latest > id {
            break
        }

        latest = id;

        match notification["type"].as_str().unwrap_or("None") {
            "AIRING" => {
                let title = notification["media"]["title"]["userPreferred"].as_str().unwrap_or("Unknown");
                let message = format!("{}{}{}{}{}",
                                      notification["contexts"][0].as_str().unwrap_or("? "),
                                      notification["episode"].as_u64().unwrap_or(0),
                                      notification["contexts"][1].as_str().unwrap_or(" ? "),
                                      notification["media"]["title"]["userPreferred"].as_str().unwrap_or("Unknown"),
                                      notification["contexts"][2].as_str().unwrap_or(" ?")
                );
                notify(title, &message, notification["media"]["siteUrl"].as_str()).await?;
            }
            "FOLLOWING" => {
                let title = "New Follower";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, None).await?;
            }
            "ACTIVITY_MESSAGE" => {
                let title = "New Message";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["message"]["siteUrl"].as_str()).await?;
            }
            "ACTIVITY_MENTION" => {
                let title = "New Mention";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["activity"]["siteUrl"].as_str()).await?;
            }
            "ACTIVITY_REPLY" | "ACTIVITY_REPLY_SUBSCRIBED" => {
                let title = "New Reply";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["activity"]["siteUrl"].as_str()).await?;
            }
            "ACTIVITY_LIKE" | "ACTIVITY_REPLY_LIKE" => {
                let title = "New Like";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["activity"]["siteUrl"].as_str()).await?;
            }
            "THREAD_COMMENT_MENTION" => {
                let title = "New Mention";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["comment"]["siteUrl"].as_str()).await?;
            }
            "THREAD_COMMENT_REPLY" | "THREAD_COMMENT_SUBSCRIBED" => {
                let title = "New Reply";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["comment"]["siteUrl"].as_str()).await?;
            }
            "THREAD_COMMENT_LIKE" | "THREAD_LIKE" => {
                let title = "New Like";
                let message = format!("{}{}",
                                      notification["user"]["name"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["comment"]["siteUrl"].as_str()).await?;
            }
            "RELATED_MEDIA_ADDITION" => {
                let title = "New Related Media";
                let message = format!("{}{}",
                                      notification["media"]["title"]["userPreferred"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["media"]["siteUrl"].as_str()).await?;
            }
            "MEDIA_DATA_CHANGE" => {
                let title = "New Data Change";
                let message = format!("{}{}",
                                      notification["media"]["title"]["userPreferred"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["media"]["siteUrl"].as_str()).await?;
            }
            "MEDIA_MERGE" => {
                let title = "New Media Merge";
                let message = format!("{}{}",
                                      notification["media"]["title"]["userPreferred"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["media"]["siteUrl"].as_str()).await?;
            }
            "MEDIA_DELETION" => {
                let title = "New Media Deletion";
                let message = format!("{}{}",
                                      notification["deletedMediaTitle"].as_str().unwrap_or("Unknown"),
                                      notification["context"].as_str().unwrap_or("?")
                );
                notify(title, &message, notification["media"]["siteUrl"].as_str()).await?;
            }
            _ => {}
        }
    }
    
    store_latest(latest)?;
    
    Ok(())
}