#[macro_use] extern crate diesel;

use std::sync::Arc;
use diesel::PgConnection;
use serenity::prelude::*;
use serenity::client::ClientBuilder;
use serenity::framework::standard::{CommandGroup, CommandResult};
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::*;
use serenity::model::prelude::*;
use config::Config;
use database::models;
use crate::util::display_name;

mod config;
mod event_handler;
mod util;

mod commands;

mod database;

#[hook]
async fn after_hook(_ctx: &Context, msg: &Message, _cmd_name: &str, err: CommandResult) {
    if let Err(err) = err {
        println!("Failed to execute command \"{}\" by {}, with error: {:?}",
                 msg.content, display_name(&msg.author), err);
    }
}

fn create_client(cfg: Config, command_groups: &[&'static CommandGroup]) -> ClientBuilder<'static> {
    let mut framework = StandardFramework::new()
        .configure(|c| c.prefix(cfg.prefix))
        .after(after_hook);

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
