/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - guild_user
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: April 27, 2024 [12:52 PM]
//     ||  '-'
/* ************************************************************************** */

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "guild_users")]
pub struct Model
{
    /// Unique ID to reference this entry
    #[sea_orm(primary_key)]
    pub entry_id: i32,

    /// Determines which user the entry is related to
    #[sea_orm(unique)]
    pub user_id: String,

    /// Determines which guild the entry is related to
    #[sea_orm(unique)]
    pub guild_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::GlobalUser",
        from = "Column::UserId",
        to = "super::global_user::Column::UserId"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::Guild",
        from = "Column::GuildId",
        to = "super::guild::Column::GuildId"
    )]
    Guild
}

impl Related<super::GlobalUser> for Entity
{
    fn to() -> RelationDef
    {
        return Relation::User.def();
    }
}

impl Related<super::Guild> for Entity
{
    fn to() -> RelationDef
    {
        return Relation::Guild.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}