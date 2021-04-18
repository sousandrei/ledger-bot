use chrono::{DateTime, Utc};
use mongodb::{
    bson::{self, oid::ObjectId, Document},
    Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Location {
    pub map: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct SellingItem {
    pub item_id: i32,
    pub amount: i32,
    pub price: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum ShopType {
    V,
    B,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Market {
    #[serde(default)]
    pub _id: ObjectId,
    pub title: String,
    pub owner: String,
    pub location: Location,
    pub creation_date: DateTime<Utc>,
    #[serde(rename = "type")]
    pub tipo: ShopType,
    pub items: Vec<SellingItem>,
}

impl From<Market> for Document {
    fn from(item: Market) -> Self {
        bson::to_document(&item).expect("Error converting to bson document")
    }
}

pub async fn get(query: Document, db: Database) -> Result<Option<Market>, Error> {
    let items: Collection<Market> = db.collection("market");

    match items.find_one(query, None).await? {
        Some(item) => Ok(Some(item)),
        None => Ok(None),
    }
}

pub async fn add_bulk(item_list: Vec<Market>, db: Database) -> Result<(), Error> {
    let items: Collection<Market> = db.collection("market");

    items.insert_many(item_list, None).await?;

    Ok(())
}

pub async fn clear(db: Database) -> Result<(), Error> {
    let items: Collection<Market> = db.collection("market");

    items.drop(None).await?;

    Ok(())
}
