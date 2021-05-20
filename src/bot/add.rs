use mongodb::{
    bson::{self, oid::ObjectId},
    Database,
};
use regex::Regex;

use telegram_bot::{MessageEntity, MessageEntityKind, User};
use tracing::{error, info};

use crate::db::{
    self,
    item::Item,
    market,
    sale::{self, Sale, Seller, UserMention},
};
use crate::Error;

pub async fn handler(
    msg: &str,
    entities: &[MessageEntity],
    db: &Database,
) -> Result<String, Error> {
    let AddParams {
        item,
        seller_name,
        users,
    } = match parse_add_params(msg, entities) {
        Ok(params) => params,
        Err(error) => {
            return Ok(error.message);
        }
    };

    info!("item {} on shop {}", item, seller_name.clone());

    let shop = market::get(bson::doc! { "owner": seller_name.clone() }, db).await?;

    if shop.is_none() {
        return Ok("Não achei esta lojinha".into());
    }

    let shop = shop.unwrap();

    let seller = Seller {
        id: shop._id,
        name: seller_name.clone(),
    };
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
            seller,
            users,
            value: shop_item.price,
            killcount: 0,
        },
        db,
    )
    .await?;

    Ok(format!(
        "Show, registrei aqui o item {} sendo vendido por {}",
        name, seller_name
    ))
}

#[derive(Debug)]
struct AddParams {
    item: i32,
    seller_name: String,
    users: Vec<UserMention>,
}

fn parse_add_params(message: &str, entities: &[MessageEntity]) -> Result<AddParams, Error> {
    if message.contains('”') || message.contains('“') {
        return Err(Error::new(
            "Opa, to vendo que você tá usando umas aspas diferenciadas. Vamo parar ai ou vou ser obrigado a comunicar pro meu primo miliciano 🔫.\nSe você tiver usando um mac, vá em:\nSystem Preferences > Keyboard > Text\n e desabilite “””““smart”“””” quotes. Mula."
        ));
    }

    let re = Regex::new("/[\\w@]+ (\\d+) \"([\\w\\s]+)\" [@\\w\\s]+")?;

    let caps = re.captures(&message);

    if caps.is_none() {
        error!("Not enough parameters: {:?}", message);

        return Err(Error::new(
            "Tá faltando coisa aí!\nO comando é /add [cod item] \"NomeVendedor\" @interessado1 @interessado2 \nExemplo de uso do comando:\n/add 501 \"Vendi Sai Chorano\" @yurick @sousandrei",
        ));
    }

    let caps = caps.unwrap();

    let item: i32 = caps[1].parse()?;
    let seller_name = caps[2].to_owned();

    let users: Vec<UserMention> = entities
        .iter()
        .filter_map(|entity| {
            if entity.kind == MessageEntityKind::Mention {
                let offset = entity.offset as usize;
                let length = entity.length as usize;

                let user_name = message[offset..(offset + length)].to_string();

                Some(UserMention::TagMention(user_name))
            } else if let MessageEntityKind::TextMention(User {
                id, ref first_name, ..
            }) = entity.kind
            {
                Some(UserMention::TextMention(id, first_name.clone()))
            } else {
                None
            }
        })
        .collect();

    let params = AddParams {
        item,
        seller_name,
        users,
    };

    Ok(params)
}
