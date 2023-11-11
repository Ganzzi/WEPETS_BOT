// region: IMPORTS
use anyhow;
use dotenvy::dotenv;
use futures::{future, stream::StreamExt};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::str::FromStr;
use sui_json_rpc_types::Coin;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::{SuiClient, SuiClientBuilder};
// endregion IMPORTS

struct Handler {
    sui_client: SuiClient,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!balance" {
            let address = SuiAddress::from_str(
                "0xaaefb759f59e15131cfdb31628347b0567f21ee146c3656bc6af913b340ff6ad",
            )
            .unwrap_or_default();

            let coin = fetch_coin(&self.sui_client, &address)
                .await
                .unwrap()
                .unwrap();

            let res = get_balance(address.to_string().as_str(), coin);

            if let Err(why) = msg.channel_id.say(&ctx.http, res).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let sui_client = Handler {
        sui_client: SuiClientBuilder::default().build_localnet().await?,
    };
    let mut client = Client::builder(&token, intents)
        .event_handler(sui_client)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}

pub async fn fetch_coin(
    sui: &SuiClient,
    sender: &SuiAddress,
) -> Result<Option<Coin>, anyhow::Error> {
    let coin_type = "0x2::sui::SUI".to_string();
    let coins_stream = sui
        .coin_read_api()
        .get_coins_stream(*sender, Some(coin_type));

    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_000_000))
        .boxed();
    let coin = coins.next().await;
    Ok(coin)
}

fn truncate_hex_string(input: &str, n: usize) -> String {
    let prefix = &input[..n];
    let suffix = &input[input.len() - n..];
    let ellipsis = "..";

    format!("{}{}{}", prefix, ellipsis, suffix)
}

fn get_balance(address: &str, coin: Coin) -> String {
    format!(
        "----------------------------------------------\naddress: {:<30}\nbalance: {:<20}{:<10}",
        truncate_hex_string(address, 15),
        coin.balance,
        coin.coin_type
    )
}
