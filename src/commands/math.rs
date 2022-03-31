use crate::math::calculator;

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

#[command]
async fn calc(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let math = calculator::evaluate("".to_owned());
    msg.channel_id.say(&ctx.http, math).await?;

    Ok(())
}