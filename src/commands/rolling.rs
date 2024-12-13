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

/// Specialty command for CofD/nWoD rolls! ｶﾀｶﾀｶﾀ(((;ﾟ;Д;ﾟ;)))ｶﾀｶﾀｶﾀ
/// It was written with CofD/nWoD 2e in mind, but should be backwards compatible with 1e.
/// 
/// ### Basic Usage
/// For basic usage, just supply a number, e.g.:
/// > ~cofd 9
/// This will roll 9 dice with the usual rules for success counting (sux on 8+, explode 10s).
/// 
/// ### Advanced Usage
/// If you want to write out a bunch of bonuses and penalties, the command will also accept \$
/// arbitrary mathematical expressions, e.g.:
/// > ~cofd 5+3
/// Mathematical operations will always be resolved first, even if you don't put them in parentheses.
/// 
/// To apply the 9-again or 8-again rules, use `a` followed by the number you want:
/// > ~cofd 5a9
/// 
/// To roll without 10-again, add the `m` flag:
/// > ~cofd 10m
/// 
/// To apply the rote quality, add the `r` flag:
/// > ~cofd 8r
/// 
/// You can also add successes after rolling (or do whatever other math you want), but you'll \$
/// have to distinguish post-roll math from pre-roll math by putting it after a semicolon:
/// > ~cofd 5+3; +1
/// The above would roll 5+3=8 dice, count successes as normal, then add 1 automatic success.
/// 
/// ### Weird Stuff
/// Finally, if you need to do something complicated, you can provide arbitrary dice \$
/// operations to be resolved **after dice are rolled, but before successes are counted**, \$
/// by supplying them inside {curly braces} before the semicolon if you're using one, e.g.:
/// > ~cofd 6{rr1}
/// The above will roll 6 dice and reroll 1s until 1s fail to appear.
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

/// Specialty command for Exalted rolls! (oﾟ□ﾟ)o≪≪≪ﾜｱｧｧｧｧｧｧｯ!!
/// 
/// ### Basic Usage Instruction
/// For basic usage, just supply a number, e.g.:
/// > ~exroll 9
/// This will roll 9 dice with the usual rules for success counting (sux on 7+, 2 sux on 10).
/// 
/// ### Advanced Player's Precepts
/// If you want to write out a bunch of bonuses and penalties, the command will also accept \$
/// arbitrary mathematical expressions, e.g.:
/// > ~exroll 5+3
/// Mathematical operations will always be resolved first, even if you don't put them in parentheses.
/// 
/// To roll without counting 10s twice (say, for decisive damage), add the `m` flag:
/// > ~exroll 10m
/// 
/// For double 7s, 8s, or 9s, use `d` followed by the number you want:
/// > ~exroll 5d9
/// (This looks like a normal command to roll 5 9-sided dice, but won't be treated as one.)
/// 
/// You can also add successes after rolling (or do whatever other math you want), but you'll \$
/// have to distinguish post-roll math from pre-roll math by putting it after a semicolon:
/// > ~exroll 5+3; +1
/// The above would roll 5+3=8 dice, count successes as normal, then add 1 automatic success.
/// 
/// Old legends speak of an `s` option that can set the target number, e.g.:
/// > ~exroll 8s5
/// This would have counted results of 5 or higher as 1 success, with the usual 2 for a 10.
/// But if such a thing ever existed, it was forgotten when the Mask shattered. (｡•̀ᴗ-)
/// 
/// ### Master Gambler's Technique
/// Finally, if you need to do something complicated, you can provide arbitrary dice \$
/// operations to be resolved **after dice are rolled, but before successes are counted**, \$
/// by supplying them inside {curly braces} before the semicolon if you're using one, e.g.:
/// > ~exroll 6{rr1}
/// The above will roll 6 dice and reroll 1s until 1s fail to appear.
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
