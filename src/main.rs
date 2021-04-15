use std::{
    fs,
    collections::HashSet,
};

use serenity::{
    framework::{
        StandardFramework,
        standard::{
            // CommandResult,
            macros::{
                // command,
                group,
            },
        },
    },
    http::Http,
    // model::channel::Message,
    prelude::*,
};

mod messaging;
use messaging::{
    message_handler::Handler,
};

mod commands;
use commands::{
    general::*,
    logging::*,
    rolling::*,
};

#[group]
#[commands(ping, hello, squid, shadow, unyu, atom, yuru, pfp, bye)]
struct General;

#[group]
#[commands(roll, wod, l5r, sroll, exroll)]
struct Roll;

#[group]
#[commands(log, unlog, logging)]
struct Logging;

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
        .group(&GENERAL_GROUP)
        .group(&ROLL_GROUP)
        .group(&LOGGING_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}