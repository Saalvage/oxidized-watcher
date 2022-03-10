#[macro_use] extern crate diesel;

use std::sync::Arc;
use diesel::PgConnection;
use serenity::prelude::*;
use serenity::client::ClientBuilder;
use serenity::framework::standard::CommandGroup;
use serenity::framework::StandardFramework;
use config::Config;
use database::models;

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
    type Value = Arc<Mutex<PgConnection>>;
}

#[tokio::main]
async fn main() {
    let mut client = create_client(Config::new(), &[&commands::GENERAL_GROUP])
        .await
        .expect("Failed to create client!");

    let connection = Arc::new(Mutex::new(database::set_up_db()));
    client.data.write().await.insert::<DatabaseConnection>(connection);

    client.start()
        .await
        .expect("Failed to start client!");
}
