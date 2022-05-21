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

    if args.current() == Some("random") {
        let output = match requests::get_scryfall_random_text(client).await {
            Ok(text) => text,
            Err(why) => format!("{}", why)
        };
    
        msg.reply_ping(&ctx.http, format!("```{}```", output)).await?;

        return Ok(());
    }

    let request_vector = request_from_args(args);
    let output = match requests::get_scryfall_text(client, request_vector).await {
        Ok(text) => text,
        Err(why) => format!("{}", why)
    };

    msg.reply_ping(&ctx.http, format!("```{}```", output)).await?;
    
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
