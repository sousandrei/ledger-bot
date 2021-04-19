use mongodb::{
    bson::{self, oid::ObjectId},
    Database,
};
use regex::Regex;
use teloxide::{adaptors::AutoSend, prelude::UpdateWithCx, types::Message, Bot};
use tracing::{error, info};

use crate::db;
use crate::db::{
    item::Item,
    market,
    sale::{self, Sale},
};
use crate::Error;

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
            cx.answer(error.message).await?;
            return Ok(());
        }
    };

    info!("item {} on shop {}", item, seller.clone());

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

    let shop_item = shop_item.unwrap();
    let Item { name, .. } = db::item::get(item, db.clone()).await?.unwrap();

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

    cx.answer(format!(
        "Show, registrei aqui o item {} vendido por {}",
        name, seller
    ))
    .await?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AddParams {
    item: i32,
    seller: String,
    users: Vec<String>,
}

fn parse_add_params(input: String) -> Result<AddParams, Error> {
    let re = Regex::new("(\\d+) \"([\\w\\s]+)\" ([@\\w\\s]+)")?;

    let caps = re.captures(&input);

    if caps.is_none() {
        error!("Not enough parameters: {:?}", input);

        return Err(Error::new(
            "Tá faltando coisa aí! Exemplo de uso do comando:\n/add 501 \"Vendi Sai Chorano\" @yurick @sousandrei",
        ));
    }

    let caps = caps.unwrap();

    let item: i32 = caps[1].parse()?;
    let seller = caps[2].to_owned();
    let users = caps[3]
        .split(' ')
        .into_iter()
        .filter(|user| user.starts_with('@'))
        .map(|user| user.to_owned())
        .collect();

    let params = AddParams {
        item,
        seller,
        users,
    };

    Ok(params)
}
