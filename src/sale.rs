use mongodb::{
    bson::{self, doc},
    Database,
};
use teloxide::{
    prelude::{Requester, RequesterExt},
    Bot,
};

use crate::{
    db::{market, sale},
    error::Error,
};

pub async fn compare_sales(db: Database) -> Result<(), Error> {
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
            bot.send_message(-580689714, text).await?;

            continue;
        }

        let shop = shop.unwrap();

        if let None = shop.items.iter().find(|item| item.item_id == sale.item) {
            sale::del(bson::to_document(&sale)?, db.clone()).await?;

            let text = format!(
                "O item {} de {} vendeu no shop {}",
                sale.item,
                sale.users.join(" "),
                shop.owner
            );
            bot.send_message(-580689714, text).await?;

            continue;
        }
    }

    Ok(())
}
