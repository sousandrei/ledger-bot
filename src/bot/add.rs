use mongodb::{
    bson::{self, oid::ObjectId},
    Database,
};
use teloxide::{adaptors::AutoSend, prelude::UpdateWithCx, types::Message, Bot};
use tracing::{error, info};

use crate::db::{
    market,
    sale::{self, Sale},
};
use crate::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AddParams {
    item: i32,
    seller: String,
    users: Vec<String>,
}

pub async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    input: String,
    db: Database,
) -> Result<(), Error> {
    let AddParams {
        item,
        seller,
        users,
    } = match parse_add_params(input) {
        Ok(params) => params,
        Err(error) => {
            error!("Error: {:#?}", error);
            cx.answer("Something failed").await?;
            return Ok(());
        }
    };

    let shop = market::get(bson::doc! { "owner": seller.clone() }, db.clone()).await?;

    if shop.is_none() {
        cx.answer("Não achei esta lojinha").await?;
        return Ok(());
    }

    let shop = shop.unwrap();

    let shop_item = shop.items.iter().find(|i| i.item_id == item);

    if shop_item.is_none() {
        cx.answer("Este vendedor não esta vendendo este item")
            .await?;
        return Ok(());
    }

    sale::add(
        Sale {
            _id: ObjectId::new(),
            item,
            seller: seller.clone(),
            users,
        },
        db,
    )
    .await?;

    info!("add item {} on shop {}", item, seller.clone());

    cx.answer(format!(
        "Show, registrei aqui o item {} vendido por {}",
        item, seller
    ))
    .await?;

    Ok(())
}

fn parse_add_params(input: String) -> Result<AddParams, Error> {
    let mut parts: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

    if parts.len() < 3 {
        Err(Error::new(
            "Tá faltando coisa aí! Exemplo de uso do comando:\n/add 501 \"Vendi Sai Chorano\" @yurick @sousandrei",
        ))
    } else {
        let params = AddParams {
            item: parts[0].parse()?,
            seller: parts[1].to_owned().replace("\"", ""),
            users: parts[2..]
                .iter_mut()
                .filter(|user| user.starts_with("@"))
                .map(|user| user.to_owned())
                .collect(),
        };

        Ok(params)
    }
}
