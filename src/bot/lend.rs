use regex::Regex;

use crate::error::Error;

pub async fn handler(msg: &str) -> Result<String, Error> {
    let re = Regex::new("Me empresta\\? (\\d+) zenys")?;

    let caps = re.captures(msg);

    if caps.is_none() {
        return Ok(
            "Errou feio, errou rude! Exemplo de uso do comando:\nMe empresta? 10000 zenys".into(),
        );
    }

    Ok("Não...".into())
}
