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
use crate::funsies::funsies;

#[command]
async fn squid(ctx: &Context, msg: &Message) -> CommandResult {
    let squid = format!("{} {}", msg.author, funsies::squid());
    msg.channel_id.say(&ctx.http, squid).await?;

    Ok(())
}

#[command]
#[aliases("shadowruns", "fixalot", "rules")]
async fn shadow(ctx: &Context, msg: &Message) -> CommandResult {
    let rules = funsies::rules();
    msg.channel_id.say(&ctx.http, rules).await?;

    Ok(())
}

#[command]
async fn unyu(ctx: &Context, msg: &Message) -> CommandResult {
    let unyu = format!("{} {}", msg.author, funsies::unyu());
    msg.channel_id.say(&ctx.http, unyu).await?;

    Ok(())
}

#[command]
async fn atom(ctx: &Context, msg: &Message) -> CommandResult {
    let atom = funsies::atom();
    msg.channel_id.say(&ctx.http, atom).await?;

    Ok(())
}

#[command]
#[aliases("sway", "shimarin", "shima")]
async fn yuru(ctx: &Context, msg: &Message) -> CommandResult {
    let sway = funsies::yuru();
    msg.channel_id.say(&ctx.http, sway).await?;

    Ok(())
}

#[command]
#[aliases("reiaq", "reiakyu", "brainrot", "dailydose")]
async fn them(ctx: &Context, msg: &Message) -> CommandResult {
    let dose = funsies::dailydose();
    msg.channel_id.say(&ctx.http, dose).await?;

    Ok(())
}
