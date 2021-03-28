use teloxide::{
    adaptors::AutoSend,
    prelude::{Request, Requester, RequesterExt, UpdateWithCx},
    types::Message,
    utils::command::BotCommand,
    Bot,
};

use crate::error::RuntimeError;

fn parse_add_params(input: String) -> Result<AddParams, RuntimeError> {
    let parts: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

    if parts.len() < 3 {
        Err(RuntimeError::new(
            "Tá faltando coisa aí! Exemplo de uso do comando:\n/add 501 \"Vendi Sai Chorano\" @yurick @sousandrei",
        ))
    } else {
        let params = AddParams {
            item: parts[0].to_owned(),
            store: parts[1].to_owned(),
            users: parts[2..].to_vec(),
        };

        Ok(params)
    }
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Eu entendo só isso aqui ó:")]
enum Command {
    #[command(description = "Amostra esse texto.")]
    Help,
    #[command(description = "Adiciona um ouvinte para um item, e notifica uma série de usuários.")]
    Add(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AddParams {
    item: String,
    store: String,
    users: Vec<String>,
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), RuntimeError> {
    println!("Message {:#?}", cx.update.text());
    println!("Chat {:#?}", cx.update.chat);
    println!("Chat_id {:#?}", cx.update.chat_id());

    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await?;
        }
        Command::Add(input) => match parse_add_params(input) {
            Ok(AddParams { item, store, users }) => {
                println!("{:#?}", item);
                println!("{:#?}", store);
                println!("{:#?}", users);

                cx.answer(
                    "Show, registrei aqui o seu {} e vou ficar de olho pra quando ele for vendido.",
                )
                .await?;
            }
            Err(RuntimeError { message }) => {
                cx.answer(message).await?;
            }
        },
    };

    Ok(())
}

pub async fn run() -> Result<(), RuntimeError> {
    teloxide::enable_logging!();
    log::info!("Starting bot");

    let bot = Bot::from_env().auto_send();

    // Isso funciona
    // bot.send_message(-580689714, "O PAI TA ON").send().await?;

    teloxide::commands_repl(bot, "RobertaoBot".to_string(), answer).await;

    Ok(())
}
