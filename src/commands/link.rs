use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description = "Link this channel to another; messages from this channel will be re-posted to the linked channel"]
pub async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "lanked").await?;
    Ok(())
}
