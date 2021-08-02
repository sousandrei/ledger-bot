use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use tracing::{error, info};

use crate::db::sale;
use crate::Error;

pub async fn handler(msg: &str, db: &Database) -> Result<String, Error> {
    let id = msg.split(' ').collect::<Vec<&str>>()[1];

    let id: ObjectId = match id.parse() {
        Ok(id) => id,
        Err(e) => {
            error!("{:#?}", e);
            return Ok("Id invalido, ta de sanacagem?".into());
        }
    };

    let sale = sale::get(doc! { "_id": id }, db).await?;

    info!("deleting sale {:?}", sale.unwrap());
    sale::del(doc! { "_id": id }, db).await?;

    Ok("Deletado!".into())
}
