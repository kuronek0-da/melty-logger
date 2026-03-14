use std::{ops::Deref, sync::{Arc, Mutex}};

use rand::{Rng, RngExt};
use reqwest::blocking::Client;
use crate::{client::state::ClientState, config::{Config, ConfigError}, validation::result::MatchResult};
use thiserror::Error;
use crate::update_status;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("could not send request for '{0}'")]
    RequestError(String),
    #[error("something went wrong, server status response: '{0}'")]
    ServerError(u16)
}

pub struct ClientManager {
    player_id: u32,
    server_url: String,
    state: Arc<Mutex<ClientState>>,
    client: Client
}

impl ClientManager {
    pub fn new() -> Result<Self, ConfigError> {
        Self::from_config(Config::load()?)
    }

    pub fn new_test() -> Result<Self, ConfigError> {
        Self::from_config(Config::load_test()?)
    }

    fn from_config(config: Config) -> Result<Self, ConfigError> {
        Ok(ClientManager {
            player_id: config.player_id,
            server_url: config.server_url,
            state: Arc::new(Mutex::new(ClientState::Idle)),
            client: Client::new(),
        })
    }

    pub fn clone_state(&self) -> Arc<Mutex<ClientState>> {
        Arc::clone(&self.state)
    }

    pub fn send_result(&self, result: &MatchResult) -> Result<String, ClientError> {
        let res = self.client.post(self.result_path())
            .json(&result)
            .send().map_err(|_| ClientError::RequestError("result".to_string()))?;
        
        if res.status().is_success() {
            Ok(res.text().unwrap())
        } else {
            Err(ClientError::ServerError(res.status().as_u16()))
        }
    }

    fn result_path(&self) -> String {
        format!("{}/api/match?playerId={}", self.server_url, self.player_id)
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::{game::{character::{GameChar, Moon}, state::{GameTimers, Player, Players}}, validation::result::MatchTimers};

    use super::*;
    
    fn mock_match_result(session_id: String) -> MatchResult {
        let p1 = Player {
            char: GameChar::Akiha, moon: Moon::Half, score: 2
        };
        let p2 = Player {
            char: GameChar::Seifuku, moon: Moon::Crescent, score: 1
        };
        MatchResult::new(
            1u8,
            Players {p1, p2},
            GameTimers::new(0, 25, 120),
            session_id).unwrap()
    }

    #[test]
    fn test_send_result() {
        let client = ClientManager::new_test().expect("Failed to load config.");
        let result1 = mock_match_result("ABCDEFG".to_string());
        let result2 = mock_match_result("ABCDEFG".to_string());

        assert!(client.send_result(&result1).is_ok(), "Failed to send result to the server.");
        sleep(Duration::from_secs(1));
        let sec_res = client.send_result(&result2);
        assert!(sec_res.is_ok());

        println!("{}", sec_res.unwrap());
    }
}