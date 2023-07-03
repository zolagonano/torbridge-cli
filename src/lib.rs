use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::error::Error;

pub async fn get_transport_modes() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let options_page = reqwest::get("https://bridges.torproject.org/options/")
        .await?
        .text()
        .await?;

    let parsed_page = Html::parse_document(&options_page);
    let select_selector = Selector::parse(r#"select[id="bridgedb-advanced-options-transport"]"#)?;

    let select_options = parsed_page
        .select(&select_selector)
        .next()
        .ok_or("No options found")?;

    let option_selector = Selector::parse("option")?;
    let transports = select_options
        .select(&option_selector)
        .map(|option| {
            let value = option.value().attr("value").unwrap().to_string();
            let inner_html = option.inner_html();
            Ok((value, inner_html))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(transports)
}

pub async fn request_transport(transport_type: &str) -> Result<(String, String), Box<dyn Error>> {
    let url = Url::parse_with_params(
        "https://bridges.torproject.org/bridges",
        &[("transport", transport_type)],
    )?;

    let bridges_page = reqwest::get(url).await?.text().await?;
    let parsed_page = Html::parse_document(&bridges_page);

    let img_selector =
        Selector::parse(r#"img[alt="Your browser is not displaying images properly."]"#)?;
    let image_base64 = parsed_page
        .select(&img_selector)
        .next()
        .and_then(|img| img.value().attr("src"))
        .ok_or("Image source not found")?
        .to_string();

    let input_selector = Selector::parse(r#"input[name="captcha_challenge_field"]"#)?;
    let challenge_field = parsed_page
        .select(&input_selector)
        .next()
        .and_then(|input| input.value().attr("value"))
        .ok_or("Challenge field not found")?
        .to_string();

    Ok((image_base64, challenge_field))
}

pub async fn submit_answer(
    transport_type: &str,
    challenge_field: String,
    captcha_response: String,
) -> Result<String, Box<dyn Error>> {
    let mut challenge_data: HashMap<&str, String> = HashMap::new();
    challenge_data.insert("captcha_challenge_field", challenge_field);
    challenge_data.insert("captcha_response_field", captcha_response);
    challenge_data.insert("submit", "submit".to_string());

    let client = Client::new();
    let url = Url::parse_with_params(
        "https://bridges.torproject.org/bridges",
        &[("transport", transport_type)],
    )?;
    let response_page = client
        .post(url)
        .form(&challenge_data)
        .send()
        .await?
        .text()
        .await?;

    let parsed_page = Html::parse_document(&response_page);

    let bridgelines_selector = Selector::parse(r#"div[id="bridgelines"]"#)?;
    let bridge_lines = parsed_page
        .select(&bridgelines_selector)
        .next()
        .map(|div| {
            div.inner_html()
                .lines()
                .map(|line| line.trim().replace("<br>", "\n"))
                .collect::<String>()
        })
        .ok_or("Bridge lines not found")?;

    Ok(bridge_lines)
}
