use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use teloxide::{adaptors::AutoSend, prelude::UpdateWithCx, types::Message, Bot};
use tracing::{error, info};

use crate::db::sale;
use crate::Error;

pub async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    input: String,
    db: Database,
) -> Result<(), Error> {
    let id: ObjectId = match input.parse() {
        Ok(id) => id,
        Err(e) => {
            error!("{:#?}", e);
            cx.answer("Id invalido, ta de sanacagem?").await?;
            return Ok(());
        }
    };

    let sale = sale::get(doc! { "_id": id.clone() }, db.clone()).await?;

    info!("deleting sale {:?}", sale.unwrap());
    sale::del(doc! { "_id": id }, db).await?;

    cx.answer("Deletado!").await?;

    Ok(())
}
