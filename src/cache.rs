use chrono::{Duration, Utc};
use mongodb::{bson::oid::ObjectId, Database};
use std::env;
use tokio::{task::JoinHandle, time::sleep};
use tracing::info;

use crate::{
    db::{
        cache::{self, Cache},
        fame::{self, Fame},
        item::{self, Item},
        market::{self, Market},
    },
    error::Error,
    sale,
};

pub async fn refresh(db: Database) -> Result<(), Error> {
    schedule_cache_refresh("items".to_owned(), Duration::days(7), db.clone()).await;
    schedule_cache_refresh("market".to_owned(), Duration::minutes(10), db.clone()).await;
    schedule_cache_refresh("fame".to_owned(), Duration::minutes(10), db.clone()).await;

    Ok(())
}

async fn schedule_cache_refresh(
    collection: String,
    duration: Duration,
    db: Database,
) -> JoinHandle<Result<(), Error>> {
    tokio::spawn(async move {
        loop {
            refresh_cache(&collection.clone(), duration, db.clone()).await?;

            if collection == "items" {
                sale::compare_sales(db.clone()).await?;
            }

            sleep(duration.to_std().unwrap()).await;
        }
    })
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
    let api_key = env::var("API_KEY").expect("API_KEY not present on environment");

    let data: serde_json::Value = reqwest::Client::new()
        .get(format!(
            // "http://localhost:3000/{}",
            "https://api.originsro.org/api/v1/{}/list",
            collection
        ))
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
