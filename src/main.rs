use std::env;

use rand::{self, Rng};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use tokio;

const FIRST_SYL: [&str; 10] = [
    "sc", "st", "sp", "sl", "sq", "sw", "scr", "str", "spr", "sch",
];
const SECOND_SYL: [&str; 10] = ["i", "u", "o", "oo", "uwu", "ui", "ou", "a", "ee", "u"];
const THIRD_SYL: [&str; 10] = [
    "ndle", "nkle", "mble", "ngle", "nger", "nder", "nker", "nple", "nper", "mper",
];

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let new_nick_first_syl: &str = FIRST_SYL[rand::thread_rng().gen_range(0..10)];
        let new_nick_second_syl: &str = SECOND_SYL[rand::thread_rng().gen_range(0..10)];
        let new_nick_third_syl: &str = THIRD_SYL[rand::thread_rng().gen_range(0..10)];
        let mut new_nick: String = new_nick_first_syl.to_string();

        new_nick.push_str(new_nick_second_syl);
        new_nick.push_str(new_nick_third_syl);

        if let Err(why) = ChannelId(746886412963676170)
            .say(
                &ctx.http,
                format!("Who tf is this? nah bruh your name is {} now", &new_nick),
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }

        if let Err(why) = GuildId(742346667541397574)
            .edit_member(&ctx, 751544722534694913, |m| m.nickname(&new_nick))
            .await
        {
            println!("Error changing nickname: {:?}", why);
        }

        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    let token: String = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
