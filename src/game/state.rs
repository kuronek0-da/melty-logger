use std::default;

use thiserror::Error;

use super::character::{Character, Moon};
use crate::memory::addresses::GameMode;

#[derive(Debug)]
pub enum GameState {
    InGame { mode: GameMode, timers: GameTimers, players: Players },
    NotInGame { mode: GameMode }
}

impl GameState {
    pub fn new_in_game(mode: GameMode, timers: GameTimers, players: Players) -> Self {
        GameState::InGame {
            mode,
            timers,
            players
        }
    }

    pub fn new_not_ingame(mode: GameMode) -> Self {
        GameState::NotInGame { mode }
    }
}

#[derive(Debug)]
pub struct GameTimers {
    world_timer: u32,
    round_timer: u32,
    real_timer: u32
}

impl GameTimers {
    pub fn new(world_timer: u32, round_timer: u32, real_timer: u32) -> Self {
        GameTimers { world_timer, round_timer, real_timer }
    }
    pub fn world_timer(&self) -> u32 {
        self.world_timer
    }
    pub fn round_timer(&self) -> u32 {
        self.round_timer
    }
    pub fn real_timer(&self) -> u32 {
        self.real_timer
    }
}

#[derive(Debug, Clone)]
pub struct Players {
    pub p1: Player,
    pub p2: Player,
}

impl Players {
    pub fn new(p1: Player, p2: Player) -> Self {
        Players { p1, p2 }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub char: Character,
    pub score: u32,
    pub moon: Moon,
}