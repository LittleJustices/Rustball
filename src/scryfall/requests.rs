use super::{
    card::{
        Card,
        ErrorObject,
    },
    req_token::ReqToken,
    scryfall_errors::ScryfallError,
    booru_post::BooruPost,
};
use reqwest::Client;

#[allow(dead_code)]
pub async fn get_scryfall_text(client: &Client, request_vector: Vec<ReqToken>) -> Result<String, ScryfallError> {
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

    Ok(client.get(request_url).send().await?.text().await?)
}

pub async fn get_scryfall_json(client: &Client, request_vector: Vec<ReqToken>) -> Result<Card, ScryfallError> {
    let mut request_url = String::from("https://api.scryfall.com/cards/named?");
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

    match client.get(&request_url).send().await?.json::<Card>().await {
        Ok(card) => {
            if let Some(true) = card.content_warning {
                return Err(ScryfallError::ContentWarning);
            }
            Ok(card)
        },
        Err(_) => {
            let why = client.get(&request_url).send().await?.json::<ErrorObject>().await?;
            Err(ScryfallError::ApiError(why))
        }
    }
}

#[allow(dead_code)]
pub async fn get_scryfall_random_text(client: &Client) -> Result<String, ScryfallError> {
    let request_url = "https://api.scryfall.com/cards/random?format=text";

    Ok(client.get(request_url).send().await?.text().await?)
}

pub async fn get_scryfall_random_json(client: &Client) -> Result<Card, ScryfallError> {
    let request_url = "https://api.scryfall.com/cards/random";

    match client.get(request_url).send().await?.json::<Card>().await {
        Ok(card) => Ok(card),
        Err(_) => {
            let why = client.get(request_url).send().await?.json::<ErrorObject>().await?;
            Err(ScryfallError::ApiError(why))
        }
    }
}

pub async fn get_booru_random_json(client: &Client, search_tags: &[&str]) -> Result<BooruPost, ScryfallError> {
    let request_url = format!(
        "https://danbooru.donmai.us/posts/random.json?tags={}",
        search_tags.join("+")
    );

    Ok(client.get(request_url).send().await?.json::<BooruPost>().await?)
}
