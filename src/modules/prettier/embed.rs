/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - embed.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 13, 2024 [1:33 PM]
//     ||  '-'
/* ************************************************************************** */

use async_trait::async_trait;
use serenity::all::{CreateCommand};
use crate::core::InteractionContext;
use crate::modules::{helper, SubModuleBase};
use crate::modules::helper::Reply;

pub struct EmbedCreator {}

#[async_trait]
impl SubModuleBase for EmbedCreator
{
    fn name(&self) -> &'static str
    {
        return "embed";
    }

    fn register_command(&self) -> Option<CreateCommand>
    {
        return Some(helper::new_command(self)
            .dm_permission(false));
    }

    async fn run_command(&self, _: &InteractionContext<'_>, reply: &mut Reply<'_>) -> Result<(), ()>
    {
        drop(reply
            .content(Some("Coming soon..."))
            .send().await);
        return Ok(());
    }
}