use crate::channel_links::{ChannelLinks, SavedChannelLinks, CHANNEL_LINKS_PATH};
use crate::new_link_message_sender::NewLinkMessageSender;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::{Embed, GuildChannel, Message};
use serenity::model::id::ChannelId;
use serenity::model::Permissions;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

#[command]
#[only_in(guilds)]
#[description = "Link this channel to another; messages from this channel will be re-posted to the linked channel"]
pub async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    let mut is_admin = false;

    if let Some(guild) = msg.guild(ctx).await {
        is_admin = msg.author.id == guild.owner_id;
    }

    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(ctx)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
                is_admin = true;
            }
        }
    }

    if !is_admin {
        msg.reply(ctx, "Only administrators can link channels!")
            .await?;
        return Ok(());
    }

    for channel in msg.content.split_ascii_whitespace().skip(1) {
        let guild: HashMap<ChannelId, GuildChannel> = msg.guild_id.unwrap().channels(ctx).await?;

        if let Some(channel) = guild.get(&ChannelId::from_str(channel)?) {
            let (channel_links, sender) = {
                let data = ctx.data.read().await;
                (
                    data.get::<ChannelLinks>()
                        .expect("Expected ChannelLinks were not set up")
                        .clone(),
                    data.get::<NewLinkMessageSender>()
                        .expect("Expected new channel message sender was not set up")
                        .clone(),
                )
            };

            {
                let mut links = channel_links.write().await;
                if let Some(previous_link) = links.insert(msg.channel_id, channel.id) {
                    println!("removing link from {} to {}", msg.channel_id, previous_link);
                }

                if let Err(e) = sender.send(()) {
                    println!("Error sending new channel link notification: {}", e);
                }
            }

            msg.reply(ctx, "linked channels").await?;
        }
    }

    Ok(())
}
