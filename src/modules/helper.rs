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

use crate::core::LOCALES;
use serenity::all::CreateCommand;
use crate::get_locale;
use super::SubModuleBase;

pub fn new_command(module: &impl SubModuleBase) -> CreateCommand
{
    let locale_name = format!("commands.{}.name", module.name());
    let locale_desc = format!("commands.{}.description", module.name());

    let mut cmd = CreateCommand::new(module.name())
        .description(get_locale!(&locale_desc));
    for locale in LOCALES.get_available()
    {
        cmd = cmd
            .name_localized(locale, get_locale!(&locale_name, locale = locale))
            .description_localized(locale, get_locale!(&locale_desc, locale = locale));
    }
    return cmd;
}