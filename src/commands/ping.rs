use super::prelude::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "🏓").await?;

    Ok(())
}