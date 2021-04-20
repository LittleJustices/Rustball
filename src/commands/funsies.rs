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
async fn squid(ctx: &Context, msg: &Message) -> CommandResult {
    let squid = format!("{} ＜コ:彡", msg.author);
    msg.channel_id.say(&ctx.http, squid).await?;

    Ok(())
}

#[command]
#[aliases("shadowruns", "fixalot", "rules")]
async fn shadow(ctx: &Context, msg: &Message) -> CommandResult {
    let rules = String::from(
        "Say it with me now:
\t\t\t\tWatch your back.
\t\t\t\tShoot straight.
\t\t\t\tConserve ammo.
\t\t\t\tAnd never, ever deal with a dragon!
(ﾉ≧∀≦)ﾉ"
    );
    msg.channel_id.say(&ctx.http, rules).await?;

    Ok(())
}

#[command]
async fn unyu(ctx: &Context, msg: &Message) -> CommandResult {
    let unyu = format!("{} うにゅうー！", msg.author);
    msg.channel_id.say(&ctx.http, unyu).await?;

    Ok(())
}

#[command]
async fn atom(ctx: &Context, msg: &Message) -> CommandResult {
    let atom = String::from("(●o≧д≦) Up and atom! ☢ 😤 ☢");
    msg.channel_id.say(&ctx.http, atom).await?;

    Ok(())
}

#[command]
#[aliases("sway", "shimarin", "shima")]
async fn yuru(ctx: &Context, msg: &Message) -> CommandResult {
    let sway = String::from("https://tenor.com/view/yuru-camp-shima-rin-gif-19870064");
    msg.channel_id.say(&ctx.http, sway).await?;

    Ok(())
}