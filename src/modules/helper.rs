/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - helper.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 14, 2024 [11:50 AM]
//     ||  '-'
/* ************************************************************************** */

use serenity::all::CreateCommand;
use super::SubModuleBase;

pub fn new_command<T: SubModuleBase>(module: T) -> CreateCommand
{
    let local_base = format!("commands.{}", module.name());
    let cmd = CreateCommand::new(module.name())
        .description(format!("{}.description", local_base));
    return cmd;
}