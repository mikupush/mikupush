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
