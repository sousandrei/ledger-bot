use mongodb::{Client, Database};
use tracing::info;
// use std::env;
// use tracing::info;

use crate::Error;

pub mod fame;
pub mod item;
pub mod market;

pub async fn get_db() -> Result<Database, Error> {
    // let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not present on environment");
    let mongo_url = "mongodb://127.0.0.1";

    // Investigate performance of returning client vs db
    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("robertao");

    info!("Mongo connected");

    Ok(db)
}
