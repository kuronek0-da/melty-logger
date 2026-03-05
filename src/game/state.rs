use crate::memory::addresses::{GameMode};

pub struct GameState {
    pub world_timer: u32,
    pub round_timer: u32,
    pub real_timer: u32,
    pub game_mode: u32,
}

pub enum MatchState {
    WaitingInCharSelect,
    InGame,
    RetryMenu,
    Finished,
    Invalid
}

pub struct MatchTracker {
    seen_charselect: bool,
    seen_ingame: bool
}