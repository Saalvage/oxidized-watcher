use diesel::{ExpressionMethods, Insertable, Queryable, QueryDsl, RunQueryDsl};
use serenity::prelude::Context;
use crate::database::{database_action, schema};
use crate::database::schema::notes::dsl;
use crate::models;
use super::schema::notes;

pub struct Note {
    pub id: u64,
    pub kind: i16,
    pub user_id: u64,
    pub issuer_id: u64,
    pub info: String,
}

impl Queryable<notes::SqlType, diesel::pg::Pg> for Note {
    type Row = (i64, i16, i64, i64, String);

    fn build(row: Self::Row) -> Self {
        Note {
            id: row.0 as u64,
            kind: row.1,
            user_id: row.2 as u64,
            issuer_id: row.3 as u64,
            info: row.4
        }
    }
}

pub struct NewNote {
    pub kind: i16,
    pub user_id: u64,
    pub issuer_id: u64,
    pub info: String,
}

#[derive(Insertable)]
#[table_name="notes"]
struct InsertableNewNote {
    pub kind: i16,
    pub user_id: i64,
    pub issuer_id: i64,
    pub info: String,
}

pub async fn get(ctx: &Context, id: u64) -> Vec<Note> {
    database_action(ctx, |connection| {
        dsl::notes
            .filter(dsl::issuer_id.eq(id as i64))
            .load::<models::Note>(connection)
    }).await.unwrap()
}

pub async fn insert(ctx: &Context, note: NewNote) {
    database_action(ctx, |connection| {
        InsertableNewNote{
            kind: note.kind,
            user_id: note.user_id as i64,
            issuer_id: note.issuer_id as i64,
            info: note.info
        }.insert_into(schema::notes::table)
            .execute(connection)
    }).await.unwrap();
}
