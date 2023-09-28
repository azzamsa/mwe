use std::time::Duration;

use sqlx::{self, postgres::PgPoolOptions, Pool, Postgres};

pub type DB = Pool<Postgres>;

pub async fn connect() -> DB {
    PgPoolOptions::new()
        .max_connections(100)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect("postgres://durin:SpeakFriendAndEnter@127.0.0.1:5432/mwe")
        .await
        .map_err(|err| {
            tracing::error!("{}", err);
        })
        .unwrap()
}

pub async fn migrate(db: &DB) {
    match sqlx::migrate!("db/migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!("{}", &err);
            Err(err)
        }
    }
    .unwrap();
}
