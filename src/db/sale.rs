use bson::doc;
use futures::stream::StreamExt;
use mongodb::{
    bson::{self, oid::ObjectId, Document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection, Database,
};
use serde::{Deserialize, Serialize};
use telegram_bot::UserId;
use tracing::info;

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum UserMention {
    TextMention(UserId, String),
    TagMention(String),
}

impl ToString for UserMention {
    fn to_string(&self) -> String {
        match self {
            UserMention::TextMention(id, name) => format!("[{}](tg://user?id={})", name, id),
            UserMention::TagMention(name) => name.clone(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Seller {
    pub id: ObjectId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sale {
    #[serde(default)]
    pub _id: ObjectId,
    pub item: i32,
    pub seller: Seller,
    pub users: Vec<UserMention>,
    pub value: i32,
    pub killcount: i32,
}

impl From<Sale> for Document {
    fn from(sale: Sale) -> Self {
        bson::to_document(&sale).expect("Error converting to bson document")
    }
}

pub async fn list(db: &Database) -> Result<Vec<Sale>, Error> {
    let items = db.collection("sale");

    let mut sales: Vec<Sale> = Vec::new();

    let mut cursor = items.find(None, None).await?;
    while let Some(item) = cursor.next().await {
        sales.push(item?);
    }

    Ok(sales)
}

pub async fn get(query: Document, db: &Database) -> Result<Option<Sale>, Error> {
    let items: Collection<Sale> = db.collection("sale");

    match items.find_one(query, None).await? {
        Some(item) => Ok(Some(item)),
        None => Ok(None),
    }
}

pub async fn add(sale: Sale, db: &Database) -> Result<ObjectId, Error> {
    let items: Collection<Sale> = db.collection("sale");

    let InsertOneResult { inserted_id, .. } = items.insert_one(sale, None).await?;

    match inserted_id.as_object_id() {
        Some(id) => Ok(id.to_owned()),
        None => Err(Error::new("ID missing from mongo call")),
    }
}

pub async fn update(
    sale_id: &ObjectId,
    update_query: Document,
    db: &Database,
) -> Result<(), Error> {
    let items: Collection<Sale> = db.collection("sale");

    let UpdateResult { modified_count, .. } = items
        .update_one(doc! { "_id": sale_id }, doc! { "$set": update_query }, None)
        .await?;

    if modified_count == 0 {
        info!("Failed to update item");
        Err(Error::new("Failed to update item"))
    } else {
        Ok(())
    }
}

pub async fn del(query: Document, db: &Database) -> Result<u64, Error> {
    let items: Collection<Sale> = db.collection("sale");

    let DeleteResult { deleted_count, .. } = items.delete_one(query, None).await?;

    Ok(deleted_count)
}
