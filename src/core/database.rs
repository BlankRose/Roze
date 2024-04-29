/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - database.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: April 29, 2024 [6:40 AM]
//     ||  '-'
/* ************************************************************************** */

use sea_orm::{ConnectionTrait, ConnectOptions, DatabaseConnection, DbBackend, EntityTrait, Schema};
use crate::entities::{GlobalUser, GuildUser};

pub struct Database
{
    conn: DatabaseConnection
}

impl Database
{
    pub async fn new(database_url: &String) -> Self
    {
        let mut db_opts = ConnectOptions::new(database_url);
        db_opts.sqlx_logging(true)
            .min_connections(5)
            .max_connections(50);
        let conn = sea_orm::Database::connect(db_opts).await
            .expect("Failed to connect to database: ");
        Self::prepare_database(&conn);
        return Self{ conn };
    }

    fn prepare_database(database: &DatabaseConnection)
    {
        DatabaseBuilder::new(database)
            .create_table(GuildUser)
            .create_table(GlobalUser);
    }
}

struct DatabaseBuilder<'a>
{
    connection: &'a DatabaseConnection,
    backend: DbBackend,
    schema: Schema,
}

impl<'a> DatabaseBuilder<'a>
{
    pub fn new(connection: &'a DatabaseConnection) -> Self
    {
        let backend = connection.get_database_backend();
        let schema = Schema::new(backend);
        return Self{connection, backend, schema};
    }

    pub fn create_table<T: EntityTrait>(&self, entity: T) -> &Self
    {
        let query = self.backend.build(&self.schema.create_table_from_entity(entity));
        let _ = self.connection.execute(query);
        return self;
    }
}