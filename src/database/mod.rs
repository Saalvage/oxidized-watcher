use std::env;
use std::ops::Deref;
use diesel::{Connection, PgConnection};
use serenity::prelude::Context;
use crate::DatabaseConnection;

pub mod models;
pub mod schema;

pub fn set_up_db() -> PgConnection {
    dotenv::dotenv().ok();
    let db = env::var("DATABASE_URL").expect("Did not find database url");

    diesel::pg::PgConnection::establish(&db)
        .expect("Failed to connect to database")
}

pub async fn database_action<F, T>(ctx: &Context, fun: F) -> T
    where F: FnOnce(&PgConnection) -> T {
    let data = ctx.data.read().await;
    let connection = data.get::<DatabaseConnection>()
        .expect("Database not configured!").lock().await;

    fun(connection.deref())
}
