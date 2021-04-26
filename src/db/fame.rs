use mongodb::{
    bson::{self, oid::ObjectId, Document},
    results::InsertOneResult,
    Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Fame {
    #[serde(default)]
    _id: ObjectId,
    char_id: i32,
    name: String,
    points: i32,
}

impl From<Fame> for Document {
    fn from(fame: Fame) -> Self {
        bson::to_document(&fame).expect("Error converting to bson document")
    }
}

pub async fn _get(id: i32, db: &Database) -> Result<Option<Fame>, Error> {
    let items: Collection<Fame> = db.collection("fame");

    let filter = bson::doc! { "item_it": id };

    match items.find_one(filter, None).await? {
        Some(item) => Ok(Some(item)),
        None => Ok(None),
    }
}

pub async fn _add(item: Fame, db: &Database) -> Result<ObjectId, Error> {
    let items: Collection<Fame> = db.collection("fame");

    let InsertOneResult { inserted_id, .. } = items.insert_one(item, None).await?;

    match inserted_id.as_object_id() {
        Some(id) => Ok(id.to_owned()),
        None => Err(Error::new("ID missing from mongo call")),
    }
}

pub async fn add_bulk(item_list: Vec<Fame>, db: &Database) -> Result<(), Error> {
    let items: Collection<Fame> = db.collection("fame");

    items.insert_many(item_list, None).await?;

    Ok(())
}

pub async fn clear(db: &Database) -> Result<(), Error> {
    let items: Collection<Fame> = db.collection("fame");

    items.drop(None).await?;

    Ok(())
}
