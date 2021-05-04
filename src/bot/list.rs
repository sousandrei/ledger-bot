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

        let line = format!(
            "Id: {}\nItem: {}({})\nSeller: {}\nValor: {}\nInteressados: {}\nValor por pessoa: {}\n======\n",
            sale._id,
            name,
            sale.item,
            sale.seller,
            sale.value,
            join_users(sale.users).replace("@", ""),
            shared_amount
        );
        text.push_str(line.as_str());
    }

    if text.is_empty() {
        text = "Nada registrado no momento".to_owned();
    }

    Ok(text)
}
