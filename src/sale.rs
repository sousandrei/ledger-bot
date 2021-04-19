use mongodb::{
    bson::{self, doc},
    Database,
};
use std::env;
use teloxide::{
    prelude::{Requester, RequesterExt},
    types::ChatId,
    Bot,
};

use crate::{
    db::{market, sale},
    error::Error,
};

pub async fn compare_sales(db: Database) -> Result<(), Error> {
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("CHAT_ID not present on environment")
        .parse()?;

    let chat_id = ChatId::from(chat_id);

    let bot = Bot::from_env().auto_send();

    let sales = sale::list(db.clone()).await?;

    for sale in sales {
        let shop = market::get(doc! { "owner": sale.clone().seller}, db.clone()).await?;

        if shop.is_none() {
            sale::del(bson::to_document(&sale)?, db.clone()).await?;

            let text = format!(
                "o shop de {} fechou, removendo da lista o item {} de {}",
                sale.seller,
                sale.item,
                sale.users.join(" ")
            );
            bot.send_message(chat_id.clone(), text).await?;

            continue;
        }

        let shop = shop.unwrap();

        let shop_item = shop.items.iter().find(|item| item.item_id == sale.item);

        if shop_item.is_none() {
            sale::del(bson::to_document(&sale)?, db.clone()).await?;

            let shared_amount = ((sale.value as f32 * 0.98) / sale.users.len() as f32).floor();

            let text = format!(
                "O item {} de {} vendeu no shop {}\nno valor de {}z, o que dá {}z coletado por interessado.",
                sale.item,
                sale.users.join(" "),
                shop.owner,
                sale.value,
                shared_amount
            );
            bot.send_message(chat_id.clone(), text).await?;

            continue;
        }

        let shop_item = shop_item.unwrap();

        if shop_item.price != sale.value {
            sale::update(sale.item, doc! { "value": shop_item.price }, db.clone()).await?;

            let text = format!(
                "O item {} teve seu preço modificado na shop {}\nDe {}z para {}z\nSeguimos de olho nessa malandragem.",
                sale.item, shop.owner, sale.value, shop_item.price
            );
            bot.send_message(chat_id.clone(), text).await?;

            continue;
        }
    }

    Ok(())
}
