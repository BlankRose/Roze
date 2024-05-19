/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - all.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 13, 2024 [4:27 AM]
//     ||  '-'
/* ************************************************************************** */

use serenity::all::{CacheHttp, Command};
use crate::core::InteractionContext;
use crate::{LOCALES, get_locale};
use crate::modules::helper::Reply;
use super::{Basic, ModuleBase, ModulesArray, Prettier};

pub struct Modules
{
    modules: ModulesArray
}

impl Modules
{
    pub fn new() -> Self
    {
        return Self{ modules: vec![
            Prettier::new(),
            Basic::new(),
        ] };
    }

    pub async fn register_modules(&self, http: impl CacheHttp)
    {
        for module in &self.modules
        {
            for sub_module in module.sub_modules()
            {
                if let Some(command) = sub_module.register_command()
                {
                    if let Err(res) = Command::create_global_command(&http, command).await
                        { eprintln!("Failed to register {}:\n{}", sub_module.name(), res); }
                    else
                        { println!("Registered {}/{}!", module.name(), sub_module.name()); }
                }
            }
        }
    }

    pub async fn run_command(&self, ctx: InteractionContext<'_>)
    {
        let mut reply = Reply::new(&ctx);
        for module in &self.modules
        {
            for sub_module in module.sub_modules()
            {
                if sub_module.name() == ctx.interaction.data.name
                {
                    let _ = sub_module.run_command(&ctx, &mut reply).await;
                    return;
                }
            }
        }
        let _ = Reply::new(&ctx)
            .content(Some(get_locale!(
                "commands.not_found",
                locale = &ctx.interaction.locale
            )))
            .ephemeral(true)
            .send().await;
    }
}