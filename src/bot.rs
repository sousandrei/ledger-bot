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
mod lend;
mod list;
#[path = "translations/str.rs"] mod str;

#[derive(BotCommand, Debug)]
#[command(rename = "lowercase",description = str::intro())]
enum Command {
    #[command(description = str::help())]
    Help,
    #[command(description = str::add())]
    Add(String),
    #[command(description = str::list())]
    List,
    #[command(description = str::del())]
    Del(String),
    #[command(description = str::lend())]
    Lend(String),
}

async fn answer(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> Result<(), Error> {
    let db = db::get_db().await?;

    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await?;
        }
        Command::Add(input) => {
            info!("{}", cx.update.text().unwrap());
            add::handler(cx, input, db).await?
        }
        Command::List => list::handler(cx, db).await?,
        Command::Del(input) => del::handler(cx, input, db).await?,
        Command::Lend(input) => lend::handler(cx, input).await?,
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
