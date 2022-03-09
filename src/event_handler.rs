use serenity::prelude::{ EventHandler, Context };
use serenity::model::prelude::Ready;
use serenity::async_trait;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context,
                   ready: Ready) {
        println!("Bot {}#{} started!", ready.user.name, ready.user.discriminator);
    }
}
