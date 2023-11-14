use futures::{future, StreamExt};
use serde::Deserialize;
use std::{env, str::FromStr};
use sui_json_rpc_types::Coin;
use sui_sdk::{types::base_types::SuiAddress, SuiClient};

use crate::truncate_hex_string;

#[derive(Debug, Deserialize)]
pub struct GameState {
    address: SuiAddress,
    sui_coin: Option<Coin>,
    game_token: Option<Coin>,
}

impl GameState {
    pub async fn new(sui_client: &SuiClient, address: SuiAddress) -> Self {
        let coin_type: String = "0x2::sui::SUI".to_string();
        let coins_stream = sui_client
            .coin_read_api()
            .get_coins_stream(address, Some(coin_type));

        let mut coins = coins_stream
            .skip_while(|c| future::ready(c.balance < 5_000_000))
            .boxed();
        let coin = coins.next().await;

        GameState {
            address,
            sui_coin: coin,
            game_token: None,
        }
    }

    pub fn get_game_state_board(&self) -> String {
        let mut game_state_board = String::from("----------------------------------------------\n");
        game_state_board.push_str(&format!(
            "address: {:<30}\n",
            truncate_hex_string(self.address.to_string().as_str(), 15),
        ));

        if let Some(sui_coin) = &self.sui_coin {
            game_state_board.push_str(&format!(
                "balance: {:<20}{:<10}\n",
                sui_coin.balance, sui_coin.coin_type,
            ));
        }

        if let Some(game_token) = &self.game_token {
            game_state_board.push_str(&format!(
                "balance: {:<20}{:<10}\n",
                game_token.balance, game_token.coin_type,
            ));
        }

        game_state_board
    }
}
