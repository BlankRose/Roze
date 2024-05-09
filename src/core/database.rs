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

use sea_orm::{ConnectionTrait, ConnectOptions, DbBackend, DbConn, EntityTrait, Schema};
use crate::entities::{GlobalUser, Guild, GuildUser};
use crate::{debug_ln, debug_run};

pub struct Database
{
    conn: DbConn
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
        Self::prepare_database(&conn).await;
        return Self{conn};
    }

    async fn prepare_database(database: &DbConn)
    {
        DatabaseBuilder::new(database)
            .create_table(GuildUser).await
            .create_table(GlobalUser).await
            .create_table(Guild).await;
    }
}

struct DatabaseBuilder<'a>
{
    connection: &'a DbConn,
    backend: DbBackend,
    schema: Schema,
}

impl<'a> DatabaseBuilder<'a>
{
    pub fn new(connection: &'a DbConn) -> Self
    {
        let backend = connection.get_database_backend();
        let schema = Schema::new(backend);
        return Self{connection, backend, schema};
    }

    pub async fn create_table<T: EntityTrait>(&self, entity: T) -> &Self
    {
        debug_run!(drop(self.connection.execute_unprepared(
            &("DROP TABLE ".to_string() + entity.table_name() + " CASCADE;")).await));

        let query = self.backend.build(&self.schema.create_table_from_entity(entity));
        debug_ln!("Creating table {}: {}", entity.table_name(), query.sql);

        let res = self.connection.execute(query).await;
        debug_run!(
            if res.is_err() {
                panic!("Failed creating table {}: {:?}", entity.table_name(), res.unwrap_err());
            });

        return self;
    }
}