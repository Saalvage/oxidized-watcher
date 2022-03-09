use super::*;

#[group]
#[commands(ping)]
struct General;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "🏓").await?;

    Ok(())
}