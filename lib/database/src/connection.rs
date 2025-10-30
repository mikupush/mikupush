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