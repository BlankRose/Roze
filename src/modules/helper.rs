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
use serenity::all::{CommandOptionType, CommandType, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, Result};
use crate::{get_locale, get_locales};
use super::SubModuleBase;

pub fn new_command(module: &impl SubModuleBase) -> CreateCommand
{
    let key_name = format!("commands.{}.name", module.name());
    let key_desc = format!("commands.{}.description", module.name());

    let mut command = CreateCommand::new(module.name())
        .description(get_locale!(&key_desc))
        .kind(CommandType::ChatInput);
    for locale in get_locales!()
    {
        command = command
            .name_localized(locale, get_locale!(&key_name, locale = locale))
            .description_localized(locale, get_locale!(&key_desc, locale = locale))
    }
    return command;
}

pub fn new_option(module: &impl SubModuleBase, name: &str, kind: CommandOptionType) -> CreateCommandOption
{
    let base_key = format!("commands.{}.options.{}", module.name(), name);
    let key_name = format!("{}.name", base_key);
    let key_desc = format!("{}.description", base_key);

    let mut option = CreateCommandOption::new(kind, name, get_locale!(&key_desc));
    for locale in get_locales!()
    {
        option = option
            .name_localized(locale, get_locale!(&key_name, locale = locale))
            .description_localized(locale, get_locale!(&key_desc, locale = locale))
    }
    return option;
}

pub struct Reply<'a>
{
    context: &'a InteractionContext<'a>,
    pub has_replied: bool,

    content: Option<String>,
    embeds: Option<Vec<CreateEmbed>>,
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
            embeds: None,
            ephemeral: false,
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

    pub fn embeds(&mut self, embeds: Option<Vec<CreateEmbed>>) -> &mut Self
    {
        self.embeds = embeds;
        return self;
    }

    pub fn ephemeral(&mut self, ephemeral: bool) -> &mut Self
    {
        self.ephemeral = ephemeral;
        return self;
    }

    pub fn locale(&self) -> &String
    {
        return &self.context.interaction.locale;
    }

    pub async fn send(&self) -> Result<()>
    {
        if !self.has_replied
        {
            let mut builder = CreateInteractionResponseMessage::new()
                .ephemeral(self.ephemeral);
            if let Some(content) = &self.content
                { builder = builder.content(content); }
            if let Some(embeds) = &self.embeds
                { builder = builder.embeds(embeds.to_owned()); }

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
            if let Some(embeds) = &self.embeds
                { builder = builder.embeds(embeds.to_owned()); }

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

    pub async fn fast<T>(&mut self, content: T) -> Result<()>
        where T: Into<String>
    {
        self.content = Some(content.into());
        return self.send().await;
    }

    pub async fn fast_ephemeral<T>(&mut self, content: T) -> Result<()>
        where T: Into<String>
    {
        self.ephemeral = true;
        return self.fast(content).await;
    }
}

pub async fn reply_unhandled_sub(reply: &mut Reply<'_>)
{
    drop(reply.fast_ephemeral(
        get_locale!(
            "commands.not_implemented_sub",
            locale = reply.locale())
    ).await);
}

pub async fn reply_missing_args(reply: &mut Reply<'_>)
{
    drop(reply.fast_ephemeral(
        get_locale!(
            "commands.missing_required",
            locale = reply.locale())
    ).await);
}