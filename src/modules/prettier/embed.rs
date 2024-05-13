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

use serenity::all::CreateCommand;
use crate::modules::SubModuleBase;

pub struct EmbedCreator {}

impl SubModuleBase for EmbedCreator
{
    fn name(&self) -> &'static str
    {
        return "embed";
    }

    fn register_command(&self) -> Option<CreateCommand>
    {
        return Some(CreateCommand::new(self.name())
            .description("Handles embed creation")
            .dm_permission(false));
    }
}