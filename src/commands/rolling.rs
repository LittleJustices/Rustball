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
use crate::dice::tray::Tray;

pub type TrayMap = HashMap<TrayId, Tray>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TrayId {
    Private(ChannelId),
    Guild(Option<GuildId>),
}

#[command]
#[description="The basic roll command! Currently under construction.\n
Use standard die roll notation of the form `XdY`. I can roll up to 255 dice with up to 255 sides at once!\n
I can also do math with dice! (　-\\`ω-)✧ﾄﾞﾔｯ Just plug your dice into any math expression, e.g. `1d20+5`. All die rolls are resolved before any math is handled, so don't try to get cute with nested die rolls or something like `(X+Y)dZ`! Other than that, if the `calc` command can handle it, so can the `roll` command!\n
Additional dice operations to be added. Please wait warmly!"]
#[aliases("r", "rill", "rol", "rll")]
async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let roll_command;
    let comment;
    // Get config data as read-only to look up the comment separator. It is then freed up at the end of the subscope
    {
        let config_data = ctx.data.read().await;
        let cfg = config_data.get::<crate::ConfigKey>().expect("Failed to retrieve config!");

        (roll_command, comment) = match args.message().split_once(&cfg.comment_separator) {
            Some(res) => res,
            None => (args.message(), "")
        };
    }

    if roll_command == "" {
        let no_args_error = "What do you want me to roll?";
        msg.reply_ping(&ctx.http, no_args_error).await?;
        return Ok(());
    }

    let verbose = false; // to be set inside the roll

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

    let result;
    let compact_breakdown;
    match tray.process_roll_command(roll_command) {
        Ok(res) => (result, compact_breakdown) = res,
        Err(why) => {
            let roll_error = format!("{}", why);
            msg.reply_ping(&ctx.http, roll_error).await?;
            return Ok(());
        }
    };

    if verbose {
        let breakdown = "VERBOSE ROLL BREAKDOWN GOES HERE";
        let message = format!("{} rolled {}: {}", msg.author, roll_command, result);
        msg.channel_id.send_message(&ctx.http, |m| {
            m.content(message);
            m.embed(|e| {
                e.title(comment);
                e.description(breakdown);
    
                e
            });
            m
        }).await?;
    } else {
        let annotation = match comment.trim() {
            "" => "".to_owned(),
            _ => format!(" ({})", comment.trim())
        };
        let message = format!("`{}`{}:\n**{}** ({})", roll_command.trim(), annotation, result, compact_breakdown);
        msg.reply_ping(&ctx.http, message).await?;
    }

    Ok(())
}

#[command]
#[description="Under construction. Please wait warmly!"]
async fn reroll(ctx: &Context, msg: &Message) -> CommandResult {
    // Get config data with write permission to manipulate the tray
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
#[aliases("cod", "cofd")]
async fn wod(ctx: &Context, msg: &Message) -> CommandResult {
    let roll = format!("{} I'm not edgy enough for that yet!", msg.author);
    msg.channel_id.say(&ctx.http, roll).await?;

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
async fn exroll(ctx: &Context, msg: &Message) -> CommandResult {
    let roll = format!("{} I'm not epic enough for that yet!", msg.author);
    msg.channel_id.say(&ctx.http, roll).await?;

    Ok(())
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
