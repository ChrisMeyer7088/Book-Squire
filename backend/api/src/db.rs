use migration::{DbErr};
use migration::sea_orm::{Database, DbConn};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    Ok(db)
}
