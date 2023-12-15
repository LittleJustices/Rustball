use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct BooruPost {
    pub id: u32,
}

impl BooruPost {
    pub fn post_url(&self) -> String {
        format!("https://danbooru.donmai.us/posts/{}", self.id)
    }
}
