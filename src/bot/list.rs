use mongodb::Database;
use teloxide::{adaptors::AutoSend, prelude::UpdateWithCx, types::Message, Bot};
use tracing::info;

use crate::db;
use crate::db::item::Item;
use crate::db::sale;

use crate::Error;

pub async fn handler(cx: UpdateWithCx<AutoSend<Bot>, Message>, db: Database) -> Result<(), Error> {
    let sales = sale::list(db).await?;

    info!("listing sales");

    let mut text = "".to_owned();

    for sale in sales {
        let Item { name, .. } = db::item::get(sale.item, db.clone()).await?.unwrap();
        let shared_amount = ((sale.value as f32 * 0.98) / sale.users.len() as f32).floor();

        let line = format!(
            "Id: {}\nItem: {}({})\nSeller: {}\nValor: {}\nInteressados: {}\nValor por pessoa: {}\n======\n",
            sale._id,
            name,
            sale.item,
            sale.seller,
            sale.value,
            sale.users.join(", ").replace("@", ""),
            shared_amount
        );
        text.push_str(line.as_str());
    }

    if text.is_empty() {
        text = "Nada registrado no momento".to_owned();
    }

    cx.answer(text).await?;

    Ok(())
}
