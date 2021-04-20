use mongodb::Database;
use teloxide::{adaptors::AutoSend, prelude::UpdateWithCx, types::Message, Bot};
use tracing::info;

use crate::db::sale;
use crate::Error;

pub async fn handler(cx: UpdateWithCx<AutoSend<Bot>, Message>, db: Database) -> Result<(), Error> {
    let sales = sale::list(db).await?;

    info!("listing sales");

    let mut text = "".to_owned();

    for sale in sales {
        let line = format!(
            "id: {}\nitem: {}\nseller: {}\ninteressados: {}\n======\n",
            sale._id,
            sale.item,
            sale.seller,
            sale.users.join(", ").replace("@", "")
        );
        text.push_str(line.as_str());
    }

    if text.is_empty() {
        text = "Nada registrado no momento".to_owned();
    }

    cx.answer(text).await?;

    Ok(())
}
