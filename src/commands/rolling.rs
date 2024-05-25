use serenity::{
    framework::standard::{
            Args,
            CommandResult,
            macros::command,
        },
    model::{
        channel::Message, 
        id::{
            GuildId, 
            ChannelId
        }
    },
    prelude::*,
};
use std::collections::HashMap;
use crate::{
    dice::{
        command_translations,
        tray::Tray, roll::Roll,
    }, 
    sixball_errors::SixballError
};

pub type TrayMap = HashMap<TrayId, Tray>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TrayId {
    Private(ChannelId),
    Guild(Option<GuildId>),
}

#[command]
#[description="The basic roll command!\n
Use standard die roll notation of the form `XdY`. I can roll up to 255 dice with up to 255 sides at once!\n
I can also do math with dice! (　-\\`ω-)✧ﾄﾞﾔｯ Just plug your dice into any math expression, e.g. `1d20+5`. If the `calc` command can handle it, so can the `roll` command!\n
Additional dice operations are added as the Boss thinks of them and has time. Please wait warmly!\n
Want to roll the same kind of roll multiple times in one go? Put the number in front of the roll separated by a hash (#)! Like this: `6#3d6`.
To append a comment to your roll, put it after the roll separated by a colon (:) like this: `1d20+5: Comment here`.\n
For full documentation on roll syntax, check out the [readme](https://github.com/LittleJustices/Rustball/blob/master/ROLLSYNTAX.md)!
Or look over the [quick reference](https://github.com/LittleJustices/Rustball/blob/master/ROLLSYNTAX.md#quick-reference) if you just need a refresher. (* ˘꒳˘)⁾⁾ｳﾝｳﾝ"]
#[aliases("r", "rill", "rol", "rll")]
async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (repeat, roll_command, roll_comment) = match extract_arguments(ctx, args).await {
        Ok(arguments) => arguments,
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("{}", why)).await?;
            return Ok(());
        },
    };
    let in_command = &roll_command;

    let response = match new_roll_output(&ctx, &msg, repeat, &in_command, &roll_command, &roll_comment, true).await {
        Ok(res) => format!("{}", res),
        Err(why) => format!("{}", why),
    };
    msg.reply_ping(&ctx.http, response).await?;

    Ok(())
}

#[command]
#[description="Under construction. Please wait warmly!"]
async fn reroll(ctx: &Context, msg: &Message) -> CommandResult {
    // Get context data with write permission to manipulate the tray
    let mut tray_data = ctx.data.write().await;
    let mut tray_map = tray_data
        .get_mut::<crate::TrayKey>()
        .expect("Failed to retrieve tray map!")
        .lock().await;

    if let Some(tray) = tray_map.get_mut(&make_tray_id(msg)) {
        match tray.reroll_latest() {
            Ok(reroll) => {
                let message = format!("Reroll: {}", reroll);
                msg.reply_ping(&ctx.http, message).await?;
            },
            Err(why) => {
                let roll_error = format!("{}", why);
                msg.reply_ping(&ctx.http, roll_error).await?;
            }
        }
    } else {
        msg.reply_ping(&ctx.http, "There's nothing to reroll!").await?;
    }

    Ok(())
}

#[command]
#[description="Under construction. Please wait warmly!"]
#[aliases("mod", "revise", "rev", "amend", "am")]
async fn modify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (_, revision_command, revision_comment) = match extract_arguments(ctx, args).await {
        Ok(arguments) => {
            arguments
        },
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("{}", why)).await?;
            return Ok(());
        },
    };
    // Get context data with write permission to manipulate the tray
    let mut tray_data = ctx.data.write().await;
    let mut tray_map = tray_data
        .get_mut::<crate::TrayKey>()
        .expect("Failed to retrieve tray map!")
        .lock().await;

    if let Some(tray) = tray_map.get_mut(&make_tray_id(msg)) {
        let reviser = msg.author_nick(&ctx).await.unwrap_or(msg.author.name.clone());
        match tray.modify_latest(&revision_command, &revision_comment, &reviser) {
            Ok(revised_roll) => {
                let annotation = match revised_roll.comment().trim() {
                    "" => "".to_string(),
                    other => format!(" ({})", other),
                };
                let message = format!(
                    "Amended roll: `{}`{}\n{}",
                    revised_roll.command(),
                    annotation,
                    roll_format_discord(revised_roll, true, ""),
                );
                msg.reply_ping(&ctx.http, message).await?;
            },
            Err(why) => {
                let roll_error = format!("{}", why);
                msg.reply_ping(&ctx.http, roll_error).await?;
            }
        }
    } else {
        msg.reply_ping(&ctx.http, "There's nothing to modify!").await?;
    }

    Ok(())
}

