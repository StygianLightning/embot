use crate::channel_links::ChannelLinks;
use serenity::framework::standard::macros::hook;
use serenity::model::channel::{Message};

use serenity::prelude::Context;
use serenity::utils::MessageBuilder;



#[hook]
pub async fn embed(ctx: &Context, msg: &Message) {
    let channel_links = {
        let data = ctx.data.read().await;
        data.get::<ChannelLinks>()
            .expect("Expected ChannelLinks were not set up")
            .clone()
    };

    let target_channel = {
        let channel_links = channel_links.read().await;
        channel_links.get(&msg.channel_id).cloned()
    };

    if let Some(target_channel) = target_channel {
        let message_link = format!(
            "https://discord.com/channels/{}/{}/{}",
            msg.guild_id.unwrap(), // this is safe because only guilds can be linked in the first place.
            msg.channel_id,
            msg.id
        );

        if let Err(e) = target_channel
            .send_message(ctx, |m| {
                m.content(
                    MessageBuilder::new()
                        .push("New message in ")
                        .mention(&msg.channel_id)
                        .push_line_safe(":")
                        .build(),
                );

                m.embed(|e| {
                    e.author(|author| {
                        if let Some(avatar_url) = msg.author.avatar_url() {
                            author.icon_url(avatar_url);
                        }
                        author.name(&msg.author.name);
                        author
                    });

                    e.description(&msg.content);
                    e.field("Original message", &message_link, false);

                    if let Some(embed_url) = msg.embeds.iter().filter(|e| e.url.is_some()).next() {
                        if let Some(title) = &embed_url.title {
                            e.title(title);
                        }
                        if let Some(url) = embed_url.url.as_ref() {
                            e.url(url);
                        }
                    }

                    e
                });
                m
            })
            .await
        {
            eprintln!("error: {}", e);
        }
    }
}
