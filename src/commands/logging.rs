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
async fn log(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn unlog(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn logging(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}