use mongodb::{
    bson::{self, doc},
    Database,
};
use std::env;
use telegram_bot::{Api, ChatId, SendMessage};
use tracing::info;

use crate::{
    db,
    db::{item::Item, market, sale},
    error::Error,
};

// Sales with killcount equal to this number will be removed from the database.
const KILLCOUNT_THRESHOLD: i32 = 3;

pub async fn compare_sales(db: &Database) -> Result<(), Error> {
    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not present on environment");
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("CHAT_ID not present on environment")
        .parse()?;

    let chat_id = ChatId::from(chat_id);
    let api = Api::new(bot_token);

    let sales = sale::list(db).await?;

    for sale in sales {
        let shop = market::get(doc! { "owner": sale.clone().seller}, db).await?;

        if shop.is_none() {
            if sale.killcount >= KILLCOUNT_THRESHOLD {
                info!(
                    "Item {} has reached the killcount threshold. Removing it.",
                    sale.item,
                );
                sale::del(bson::to_document(&sale)?, db).await?;

                let msg = SendMessage::new(
                    chat_id,
                    format!(
                        "o shop de {} fechou, removendo da lista o item {} de {}",
                        sale.seller,
                        sale.item,
                        sale.users.join(" ")
                    ),
                );
                api.send(msg).await?;
            } else {
                info!(
                    "Item {} is assigned to an inactive shop. Incrementing its killcount {} -> {}",
                    sale.item,
                    sale.killcount,
                    sale.killcount + 1
                );
                sale::update(sale.item, doc! { "killcount": sale.killcount + 1 }, db).await?;
            }

            continue;
        }

        let shop = shop.unwrap();

        let shop_item = shop.items.iter().find(|item| item.item_id == sale.item);
        let Item {
            name: item_name, ..
        } = db::item::get(sale.item, db).await?.unwrap();

        if shop_item.is_none() {
            info!(
                "Item {} could not be found in {}'s store. Removing and reporting as sold",
                sale.item, shop.owner
            );
            sale::del(bson::to_document(&sale)?, db).await?;

            let shared_amount = ((sale.value as f32 * 0.98) / sale.users.len() as f32).floor();

            let msg = SendMessage::new(
                chat_id,
                format!(
                    "O item {} de {} vendeu no shop {}\nno valor de {}z, o que dá {}z coletado por interessado",
                    item_name,
                    sale.users.join(" "),
                    shop.owner,
                    sale.value,
                    shared_amount
                ),
            );
            api.send(msg).await?;
            continue;
        }

        let shop_item = shop_item.unwrap();

        if shop_item.price != sale.value {
            info!(
                "Item {} has changed prices. Updating {} -> {}",
                sale.item, sale.value, shop_item.price
            );
            sale::update(
                sale.item,
                doc! { "value": shop_item.price, "killcount": 0 },
                db,
            )
            .await?;

            let msg = SendMessage::new(
                chat_id,
                format!(
                    "O item {} teve seu preço modificado na shop {}\nDe {}z para {}z\nSeguimos de olho nessa malandragem",
                    item_name, shop.owner, sale.value, shop_item.price
                ),
            );
            api.send(msg).await?;

            continue;
        }

        // If we reached this point of this function, it means:
        //    - the store is properly open
        //    - the item is still being offered by it
        //    - the item has the same value as it had in the previous check
        if sale.killcount > 0 {
            sale::update(sale.item, doc! { "killcount": 0 }, db).await?;

            continue;
        }
    }

    Ok(())
}
