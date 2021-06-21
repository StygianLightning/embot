mod channel_links;
mod commands;
mod embed_hook;
mod new_link_message_sender;

use crate::channel_links::{ChannelLinks, SavedChannelLinks};
use crate::commands::help::*;
use crate::commands::link::*;
use crate::embed_hook::embed;
use crate::new_link_message_sender::new_link_message_receiver_loop;
use crate::new_link_message_sender::NewLinkMessageSender;

use anyhow::anyhow;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::gateway::Ready;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::{PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::Subscriber;

const CHANNEL_LINKS_PATH_ENV_VAR: &'static str = "EMBOT_CHANNEL_LINKS_PATH";
const EMBOT_DISCORD_TOKEN: &'static str = "EMBOT_DISCORD_TOKEN";

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

    let channel_links_string =
        std::env::var(CHANNEL_LINKS_PATH_ENV_VAR).unwrap_or(String::from("channel_links.json"));
    println!("channel links env var: {}", channel_links_string);

    if !channel_links_string.ends_with(".json") {
        anyhow!("Channel links env var is not pointing to a json file!");
    }

    let channel_links_path = PathBuf::from(channel_links_string);

    if let Some(parent) = channel_links_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let saved_channel_links = std::fs::read_to_string(&channel_links_path)
        .map(|json| {
            serde_json::from_str::<SavedChannelLinks>(&json)
                .expect("Faile to load saved channel links")
                .into()
        })
        .unwrap_or_else(|_| HashMap::new());

    // Login with a bot token from the environment
    let token = env::var(EMBOT_DISCORD_TOKEN).expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    let channel_links = Arc::new(RwLock::new(saved_channel_links));

    {
        let mut data = client.data.write().await;
        data.insert::<ChannelLinks>(Arc::clone(&channel_links));
        data.insert::<NewLinkMessageSender>(sender);
    }

    tokio::spawn(new_link_message_receiver_loop(
        receiver,
        channel_links,
        channel_links_path,
    ));

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    Ok(())
}
