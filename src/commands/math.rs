use serenity::{
    framework::{
        standard::{
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
async fn calc(ctx: &Context, msg: &Message) -> CommandResult {
    let math = format!("https://xkcd.com/2034/");
    msg.channel_id.say(&ctx.http, math).await?;

    Ok(())
}