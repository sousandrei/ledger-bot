use serde::{Deserialize, Serialize};
use tracing::Level;

mod bot;
mod db;
mod error;

use db::item::Item;
use error::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct GetItems {
    version: i32,
    generation_timestamp: String,
    items: Vec<Item>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // let api_key = "API_KEY";
    let api_key = "cu";

    {
        let items: GetItems = reqwest::Client::new()
            // .get("https://api.originsro.org/api/v1/ping")
            .get("http://localhost:3000/items")
            .header("x-api-key", api_key)
            .send()
            .await?
            .json()
            .await?;

        let db = db::get_db().await?;

        for item in items.items.iter() {
            db::item::get(item.item_id, db.clone()).await?;
            println!("{:#?}", item);
        }
    }

    let db = db::get_db().await?;
    bot::run(db).await?;

    Ok(())
}
