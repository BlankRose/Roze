/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - handler.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: April 27, 2024 [3:21 PM]
//     ||  '-'
/* ************************************************************************** */

use sea_orm::{DatabaseConnection};
use serenity::all::{Context, EventHandler, Ready};
use serenity::async_trait;

pub struct Handler
{
    pub database: DatabaseConnection
}

#[async_trait]
impl EventHandler for Handler
{
    async fn ready(&self, _: Context, _: Ready)
    {
        println!("Client is ready and now online!");
    }
}