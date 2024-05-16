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

// Explicit returns helps readability
#![allow(clippy::needless_return)]

// Init localization
rust_i18n::i18n!("locales", fallback = "en-US");

mod entities;
mod events;
mod core;
mod modules;

use std::env;
use serenity::all::GatewayIntents;
use serenity::Client;
use crate::core::{Database, LOCALES};
use crate::events::EventHandler;

#[tokio::main]
async fn main()
{
    let db_url = env::var("DATABASE")
        .expect("Missing environment variable: DATABASE");
    let database = Database::new(&db_url).await;

    println!("{}", LOCALES.get("test.value", "en-US"));

    let token = env::var("TOKEN")
        .expect("Missing environment variable: TOKEN");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;
    let handler = EventHandler::new(database);
    let mut client = Client::builder(token, intents).event_handler(handler).await
        .expect("Failed to build the client.. Please make sure the provided token is correct!");

    if let Err(what) = client.start().await
        { eprintln!("An error occurred: {what:?}"); }
}