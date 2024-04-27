/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - main.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: April 27, 2024 [12:25 PM]
//     ||  '-'
/* ************************************************************************** */

mod entities;
mod events;

use std::env;
use serenity::all::GatewayIntents;
use serenity::Client;
use crate::events::EventHandler;

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("Missing environment variable: TOKEN");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(token, intents).event_handler(EventHandler)
        .await.expect("Failed to build the client.. Please make sure the provided token is correct!");

    if let Err(what) = client.start().await {
        eprintln!("An error occurred: {what:?}");
    }
}