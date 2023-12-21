use std::env;

use serenity::async_trait;
use serenity::all::GatewayIntents;
use serenity::client::EventHandler;
use serenity::prelude::Context;

#[inline]
fn get_intents() -> GatewayIntents {
    let mut intents = GatewayIntents::empty();
        intents.insert(GatewayIntents::GUILDS);
        intents.insert(GatewayIntents::GUILD_MESSAGES);
        intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, r: serenity::model::prelude::Ready) {
        log::info!("Connected as {}", r.user.name);
    }
    async fn message(&self, ctx: Context, msg: serenity::model::channel::Message) {
        if msg.content.starts_with("!ping") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                log::error!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    env_logger::builder()
        .filter_module("gemini_moderator", {
            if cfg!(debug_assertions) {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Info
            }
        })
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = serenity::Client::builder(token, get_intents())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }
}
