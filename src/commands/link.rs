use crate::channel_links::{ChannelLinks, SavedChannelLinks, CHANNEL_LINKS_PATH};
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::{Embed, GuildChannel, Message};
use serenity::model::id::ChannelId;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

#[command]
#[only_in(guilds)]
#[description = "Link this channel to another; messages from this channel will be re-posted to the linked channel"]
pub async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    for channel in msg.content.split_ascii_whitespace().skip(1) {
        let guild: HashMap<ChannelId, GuildChannel> = msg.guild_id.unwrap().channels(ctx).await?;

        if let Some(channel) = guild.get(&ChannelId::from_str(channel)?) {
            let channel_links = {
                let data = ctx.data.read().await;
                data.get::<ChannelLinks>()
                    .expect("Expected ChannelLinks were not set up")
                    .clone()
            };

            {
                let mut links = channel_links.write().await;
                links.insert(msg.channel_id, channel.id);

                let saved_links: SavedChannelLinks = (&*links).into();

                match serde_json::to_string(&saved_links) {
                    Ok(json) => {
                        if let Err(e) = std::fs::write(Path::new(CHANNEL_LINKS_PATH), &json) {
                            println!("Error {} occurred while saving links {}", e, json);
                        }
                    }
                    Err(e) => println!("Error trying to convert {:?} to json: {}", saved_links, e),
                }
            }

            msg.reply(ctx, "linked channels").await?;
        }
    }

    Ok(())
}
