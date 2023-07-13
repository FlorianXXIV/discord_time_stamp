use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::Client;
use serenity::client::{Context, EventHandler};
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandResult,
    DispatchError,
    HelpOptions,
    StandardFramework};
use serenity::framework::standard::buckets::LimitedFor;
use serenity::framework::standard::macros::{command, group, hook, help};
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::UserId;
use serenity::prelude::*;
use tokio::sync::Mutex;

use crate::unix_time_stamp::TimeStamp;

pub mod unix_time_stamp;

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
#[commands(time,time_rel,time_f)]
struct General;

#[group]
#[commands(stamp,stamp_rel,stamp_f)]
struct Stamps;

// TODO Add better help
#[help]
#[individual_command_tip="Hey\n\n\
    If you want more info on a command, just pass the command as an argument :)"]
#[command_not_found_text = "Could not find '{}'."]
#[max_levenshtein_distance(3)]
#[lacking_permissions="Hide"]
#[lacking_role="Nothing"]
#[wrong_channel="Strike"]
async fn my_help(
    context:&Context,
    msg:&Message,
    args:Args,
    help_options:&'static HelpOptions,
    groups:&[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context,msg,args,help_options,groups,owners).await?;
    Ok(())
}

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
        .group(&GENERAL_GROUP)
        .group(&STAMPS_GROUP)
        .help(&MY_HELP);

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

#[command]
#[bucket = "stamp"]
async fn time_rel(ctx:&Context, msg:&Message,mut args:Args) -> CommandResult {
    let date = args.single::<String>()?;
    let time = args.single::<String>()?;
    let offset = args.single::<String>()?;

    msg.channel_id.say(&ctx.http, TimeStamp::get_rel_time_stamp(&date, &time, &offset)).await?;

    Ok(())
}

#[command]
#[bucket = "stamp"]
async fn time_f(ctx:&Context, msg:&Message,mut args:Args) -> CommandResult {
    let date = args.single::<String>()?;
    let time = args.single::<String>()?;
    let offset = args.single::<String>()?;
    let formatter = args.single::<String>()?;

    msg.channel_id.say(&ctx.http,
                       TimeStamp::get_dynamic_time_stamp(&date, &time, &offset, &formatter)).await?;

    Ok(())
}

#[command]
#[bucket = "stamp"]
async fn stamp(ctx:&Context, msg:&Message,mut args:Args) -> CommandResult {
    let date = args.single::<String>()?;
    let time = args.single::<String>()?;
    let offset = args.single::<String>()?;

    msg.channel_id.say(&ctx.http,
                       TimeStamp::get_actual_time_stamp(&date, &time, &offset)).await?;

    Ok(())
}

#[command]
#[bucket = "stamp"]
async fn stamp_rel(ctx:&Context, msg:&Message,mut args:Args) -> CommandResult {
    let date = args.single::<String>()?;
    let time = args.single::<String>()?;
    let offset = args.single::<String>()?;

    msg.channel_id.say(&ctx.http,
                       TimeStamp::get_rel_actual_time_stamp(&date, &time, &offset)).await?;

    Ok(())
}

#[command]
#[bucket = "stamp"]
async fn stamp_f(ctx:&Context, msg:&Message,mut args:Args) -> CommandResult {
    let date = args.single::<String>()?;
    let time = args.single::<String>()?;
    let offset = args.single::<String>()?;
    let formatter = args.single::<String>()?;

    msg.channel_id.say(&ctx.http,
                       TimeStamp::get_dynamic_actual_time_stamp(&date, &time, &offset, &formatter)).await?;

    Ok(())
}