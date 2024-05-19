/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - ping.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 19, 2024 [7:02 AM]
//     ||  '-'
/* ************************************************************************** */

use async_trait::async_trait;
use serenity::all::{CreateCommand};
use crate::core::InteractionContext;
use crate::{LOCALES, get_locale};
use crate::modules::helper::{new_command, Reply};
use crate::modules::SubModuleBase;

pub struct PingShard {}

#[async_trait]
impl SubModuleBase for PingShard
{
    fn name(&self) -> &'static str
    {
        return "ping";
    }

    fn register_command(&self) -> Option<CreateCommand>
    {
        return Some(new_command(self));
    }

    async fn run_command(&self, ctx: &InteractionContext<'_>, reply: &mut Reply<'_>) -> Result<(), ()>
    {
        drop(reply
            .content(Some(get_locale!(
                format!("commands.{}.reply", self.name()),
                locale = &ctx.interaction.locale,
                args = ctx.client.shard_id.to_string()
            )))
            .send().await);
        return Ok(());
    }
}