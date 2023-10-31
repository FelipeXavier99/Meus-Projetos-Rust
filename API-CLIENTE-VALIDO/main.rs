use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "LINK HTTP API DE ALGUM CLIENTE VÁLIDO";
    let params = [("email", "seu-email-aqui"), ("senha", "sua-senha-aqui")];

    let url = reqwest::Url::parse_with_params(url, &params);

    match url {
        Ok(url) => {
            println!("URL: {}", &url.to_string());

            let response = reqwest::Client::new()
                .get(url.to_string())
                .send()
                .await?;

            let body = response.text().await?;
            println!("{}", body);
        }
        Err(err) => {
            eprintln!("Erro ao analisar a URL: {}", err);
        }
    }

    Ok(())
}
