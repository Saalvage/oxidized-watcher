use serenity::prelude::*;
use serenity::client::ClientBuilder;
use serenity::framework::standard::CommandGroup;
use serenity::framework::StandardFramework;
use crate::commands::ping::*;
use crate::config::Config;

mod config;
mod event_handler;
mod commands;

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

#[tokio::main]
async fn main() {
    create_client(Config::new(), &[&GENERAL_GROUP])
        .await
        .expect("Failed to create client!")
        .start()
        .await
        .expect("Failed to start client!");
}
