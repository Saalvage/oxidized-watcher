use super::prelude::*;
use crate::models;
use crate::models::{get, insert};

#[command]
pub async fn note(ctx: &Context, msg: &Message) -> CommandResult {
    insert(ctx, models::NewNote {
        kind: 1,
        user_id: 0,
        issuer_id: msg.author.id.0,
        info: msg.content.clone(),
    }).await;

    Ok(())
}

#[command]
pub async fn notes(ctx: &Context, msg: &Message) -> CommandResult {
    let id = msg.content.split_terminator(' ').nth(1).ok_or("No user!")?;
    let id = id.parse::<u64>()?;

    let mut result = "```\n".to_string();

    for n in get(ctx, id).await {
        result.push_str(&n.info);
        result.push('\n');
    }

    result.push_str("```");

    msg.reply(ctx, result).await?;

    Ok(())
}
