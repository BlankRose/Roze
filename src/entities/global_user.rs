/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - global_user.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: April 27, 2024 [2:33 PM]
//     ||  '-'
/* ************************************************************************** */

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "global_users")]
pub struct Model
{
    /// Determines which user the entry is related to
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: String,

    /// User's birthday date
    pub birthday: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::GuildUser")]
    Guilds
}

impl Related<super::GuildUser> for Entity
{
    fn to() -> RelationDef
    {
        return Relation::Guilds.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}