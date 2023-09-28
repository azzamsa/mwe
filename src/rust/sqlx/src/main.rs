pub mod db;
pub mod logger;

#[tokio::main]
async fn main() {
    logger::init();
    tracing::info!("App started.");

    let db = db::connect().await;
    // db::migrate(&db).await?;
    find_user_by_id(db).await;
}


#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
}

pub async fn find_user_by_id(db: db::DB) {
    const QUERY: &str = "SELECT * FROM user_ WHERE id = $1";

    match sqlx::query_as::<_, User>(QUERY)
        .bind("1")
        .fetch_optional(&db)
        .await
    {
        Err(err) => {
            tracing::error!("{}", &err);
        }
        Ok(None) => {
            println!("User not found");
        },
        Ok(Some(result)) => {
            println!("{:?}", result);
        }
   }
}
