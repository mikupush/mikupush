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