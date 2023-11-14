mod commands;
mod states;

// region: IMPORTS
use anyhow;
use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use states::GameState;
use std::env;
use std::str::FromStr;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::{SuiClient, SuiClientBuilder};
// endregion IMPORTS

struct Handler {
    sui_client: SuiClient,
    package_id: ObjectID,
    address: SuiAddress,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "world").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            let res = GameState::new(&self.sui_client, self.address.clone())
                .await
                .get_game_state_board();

            // TODO: add command -> function
            match command.data.name.as_str() {
                "hunt" => {
                    let data = commands::hunt::do_hunt(
                        &self.sui_client,
                        &self.package_id,
                        &command.data.options,
                    )
                    .await
                    .map_err(|e| println!("!!error: {e:?}"))
                    .unwrap();

                    println!("{data:?}");
                }
                "battle" => {
                    let _ =
                        commands::battle::do_battle(&self.sui_client, &command.data.options).await;
                }
                _ => println!("not implemented :("),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(res))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| commands::hunt::register(command));
            commands.create_application_command(|command| commands::battle::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let sui_client = SuiClientBuilder::default().build_localnet().await?;
    let address = SuiAddress::from_str(env::var("SUI_CLIENT_ADDRESS").expect("").as_str())
        .unwrap_or_default();

    let handler = Handler {
        sui_client,
        package_id: ObjectID::from_str(
            "0x229ce700bb2bbf4cfa17cb9d92d18c80885252b464258ecaa410c2d1b7f88512",
        )?,
        address,
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}

pub fn truncate_hex_string(input: &str, n: usize) -> String {
    let prefix = &input[..n];
    let suffix = &input[input.len() - n..];
    let ellipsis = "..";

    format!("{}{}{}", prefix, ellipsis, suffix)
}
