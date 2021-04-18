use teloxide::{adaptors::AutoSend, prelude::UpdateWithCx, types::Message, Bot};

use crate::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AddParams {
    item: i32,
    store: String,
    users: Vec<String>,
}

pub async fn handler(cx: UpdateWithCx<AutoSend<Bot>, Message>, input: String) -> Result<(), Error> {
    match parse_add_params(input) {
        Ok(AddParams { item, store, users }) => {
            println!("{:#?}", item);
            println!("{:#?}", store);
            println!("{:#?}", users);

            cx.answer(
                "Show, registrei aqui o seu {} e vou ficar de olho pra quando ele for vendido.",
            )
            .await?;
        }
        Err(error) => {
            println!("Error: {:#?}", error);
            cx.answer("Something failed").await?;
        }
    };

    Ok(())
}

fn parse_add_params(input: String) -> Result<AddParams, Error> {
    let mut parts: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

    if parts.len() < 3 {
        Err(Error::new(
            "Tá faltando coisa aí! Exemplo de uso do comando:\n/add 501 \"Vendi Sai Chorano\" @yurick @sousandrei",
        ))
    } else {
        let params = AddParams {
            item: parts[0].parse()?,
            store: parts[1].to_owned().replace("\"", ""),
            users: parts[2..]
                .iter_mut()
                .filter(|user| user.starts_with("@"))
                .map(|user| user.to_owned())
                .collect(),
        };

        Ok(params)
    }
}
