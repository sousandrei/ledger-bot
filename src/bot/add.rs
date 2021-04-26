use mongodb::{
    bson::{self, oid::ObjectId},
    Database,
};
use regex::Regex;

use tracing::{error, info};

use crate::db;
use crate::db::{
    item::Item,
    market,
    sale::{self, Sale},
};
use crate::Error;

pub async fn handler(msg: &str, db: &Database) -> Result<String, Error> {
    let AddParams {
        item,
        seller,
        users,
    } = match parse_add_params(msg) {
        Ok(params) => params,
        Err(error) => {
            return Ok(error.message);
        }
    };

    info!("item {} on shop {}", item, seller.clone());

    let shop = market::get(bson::doc! { "owner": seller.clone() }, db).await?;

    if shop.is_none() {
        return Ok("Não achei esta lojinha".into());
    }

    let shop = shop.unwrap();

    let shop_item = shop.items.iter().find(|i| i.item_id == item);

    if shop_item.is_none() {
        return Ok("Este vendedor não esta vendendo este item".into());
    }

    let shop_item = shop_item.unwrap();
    let Item { name, .. } = db::item::get(item, db).await?.unwrap();

    sale::add(
        Sale {
            _id: ObjectId::new(),
            item,
            seller: seller.clone(),
            users,
            value: shop_item.price,
            killcount: 0,
        },
        db,
    )
    .await?;

    Ok(format!(
        "Show, registrei aqui o item {} sendo vendido por {}",
        name, seller
    ))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AddParams {
    item: i32,
    seller: String,
    users: Vec<String>,
}

fn parse_add_params(message: &str) -> Result<AddParams, Error> {
    let re = Regex::new("/[\\w@]+ (\\d+) [\"“]([\\w\\s]+)[\"”] ([@\\w\\s]+)")?;

    let caps = re.captures(&message);

    if caps.is_none() {
        error!("Not enough parameters: {:?}", message);

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
