use mongodb::{Client, Database};
use std::env;
use tracing::info;

use crate::Error;

pub mod cache;
pub mod fame;
pub mod item;
pub mod market;
pub mod sale;

pub async fn get_db() -> Result<Database, Error> {
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not present on environment");

    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("robertao");

    info!("Mongo connected");

    Ok(db)
}
