// Copyright 2025 Miku Push! Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn create_database_connection(uri: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(uri);
    let pool = Pool::builder()
        .build(manager)
        .expect("Error creating pool");

    let mut connection = pool.get().expect("Error connecting to database");

    connection.run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    pool
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_database_connection() -> DbPool {
        create_database_connection("sqlite://test-database.db")
    }
}