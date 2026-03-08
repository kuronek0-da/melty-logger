use std::fmt::format;

use crate::{game::state::GameState, memory::addresses::GameMode, validation::result::{MatchResult, StateError}};

pub struct Validator {
    matchstate: MatchState,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            matchstate: MatchState::default(),
        }
    }

    pub fn validate(&mut self, state: GameState) -> Result<Validity, StateError> {
        match state {
            GameState::InGame { mode, timers, players } => {
                self.update_matchstate(&mode);
                match &self.matchstate {
                    MatchState::MatchFinished => {
                        let result = MatchResult::new(players, timers)?;
                        return Ok(Validity::MatchFinished(result));
                    },
                    MatchState::Invalid(reason) => Ok(Validity::Invalid(reason.clone())),
                    _ => Ok(Validity::Valid)
                }
            },
            GameState::NotInGame { mode } => {
                self.update_matchstate(&mode);
                match &self.matchstate {
                    MatchState::Invalid(reason) => Ok(Validity::Invalid(reason.clone())),
                    _ => Ok(Validity::Valid)
                }
            }
        }
    }

    pub fn update_matchstate(&mut self, mode: &GameMode) {
        if !self.matchstate.is_valid_before(mode) {
            self.matchstate = MatchState::invalid_mode(mode);
            return;
        }

        match mode {
            GameMode::CharSelect => self.matchstate = MatchState::WaitingInCharSelect,
            GameMode::InGame => self.matchstate = MatchState::InGame,
            GameMode::Retry => {
                if self.matchstate == MatchState::InGame {
                    self.matchstate = MatchState::MatchFinished;
                    return;
                }
                self.matchstate = MatchState::RetryMenu;
            },
            GameMode::ReplayMenu => self.matchstate = MatchState::invalid_mode(mode),
            _ => ()
        }
    }
}

#[derive(Debug)]
pub enum Validity {
    Valid,
    Invalid(String),
    MatchFinished(MatchResult)
}

#[derive(Debug, Eq, PartialEq, Default)]
pub enum MatchState {
    #[default]
    Idle, // Before char select and after retry menu
    WaitingInCharSelect,
    InGame,
    RetryMenu,
    MatchFinished, // Will only happen once every match, then go back to retry
    Invalid(String),
}

impl MatchState {
    pub fn invalid_mode(mode: &GameMode) -> MatchState {
        MatchState::Invalid(format!("invalid match state detected before {:?}", mode))
    }

    fn is_valid_before(&self, mode: &GameMode) -> bool {
        match mode {
            GameMode::CharSelect => matches!(self,
                MatchState::Idle | MatchState::RetryMenu |
                MatchState::InGame | MatchState::WaitingInCharSelect
            ),
            GameMode::InGame => matches!(self,
                MatchState::RetryMenu | MatchState::InGame | MatchState::WaitingInCharSelect
            ),
            GameMode::Retry => matches!(self,
                MatchState::InGame | MatchState::RetryMenu | MatchState::MatchFinished
            ),
            _ => true
        }
    }

}