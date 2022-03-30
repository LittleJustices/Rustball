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
async fn math(ctx: &Context, msg: &Message) -> CommandResult {
    let no_math = format!("{} I don't know math yet!", msg.author);
    msg.channel_id.say(&ctx.http, no_math).await?;

    Ok(())
}