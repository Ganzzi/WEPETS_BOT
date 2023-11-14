use serenity::builder;
use serenity::model::application::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;
use sui_sdk::SuiClient;

// TODO: implement function
pub async fn do_battle(
    sui_client: &SuiClient,
    options: &[CommandDataOption],
) -> Result<(), anyhow::Error> {
    Ok(())
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
        .name("battle")
        .description("Command for battle ...")
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
