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

        if shop
            .items
            .iter()
            .find(|item| item.item_id == sale.item)
            .is_none()
        {
            sale::del(bson::to_document(&sale)?, db.clone()).await?;

            let text = format!(
                "O item {} de {} vendeu no shop {}",
                sale.item,
                sale.users.join(" "),
                shop.owner
            );
            bot.send_message(chat_id.clone(), text).await?;

            continue;
        }
    }

    Ok(())
}
