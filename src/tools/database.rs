/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - database.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: April 27, 2024 [7:53 PM]
//     ||  '-'
/* ************************************************************************** */

use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};
use crate::entities::{GlobalUser, GuildUser};

pub async fn prepare_database(database: &DatabaseConnection)
{
    let builder = database.get_database_backend();
    let schema = Schema::new(builder);

    let query = builder.build(&schema.create_table_from_entity(GuildUser));
    let _result = database.execute(query).await;

    let query = builder.build(&schema.create_table_from_entity(GlobalUser));
    let _result = database.execute(query).await;
}