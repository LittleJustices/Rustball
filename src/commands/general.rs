use serenity::{
    framework::{
        standard::{
            CommandResult,
            macros::{
                command,
            },
        },
    },
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};

use std::{
    fs,
    path::Path,
};

#[command]
#[description = "Ping-pong command to check if I'm online."]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let pong = format!("{} Pong!", msg.author);
    msg.channel_id.say(&ctx.http, pong).await?;

    Ok(())
}

#[command]
#[description = "Prints a goodbye message and shuts me down."]
async fn bye(ctx: &Context, msg: &Message) -> CommandResult {
    let bye = String::from("Bye~! ❤");
    msg.channel_id.say(&ctx.http, bye).await?;

    std::process::exit(0);
}

#[command]
#[description = "Gives the source for my profile picture."]
async fn pfp(ctx: &Context, msg: &Message) -> CommandResult {
    let link = fs::read_to_string("PFP_Source.txt");

    let sauce = match link {
        Ok(s) => format!("{} My profile picture is sourced from: {}", msg.author, s),
        Err(e) => {
            format!("☢ I'm sorry, I lost the source! ☢\nError reading PFP source file: {}", e)
        }
    };
    msg.channel_id.say(&ctx.http, sauce).await?;

    Ok(())
}

#[command]
#[description = "A more detailed hello-world command to test sending complicated messages."]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("Hello, World!");
        m.embed(|e| {
            e.title("This is a title");
            e.description("This is a description");
            e.image("attachment://PFP_01.png");
            e.fields(vec![
                ("This is the first field", "This is a field body", true),
                ("This is the second field", "Both of these fields are inline", true),
            ]);
            e.field("This is the third field", "This is not an inline field", false);
            e.footer(|f| {
                f.text("This is a footer");

                f
            });

            e
        });
        m.add_file(AttachmentType::Path(Path::new("./PFP_01.png")));
        m
    }).await?;

    Ok(())
}