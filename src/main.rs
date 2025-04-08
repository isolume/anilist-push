use anyhow::Error;
use clap::Parser;

mod config;
mod anilist;
mod pushover;
mod notifications;

use config::{load_config, save_config};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the Pushover App Token
    #[arg(short, long)]
    app_token: Option<String>,

    /// Set the Pushover User Token
    #[arg(short, long)]
    user_token: Option<String>,

    /// Set the AniList Token
    #[arg(short = 't', long)]
    anilist_token: Option<String>,

    /// Reset the latest notification ID
    #[arg(short, long)]
    reset: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut config = load_config()?;

    // Update config if tokens are provided
    if let Some(app_token) = args.app_token.clone() {
        config.app_token = app_token;
    }
    if let Some(user_token) = args.user_token.clone() {
        config.user_token = user_token;
    }
    if let Some(anilist_token) = args.anilist_token.clone() {
        config.anilist_token = anilist_token;
    }
    if args.reset {
        config.latest_notification_id = 0;
    }

    // Save config if it was modified
    if args.app_token.is_some() || args.user_token.is_some() || args.anilist_token.is_some() || args.reset {
        save_config(&config)?;
    }

    // Check if tokens are set
    if config.app_token.is_empty() || config.user_token.is_empty() || config.anilist_token.is_empty() {
        eprintln!("Error: Tokens not set. Please set them using:");
        eprintln!("  --app-token or -a for Pushover App Token");
        eprintln!("  --user-token or -u for Pushover User Token");
        eprintln!("  --anilist-token or -t for AniList Token");
        std::process::exit(1);
    }

    let result = anilist::fetch_notifications(&config.anilist_token).await?;
    let latest_id = notifications::process_notifications(result, &config).await?;
    
    config.latest_notification_id = latest_id;
    save_config(&config)?;
    
    Ok(())
}