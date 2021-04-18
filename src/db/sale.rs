use futures::stream::StreamExt;
use mongodb::{
    bson::{self, oid::ObjectId, Document},
    results::{DeleteResult, InsertOneResult},
    Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sale {
    #[serde(default)]
    pub _id: ObjectId,
    pub item: i32,
    pub seller: String,
    pub users: Vec<String>,
}

impl From<Sale> for Document {
    fn from(item: Sale) -> Self {
        bson::to_document(&item).expect("Error converting to bson document")
    }
}

pub async fn list(db: Database) -> Result<Vec<Sale>, Error> {
    let items = db.collection("sale");

    let mut sales: Vec<Sale> = Vec::new();

    let mut cursor = items.find(None, None).await?;
    while let Some(item) = cursor.next().await {
        sales.push(item?);
    }

    Ok(sales)
}

pub async fn get(query: Document, db: Database) -> Result<Option<Sale>, Error> {
    let items: Collection<Sale> = db.collection("sale");

    match items.find_one(query, None).await? {
        Some(item) => Ok(Some(item)),
        None => Ok(None),
    }
}

pub async fn add(item: Sale, db: Database) -> Result<ObjectId, Error> {
    let items: Collection<Sale> = db.collection("sale");

    let InsertOneResult { inserted_id, .. } = items.insert_one(item, None).await?;

    match inserted_id.as_object_id() {
        Some(id) => Ok(id.to_owned()),
        None => Err(Error::new("ID missing from mongo call")),
    }
}

pub async fn del(query: Document, db: Database) -> Result<i64, Error> {
    let items: Collection<Sale> = db.collection("sale");

    let DeleteResult { deleted_count, .. } = items.delete_one(query, None).await?;

    Ok(deleted_count)
}
