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
        Ok(res) => format!("`{}` = {}", infix_expression, res),
        Err(why) => format!("{}", why)
    };
    msg.reply_ping(&ctx.http, result).await?;

    Ok(())
}

#[command]
#[description="Lets me execute an arbitrary expression, even really complicated ones that ~calc can't handle, including actual code! Just pass the code you want me to run as an argument in a code block under the command and whatever happens, happens!\n
Careful with this one! (*＞ωб)ﾊﾞﾁｺｰﾝ"]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.reply_ping(&ctx.http, format!("Executing... <(\\`∨´+.)>ﾄﾞﾔｯ! I am a genius! {}", args.message())).await?;
    msg.channel_id.say(&ctx.http, "Executing...").await?;
    msg.channel_id.say(&ctx.http, "Executing...").await?;
    msg.channel_id.say(&ctx.http, "Executing...").await?;
    msg.channel_id.say(&ctx.http, "☢ ...Oh no! (゜Д゜) ☢").await?;
    msg.channel_id.say(&ctx.http, "https://tenor.com/view/bomba-nuclear-bomb-explode-explosion-gif-16571885").await?;

    Ok(())
}
