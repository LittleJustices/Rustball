use chrono::prelude::*;

use serenity::{
    framework::{
        standard::{
            Args,
            ArgError,
            CommandResult,
            macros::{
                command,
            },
        },
    },
    http::AttachmentType,
    model::{
        channel::{
            Channel,
            Message,
        },
        id::ChannelId,
    },
    prelude::*,
};
use std::{
    collections::HashMap,
};
use crate::messaging::logger::Logger;

pub type LogsMap = HashMap<ChannelId, Logger>;

#[command]
#[only_in(guilds)]
#[description = "Start logging a channel.\n\n
I'll keep logging until someone tells me to stop with !unlog.\n
!log without an argument will log the channel the command was used in. To log a different channel, pass a mention to that channel as a command: `!log #general`."]
async fn log(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let target;

    match resolve_channel_mention(msg, args) {
        Ok(id) => target = id,
        Err(why) => {
            let chan_error = format!("☢ That's not a channel I recognize! ☢\n Error parsing channel id: {}", why);
            msg.reply_ping(&ctx.http, chan_error).await?;
            return Ok(());
        }
    };

    let allowed;
    match check_logging_permission(target, msg.channel_id, ctx).await {
        Ok(perm) => allowed = perm,
        Err(why) => {
            let check_error = format!("☢ I don't know if I'm allowed to do that! ☢\n Error checking logging permission: {}", why);
            msg.reply_ping(&ctx.http, check_error).await?;
            return Ok(());
        }
    };

    if !allowed {
        let perm_error = "☢ I'm not allowed to log that channel! ☢\nI can only start or stop logging a channel from within the same server.".to_string();
        msg.reply_ping(&ctx.http, perm_error).await?;
        return Ok(());
    }

    let filename;
    match construct_log_filename(target, ctx).await {
        Ok(name) => filename = name,
        Err(why) => {
            let name_error = format!("☢ Something went wrong! ☢\n Error constructing log filename: {}", why);
            msg.reply_ping(&ctx.http, name_error).await?;
            return Ok(());
        }
    }

    let log;
    {
        let config_data = ctx.data.read().await;
        let cfg = config_data.get::<crate::ConfigKey>().expect("Failed to retrieve config!");
        match Logger::new(&cfg.log_folder_path, &filename) {
            Ok(logger) => log = logger,
            Err(why) => {
                let log_error = format!("☢ Something went wrong! ☢\n Error creating log file: {}", why);
                msg.reply_ping(&ctx.http, log_error).await?;
                return Ok(());
            }
        }
    }

    let mut log_data = ctx.data.write().await;
    let log_map = log_data
        .get_mut::<crate::LogsKey>()
        .expect("Failed to retrieve logs map!");
    log_map
        .lock().await
        .insert(target, log);

    let log_confirm = format!("Logging <#{}> now! ❤", target);
    msg.reply_ping(&ctx.http, log_confirm).await?;
    
    Ok(())
}

