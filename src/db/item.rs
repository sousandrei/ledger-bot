use mongodb::{
    bson::{self, oid::ObjectId, Document},
    Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::Error;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum ItemSubType {
    W_FIST,
    W_DAGGER,
    W_1HSWORD,
    W_2HSWORD,
    W_1HSPEAR,
    W_2HSPEAR,
    W_1HAXE,
    W_2HAXE,
    W_MACE,
    W_2HMACE,
    W_STAFF,
    W_2HSTAFF,
    W_BOW,
    W_KNUCKLE,
    W_MUSICAL,
    W_WHIP,
    W_BOOK,
    W_KATAR,
    W_REVOLVER,
    W_RIFLE,
    W_GATLING,
    W_SHOTGUN,
    W_GRENADE,
    W_HUUMA,

    A_ARROW,
    A_DAGGER,
    A_BULLET,
    A_SHELL,
    A_GRENADE,
    A_SHURIKEN,
    A_KUNAI,
    A_CANNONBALL,
    A_THROWWEAPON,

    None,
}

fn default_subtype() -> ItemSubType {
    ItemSubType::None
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum ItemType {
    IT_HEALING,
    IT_UNKNOWN,
    IT_USABLE,
    IT_ETC,
    IT_WEAPON,
    IT_ARMOR,
    IT_CARD,
    IT_PETEGG,
    IT_PETARMOR,
    IT_UNKNOWN2,
    IT_AMMO,
    IT_DELAYCONSUME,
    IT_CASH,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Item {
    #[serde(default)]
    _id: ObjectId,
    item_id: i32,
    unique_name: String,
    pub name: String,
    #[serde(rename = "type")]
    tipo: ItemType,
    #[serde(default = "default_subtype")]
    subtype: ItemSubType,
    npc_price: i32,
    #[serde(default)]
    slots: i32,
}

impl From<Item> for Document {
    fn from(item: Item) -> Self {
        bson::to_document(&item).expect("Error converting to bson document")
    }
}

pub async fn get(id: i32, db: &Database) -> Result<Option<Item>, Error> {
    let items: Collection<Item> = db.collection("items");

    let filter = bson::doc! { "item_id": id };

    match items.find_one(filter, None).await? {
        Some(item) => Ok(Some(item)),
        None => Ok(None),
    }
}

pub async fn add_bulk(item_list: Vec<Item>, db: &Database) -> Result<(), Error> {
    let items: Collection<Item> = db.collection("items");

    items.insert_many(item_list, None).await?;

    Ok(())
}

pub async fn clear(db: &Database) -> Result<(), Error> {
    let items: Collection<Item> = db.collection("items");

    items.drop(None).await?;

    Ok(())
}
