use tracing::Level;

mod bot;
mod cache;
mod db;
mod error;

use error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let db = db::get_db().await?;
    cache::refresh(db.clone()).await?;
    bot::run(db).await?;

    Ok(())
}
