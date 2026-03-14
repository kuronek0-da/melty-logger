mod game;
mod memory;
mod validation;
mod client;
mod config;
use std::array::repeat;
use std::fmt::format;
use std::io::{self, Write};
use std::sync::Mutex;
use std::{
    io::stdin,
    sync::{
        Arc, atomic::{AtomicBool, Ordering}, mpsc::{Receiver, Sender, channel}
    },
    thread::sleep,
    time::Duration,
};

use uuid::Uuid;

use crate::client::ClientManager;
use crate::client::state::ClientState;
use crate::validation::result::MatchResult;
use crate::{game::state::GameState, memory::addresses::GameMode};
use crate::memory::manager::MemoryManager;
use crate::validation::validator::{Validator, Validity};

fn main() {
    println!("=== ATLAS OBSERVER ===\n");

    let client = ClientManager::new().unwrap();
    let state = client.clone_state();
    let (tx, rx) = channel();

    *state.lock().unwrap() = if let Some(s) = host_or_join_input() {
        s
    } else {
        println!("Exiting app...");
        std::process::exit(1);
    };
    
    let m = std::thread::spawn(move || memory_thread(tx));
    let v = std::thread::spawn(move || validator_thread(rx, client));

    m.join();
    v.join();
}

fn memory_thread(tx: Sender<GameState>) {
    let mut memory = MemoryManager::new();

    'outer: loop {
        if memory.is_running() {
            update_status(format!("MBAA session detected. Restart MBAA.exe."));

            while memory.is_running() {
                sleep(Duration::from_secs(2));
            }
        }

        // print!("\r{}", " ".repeat(100));
        update_status(format!("Waiting for MBAA.exe..."));
        while let Err(_) = memory.attach() {
            sleep(Duration::from_secs(2));
        }

        update_status(format!("Attached to MBAA.exe"));
        loop {

            match memory.poll() {
                Ok(state) => {
                    if tx.send(state).is_err() {
                        update_status(format!("Receiver dropped, shutting down memory thread."));
                        return;
                    }
                },
                Err(e) => {
                    if !memory.is_running() {
                        update_status(format!("Game closed. Ended ranked mode"));
                    } else {
                        update_status(format!("Lost connection: {:?}, Ended ranked mode", e));
                    }

                    memory.detach();
                    break;
                }
            }
            sleep(Duration::from_millis(16));
        }
    }
}

fn validator_thread(rx: Receiver<GameState>, client: ClientManager) {
    let mut validator = Validator::new(client.clone_state());

    for state in rx {
        match validator.validate(state) {
            Ok(validity) => match validity {
                Validity::Invalid(reason) => {
                    update_status(format!("Invalid game state: {}", reason));
                    break;
                },
                Validity::MatchFinished(result) => {
                    client.send_result(&result);
                    update_status(format!("Finished = {:?}", result));
                },
                _ => {}
            },
            Err(e) => {
                update_status(format!("Validator error: {:?}", e));
                break;
            }
        }
    }
}

fn host_or_join_input() -> Option<ClientState> {
    println!("Commands: 'host' to generate a code, 'join <code>' to join, 'stop' to cancel");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "host" => {
                let host_state = ClientState::hosting();
                if let Some(session) = host_state.get_session() {
                    println!("Your code: {}", session);
                }
                break Some(host_state);
            },
            cmd if cmd.starts_with("join ") => {
                let session = cmd[5..].trim().to_string();
                println!("Joined ranked match with code: {}", session);
                break Some(ClientState::JoinedRanked(session.clone()));
            },
            "stop" => {
                break None;
            }
            _ => println!("Unknown command.")
        }
    }
}

fn update_status(msg: String) {
    // print!("{}", " ".repeat(100));
    println!("\r[Status: {msg}]");
    // std::io::stdout().flush().unwrap();
}