#[command]
#[aliases("tray")]
async fn pastrolls(ctx: &Context, msg: &Message) -> CommandResult {
    let tray_data = ctx.data.read().await;
    let tray_map = tray_data
        .get::<crate::TrayKey>()
        .expect("Failed to retrieve tray map!")
        .lock().await;

    if let Some(tray) = tray_map.get(&make_tray_id(msg)) {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Currently Stored Rolls");
                for (i, roll) in tray.rolls().iter().enumerate() {
                    // Build the title here containing i, person who rolled, and maybe timestamp?
                    let title = format!("{}: By {} at {}", i, roll.roller(), roll.timestamp().format("%y/%m/%d %H:%M:%S"));
                    let text = format!("{}", roll);
                    e.field(title, text, false);
                }
                e
            });
            m
        }).await?;
    } else {
        msg.reply_ping(&ctx.http, "I haven't even set up a tray for this server yet!").await?;
    }

    Ok(())
}

#[command]
#[description="Under construction. Please wait warmly!"]
async fn verbose(ctx: &Context, msg: &Message) -> CommandResult {
    let tray_data = ctx.data.read().await;
    let tray_map = tray_data
        .get::<crate::TrayKey>()
        .expect("Failed to retrieve tray map!")
        .lock().await;

    if let Some(tray) = tray_map.get(&make_tray_id(msg)) {
        let latest_roll = match tray.get_newest_roll() {
            Ok(roll) => roll,
            Err(why) => {
                msg.reply_ping(&ctx.http, format!("{}", why)).await?;
                return Ok(());
            }
        };

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                let annotation = match latest_roll.comment().trim() {
                    "" => "".into(),
                    other => format!(" ({})", other),
                };
                let title = format!("{}{}", latest_roll.command(), annotation);
                e.title(title);
                for operation in latest_roll.operations() {
                    let name = operation.description();
                    let value = operation.verbose();
                    e.field(name, value, false);
                }
                e.field("Total", latest_roll.result(), false);
                e
            });
            m
        }).await?;
    } else {
        msg.reply_ping(&ctx.http, "I haven't even set up a tray for this server yet!").await?;
    }

    Ok(())
}

#[command]
#[aliases("cod", "nwod")]
async fn cofd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (repeat, in_command, roll_comment) = match extract_arguments(ctx, args).await {
        Ok(arguments) => arguments,
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("{}", why)).await?;
            return Ok(());
        },
    };

    let response = match command_translations::cofd(&in_command) {
        Ok(roll_command) => match new_roll_output(&ctx, &msg, repeat, &in_command, &roll_command, &roll_comment, true).await {
            Ok(res) => format!("{}", res),
            Err(why) => format!("{}", why),
        },
        Err(why) => format!("{}", SixballError::RollError(why)),
    };
    msg.reply_ping(&ctx.http, response).await?;

    Ok(())
}

#[command]
#[aliases("l5roll", "rings")]
async fn l5r(ctx: &Context, msg: &Message) -> CommandResult {
    let roll = format!("{} I'm not weeb enough for that yet!", msg.author);
    msg.channel_id.say(&ctx.http, roll).await?;

    Ok(())
}

#[command]
#[aliases("sroll")]
async fn sr(ctx: &Context, msg: &Message) -> CommandResult {
    let roll = format!("{} I'm not shady enough for that yet!", msg.author);
    msg.channel_id.say(&ctx.http, roll).await?;

    Ok(())
}

#[command]
#[aliases("ex")]
async fn exroll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (repeat, in_command, roll_comment) = match extract_arguments(ctx, args).await {
        Ok(arguments) => arguments,
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("{}", why)).await?;
            return Ok(());
        },
    };

    let response = match command_translations::exalted(&in_command) {
        Ok(roll_command) => match new_roll_output(&ctx, &msg, repeat, &in_command, &roll_command, &roll_comment, true).await {
            Ok(res) => format!("{}", res),
            Err(why) => format!("{}", why),
        },
        Err(why) => format!("{}", SixballError::RollError(why)),
    };
    msg.reply_ping(&ctx.http, response).await?;

    Ok(())
}

