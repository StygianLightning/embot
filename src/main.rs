mod channel_links;
mod commands;
mod embed_hook;

use channel_links::{ChannelLinks, SavedChannelLinks, CHANNEL_LINKS_PATH};
use commands::help::*;
use commands::link::*;
use embed_hook::embed;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::gateway::Ready;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::Subscriber;

#[group]
#[commands(link)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = Subscriber::builder()
        .with_ansi(false)
        .with_max_level(LevelFilter::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting global tracing default subscriber failed.");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .help(&MY_HELP)
        .normal_message(embed)
        .group(&GENERAL_GROUP);

    let saved_channel_links = std::fs::read_to_string(Path::new(CHANNEL_LINKS_PATH))
        .map(|json| {
            serde_json::from_str::<SavedChannelLinks>(&json)
                .expect("Faile to load saved channel links")
                .into()
        })
        .unwrap_or_else(|_| HashMap::new());

    // Login with a bot token from the environment
    let token = env::var("EMBOT_DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ChannelLinks>(Arc::new(RwLock::new(saved_channel_links)));
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    Ok(())
}
