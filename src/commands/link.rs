use crate::channel_links::ChannelLinks;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::{Embed, GuildChannel, Message};
use serenity::model::id::ChannelId;
use std::collections::HashMap;
use std::str::FromStr;

#[command]
#[description = "Link this channel to another; messages from this channel will be re-posted to the linked channel"]
pub async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    println!("msg: {:#?}", msg);
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
                println!("links: {:?}", *links)
            }

            msg.reply(ctx, "linked channels").await?;
        }
    }

    Ok(())
}
