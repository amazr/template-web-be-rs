use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn connect_and_migrate(db_url: &str) -> DatabaseConnection {
    let db = Database::connect(db_url)
        .await
        .expect("failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("failed to run migrations");

    db
}
