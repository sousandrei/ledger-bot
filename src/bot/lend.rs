use regex::Regex;
use teloxide::{adaptors::AutoSend, prelude::UpdateWithCx, types::Message, Bot};

pub async fn handler(cx: UpdateWithCx<AutoSend<Bot>, Message>, input: String) -> Result<(), Error> {
    let re = Regex::new("Me empresta\\? (\\d+) zenys")?;

    let caps = re.captures(&input);

    if caps.is_none() {
        cx.answer("Errou feio, errou rude! Exemplo de uso do comando:\nMe empresta? 10000 zenys")
            .await?;
        return Ok(());
    }

    cx.answer("Não...").await?;
    Ok(())
}
