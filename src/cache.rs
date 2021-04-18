use chrono::{Duration, Utc};
use mongodb::{bson::oid::ObjectId, Database};
use tracing::info;

use crate::{
    db::{
        cache::{self, Cache},
        fame::{self, Fame},
        item::{self, Item},
        market::{self, Market},
    },
    error::Error,
};

pub async fn refresh(db: Database) -> Result<(), Error> {
    refresh_cache("items", Duration::days(7), db.clone()).await?;
    refresh_cache("market", Duration::minutes(10), db.clone()).await?;
    refresh_cache("fame", Duration::minutes(10), db.clone()).await?;

    Ok(())
}

async fn refresh_cache(collection: &str, duration: Duration, db: Database) -> Result<(), Error> {
    let c = cache::get(collection, db.clone()).await?;

    if !c.is_none() && c.unwrap().date > Utc::now().into() {
        info!("Using {} cache", collection);
        return Ok(());
    }

    info!("Renewing {} cache", collection);

    cache::del(collection, db.clone()).await?;

    cache_data(collection, db.clone()).await?;

    let expiry_date = Utc::now() + duration;

    cache::add(
        Cache {
            _id: ObjectId::new(),
            collection: collection.to_owned(),
            date: expiry_date.into(),
        },
        db.clone(),
    )
    .await?;

    Ok(())
}

async fn cache_data(collection: &str, db: Database) -> Result<(), Error> {
    let api_key = "cu";

    let data: serde_json::Value = reqwest::Client::new()
        // let api_key = "API_KEY";
        // .get("https://api.originsro.org/api/v1/ping")
        .get(format!("http://localhost:3000/{}", collection))
        .header("x-api-key", api_key)
        .send()
        .await?
        .json()
        .await?;

    match collection {
        "items" => {
            let items = data.get("items").unwrap();
            let items_vec: Vec<Item> = serde_json::from_value(items.to_owned()).unwrap();

            item::clear(db.clone()).await?;
            item::add_bulk(items_vec, db).await?;
        }
        "market" => {
            let market = data.get("shops").unwrap();
            let market_vec: Vec<Market> = serde_json::from_value(market.to_owned()).unwrap();

            market::clear(db.clone()).await?;
            market::add_bulk(market_vec, db).await?;
        }
        "fame" => {
            // TODO: get forgers | same collection with type? hmmmm
            let fame = data.get("brewers").unwrap();
            let fame_vec: Vec<Fame> = serde_json::from_value(fame.to_owned()).unwrap();

            fame::clear(db.clone()).await?;
            fame::add_bulk(fame_vec, db).await?;
        }
        _ => unreachable!(),
    };

    Ok(())
}
