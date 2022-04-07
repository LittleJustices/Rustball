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
    let result = match calculator::evaluate(infix_expression) {
        Ok(res) => res,
        Err(why) => format!("{} ☢ I don't know how to calculate that! ☢ {}", msg.author, why)
    };
    msg.channel_id.say(&ctx.http, format!("{} {}", msg.author, result)).await?;

    Ok(())
}