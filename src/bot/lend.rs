
pub async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>, 
    input: String,
) -> Result<(), Error> {
    
    let re = Regex::new("(Me empresta?) (\\d+) (zenys)")?;

    let caps = re.captures(&.input);

    if caps.is_none() {
        error!("Not enough parameters: {:?}", input);

        let error = Err(Error::new(
            "Errou feio, errou rude! Exemplo de uso do comando:\nMe empresta? 10000 zenys",
        ));

        cx.answer(error.message).await?;
        return Ok(());
    }

    cx.answer("Não...").await?;
    return Ok(());
}
