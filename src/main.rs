use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::EditMember;
use serenity::cache::Cache;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandError, CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild::EntityType::Str;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::any::Any;
use std::fmt::{format, Error};
use std::io::Stderr;
use std::str::FromStr;
use std::string::String;
use std::{env, io};

#[group]
#[commands(test)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _context: Context, ready: Ready) {
        println!("Client {} successfully started ", ready.user.tag());
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("TOKEN").expect("token");

    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn test(ctx: &Context, message: &Message) -> CommandResult {
    message.reply(ctx, String::from("I'm alive!")).await?;

    Ok(())
}
