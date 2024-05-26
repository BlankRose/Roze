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
use serenity::all::{CommandOptionType, CreateCommand, ResolvedOption, ResolvedValue};
use crate::core::InteractionContext;
use crate::modules::{helper, SubModuleBase};
use crate::modules::helper::{new_command, new_option, Reply};

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
        let opt_title = new_option(self, "title", CommandOptionType::String)
                .max_length(256);
        let opt_desc = new_option(self, "description", CommandOptionType::String)
                .max_length(4096);
        let opt_color = new_option(self, "color", CommandOptionType::String)
                .min_length(6)
                .max_length(6);

        Some(new_command(self)
            .dm_permission(false)
            .add_option(new_option(self, "new", CommandOptionType::SubCommand)
                .add_sub_option(opt_title.to_owned().required(true))
                .add_sub_option(opt_desc.to_owned())
                .add_sub_option(opt_color.to_owned()))
            .add_option(new_option(self, "set_title", CommandOptionType::SubCommand)
                .add_sub_option(opt_title.to_owned()))
            .add_option(new_option(self, "set_description", CommandOptionType::SubCommand)
                .add_sub_option(opt_desc.to_owned()))
            .add_option(new_option(self, "set_color", CommandOptionType::SubCommand)
                .add_sub_option(opt_color.to_owned()))
            .add_option(new_option(self, "delete", CommandOptionType::SubCommand)))
    }

    async fn run_command(&self, ctx: &InteractionContext<'_>, reply: &mut Reply<'_>) -> Result<(), ()>
    {
        let data = &ctx.interaction.data.options();
        if let Some(sub_options) = data.first()
        {
            match sub_options.name
            {
                "new" => {},
                _ => { helper::reply_unhandled_sub(reply).await; }
            }
        }
        else
            { helper::reply_missing_args(reply).await; }
        return Ok(());
    }
}

impl EmbedCreator
{
    pub async fn new(title: Option<String>, description: Option<String>, color: Option<u32>)
    {

    }
}

#[derive(Default)]
struct EmbedCreatorOptions
{
    pub title: Option<String>,
    pub desc: Option<String>,
    pub color: Option<String>,
}