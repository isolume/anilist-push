use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub app_token: String,
    pub user_token: String,
    pub anilist_token: String,
    pub latest_notification_id: u32,
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    confy::load("anilist-push", None)
}

pub fn save_config(config: &Config) -> Result<(), confy::ConfyError> {
    confy::store("anilist-push", None, config)
} 