use std::{
    collections::HashSet,
    sync::Arc,
};

use serenity::{
    framework::{
        StandardFramework,
        standard::{
            Args, CommandGroup, CommandResult, help_commands, HelpOptions,
            macros::{
                group,
                help,
                hook,
            },
        },
    },
    http::Http,
    model::id::UserId,
    model::channel::Message,
    prelude::*,
};

mod config;
use config::Config;

mod messaging;
use messaging::{
    message_handler::Handler,
};

mod commands;
use commands::{
    funsies::*,
    general::*,
    logging::*,
    rolling::*,
    math::*,
};

mod dice;

mod math;

struct LogsKey;

impl TypeMapKey for LogsKey {
    type Value = Arc<Mutex<commands::logging::LogsMap>>;
}

struct ConfigKey;

impl TypeMapKey for ConfigKey {
    type Value = Config;
}

#[group]
#[description = "General commands related to bot operation."]
#[commands(bye, hello, pfp, ping)]
struct General;

#[group]
#[description = "Miscellaneous call and response commands for fun.\n\n
Feel free to try them out, but don't spam! ❤"]
#[commands(atom, shadow, squid, unyu, yuru)]
struct Funsies;

#[group]
#[description = "Commands related to rolling dice.\n\n
Use !roll for generic dice rolls or one of the specialized functions to use simplified syntax tailored to the system."]
#[commands(roll, exroll, l5r, sr, wod)]
struct Dice;

#[group]
#[description = "Commands that make me do math. Currently under construction!"]
#[commands(calc)]
struct Math;

#[group]
#[description = "Commands for logging channels. Servers only (not available in DMs)!\n\n
Use !log to start logging, !unlog to stop logging, and !logging to check whether I'm already logging.\n
For all of these commands, the command used without an argument (e.g. `!log`) will apply to the channel the command is used in, but you can give me a channel mention as an argument (e.g. `!logging #general`) to target a specific other channel."]
#[only_in(guilds)]
#[commands(log, logging, unlog)]
struct Logging;

#[help]
#[individual_command_tip =
"Hi~! ❤\n\n\
If you want more information about a specific command, just pass the command as argument. For info about groups, pass the group as an argument!"]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(2)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    let mut log_data = ctx.data.write().await;
    let log_map = log_data
                    .get_mut::<LogsKey>()
                    .expect("Failed to retrieve logs map!")
                    .lock().await;

    if let Some(log) = log_map.get(&msg.channel_id) {
        match log.record(msg) {
            Ok(_) => return,
            Err(why) => println!("Error recording log message: {}", why)
        }
    }
    return;
}

#[tokio::main]
async fn main() {
    let config = Config::new();

    let Config { discord_token, prefix, log_folder_path: _, pfp_source: _} = &config;

    let http = Http::new_with_token(discord_token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .case_insensitivity(true)
            .owners(owners)
            .prefix(prefix)
            .with_whitespace(true)
        )
        .normal_message(normal_message)
        .help(&MY_HELP)
        .group(&DICE_GROUP)
        .group(&MATH_GROUP)
        .group(&GENERAL_GROUP)
        .group(&LOGGING_GROUP)
        .group(&FUNSIES_GROUP);

    let mut client = Client::builder(&discord_token)
        .framework(framework)
        .event_handler(Handler::new())
        .type_map_insert::<LogsKey>(Arc::new(Mutex::new(commands::logging::LogsMap::new())))
        .type_map_insert::<ConfigKey>(config)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}