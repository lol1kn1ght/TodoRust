pub mod commands;
pub mod controllers;
pub mod structs;

use crate::structs::{Command, Commands};
use commands::register;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::prelude::{GuildId, Message, Ready};
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::collections::HashMap;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(827620881529307217);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("hello").description("Say hello")
            })
        })
        .await
        .unwrap();

        println!("{:#?}", commands);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                println!("Error sending message: {:?}", e);
            }
        }
    }
}

pub async fn init() {
    let TOKEN: String = env::var("TOKEN").expect("TOKEN");
    let mut map: Commands = HashMap::new();

    register(&mut map).await;

    let framework = StandardFramework::new().configure(|c| c.prefix("!"));

    let intents = GatewayIntents::all();
    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
