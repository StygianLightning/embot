use crate::channel_links::ChannelLinks;
use serenity::framework::standard::macros::hook;
use serenity::model::channel::{GuildChannel, Message};
use serenity::model::id::ChannelId;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;
use std::collections::HashMap;
use std::str::FromStr;

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
        println!("channel links: {:?}", *channel_links);
        channel_links.get(&msg.channel_id).cloned()
    };

    println!(
        "input channel: {:?}, output channel: {:?}",
        msg.channel_id, target_channel
    );

    if let Some(target_channel) = target_channel {
        let message_link = format!(
            "https://discord.com/channels/{}/{}/{}",
            msg.guild_id.unwrap(),
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
                        author.icon_url(msg.author.avatar_url().unwrap());
                        author.name(&msg.author.name);
                        author
                    });

                    e.description(&msg.content);
                    e.field("Original message", &message_link, false);

                    if let Some(embed_url) = msg.embeds.iter().filter(|e| e.url.is_some()).next() {
                        if let Some(title) = &embed_url.title {
                            e.title(title);
                        }
                        e.url(embed_url.url.as_ref().unwrap());
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