#[command]
#[description="This command is for rolling Genesys narrative dice! Just tell me how many of which kinds to roll and I'll handle the rest under the hood! Σd(≧▽≦*)
Format the command like this: `[kind of die][number of dice]`. The different kinds of dice can be in any order, and you can put as many spaces as you want between them if it helps you organize the roll.
For example: `~genroll a2 p2 d3` -> 2 Ability dice, 2 Proficiency dice, 3 Difficulty dice
You can even have the same kind of die multiple times if you want, for example to keep track of different sources of dice! I'll add them all up for you.\n
The dice codes are:
\t• b: Boost (Blue d6)
\t• a: Ability (Green d8)
\t• p: Proficiency (Yellow d12)
\t• s: Setback (Black d6)
\t• d: Difficulty (Purple d8)
\t• c: Challenge (Red d12)\n
Documentation can be found [here](https://github.com/LittleJustices/Rustball/blob/master/ROLLSYNTAX.md#genroll-genesys-narrative-dice)!"]
#[aliases("gr", "genesys", "groll")]
async fn genroll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (repeat, in_command, roll_comment) = match extract_arguments(ctx, args).await {
        Ok(arguments) => arguments,
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("{}", why)).await?;
            return Ok(());
        },
    };

    let response = match command_translations::genesys(&in_command) {
        Ok(roll_command) => match new_roll_output(&ctx, &msg, repeat, &in_command, &roll_command, &roll_comment, false).await {
            Ok(res) => format!("{}", res),
            Err(why) => format!("{}", why),
        },
        Err(why) => format!("{}", SixballError::RollError(why)),
    };
    msg.reply_ping(&ctx.http, response).await?;

    Ok(())
}

#[command]
#[description="This command is for rolling Story Shaper System dice! Help string to be added."]
#[aliases("s3r")]
async fn s3roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (repeat, in_command, roll_comment) = match extract_arguments(ctx, args).await {
        Ok(arguments) => arguments,
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("{}", why)).await?;
            return Ok(());
        },
    };

    let response = match command_translations::story_shaper(&in_command) {
        Ok(roll_command) => match new_roll_output(&ctx, &msg, repeat, &roll_command, &roll_command, &roll_comment, true).await {
            Ok(res) => format!("{}", res),
            Err(why) => format!("{}", why),
        },
        Err(why) => format!("{}", SixballError::RollError(why)),
    };
    msg.reply_ping(&ctx.http, response).await?;

    Ok(())
}

async fn extract_arguments(ctx: &Context, args: Args) -> Result<(u8, String, String), SixballError> {
    // Get config data as read-only to look up the comment separator. It is then freed up when we move out of the function
    let config_data = ctx.data.read().await;
    let cfg = config_data.get::<crate::ConfigKey>().expect("Failed to retrieve config!");

    let (full_command, comment) = match args.message().split_once(&cfg.comment_separator) {
        Some((command, comment)) => (command.to_lowercase(), comment.into()),
        None => (args.message().to_lowercase(), "".into())
    };
    let (repeat, command) = match full_command.split_once(&cfg.repeater_separator) {
        Some((number, command)) => (Tray::repeat_rolls(number)?, command.into()),
        None => (1, full_command)
    };

    Ok((repeat, command, comment))
}

async fn new_roll_output(
    ctx: &Context,
    msg: &Message,
    repeat: u8,
    in_command: &str,
    roll_command: &str,
    roll_comment: &str,
    breakdown: bool,
) -> Result<String, SixballError> {
    // Get config data with write permission to manipulate the tray
    let mut tray_data = ctx.data.write().await;
    let mut tray_map = tray_data
        .get_mut::<crate::TrayKey>()
        .expect("Failed to retrieve tray map!")
        .lock().await;
    
    let tray = match tray_map.get_mut(&make_tray_id(msg)) {
        Some(extant_tray) => extant_tray,
        None => {
            let new_tray = Tray::new();
            tray_map.insert(make_tray_id(msg), new_tray);
            tray_map.get_mut(&make_tray_id(msg)).expect("Failed to get tray we literally just inserted!")
        }
    };
    let roller = msg.author_nick(&ctx).await.unwrap_or(msg.author.name.clone());
    
    let annotation = match roll_comment.trim() {
        "" => "".to_owned(),
        other => format!(" ({})", other)
    };

    let mut output = format!("`{}`{}:", in_command.trim(), annotation);

    for i in 1..=repeat {
        let roll = tray.add_roll_from_command(roll_command, roll_comment, &roller)?;
        let numbering = match repeat {
            1 => "\n".to_string(),
            _ => format!("\n{}: ", i)
        };
        let next = roll_format_discord(roll, breakdown, &numbering);
        output.push_str(&next);
    }

    Ok(output)
}

fn roll_format_discord(roll: &Roll, breakdown: bool, prepend: &str) -> String {
    match breakdown {
        true => format!("{}**{}** ({})", prepend, roll.result(), roll),
        false => format!("{}**{}** (use `verbose` or `tray` commands for details)", prepend, roll.result()),
    }
}

fn make_tray_id(msg: &Message) -> TrayId {
    let tray_id;
    if msg.is_private() {
        tray_id = TrayId::Private(msg.channel_id);
    } else {
        tray_id = TrayId::Guild(msg.guild_id);
    }

    tray_id
}
