use thiserror::Error;

use crate::game::state::{ GameState, GameTimers, Players };


#[derive(Debug, Error)]
pub enum StateError {
    #[error("could not create a valid result: '{0}'")]
    MatchResultError(String)
}

#[derive(Debug)]
pub enum Winner {
    Player1,
    Player2
}

#[derive(Debug)]
pub struct MatchResult {
    winner: Winner,
    players: Players,
    // Should be the same for both players unless desyncs occur
    timers: GameTimers
}

impl MatchResult {
    pub fn new(players: Players, timers: GameTimers) -> Result<Self, StateError> {
        let p1 = &players.p1;
        let p2 = &players.p2;
        let winner: Winner;
        
        // Max score = 3
        if p1.score >= 3 || p2.score >= 3 {
            return Err(StateError::MatchResultError("max score is 3.".to_string()));
        }
        if p1.score > p2.score {
            return Ok(MatchResult { winner: Winner::Player1, players, timers });
        }
        return Ok(MatchResult { winner: Winner::Player2, players, timers });
    }
}