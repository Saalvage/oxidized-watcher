#[macro_use] extern crate diesel;

use std::env;
use std::sync::Arc;
use diesel::{Connection, SqliteConnection};
use serenity::prelude::*;
use serenity::client::ClientBuilder;
use serenity::framework::standard::CommandGroup;
use serenity::framework::StandardFramework;
use crate::config::Config;
use crate::database::models;

mod config;
mod event_handler;

mod commands;

mod database;

fn create_client(cfg: Config, command_groups: &[&'static CommandGroup]) -> ClientBuilder<'static> {
    let mut framework = StandardFramework::new()
        .configure(|c| c.prefix(cfg.prefix));

    for g in command_groups {
        framework.group_add(g);
    }

    Client::builder(cfg.token)
        .event_handler(event_handler::Handler)
        .framework(framework)
}

struct DatabaseConnection;

impl TypeMapKey for DatabaseConnection {
    type Value = Arc<Mutex<SqliteConnection>>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db = env::var("DATABASE_URL").expect("Did not find database url");

    let connection = diesel::sqlite::SqliteConnection::establish(&db)
        .expect("Failed to connect to database");

    let mut client = create_client(Config::new(), &[&commands::GENERAL_GROUP])
        .await
        .expect("Failed to create client!");

    let connection = Arc::new(Mutex::new(connection));
    client.data.write().await.insert::<DatabaseConnection>(connection);

    client.start()
        .await
        .expect("Failed to start client!");
}
