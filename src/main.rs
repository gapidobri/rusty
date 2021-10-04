use commands::run_command;
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, prelude::Activity},
    prelude::*,
};
use std::env;

mod commands;
mod helper;

struct Handler;

// Hacktoberfest 2021 - #1
// Hacktoberfest 2021 - #2
// Hacktoberfest 2021 - #3
// Hacktoberfest 2021 - #4

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        run_command(context, message).await;
    }

    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        context.set_activity(Activity::listening("to r!help")).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
