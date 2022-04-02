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
async fn calc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let infix_expression = args.message();
    let math = calculator::evaluate(infix_expression);
    msg.channel_id.say(&ctx.http, format!("{} = {}", infix_expression, math)).await?;

    Ok(())
}