use std::env;
use teloxide::{
    adaptors::AutoSend,
    prelude::{Request, RequesterExt, UpdateWithCx},
    types::Message,
    utils::command::BotCommand,
    Bot,
};
use tracing::info;

use crate::db;
use crate::Error;

mod add;
mod del;
mod list;

#[derive(BotCommand, Debug)]
#[command(rename = "lowercase", description = "Eu entendo só isso aqui ó:")]
enum Command {
    #[command(description = "Amostra esse texto")]
    Help,
    #[command(description = "Adiciona um alarme para um item, e notifica uma série de usuários")]
    Add(String),
    #[command(description = "Lista os items monitorados")]
    List,
    #[command(description = "Deleta um alarme")]
    Del(String),
}

async fn answer(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> Result<(), Error> {
    let db = db::get_db().await?;

    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await?;
        }
        Command::Add(input) => {
            info!("{}", cx.update.text().clone().unwrap());
            add::handler(cx, input, db).await?
        }
        Command::List => list::handler(cx, db).await?,
        Command::Del(input) => del::handler(cx, input, db).await?,
    };

    Ok(())
}

pub async fn run() -> Result<(), Error> {
    let bot_username = env::var("BOT_USERNAME").expect("BOT_USERNAME not present on environment");

    info!("Starting bot");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, bot_username, answer).await;

    Ok(())
}
