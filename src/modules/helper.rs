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

use crate::core::{InteractionContext, LOCALES};
use serenity::all::{CommandType, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, Result};
use crate::{get_locale, get_locales};
use super::SubModuleBase;

pub fn new_command(module: &impl SubModuleBase) -> CreateCommand
{
    let key_name = format!("commands.{}.name", module.name());
    let key_desc = format!("commands.{}.description", module.name());

    let mut command = CreateCommand::new(module.name())
        .description(get_locale!(&key_desc));
    for locale in get_locales!()
    {
        command = command
            .name_localized(locale, get_locale!(&key_name, locale = locale))
            .description_localized(locale, get_locale!(&key_desc, locale = locale))
            .kind(CommandType::ChatInput);
    }
    return command;
}

pub struct Reply<'a>
{
    context: &'a InteractionContext<'a>,
    pub has_replied: bool,

    content: Option<String>,
    ephemeral: bool,
}

impl<'a> Reply<'a>
{
    pub fn new(ctx: &'a InteractionContext<'a>) -> Self
    {
        return Self{
            context: ctx,
            has_replied: false,
            content: None,
            ephemeral: false,
        };
    }

    pub fn from(src: Self) -> Self
    {
        return Self{
            context: src.context,
            has_replied: src.has_replied,
            content: src.content,
            ephemeral: src.ephemeral,
        };
    }

    pub fn content<T>(&mut self, content: Option<T>) -> &mut Self
        where T: Into<String>
    {
        if let Some(content) = content
            { self.content = Some(content.into()); }
        else
            { self.content = None; }
        return self;
    }

    pub fn ephemeral(&mut self, ephemeral: bool) -> &mut Self
    {
        self.ephemeral = ephemeral;
        return self;
    }

    pub async fn send(&self) -> Result<()>
    {
        if !self.has_replied
        {
            let mut builder = CreateInteractionResponseMessage::new()
                .ephemeral(self.ephemeral);
            if let Some(content) = &self.content
                { builder = builder.content(content); }

            return self.context.interaction.create_response(
                &self.context.client.http,
                CreateInteractionResponse::Message(builder)
            ).await;
        }
        else
        {
            let mut builder = EditInteractionResponse::new();
            if let Some(content) = &self.content
                { builder = builder.content(content); }

            return match self.context.interaction.edit_response(
                &self.context.client.http,
                builder
            ).await
            {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
        }
    }
}