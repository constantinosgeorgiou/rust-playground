use reqwest::{blocking::Client, Url};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = Url::parse("https://localhost/wordpress/wp-json/wc/v3")?;

    let wc = Client::new();

    let url = base_url.join("/products")?;

    let response = wc
        .get(url)
        .basic_auth(
            "ck_4244148d67a6459d974c53812115f8d663e18b80",
            Some("cs_177dcb5d328bbf19ab480ac9b582776901e11151"),
        )
        .send()?
        .json()?;

    println!("response: {}", response);

    Ok(())
}
