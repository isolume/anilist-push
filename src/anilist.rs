use anyhow::Error;
use reqwest::Client;
use serde_json::{json, Value};

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

pub async fn fetch_notifications(token: &str) -> Result<Value, Error> {
    let client = Client::new();
    let json = json!({"query": QUERY});

    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(json.to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&resp)?)
} 