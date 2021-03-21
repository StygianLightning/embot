mod channel_links;
mod commands;
mod embed_hook;

use commands::help::*;
use commands::link::*;

use crate::channel_links::ChannelLinks;
use crate::embed_hook::embed;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::gateway::Ready;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

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
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .help(&MY_HELP)
        .normal_message(embed)
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("EMBOT_DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ChannelLinks>(Arc::new(RwLock::new(HashMap::new())));
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    Ok(())
}
