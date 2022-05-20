use super::req_token::ReqToken;
use reqwest;

pub async fn get_scryfall_text(request_vector: Vec<ReqToken>) -> Result<String, reqwest::Error> {
    let mut request_url = String::from("https://api.scryfall.com/cards/named?format=text&");
    for token in request_vector.iter() {
        match token {
            ReqToken::Fuzzy(cardname) => {
                request_url.push_str("fuzzy=");
                request_url.push_str(&cardname);
            },
            _ => {
                request_url.push_str("fuzzy=");
                request_url.push_str("one with nothing");
            }
        }
    }

    reqwest::get(request_url).await?.text().await
}

pub async fn get_scryfall_random_text() -> Result<String, reqwest::Error> {
    let request_url = "https://api.scryfall.com/cards/random";

    reqwest::get(request_url).await?.text().await
}
