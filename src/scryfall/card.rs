use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Card {
    pub all_parts: Option<Vec<RelatedCard>>,
    pub card_faces: Option<Vec<CardFace>>,
    pub content_warning: Option<bool>,
    pub hand_modifier: Option<String>,
    pub image_uris: Option<ImageUris>,
    pub layout: String,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub rulings_uri: String,
    pub scryfall_uri: String,
    pub toughness: Option<String>,
    pub type_line: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CardFace {
    pub image_uris: Option<ImageUris>,
    pub layout: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub type_line: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RelatedCard {
    pub name: String,
    pub uri: String,
}

impl Card {
    pub fn build_description(&self) -> String {
        let mut description = String::new();

        match self.layout.as_str() {
            "normal" | "meld" | "leveler" | "class" | "saga" | "planar" | "scheme" | "vanguard" | "token" | "emblem" | "augment" | "host" => {
                description.push_str(&self.type_line);
                description.push('\n');
                if let Some(text) = &self.oracle_text {
                    description.push_str(text);
                }
                description.push('\n');
                if let (Some(power), Some(toughness)) = (&self.power, &self.toughness) {
                    description.push_str(&format!("{}/{}", power, toughness));
                }
                if let Some(loyalty) = &self.loyalty {
                    description.push_str(&format!("Loyalty: {}", loyalty));
                }
                if let Some(hand_size) = &self.hand_modifier {
                    description.push_str(&format!("Hand Size: {}", hand_size));
                }
                if let Some(starting_life) = &self.life_modifier {
                    description.push('\n');
                    description.push_str(&format!("Starting Life: {}", starting_life));
                }
            },
            "split" | "flip" | "transform" | "modal_dfc" | "adventure" | "double_faced_token" | "art_series" | "reversible_card" => {
                if let Some(faces) = &self.card_faces {
                    description.push_str(&faces[0].build_description());
                    description.push('\n');
                    description.push_str("---------");
                    description.push('\n');
                    description.push_str(&faces[1].build_description());
                }
            },
            _ => description.push_str(&format!(" （　ＴДＴ） Description for this layout ({}) not implemented yet!", &self.layout))
        }

        description
    }

    pub fn get_image(&self) -> String {
        let mut uri = String::new();
        if let Some(image_uris) = &self.image_uris {
            uri.push_str(&image_uris.normal);
        } else {
            if let Some(faces) = &self.card_faces {
                if let Some(uris) = &faces[0].image_uris {
                    uri.push_str(&uris.normal);
                }
            }
        }

        uri
    }

    pub fn get_name(&self) -> String {
        let mut name = String::from(&self.name);
        if let Some(mc) = &self.mana_cost {
            name.push('\t');
            name.push_str(mc);
        } else {
            match self.layout.as_str() {
                "transform" => {
                    if let Some(faces) = &self.card_faces {
                        let front_side = &faces[0];
                        if let Some(mana_cost) = &front_side.mana_cost {
                            name.push('\t');
                            name.push_str(mana_cost);
                        }
                    }
                },
                "modal_dfc" => {
                    if let Some(faces) = &self.card_faces {
                        name.push('\t');
                        let mut faces_iter = faces.iter();
                        if let Some(face) = faces_iter.next() {
                            if let Some(face_cost) = &face.mana_cost {
                                name.push_str(face_cost);
                            }
                        }
                        for face in faces_iter {
                            if let Some(face_cost) = &face.mana_cost {
                                name.push_str(" // ");
                                name.push_str(face_cost);
                            }
                        }
                    }
                },
                _ => {}
            }
        }

        name
    }

    pub fn get_uri(&self) -> String {
        let uri = String::from(&self.scryfall_uri);
        uri
    }
}

impl CardFace {
    pub fn build_description(&self) -> String {
        let mut description = String::new();

        description.push_str(&self.name);
        if let Some(mc) = &self.mana_cost {
            description.push('\t');
            description.push_str(mc);
        }
        description.push('\n');
        description.push_str(&self.type_line);
        description.push('\n');
        if let Some(text) = &self.oracle_text {
            description.push_str(text);
        }
        description.push('\n');
        if let (Some(power), Some(toughness)) = (&self.power, &self.toughness) {
            description.push_str(&format!("{}/{}", power, toughness));
        }
        if let Some(loyalty) = &self.loyalty {
            description.push_str(&format!("Loyalty: {}", loyalty));
        }

        description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_card_request() {
        let card_data = r#"{"object":"card","id":"02d6d693-f1f3-4317-bcc0-c21fa8490d38","oracle_id":"594f6881-c059-46f8-aa4e-7151d502de73","multiverse_ids":[398434,398435],"mtgo_id":57880,"mtgo_foil_id":57881,"tcgplayer_id":100191,"cardmarket_id":283370,"name":"Jace, Vryn's Prodigy // Jace, Telepath Unbound","lang":"en","released_at":"2015-07-17","uri":"https://api.scryfall.com/cards/02d6d693-f1f3-4317-bcc0-c21fa8490d38","scryfall_uri":"https://scryfall.com/card/ori/60/jace-vryns-prodigy-jace-telepath-unbound?utm_source=api","layout":"transform","highres_image":true,"image_status":"highres_scan","cmc":2.0,"type_line":"Legendary Creature — Human Wizard // Legendary Planeswalker — Jace","color_identity":["U"],"keywords":["Transform","Mill"],"card_faces":[{"object":"card_face","name":"Jace, Vryn's Prodigy","mana_cost":"{1}{U}","type_line":"Legendary Creature — Human Wizard","oracle_text":"{T}: Draw a card, then discard a card. If there are five or more cards in your graveyard, exile Jace, Vryn's Prodigy, then return him to the battlefield transformed under his owner's control.","colors":["U"],"power":"0","toughness":"2","flavor_text":"\"People's thoughts just come to me. Sometimes I don't know if it's them or me thinking.\"","artist":"Jaime Jones","artist_id":"92f6c2c1-fa57-4b52-99c4-0fd866c13dc9","illustration_id":"ea8de167-ee93-4282-95b4-d82291dbfe1f","image_uris":{"small":"https://c1.scryfall.com/file/scryfall-cards/small/front/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","normal":"https://c1.scryfall.com/file/scryfall-cards/normal/front/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","large":"https://c1.scryfall.com/file/scryfall-cards/large/front/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","png":"https://c1.scryfall.com/file/scryfall-cards/png/front/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.png?1651492800","art_crop":"https://c1.scryfall.com/file/scryfall-cards/art_crop/front/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","border_crop":"https://c1.scryfall.com/file/scryfall-cards/border_crop/front/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800"}},{"object":"card_face","name":"Jace, Telepath Unbound","flavor_name":"","mana_cost":"","type_line":"Legendary Planeswalker — Jace","oracle_text":"+1: Up to one target creature gets -2/-0 until your next turn.\n−3: You may cast target instant or sorcery card from your graveyard this turn. If that spell would be put into your graveyard, exile it instead.\n−9: You get an emblem with \"Whenever you cast a spell, target opponent mills five cards.\"","colors":["U"],"color_indicator":["U"],"loyalty":"5","artist":"Jaime Jones","artist_id":"92f6c2c1-fa57-4b52-99c4-0fd866c13dc9","illustration_id":"8ce7af86-2a0b-426b-8f7b-a49d6c956141","image_uris":{"small":"https://c1.scryfall.com/file/scryfall-cards/small/back/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","normal":"https://c1.scryfall.com/file/scryfall-cards/normal/back/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","large":"https://c1.scryfall.com/file/scryfall-cards/large/back/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","png":"https://c1.scryfall.com/file/scryfall-cards/png/back/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.png?1651492800","art_crop":"https://c1.scryfall.com/file/scryfall-cards/art_crop/back/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800","border_crop":"https://c1.scryfall.com/file/scryfall-cards/border_crop/back/0/2/02d6d693-f1f3-4317-bcc0-c21fa8490d38.jpg?1651492800"}}],"all_parts":[{"object":"related_card","id":"02d6d693-f1f3-4317-bcc0-c21fa8490d38","component":"combo_piece","name":"Jace, Vryn's Prodigy // Jace, Telepath Unbound","type_line":"Legendary Creature — Human Wizard // Legendary Planeswalker — Jace","uri":"https://api.scryfall.com/cards/02d6d693-f1f3-4317-bcc0-c21fa8490d38"},{"object":"related_card","id":"458e37b1-a849-41ae-b63c-3e09ffd814e4","component":"combo_piece","name":"Jace, Telepath Unbound Emblem","type_line":"Emblem — Jace","uri":"https://api.scryfall.com/cards/458e37b1-a849-41ae-b63c-3e09ffd814e4"}],"legalities":{"standard":"not_legal","future":"not_legal","historic":"not_legal","gladiator":"not_legal","pioneer":"legal","explorer":"not_legal","modern":"legal","legacy":"legal","pauper":"not_legal","vintage":"legal","penny":"not_legal","commander":"legal","brawl":"not_legal","historicbrawl":"not_legal","alchemy":"not_legal","paupercommander":"not_legal","duel":"legal","oldschool":"not_legal","premodern":"not_legal"},"games":["paper","mtgo"],"reserved":false,"foil":true,"nonfoil":true,"finishes":["nonfoil","foil"],"oversized":false,"promo":false,"reprint":false,"variation":false,"set_id":"0eeb9a9a-20ac-404d-b55f-aeb7a43a7f62","set":"ori","set_name":"Magic Origins","set_type":"core","set_uri":"https://api.scryfall.com/sets/0eeb9a9a-20ac-404d-b55f-aeb7a43a7f62","set_search_uri":"https://api.scryfall.com/cards/search?order=set\u0026q=e%3Aori\u0026unique=prints","scryfall_set_uri":"https://scryfall.com/sets/ori?utm_source=api","rulings_uri":"https://api.scryfall.com/cards/02d6d693-f1f3-4317-bcc0-c21fa8490d38/rulings","prints_search_uri":"https://api.scryfall.com/cards/search?order=released\u0026q=oracleid%3A594f6881-c059-46f8-aa4e-7151d502de73\u0026unique=prints","collector_number":"60","digital":false,"rarity":"mythic","artist":"Jaime Jones","artist_ids":["92f6c2c1-fa57-4b52-99c4-0fd866c13dc9"],"border_color":"black","frame":"2015","frame_effects":["originpwdfc"],"security_stamp":"oval","full_art":false,"textless":false,"booster":true,"story_spotlight":false,"edhrec_rank":2697,"prices":{"usd":"12.63","usd_foil":"30.72","usd_etched":null,"eur":"9.90","eur_foil":"25.45","tix":"1.73"},"related_uris":{"gatherer":"https://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=398434","tcgplayer_infinite_articles":"https://infinite.tcgplayer.com/search?contentMode=article\u0026game=magic\u0026partner=scryfall\u0026q=Jace%2C+Vryn%27s+Prodigy+%2F%2F+Jace%2C+Telepath+Unbound\u0026utm_campaign=affiliate\u0026utm_medium=api\u0026utm_source=scryfall","tcgplayer_infinite_decks":"https://infinite.tcgplayer.com/search?contentMode=deck\u0026game=magic\u0026partner=scryfall\u0026q=Jace%2C+Vryn%27s+Prodigy+%2F%2F+Jace%2C+Telepath+Unbound\u0026utm_campaign=affiliate\u0026utm_medium=api\u0026utm_source=scryfall","edhrec":"https://edhrec.com/route/?cc=Jace%2C+Vryn%27s+Prodigy"},"purchase_uris":{"tcgplayer":"https://www.tcgplayer.com/product/100191?page=1\u0026utm_campaign=affiliate\u0026utm_medium=api\u0026utm_source=scryfall","cardmarket":"https://www.cardmarket.com/en/Magic/Products/Search?referrer=scryfall\u0026searchString=Jace%2C+Vryn%27s+Prodigy\u0026utm_campaign=card_prices\u0026utm_medium=text\u0026utm_source=scryfall","cardhoarder":"https://www.cardhoarder.com/cards/57880?affiliate_id=scryfall\u0026ref=card-profile\u0026utm_campaign=affiliate\u0026utm_medium=card\u0026utm_source=scryfall"}}"#;
        let card_from_str = serde_json::from_str::<Card>(card_data).unwrap();

        println!("{:?}", &card_from_str);
        assert_eq!(card_from_str.get_uri(), "https://scryfall.com/card/ori/60/jace-vryns-prodigy-jace-telepath-unbound?utm_source=api".to_owned());
        assert_eq!(card_from_str.build_description(), "Legendary Creature — Human Wizard // Legendary Planeswalker — Jace\n".to_owned());
        assert_eq!(card_from_str.get_name(), "Jace, Vryn's Prodigy // Jace, Telepath Unbound\t{1}{U}".to_owned());
    }
}
