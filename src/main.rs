use figment::providers::{Format, Json};
use serde::Deserialize;
use serenity::prelude::*;
use serenity::framework::standard::macros::*;
use serenity::async_trait;
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
use serenity::model::prelude::*;

#[derive(Deserialize)]
struct Config {
    prefix: String,
    token: String,
}

impl Config {
    fn new() -> Config {
        figment::Figment::new()
            .join(Json::file("Config.json"))
            .extract::<Config>()
            .expect("Failed to load config")
    }
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let config = Config::new();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.prefix))
        .group(&GENERAL_GROUP);

    println!("Started!! :D");

    let mut client = Client::builder(config.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client");

    println!("Started!! :D");

    client.start().await.expect("Failed to start client");
    println!("Started!! :D");
}

#[serenity::framework::standard::macros::command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
