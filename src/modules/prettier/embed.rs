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
use serenity::all::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::core::InteractionContext;
use crate::modules::{helper, SubModuleBase};

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
            .description("Handles embed creation")
            .dm_permission(false));
    }

    async fn run_command(&self, ctx: InteractionContext<'_>) -> Result<(), ()>
    {
        let _ = ctx.interaction.create_response(&ctx.client.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("")
            )).await;
        return Ok(());
    }
}