mod bot;
mod error;

use crate::error::RuntimeError;

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    bot::run().await?;

    Ok(())
}
