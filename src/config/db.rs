use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};

pub async fn establish_connection() -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables"),
    );

    opt.max_connections(5)
        .connect_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await?;

    Ok(db)
}
