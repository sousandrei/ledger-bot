use mongodb::Database;
use tracing::info;

use crate::{
    db,
    db::{item::Item, sale},
};
use crate::{db::sale::UserMention, Error};

fn join_users(users: Vec<UserMention>) -> String {
    users
        .into_iter()
        .map(|user| user.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub async fn handler(db: &Database) -> Result<String, Error> {
    let sales = sale::list(db).await?;

    info!("listing sales");

    let mut text = "".to_owned();

    for sale in sales {
        let Item { name, .. } = db::item::get(sale.item, db).await?.unwrap();
        let shared_amount = ((sale.value as f32 * 0.98) / sale.users.len() as f32).floor();
        let market_url = format!("http://www.originsro-market.de/sells/item_id/{}", sale.item);
        let shop_url = format!(
            "http://www.originsro-market.de/shop/owner?owner={}",
            sale.seller
        );

        text.push_str(format!("<b>Id:</b> (<code>{}</code>)\n", sale._id).as_str());
        text.push_str(
            format!(
                "<b>Item:</b> <a href='{}'>{} ({})</a>\n",
                market_url, name, sale.item
            )
            .as_str(),
        );
        text.push_str(format!("<b>Valor:</b> {}\n", sale.value).as_str());
        text.push_str(
            format!(
                "<b>Seller:</b> <a href='{}'>{}</a>\n",
                shop_url, sale.seller
            )
            .as_str(),
        );
        text.push_str(
            format!(
                "<b>Interessados:</b> {}\n",
                join_users(sale.users).replace("@", "")
            )
            .as_str(),
        );
        text.push_str(format!("<b>Valor por pessoa:</b> {}\n\n", shared_amount).as_str());
    }

    if text.is_empty() {
        text = "Nada registrado no momento".to_owned();
    }

    Ok(text)
}
