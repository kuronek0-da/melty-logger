/* 
Took from Constants.hpp in CCCaster
#define CC_GAME_MODE_ADDR           ( ( uint32_t * ) 0x54EEE8 ) // Current game mode, constants below

#define CC_GAME_MODE_CHARA_SELECT   ( 20 )
#define CC_GAME_MODE_IN_GAME        ( 1 )
#define CC_GAME_MODE_RETRY          ( 5 )

#define CC_P1_CHARACTER_ADDR        ( ( uint32_t * ) 0x74D8FC )
#define CC_P1_MOON_SELECTOR_ADDR    ( ( uint32_t * ) 0x74D900 )
#define CC_P2_CHARACTER_ADDR        ( ( uint32_t * ) 0x74D920 )
#define CC_P2_MOON_SELECTOR_ADDR    ( ( uint32_t * ) 0x74D924 )

*/
use crate::character::Character;

pub const GAME_MODE_ADDR: usize = 0x54EEE8;
pub const P1_CHARACTER_ADDR: usize = 0x74D8FC;
pub const P1_MOON_SELECTOR_ADDR: usize = 0x74D900;
pub const P2_CHARACTER_ADDR: usize = 0x74D920;
pub const P2_MOON_SELECTOR_ADDR: usize = 0x74D924;

#[repr(u32)]
pub enum GameMode {
    CharSelect = 20,
    InGame = 1,
    Retry = 5
}

pub struct Player {
    pub char: Character,
    pub moon: Moon
}

#[derive(Debug)]
#[repr(u32)]
pub enum Moon {
    Crescent,
    Full,
    Half,
    None
}

#[derive(Debug)]
#[repr(u32)]
pub enum GameState {
    InGame = 1,
    Retry = 5,
    CharSelect = 20,
}