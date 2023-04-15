use serenity::{
    framework::{
        standard::{
            Args,
            CommandResult,
            macros::{
                command,
            },
        },
    },
    model::channel::Message,
    prelude::*,
};
use crate::scryfall::{
    req_token::ReqToken,
    requests,
};

#[command]
async fn card(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let client;

    let mut config_data = ctx.data.write().await;
    let mut client_handler = config_data
        .get_mut::<crate::ClientHandlerKey>()
        .expect("Failed to retrieve client handler!")
        .lock()
        .await;
    if client_handler.client_available() {
        client = client_handler.client();
    } else {
        msg.reply_ping(&ctx.http, "☢ Not so fast! ☢\nThis command is rate-limited (100ms cooldown)! Please wait warmly and try again in a bit ❤").await?;
        return Ok(());
    }

    let card_info;

    if args.current() == Some("random") {
        card_info = requests::get_scryfall_random_json(client).await;
    } else {
        let request_vector = request_from_args(args);
        card_info = requests::get_scryfall_json(client, request_vector).await;
    }

    match card_info {
        Ok(c) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.content(format!("<{}>", c.get_uri()));
                m.embed(|e| {
                    e.title(c.get_name());
                    e.url(c.get_uri());
                    e.thumbnail(c.get_image());
                    e.description(c.build_description());

                    if let Some(mut related_cards) = c.build_related() {
                        related_cards.truncate(1024);
                        e.field("Related Cards", related_cards, false);
                    }

                    e
                });
                m
            }).await?;
        },
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("{}", why)).await?;
        }
    }
    
    Ok(())
}

fn request_from_args(args: Args) -> Vec<ReqToken> {
    let request_vector;
    if let Some(arg_str) = args.remains() {
        request_vector = vec![ReqToken::Fuzzy(arg_str.to_owned())];
    } else {
        request_vector = vec![ReqToken::Fuzzy("one with nothing".to_owned())];
    }

    request_vector
}
