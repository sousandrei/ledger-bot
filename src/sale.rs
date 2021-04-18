// tbot = "0.6.7"

use mongodb::{
    bson::{self, doc},
    Database,
};

use crate::{
    db::{market, sale},
    error::Error,
};

pub async fn compare_sales(db: Database) -> Result<(), Error> {
    let sales = sale::list(db.clone()).await?;

    for sale in sales {
        let shop = market::get(doc! { "owner": sale.clone().seller}, db.clone()).await?;

        if shop.is_none() {
            println!(
                "o shop de {} fechou, removendo da lista o item {} de {}",
                sale.seller,
                sale.item,
                sale.users.join(" ")
            );
            sale::del(bson::to_document(&sale)?, db.clone()).await?;

            continue;
        }

        let shop = shop.unwrap();

        if let None = shop.items.iter().find(|item| item.item_id == sale.item) {
            println!(
                "O item {} de {} vendeu no shop {}",
                sale.item,
                sale.users.join(" "),
                shop.owner
            );
            sale::del(bson::to_document(&sale)?, db.clone()).await?;

            continue;
        }
    }

    Ok(())
}
