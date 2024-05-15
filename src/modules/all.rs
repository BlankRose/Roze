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
use super::{ModuleBase, ModulesArray, Prettier};

pub struct Modules
{
    modules: ModulesArray
}

impl Modules
{
    pub fn new() -> Self
    {
        return Self{ modules: vec![
            Prettier::new()
        ] };
    }

    pub async fn register_modules(&self, http: impl CacheHttp)
    {
        for module in &self.modules {
            println!("Registering module: {}", module.name());
            for sub_module in module.sub_modules() {
                if let Some(command) = sub_module.register_command()
                {
                    if let Err(res) = Command::create_global_command(&http, command).await {
                        eprintln!("Failed to register {}:\n{}", sub_module.name(), res);
                    } else {
                        println!("Registered {}!", sub_module.name());
                    }
                }
            }
        }
    }
}