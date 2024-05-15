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

use serenity::all::{ActivityData, Context, EventHandler, Interaction, Ready};
use serenity::async_trait;
use crate::core::{Database};
use crate::modules::Modules;

pub struct Handler
{
    database: Database,
    modules: Modules,
}

impl Handler
{
    pub fn new(database: Database) -> Self
    {
        return Self{ database, modules: Modules::new() };
    }
}

#[async_trait]
impl EventHandler for Handler
{
    async fn ready(&self, ctx: Context, _: Ready)
    {
        let activity = ActivityData::custom("Up and running!");
        ctx.set_activity(Some(activity));

        self.modules.register_modules(&ctx.http).await;

        println!("Client is ready and now online!");
    }

    async fn interaction_create(&self, _: Context, _: Interaction)
    {
    }
}