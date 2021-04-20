use std::{
    collections::HashSet,
    fs,
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
            },
        },
    },
    http::Http,
    model::id::UserId,
    model::channel::Message,
    prelude::*,
};

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
};

struct LogsKey;

impl TypeMapKey for LogsKey {
    type Value = Arc<Mutex<commands::logging::LogsMap>>;
}

#[group]
#[description = "General commands related to bot operation."]
#[commands(ping, hello, pfp, bye)]
struct General;

#[group]
#[description = "Miscellaneous call and response commands for fun.\n\n
Feel free to try them out, but don't spam! ❤"]
#[commands(squid, shadow, unyu, atom, yuru)]
struct Funsies;

#[group]
#[description = "Commands related to rolling dice.\n\n
Use !roll for generic dice rolls or one of the specialized functions to use simplified syntax tailored to the system."]
#[commands(roll, wod, l5r, sroll, exroll)]
struct Roll;

#[group]
#[description = "Commands for logging channels.\n\n
Use !log to start logging, !unlog to stop logging, and !logging to check whether I'm already logging.\n
For all of these commands, the command used without an argument (e.g. `!log`) will apply to the channel the command is used in, but you can give me a channel mention as an argument (e.g. `!logging #general`) to target a specific other channel."]
#[only_in(guilds)]
#[commands(log, unlog, logging)]
struct Logging;

#[help]
#[individual_command_tip =
"Hi~! ❤\n\n\
If you want more information about a specific command, just pass the command as argument."]
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

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("DISCORD_TOKEN")
        .expect("Expected a token in the root folder");

    let http = Http::new_with_token(&token);

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
            .owners(owners)
            .prefix("!")
        )
        .help(&MY_HELP)
        // .group(&ROLL_GROUP)
        .group(&GENERAL_GROUP)
        .group(&LOGGING_GROUP)
        .group(&FUNSIES_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler::new())
        .type_map_insert::<LogsKey>(Arc::new(Mutex::new(commands::logging::LogsMap::new())))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}