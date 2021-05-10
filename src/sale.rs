use mongodb::{
    bson::{self, doc},
    Database,
};
use std::env;
use telegram_bot::{Api, ChatId, ParseMode, SendMessage};
use tracing::info;

use crate::{
    db,
    db::{
        item::Item,
        market,
        sale::{self, UserMention},
    },
    error::Error,
};

fn join_users(users: Vec<UserMention>) -> String {
    users
        .into_iter()
        .map(|user| user.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// Sales with killcount equal to this number will be removed from the database.
const KILLCOUNT_THRESHOLD: i32 = 3;

pub async fn compare_sales(db: &Database) -> Result<(), Error> {
    info!("🔍   Checking if anything has been sold...");

    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not present on environment");
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("CHAT_ID not present on environment")
        .parse()?;

    let chat_id = ChatId::from(chat_id);
    let api = Api::new(bot_token);

    let sales = sale::list(db).await?;

    for sale in sales {
        let shop = market::get(doc! { "owner": sale.clone().seller.name}, db).await?;

        if shop.is_none() {
            if sale.killcount >= KILLCOUNT_THRESHOLD {
                info!(
                    "Item {} has reached the killcount threshold. Removing it.",
                    sale.item,
                );
                sale::del(bson::to_document(&sale)?, db).await?;

                let mut msg = SendMessage::new(
                    chat_id,
                    format!(
                        "O shop de {} fechou já tem um tempo, então a gente vai parar de ficar de olho se o item {} vendeu para {} porque tempo é dinheiro a gente não tem nem um nem outro pra perder (ง'̀-'́)ง",
                        sale.seller.name,
                        sale.item,
                        join_users(sale.users),
                    ),
                );
                msg.parse_mode(ParseMode::MarkdownV2);
                api.send(msg).await?;
            } else {
                info!(
                    "Item {} is assigned to an inactive shop. Incrementing its killcount {} -> {}",
                    sale.item,
                    sale.killcount,
                    sale.killcount + 1
                );
                sale::update(&sale._id, doc! { "killcount": sale.killcount + 1 }, db).await?;
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
                "Item {} could not be found in {}'s store. Removing",
                sale.item, shop.owner
            );
            sale::del(bson::to_document(&sale)?, db).await?;

            let shared_amount = ((sale.value as f32 * 0.98) / sale.users.len() as f32).floor();

            let mut msg = SendMessage::new(
                chat_id,
                if shop._id == sale.seller.id {
                    format!(
                        "O item {} de {} vendeu no shop {}\nno valor de {}z, o que dá {}z coletado por interessado", 
                        item_name,
                        join_users(sale.users),
                        shop.owner,
                        sale.value,
                        shared_amount)
                } else {
                    format!(
                        "Tô checando aqui e tô vendo que o item {} não tá mais na shop {}\no que quer dizer que, independentemente se vendeu ou não, essa pessoa embolsou {}z na surdina\npode ir distribuindo {}z aí pro pessoal 🔫\nse liga aí {}", 
                        item_name,
                        shop.owner,
                        sale.value,
                        shared_amount, join_users(sale.users))
                },
            );

            msg.parse_mode(ParseMode::MarkdownV2);
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
                &sale._id,
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
            sale::update(&sale._id, doc! { "killcount": 0 }, db).await?;

            continue;
        }
    }

    Ok(())
}
