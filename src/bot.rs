use mongodb::Database;
use std::env;
use tracing::{error, info};

use crate::db;
use crate::Error;

mod add;
mod del;
mod lend;
mod list;

use futures::StreamExt;
use telegram_bot::{
    types::{MessageKind, UpdateKind},
    Api, CanReplySendMessage, Message, SendMessage,
};

pub async fn run() -> Result<(), Error> {
    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not present on environment");
    let bot_username = env::var("BOT_USERNAME").expect("BOT_USERNAME not present on environment");

    let db = db::get_db().await?;

    info!("Starting bot");

    let api = Api::new(bot_token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;

        if let Err(e) = match update.kind {
            UpdateKind::Message(message) => handle_message(message, &api, &db, &bot_username).await,
            UpdateKind::EditedMessage(_) => Ok(()),
            UpdateKind::ChannelPost(_) => Ok(()),
            UpdateKind::EditedChannelPost(_) => Ok(()),
            UpdateKind::InlineQuery(_) => Ok(()),
            UpdateKind::CallbackQuery(_) => Ok(()),
            UpdateKind::Poll(_) => Ok(()),
            UpdateKind::PollAnswer(_) => Ok(()),
            UpdateKind::Error(_) => Ok(()),
            UpdateKind::Unknown => Ok(()),
        } {
            error!("Unable to respond to command: {}", e)
        }
    }

    Ok(())
}

async fn handle_message(
    message: Message,
    api: &Api,
    db: &Database,
    bot_username: &str,
) -> Result<(), Error> {
    if let MessageKind::Text {
        ref data,
        ref entities,
    } = message.kind
    {
        if !data.starts_with('/') {
            return Ok(());
        }

        info!("<{}>: {}", &message.from.first_name, data);

        let first_space = data.find(' ').unwrap_or_else(|| data.len());

        let (cmd, _) = data.split_at(first_space);
        let at_bot_username = ["@", bot_username].concat();

        match cmd.replace(&at_bot_username, "").as_str() {
            "/help" => {
                let msg = message.text_reply("@Canato disse que ia escrever, e ta ai ate hoje...");
                api.send(msg).await?;
            }
            "/add" => {
                let reply = add::handler(data, entities, db).await?;
                let msg = message.text_reply(reply);
                api.send(msg).await?;
            }
            "/list" => {
                let text = list::handler(db).await?;
                let msg = SendMessage::new(message.chat.id(), text);
                api.send(msg).await?;
            }
            "/del" => {
                let reply = del::handler(data, db).await?;
                let msg = message.text_reply(reply);
                api.send(msg).await?;
            }
            "/lend" => {
                let reply = lend::handler(data).await?;
                let msg = message.text_reply(reply);
                api.send(msg).await?;
            }
            _ => {}
        };
    }

    Ok(())
}
