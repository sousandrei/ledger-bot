use chrono::{DateTime, Utc};
use mongodb::{
    bson::{self, oid::ObjectId, Document},
    results::InsertOneResult,
    Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Location {
    map: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct SellingItem {
    item_id: i32,
    amount: i32,
    price: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum ShopType {
    V,
    B,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Market {
    #[serde(default)]
    _id: ObjectId,
    title: String,
    owner: String,
    location: Location,
    creation_date: DateTime<Utc>,
    #[serde(rename = "type")]
    tipo: ShopType,
    items: Vec<SellingItem>,
}

impl From<Market> for Document {
    fn from(item: Market) -> Self {
        bson::to_document(&item).expect("Error converting to bson document")
    }
}

pub async fn _get(id: i32, db: Database) -> Result<Option<Market>, Error> {
    let items = db.collection("market");

    let filter = bson::doc! { "item_it": id };

    match items.find_one(filter, None).await? {
        Some(document) => {
            let item: Market = bson::from_document(document)?;
            Ok(Some(item))
        }
        None => Ok(None),
    }
}

pub async fn _add(item: Market, db: Database) -> Result<ObjectId, Error> {
    let items: Collection<Market> = db.collection("market");

    let InsertOneResult { inserted_id, .. } = items.insert_one(item.into(), None).await?;

    match inserted_id.as_object_id() {
        Some(id) => Ok(id.to_owned()),
        None => Err(Error::new("ID missing from mongo call")),
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
