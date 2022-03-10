use super::prelude::*;
use std::ops::Deref;
use diesel::prelude::*;
use crate::{DatabaseConnection, models};
use crate::database::schema;
use schema::notes::dsl;

#[command]
pub async fn note(ctx: &Context, msg: &Message) -> CommandResult {
    let note = models::NewNote {
        kind: 0,
        user: 0,
        issuer: *msg.author.id.as_u64() as i64,
        info: msg.content.clone()
    };

    let data = ctx.data.read().await;
    let connection = data.get::<DatabaseConnection>()
        .expect("Database not configured!").lock().await;

    diesel::insert_into(schema::notes::table)
        .values(note)
        .execute(connection.deref())?;

    Ok(())
}

#[command]
pub async fn notes(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let connection = data.get::<DatabaseConnection>()
        .expect("Database not configured!").lock().await;

    let id = msg.content.split_terminator(' ').nth(1).ok_or("No user!")?;
    let id = id.parse::<u64>()? as i64;

    let notes = schema::notes::dsl::notes
        .filter(dsl::issuer.eq(id))
        .load::<models::Note>(connection.deref())?;

    let mut result = "```\n".to_string();

    for n in notes {
        result.push_str(&n.info);
        result.push('\n');
    }

    result.push_str("```");

    msg.reply(ctx, result).await?;

    Ok(())
}