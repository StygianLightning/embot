use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::{Embed, GuildChannel, Message};
use serenity::model::id::ChannelId;
use serenity::utils::MessageBuilder;
use std::collections::HashMap;
use std::str::FromStr;

#[command]
#[description = "Link this channel to another; messages from this channel will be re-posted to the linked channel"]
pub async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    for channel in msg.content.split_ascii_whitespace().skip(1) {
        let guild: HashMap<ChannelId, GuildChannel> = msg.guild_id.unwrap().channels(ctx).await?;
        let channel = guild.get(&ChannelId::from_str(channel)?);

        if let Some(channel) = channel {
            println!("found channel {:?}", channel);

            channel
                .send_message(ctx, |m| {
                    m.embed(|mut e| {
                        e.title("Embed title");
                        e.author(|author| {
                            author.name(&msg.author.name);
                            author
                        });

                        e.description("description");

                        e
                    });
                    m
                })
                .await?;
        }
    }

    msg.reply(ctx, "Lanked").await?;

    Ok(())
}
