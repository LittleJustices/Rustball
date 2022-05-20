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
    if args.current() == Some("random") {
        let output = match requests::get_scryfall_random_text().await {
            Ok(text) => text,
            Err(why) => format!("{}", why)
        };
    
        msg.reply_ping(&ctx.http, format!("```{}```", output)).await?;

        return Ok(());
    }

    let request_vector = request_from_args(args);
    let output = match requests::get_scryfall_text(request_vector).await {
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
