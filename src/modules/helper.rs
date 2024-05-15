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
use rust_i18n::{t, available_locales};
use super::SubModuleBase;

pub fn new_command(module: &impl SubModuleBase) -> CreateCommand
{
    let locale_name = format!("commands.{}.name", module.name());
    let locale_desc = format!("commands.{}.description", module.name());

    println!("{}", t!(&locale_desc));
    let mut cmd = CreateCommand::new(module.name())
        .description(t!(&locale_desc));
    for loc in available_locales!()
    {
        let locale_str = loc.to_string();
        println!("{}\n{}", t!(&locale_name, locale = loc), t!(&locale_desc, locale = loc));
        cmd = cmd
            .name_localized(&locale_str, t!(&locale_name, locale = loc))
            .description_localized(locale_str, t!(&locale_desc, locale = loc));
    }
    return cmd;
}