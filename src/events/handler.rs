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

use serenity::all::{Context, EventHandler, Ready};
use serenity::async_trait;
use crate::core::Database;

pub struct Handler
{
    pub database: Database
}

#[async_trait]
impl EventHandler for Handler
{
    async fn ready(&self, _: Context, _: Ready)
    {
        println!("Client is ready and now online!");
    }
}