// Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
// Copyright (C) 2025  Miku Push! Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::error::DbError;
use crate::schema::config as config_table;
use crate::DbPool;
use diesel::prelude::*;
use mikupush_common::{ConfigKey, ConfigKeyValue, ConfigMap};

#[derive(Debug, Clone, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConfigModel {
    pub key: String,
    pub value: String,
}

impl ConfigModel {
    pub fn vec_to_config_map(value: Vec<ConfigModel>) -> ConfigMap {
        let mut map = ConfigMap::new();

        value.into_iter().for_each(|item| {
            let key = ConfigKey::from_string(item.key);

            if key.is_some() {
                map.insert(key.unwrap(), item.value);
            }
        });

        map
    }
}

impl TryFrom<ConfigModel> for ConfigKeyValue {
    type Error = String;

    fn try_from(value: ConfigModel) -> Result<Self, Self::Error> {
        let key = ConfigKey::from_string(value.key.clone())
            .ok_or(format!("invalid configuration key {}", value.key.clone()))?;

        Ok((key, value.value))
    }
}

impl From<ConfigKeyValue> for ConfigModel {
    fn from(config: ConfigKeyValue) -> Self {
        Self {
            key: config.0.key(),
            value: config.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigRepository {
    connection_pool: DbPool,
}

impl ConfigRepository {
    pub fn new(db: DbPool) -> Self {
        Self { connection_pool: db }
    }

    pub fn find_all(&self) -> Result<ConfigMap, DbError> {
        let mut connection = self.connection_pool.get()?;
        let entities = config_table::table
            .select(ConfigModel::as_select())
            .load::<ConfigModel>(&mut connection)?;

        Ok(ConfigModel::vec_to_config_map(entities))
    }

    pub fn find_by_key(&self, key: ConfigKey) -> Result<Option<ConfigKeyValue>, DbError> {
        let mut connection = self.connection_pool.get()?;
        let entity = config_table::table
            .filter(config_table::key.eq(key.key()))
            .first::<ConfigModel>(&mut connection)
            .optional()?;

        let entity = entity
            .map(|e| e.try_into().ok())
            .flatten();

        Ok(entity)
    }

    pub fn save(&self, config: ConfigKeyValue) -> Result<(), DbError> {
        let existing = self.find_by_key(config.0)?;
        let model: ConfigModel = config.into();

        if existing.is_none() {
            self.insert(&model)?;
        } else {
            self.update(&model)?;
        }

        Ok(())
    }

    pub fn delete(&self, key: ConfigKey) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;
        diesel::delete(config_table::table.filter(config_table::key.eq(key.key())))
            .execute(&mut connection)?;
        Ok(())
    }

    fn insert(&self, model: &ConfigModel) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;
        diesel::insert_into(config_table::table)
            .values(model)
            .execute(&mut connection)?;
        Ok(())
    }

    fn update(&self, model: &ConfigModel) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;
        diesel::update(config_table::table)
            .filter(config_table::key.eq(&model.key))
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
    use serial_test::serial;

    #[test]
    #[serial]
    fn config_repository_find_all_should_return_all() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let config = insert_test_config(&db);
        let expected = ConfigMap::from([config]);
        let repository = ConfigRepository::new(db.clone());

        let actual: ConfigMap = repository.find_all().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[serial]
    fn config_repository_find_by_key_should_find_existing() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ConfigRepository::new(db.clone());
        let expected = insert_test_config(&db);

        let actual = repository.find_by_key(expected.0).unwrap();

        assert_eq!(true, actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    #[serial]
    fn config_repository_save_should_insert_new_model() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ConfigRepository::new(db.clone());
        let config = (ConfigKey::Theme, "system".to_string());

        repository.save(config.clone()).unwrap();
        let existing = find_by_key(config.0.clone(), &mut connection);

        assert_eq!(true, existing.is_some());
        assert_eq!(config, existing.unwrap().try_into().unwrap());
    }

    #[test]
    #[serial]
    fn config_repository_delete_should_remove_config() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = ConfigRepository::new(db.clone());
        let config = insert_test_config(&db);

        repository.delete(config.0.clone()).unwrap();
        let existing = find_by_key(config.0, &mut connection);

        assert_eq!(true, existing.is_none());
    }

    fn insert_test_config(db: &DbPool) -> ConfigKeyValue {
        let mut connection = db.get().unwrap();
        let model: ConfigKeyValue = (ConfigKey::Theme, "system".to_string());
        let result = diesel::insert_into(config_table::table)
            .values::<ConfigModel>(model.clone().into())
            .execute(&mut connection);
        println!("insert test config result: {:?}", result);
        model
    }

    fn find_by_key(key: ConfigKey, connection: &mut SqliteConnection) -> Option<ConfigModel> {
        config_table::table
            .find(key.key())
            .first::<ConfigModel>(connection)
            .optional()
            .unwrap()
    }

    fn clean(connection: &mut SqliteConnection) {
        let _ = diesel::delete(config_table::table)
            .execute(connection);
    }
}