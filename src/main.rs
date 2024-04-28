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
mod tools;

use std::env;
use sea_orm::{ConnectOptions, Database};
use serenity::all::GatewayIntents;
use serenity::Client;
use crate::events::EventHandler;
use crate::tools::database;

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE")
        .expect("Missing environment variable: DATABASE");
    let mut db_opts = ConnectOptions::new(db_url);
    db_opts.sqlx_logging(true)
        .min_connections(5)
        .max_connections(50);
    let database = Database::connect(db_opts).await
        .expect("Failed to connect to database.. Please make sure the provided connection string is valid!");
    database::prepare_database(&database).await;

    let token = env::var("TOKEN")
        .expect("Missing environment variable: TOKEN");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(token, intents).event_handler(EventHandler { database }).await
        .expect("Failed to build the client.. Please make sure the provided token is correct!");

    if let Err(what) = client.start().await {
        eprintln!("An error occurred: {what:?}");
    }
}