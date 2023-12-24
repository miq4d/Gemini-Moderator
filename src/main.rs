mod constants;
mod defs;
mod enums;

use std::env;

use defs::GeminiContentBody;
use serenity::async_trait;
use serenity::all::{GatewayIntents, Message, Ready, CreateEmbed};
use serenity::client::EventHandler;
use serenity::model::Color;
use serenity::prelude::Context;

use crate::{
    constants::{DELETE_THRESHOLD, WARN_THRESHOLD, DEBUG_LOG_CHANNEL, MOD_LOG_CHANNEL},
    defs::{GeminiPostBody, GeminiContent, GeminiPostResponse, GeminiPostBodySafetySettings, GeminiPostBodyGenerationConfig},
    enums::{GeminiHarmCategory, GeminiSafetyThreshold}
};

#[inline]
fn get_intents() -> GatewayIntents {
    let mut intents = GatewayIntents::empty();
        intents.insert(GatewayIntents::GUILDS);
        intents.insert(GatewayIntents::GUILD_MESSAGES);
        intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents
}

fn generate_embed(verb: &str, msg: &Message, score: u16, reason: &str) -> serenity::builder::CreateEmbed {
    let embed = CreateEmbed::default()
        .title(format!("{}{}", msg.author.tag(), verb))
        .color(Color::RED)
        .description(format!(">>> *Message: * :warning: ||{}||\n*Score: *{}\n*AI Thoughts: *{}", &msg.content[0..50], score, reason));
    embed
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, r: Ready) {
        log::info!("Connected as {}", r.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        let eligible = 
            !msg.author.bot &&
            !msg.content.is_empty();
        
        if !eligible {
            return;
        }

        let client = reqwest::Client::new();

        let body = GeminiPostBody {
            contents: vec![
                GeminiContent {
                    parts: vec![
                        GeminiContentBody {
                            text: format!(r#"Determine how well the posts sent by users are suitable for posting on social networking sites.
This networking site has this rules:
- Treat everyone with respect. Absolutely no harassment, witch hunting, sexism, racism, or hate speech will be tolerated.
- No spam or self-promotion (server invites, advertisements, etc) without permission from a staff member. However, please do not interpret just posting a URL as advertising.
- No age-restricted or obscene content. This includes text, images, or links featuring nudity, sex, hard violence, or other graphically disturbing content.
Decide the score of the content posted by the user based on this rule. (from 0 to 1000) 0 is a very good post that does not violate the rules, and 1000 is a post that violates the rules perfectly.

Do not output 1000 unless there is a clear discriminatory term. They should be on a much lower score.
Do not output high scores for submissions ex. "a" or "あ". These are probably just tests, and there is nothing wrong with them.

If the score is 0, you don't need to output the reason.

Specified format: score|reason
Example for "wtf": 400|possibly offensive language
Example for "Here is": 0|
Example for "ちんちん": 700|possibly sexually explicit language

Reasons should be output in detail; do not use ambiguous terms such as discriminatory terms.

Post content: 
{}

Bad score and reason:"#, msg.content_safe(&ctx.cache))
                        }
                    ],
                    role: None
                }
            ],
            safety_settings: Some(vec![
                GeminiPostBodySafetySettings {
                    category: GeminiHarmCategory::SexuallyExplicit,
                    threshold: GeminiSafetyThreshold::None,
                },
                GeminiPostBodySafetySettings {
                    category: GeminiHarmCategory::HateSpeech,
                    threshold: GeminiSafetyThreshold::None,
                },
                GeminiPostBodySafetySettings {
                    category: GeminiHarmCategory::Harassment,
                    threshold: GeminiSafetyThreshold::None,
                },
                GeminiPostBodySafetySettings {
                    category: GeminiHarmCategory::DangerousContent,
                    threshold: GeminiSafetyThreshold::None,
                },
            ]),
            generation_config: Some(GeminiPostBodyGenerationConfig {
                temperature: Some(0.0),
                ..Default::default()
            })
        };

        //println!("{:?}", serde_json::to_string(&body).unwrap());

        let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", env::var("GEMINI_API_KEY").unwrap()))
            .json(&body)
            .send()
            .await;

        if let Err(e) = res {
            log::error!("Error: {:?}", e);
            return;
        }

        let res = res.unwrap();

        if !res.status().is_success() {
            log::error!("Error: {:?}", res.text().await.unwrap());
            return;
        }

        let res = res.json::<GeminiPostResponse>().await;

        if let Err(e) = res {
            log::error!("Error: {:?}", e);
            return;
        }

        let res = res.unwrap();

        let mut content = String::new();

        for candidate in res.candidates {
            for part in candidate.content.parts {
                content.push_str(&part.text);
            }
        }

        let separated = content.split_once("|");

        if let None = separated {
            log::error!("Error: {:?}", content);
            return;
        }

        let (score, reason) = separated.unwrap();

        let score = score.parse::<u16>().unwrap_or(0);
        
        DEBUG_LOG_CHANNEL.say(&ctx, format!("\n```\n{}\n```\nScore: {}, Reason: {}", msg.content_safe(&ctx.cache), score, reason)).await.ok();

        if score >= DELETE_THRESHOLD {
            msg.delete(&ctx).await.ok();
        } else if score >= WARN_THRESHOLD {
            msg.reply(&ctx, format!("Your message has been deleted because it is not suitable for posting on social networking sites. (Score: {}, Reason: {})", score, reason)).await.ok();
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
