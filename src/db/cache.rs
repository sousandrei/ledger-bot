use mongodb::{
    bson::{self, oid::ObjectId, DateTime, Document},
    results::InsertOneResult,
    Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Cache {
    #[serde(default)]
    pub _id: ObjectId,
    pub collection: String,
    pub date: DateTime,
}

impl From<Cache> for Document {
    fn from(item: Cache) -> Self {
        bson::to_document(&item).expect("Error converting to bson document")
    }
}

pub async fn get(collection: &str, db: &Database) -> Result<Option<Cache>, Error> {
    let items: Collection<Cache> = db.collection("cache");

    let query = bson::doc! { "collection": collection };

    match items.find_one(query, None).await? {
        Some(item) => Ok(Some(item)),
        None => Ok(None),
    }
}

pub async fn add(item: Cache, db: &Database) -> Result<ObjectId, Error> {
    let items: Collection<Cache> = db.collection("cache");

    let InsertOneResult { inserted_id, .. } = items.insert_one(item, None).await?;

    match inserted_id.as_object_id() {
        Some(id) => Ok(id.to_owned()),
        None => Err(Error::new("ID missing from mongo call")),
    }
}

pub async fn del(collection: &str, db: &Database) -> Result<(), Error> {
    let items: Collection<Cache> = db.collection("cache");

    let query = bson::doc! { "collection": collection };

    items.delete_one(query, None).await?;

    Ok(())
}
