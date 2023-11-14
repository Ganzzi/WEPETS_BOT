use std::str::FromStr;

use anyhow::Error;
use serde_json::{json, Number, Value as JsonValue};
use serenity::builder;
use serenity::model::application::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;
use std::env;
use sui_sdk::json::SuiJsonValue;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::types::transaction::TransactionData;
use sui_sdk::SuiClient;

pub async fn do_hunt(
    sui_client: &SuiClient,
    package_object_id: &ObjectID,
    options: &[CommandDataOption],
) -> Result<TransactionData, anyhow::Error> {
    let signer = SuiAddress::from_str(env::var("SUI_CLIENT_ADDRESS").expect("").as_str())
        .unwrap_or_default();

    let option = get_option(options, 0).unwrap();
    let option2 = get_option(options, 1).unwrap();

    // FIXME:  MODULE_NAME, FUNCTION_NAME
    if let (CommandDataOptionValue::String(arg_1), CommandDataOptionValue::Integer(arg_2)) =
        (option, option2)
    {
        let a: Result<TransactionData, anyhow::Error> = sui_client
            .transaction_builder()
            .move_call(
                signer,
                package_object_id.clone(),
                "MODULE_NAME",
                "FUNCTION_NAME",
                vec![],
                vec![
                    SuiJsonValue::from_str(arg_1)?,
                    SuiJsonValue::new(json!(arg_2))?,
                ],
                None,
                1000,
            )
            .await;

        a
    } else {
        Err(Error::msg("error message"))
    }
}

fn get_option(options: &[CommandDataOption], index: usize) -> Option<&CommandDataOptionValue> {
    let option = options
        .get(index)
        .expect("Expected user option")
        .resolved
        .as_ref();
    option
}

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("hunt")
        .description("Test command for number input")
        .create_option(|option| {
            option
                .name("animal")
                .description("animal id")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("coin")
                .description("coin to hunt")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(100)
                .required(true)
        })
}
