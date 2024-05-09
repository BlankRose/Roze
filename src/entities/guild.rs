/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - guild.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 09, 2024 [6:43 AM]
//     ||  '-'
/* ************************************************************************** */

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "guilds")]
pub struct Model
{
    /// Determines which guild the entry is related to
    #[sea_orm(primary_key, auto_increment = false)]
    pub guild_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::GuildUser")]
    Users
}

impl Related<super::GuildUser> for Entity
{
    fn to() -> RelationDef
    {
        return Relation::Users.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}