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

#[derive(BotCommand, Debug)]
#[command(rename = "lowercase", description = "Hulk é cabuloso, ele ajuda a organizar as vendas coletivas.\nAdicione um item marcando a galera e eu aviso quando vender.\nDeixe de ser passado pra trás.\nTambém emprestro zenys a uma taxa amiga.\n\nEu entendo só isso aqui ó:")]
enum Command {
    #[command(description = "Amostra esse texto")]
    Help,
    #[command(description = "Adiciona um alarme para um item, e notifica uma série de usuários")]
    Add(String),
    #[command(description = "Lista os items monitorados")]
    List,
    #[command(description = "Deleta um alarme")]
    Del(String),
    #[command(description = "Pede dinheiro emprestado pro Hulk Agiota")]
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
