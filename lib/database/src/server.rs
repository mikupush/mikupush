use crate::error::DbError;
use crate::schema::servers as servers_table;
use crate::DbPool;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use mikupush_common::Server;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::servers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ServerModel {
    pub id: String,
    pub url: String,
    pub name: String,
    pub icon: Option<String>,
    pub alias: Option<String>,
    pub added_at: NaiveDateTime,
    pub testing: bool,
    pub connected: bool,
    pub healthy: bool,
}

impl TryFrom<ServerModel> for Server {
    type Error = DbError;

    fn try_from(model: ServerModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(model.id.as_str())?,
            url: model.url,
            name: model.name,
            icon: model.icon,
            alias: model.alias,
            added_at: model.added_at.and_utc(),
            testing: model.testing,
            connected: model.connected,
            healthy: model.healthy,
        })
    }
}

impl From<Server> for ServerModel {
    fn from(model: Server) -> Self {
        Self {
            id: model.id.to_string(),
            url: model.url,
            name: model.name,
            icon: model.icon,
            alias: model.alias,
            added_at: model.added_at.naive_utc(),
            testing: model.testing,
            connected: model.connected,
            healthy: model.healthy,
        }
    }
}

pub struct ServerRepository {
    connection_pool: DbPool,
}

impl ServerRepository {
    pub fn new(db: DbPool) -> Self {
        Self { connection_pool: db }
    }

    pub fn find_all(&self) -> Result<Vec<Server>, DbError> {
        let mut connection = self.connection_pool.get()?;
        let entities = servers_table::table
            .select(ServerModel::as_select())
            .load::<ServerModel>(&mut connection)?;

        let models: Vec<Server> = entities
            .iter()
            .map(|entity| entity.clone().try_into())
            .filter_map(Result::ok)
            .collect();

        Ok(models)
    }

    pub fn find_by_id(&self, id: Uuid) -> Result<Option<Server>, DbError> {
        let mut connection = self.connection_pool.get()?;
        let entity = servers_table::table
            .find(id.to_string())
            .first::<ServerModel>(&mut connection)
            .optional()?;

        let entity = entity
            .map(|entity| entity.try_into().ok())
            .unwrap_or(None);

        Ok(entity)
    }

    pub fn find_connected(&self) -> Result<Option<Server>, DbError> {
        let mut connection = self.connection_pool.get()?;
        let entity = servers_table::table
            .filter(servers_table::connected.eq(true))
            .first::<ServerModel>(&mut connection)
            .optional()?;

        let entity = entity
            .map(|entity| entity.try_into().ok())
            .unwrap_or(None);

        Ok(entity)
    }

    pub fn update_connected(&self, id: Uuid, connected: bool) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;

        diesel::update(servers_table::table)
            .set(servers_table::connected.eq(false))
            .execute(&mut connection)?;

        diesel::update(servers_table::table)
            .filter(servers_table::id.eq(&id.to_string()))
            .set(servers_table::connected.eq(connected))
            .execute(&mut connection)?;

        Ok(())
    }

    pub fn save(&self, server: Server) -> Result<(), DbError> {
        let existing = self.find_by_id(server.id)?;
        let model: ServerModel = server.into();

        if existing.is_none() {
            self.insert(&model)?;
        } else {
            self.update(&model)?;
        }

        Ok(())
    }

    fn insert(&self, model: &ServerModel) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;
        diesel::insert_into(servers_table::table)
            .values(model)
            .execute(&mut connection)?;
        Ok(())
    }

    fn update(&self, model: &ServerModel) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;
        diesel::update(servers_table::table)
            .filter(servers_table::id.eq(&model.id))
            .set(model)
            .execute(&mut connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::DbPool;
    use crate::tests::test_database_connection;
    use mikupush_common::Server;
    use serial_test::serial;
    use uuid::Uuid;

    #[test]
    #[serial]
    fn server_repository_find_all_should_return_all() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ServerRepository::new(db.clone());
        let expected: Vec<Server> = insert_many_test_servers(&db, 10);

        let actual: Vec<Server> = repository.find_all().unwrap();

        assert_eq!(expected.len(), actual.len());
    }

    #[test]
    #[serial]
    fn server_repository_find_by_id_should_find_existing() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ServerRepository::new(db.clone());
        let expected: Server = insert_test_server(&db);

        let actual: Option<Server> = repository.find_by_id(expected.id).unwrap();

        assert_eq!(true, actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    #[serial]
    fn server_repository_find_by_id_should_not_find_not_existing() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ServerRepository::new(db.clone());

        let actual: Option<Server> = repository.find_by_id(Uuid::new_v4()).unwrap();

        assert_eq!(true, actual.is_none());
    }

    #[test]
    #[serial]
    fn server_repository_save_should_insert_new_model() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ServerRepository::new(db.clone());
        let server = Server::test();

        repository.save(server.clone()).unwrap();
        let existing = find_by_id(server.id, &mut connection);

        assert_eq!(true, existing.is_some());
        assert_eq!(server, existing.unwrap().try_into().unwrap());
    }

    #[test]
    #[serial]
    fn server_repository_update_connected_should_set_connected() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ServerRepository::new(db.clone());

        let not_connected_server = insert_test_server(&db);
        let connected_server = insert_test_server(&db);

        repository
            .update_connected(connected_server.id, true)
            .expect("update_connected failed");

        let connected = repository.find_connected().unwrap();
        assert!(connected.is_some());

        let connected_server = connected.unwrap();
        assert_eq!(connected_server.id, connected_server.id);

        let not_connected_model = find_by_id(not_connected_server.id, &mut connection).unwrap();
        assert_eq!(false, not_connected_model.connected);
    }

    fn insert_test_server(db: &DbPool) -> Server {
        let mut connection = db.get().unwrap();
        let model = Server::test();
        let result = diesel::insert_into(servers_table::table)
            .values::<ServerModel>(model.clone().into())
            .execute(&mut connection);
        println!("insert test server result: {:?}", result);
        model
    }

    fn insert_many_test_servers(db: &DbPool, count: i8) -> Vec<Server> {
        let mut servers: Vec<Server> = vec![];

        for _ in 0..count {
            servers.push(insert_test_server(db));
        }

        servers
    }

    fn find_by_id(id: Uuid, connection: &mut SqliteConnection) -> Option<ServerModel> {
        servers_table::table
            .find(id.to_string())
            .first::<ServerModel>(connection)
            .optional()
            .unwrap()
    }

    fn clean(connection: &mut SqliteConnection) {
        let _ = diesel::delete(servers_table::table)
            .execute(connection);
    }
}
