use mongodb::Database;
use std::sync::Arc;
use teloxide::{
    adaptors::AutoSend,
    prelude::{Request, RequesterExt, UpdateWithCx},
    types::Message,
    utils::command::BotCommand,
    Bot,
};
use tokio::sync::Mutex;
use tracing::info;

use crate::db;
use crate::Error;

mod add;

// TODO: make this pretty
//====================================
use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE: Arc<Mutex<Option<Database>>> = Arc::new(Mutex::new(None));
}
//====================================

#[derive(BotCommand, Debug)]
#[command(rename = "lowercase", description = "Eu entendo só isso aqui ó:")]
enum Command {
    #[command(description = "Amostra esse texto.")]
    Help,
    #[command(description = "Adiciona um ouvinte para um item, e notifica uma série de usuários.")]
    Add(String),
}

async fn answer(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> Result<(), Error> {
    println!("Message {:#?}", cx.update.text());
    println!("Chat {:#?}", cx.update.chat);
    println!("Chat_id {:#?}", cx.update.chat_id());
    println!("Chat_id {:#?}", command);

    let db = db::get_db().await?;

    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await?;
        }
        Command::Add(input) => add::handler(cx, input, db).await?,
    };

    Ok(())
}

pub async fn run() -> Result<(), Error> {
    info!("Starting bot");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, "RobertaoBot".to_string(), answer).await;

    Ok(())
}