#[command]
#[only_in(guilds)]
#[description = "Stop logging a channel.\n\n
Once I stop logging, I'll post the log file in the channel the command was used in.\n
!unlog without an argument will unlog the channel the command was used in. To unlog a different channel, pass a mention to that channel as a command: `!unlog #general`."]
async fn unlog(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let target;

    match resolve_channel_mention(msg, args) {
        Ok(id) => target = id,
        Err(why) => {
            let chan_error = format!("☢ That's not a channel I recognize! ☢\n Error parsing channel id: {}", why);
            msg.reply_ping(&ctx.http, chan_error).await?;
            return Ok(());
        }
    };

    let allowed;
    match check_logging_permission(target, msg.channel_id, ctx).await {
        Ok(perm) => allowed = perm,
        Err(why) => {
            let check_error = format!("☢ I don't know if I'm allowed to do that! ☢\n Error checking logging permission: {}", why);
            msg.reply_ping(&ctx.http, check_error).await?;
            return Ok(());
        }
    };

    if !allowed {
        let perm_error = "☢ I'm not allowed to log that channel! ☢\nI can only start or stop logging a channel from within the same server.".to_string();
        msg.reply_ping(&ctx.http, perm_error).await?;
        return Ok(());
    }

    let mut log_data = ctx.data.write().await;
    let log_map = log_data
        .get_mut::<crate::LogsKey>()
        .expect("Failed to retrieve logs map!");

    if let Some((target, logger)) = log_map.lock().await.remove_entry(&target) {
        match logger.end_log() {
            Ok(_) => {
                let unlog_confirm = format!("Okay, I'll stop logging <#{}>! ❤ Here's your log:", target);
                let message = msg.channel_id.send_message(&ctx.http, |m| {
                    m.content(unlog_confirm);
                    m.add_file(AttachmentType::Path(&logger.log_path));
                    m
                }).await;
                if let Err(why) = message {
                    println!("Error sending message: {:?}", why);
                }
            },
            Err(why) => {
                let check_error = format!("☢ Something went wrong! ☢\n Error closing log: {}", why);
                msg.reply_ping(&ctx.http, check_error).await?;
                return Ok(());
            },
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description = "Check if a channel is being logged.\n\n
!logging without an argument will check the channel the command was used in. To check a different channel, pass a mention to that channel as a command: `!logging #general`."]
async fn logging(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let target;

    match resolve_channel_mention(msg, args) {
        Ok(id) => target = id,
        Err(why) => {
            let chan_error = format!("☢ That's not a channel I recognize! ☢\n Error parsing channel id: {}", why);
            msg.reply_ping(&ctx.http, chan_error).await?;
            return Ok(());
        }
    };

    let allowed;
    match check_logging_permission(target, msg.channel_id, ctx).await {
        Ok(perm) => allowed = perm,
        Err(why) => {
            let check_error = format!("☢ I don't know if I'm allowed to do that! ☢\n Error checking logging permission: {}", why);
            msg.reply_ping(&ctx.http, check_error).await?;
            return Ok(());
        }
    };

    if !allowed {
        let perm_error = "☢ I'm not allowed to log that channel! ☢\nI can only start or stop logging a channel from within the same server.".to_string();
        msg.reply_ping(&ctx.http, perm_error).await?;
        return Ok(());
    }

    let log_data = ctx.data.read().await;
    let log_map = log_data
                    .get::<crate::LogsKey>()
                    .expect("Failed to retrieve logs map!")
                    .lock().await;

    let logging;
    if log_map.contains_key(&target) {
        logging = format!("{} I'm logging <#{}> right now!", msg.author, target);
    } else {
        logging = format!("{} I'm not logging <#{}> yet!", msg.author, target);
    }
    msg.reply_ping(&ctx.http, logging).await?;
    
    Ok(())
}

async fn check_logging_permission(target: ChannelId, source: ChannelId, ctx: &Context) -> Result<bool, serenity::Error> {
    // Block permission by default
    let mut allowed = false;

    // If the channel to be logged is the same as the one the command was issued in, allow (this includes DMs)
    if source == target { allowed = true; }

    // Otherwise, allow if target and source are in the same guild/server
    let (target_chan, source_chan) = (target.to_channel(ctx).await?, source.to_channel(ctx).await?);
    if let (Channel::Guild(target_guild_channel), Channel::Guild(source_guild_channel)) = (target_chan, source_chan) {
        if target_guild_channel.guild_id == source_guild_channel.guild_id { allowed = true; }
    }

    Ok(allowed)
}

async fn construct_log_filename(id: ChannelId, ctx: &Context) -> Result<String, serenity::Error> {
    let mut chan_name = "no_chan_name".to_string();
    let mut guild_name = "".to_string();

    let chan = id.to_channel(ctx).await?;
    if let Channel::Guild(guild_chan) = chan {
        chan_name = guild_chan.name().to_string();
        let guild = guild_chan.guild_id;
        guild_name = match guild.name(ctx).await {
            Some(name) => format!("_{}", name),
            None => "".to_string()
        }
    }
        
    let log_start_time = Utc::now().format("%Y-%m-%d-%a_%H-%M-%S");
    let log_file_name = format!("Sixball_Log{}_{}_{}", guild_name, chan_name, log_start_time);
    Ok(log_file_name)
}

fn resolve_channel_mention(msg: &Message, mut args: Args) -> Result<ChannelId, ArgError<serenity::model::misc::ChannelIdParseError>> {
    let target;

    if args.len() == 0 {
        target = msg.channel_id;
    } else {
        target = args.single::<ChannelId>()?;
        };

    Ok(target)
}