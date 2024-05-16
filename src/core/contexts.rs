/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - contexts.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 16, 2024 [5:30 AM]
//     ||  '-'
/* ************************************************************************** */

use serenity::all::{Context, CommandInteraction};
use crate::core::Database;

pub struct InteractionContext<'a>
{
    pub database: &'a Database,
    pub client: Context,
    pub interaction: CommandInteraction
}

impl<'a> InteractionContext<'a>
{
    pub fn new(database: &'a Database, client: Context, interaction: CommandInteraction) -> Self
    {
        return Self{database, client, interaction};
    }
}