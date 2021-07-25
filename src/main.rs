use dotenv::dotenv;
use regex::Regex;
use serenity::{async_trait, model::prelude::*, prelude::*};
use std::env;
use std::option::Option::Some;

struct Handler {
    default_org: String,
    default_repo: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("got message {}", msg.content);
        let regex = Regex::new(r"(a-zA-Z0-9\-)\/(a-zA-Z)\#([0-9]+)\b").unwrap();
        for capture in regex.captures_iter(&msg.content) {
            println!("matched {:?}", capture);
            if let Some(issue_number) = capture.get(1) {
                if let Err(why) = msg
                    .reply(
                        &ctx.http,
                        format!(
                            "https://github.com/bevyengine/bevy/issues/{}",
                            issue_number.as_str()
                        ),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let default_org = env::var("DEFAULT_ORG").expect("Expected DEFAULT_ORG in the environment");
    let default_repo = env::var("DEFAULT_REPO").expect("Expected DEFAULT_REPO in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    let mut client = Client::builder(token)
        .event_handler(Handler {
            default_org,
            default_repo,
        })
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
