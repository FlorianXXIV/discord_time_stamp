pub mod unix_time_stamp;

use std::{env, io};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serenity::async_trait;
use serenity::Client;
use serenity::client::{Context, EventHandler};
use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::framework::standard::buckets::{LimitedFor, RevertBucket};
use serenity::framework::standard::macros::{check, command, group, help, hook};
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandOptions,
    CommandResult,
    DispatchError,
    HelpOptions,
    Reason,
    StandardFramework,
};
use serenity::http::Http;
use serenity::model::channel::{Channel, Message};
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::UserId;
use serenity::model::permissions::Permissions;
use serenity::prelude::*;

use tokio::sync::Mutex;
use crate::unix_time_stamp::TimeStamp;


struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter{
    type Value = HashMap<String,u64>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping"{
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await{
                println!("Error sending message: {:?}", why);
            }
        }
    }*/

    async fn ready(&self, _ctx:Context, ready:Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(time)]
struct General;

#[hook]
async fn before(ctx:&Context, msg:&Message, command_name:&str) -> bool {
    println!("Got command '{}' by User '{}'", command_name, msg.author.name);

    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true
}

#[hook]
async fn after(_ctx:&Context, _msg:&Message, command_name:&str, command_result:CommandResult) {
    match command_result {
        Ok(()) => println!("Processed Command {}", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx:&Context, _msg:&Message, unknown_command_name:&str) {
    println!("Could not find {}", unknown_command_name);
}

#[hook]
async fn normal_message(_ctx:&Context, msg:&Message){
    println!("Message is not a command named '{}'", msg.content);
}

#[hook]
async fn delay_action(ctx: &Context, msg: &Message) {
    let _ = msg.react(ctx, '⏱').await;
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
    if let DispatchError::Ratelimited(info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("Try this again in {} seconds.", info.as_secs()))
                .await;
        }
    }
}

#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token environment");

    let http = Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not acces the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not acces Application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("§")
            .delimiters(vec![", ",","])
            .owners(owners))
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .bucket("stamp", |b| b.delay(5)).await
        .bucket("complicated", |b| b.limit(2).time_span(30).delay(5)
            .limit_for(LimitedFor::Channel)
            .await_ratelimits(1)
            .delay_action(delay_action)).await
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::all();

    let mut client =
        Client::builder(&token, intents)
            .event_handler(Handler)
            .framework(framework)
            .type_map_insert::<CommandCounter>(HashMap::default())
            .await
            .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
#[bucket = "stamp"]
async fn time(ctx:&Context, msg:&Message,mut args:Args) -> CommandResult {
    let date = args.single::<String>()?;
    let time = args.single::<String>()?;
    let offset = args.single::<String>()?;

    msg.channel_id.say(&ctx.http, TimeStamp::get_discord_time_stamp(&date, &time, &offset)).await?;

    Ok(())
}

//For safekeeping
/*fn main() {
    let mut buffer = String::new();

    println!("Input date as [year]-[month]-[day]");
    io::stdin().read_line(&mut buffer).expect("failed to read line");
    let date = buffer.clone();
    buffer.clear();
    println!("Input time as [hour]:[minute]");
    io::stdin().read_line(&mut buffer).expect("failed to read line");
    let time = buffer.clone();
    buffer.clear();
    println!("Input timezone (UTC offset) as +/-[hour]:[minute]");
    io::stdin().read_line(&mut buffer).expect("failed to read line");
    let offset = buffer.clone();
    buffer.clear();

    let unix = unix_from_datetime(date.trim(), time.trim(), offset.trim());

    println!("<t:{}>", unix);
    println!("<t:{}:R>", unix);

}*/
