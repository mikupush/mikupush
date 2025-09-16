/// Copyright 2025 Miku Push! Team
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
use log::debug;
use mikupush_migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tauri::{App, Manager};

pub async fn setup_app_database_connection(app: &App) -> DatabaseConnection {
    let app_dir = app.path().app_data_dir().unwrap();
    let database_file = app_dir.join("mikupush.db");
    debug!("sqlite database file: {:?}", database_file);

    if !database_file.exists() {
        debug!("sqlite database file does not exist, creating file...");
        std::fs::create_dir_all(app_dir).unwrap();
        std::fs::File::create(&database_file).unwrap();
        debug!("sqlite database file created on: {:?}", database_file);
    }

    let database_url = format!("sqlite://{}", database_file.to_str().unwrap());
    create_database_connection(&database_url).await
}

pub async fn setup_test_database_connection() -> DatabaseConnection {
    create_database_connection("sqlite::memory:").await
}

async fn create_database_connection(database_url: &str) -> DatabaseConnection {
    debug!("sqlite database url: {:?}", database_url);
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    debug!(
        "executing migrations on sqlite database: {:?}",
        database_url
    );
    Migrator::up(&db, None).await.unwrap();

    db
}
