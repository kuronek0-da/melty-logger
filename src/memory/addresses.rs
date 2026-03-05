/* 
Took from Constants.hpp in CCCaster
Only those i might find relevant
#define CC_GAME_MODE_ADDR           ( ( uint32_t * ) 0x54EEE8 ) // Current game mode, constants below

#define CC_GAME_MODE_CHARA_SELECT   ( 20 )
#define CC_GAME_MODE_IN_GAME        ( 1 )
#define CC_GAME_MODE_RETRY          ( 5 )
#define CC_GAME_MODE_REPLAY         ( 26 ) 

#define CC_P1_CHARACTER_ADDR        ( ( uint32_t * ) 0x74D8FC )
#define CC_P1_MOON_SELECTOR_ADDR    ( ( uint32_t * ) 0x74D900 )
#define CC_P2_CHARACTER_ADDR        ( ( uint32_t * ) 0x74D920 )
#define CC_P2_MOON_SELECTOR_ADDR    ( ( uint32_t * ) 0x74D924 )

#define CC_WORLD_TIMER_ADDR         ( ( uint32_t * ) 0x55D1D4 ) // Frame step timer, always counting up
#define CC_ROUND_TIMER_ADDR         ( ( uint32_t * ) 0x562A3C ) // Counts down from 4752, may stop 
#define CC_REAL_TIMER_ADDR          ( ( uint32_t * ) 0x562A40 ) // Counts up from 0 after round start 
*/
use crate::game::character::{Character, Moon};

pub const WORLD_TIMER_ADDR: usize = 0x55D1D4;
pub const ROUND_TIMER_ADDR: usize = 0x562A3C;
pub const REAL_TIMER_ADDR: usize = 0x562A40;

pub const GAME_MODE_ADDR: usize = 0x54EEE8;

use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum GameMode {
    InGame = 1,
    Retry = 5,
    CharSelect = 20,
    ReplayMenu = 26
}

pub const P1_CHARACTER_ADDR: usize = 0x74D8FC;
pub const P2_CHARACTER_ADDR: usize = 0x74D920;

pub struct Player {
    pub char: Character,
    pub moon: Moon
}

pub const P1_MOON_SELECTOR_ADDR: usize = 0x74D900;
pub const P2_MOON_SELECTOR_ADDR: usize = 0x74D924;