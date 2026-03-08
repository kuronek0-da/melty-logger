mod game;
mod memory;
mod validation;

use std::io::Write;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::{
    io::{self},
    thread::sleep,
    time::Duration,
};
use crate::game::state::GameState;
use crate::memory::addresses::GameMode;
use crate::memory::manager::MemoryManager;
use crate::validation::validator::{Validator, Validity};

fn main() {
    println!("App started");

    let (tx, rx) = channel();
    std::thread::spawn(move || {
        memory_thread(tx);
    });
    
    validator_thread(rx);
}

fn memory_thread(tx: Sender<GameState>) {
    let mut memory = MemoryManager::new();

    loop {
        print!("\rRanked cannot start if MBAA is already open");
        io::stdout().flush().unwrap();
        while memory.is_running() {
            sleep(Duration::from_secs(2));
        }

        print!("\r{}", " ".repeat(100));
        println!("\rWaiting for MBAA.exe...");
        io::stdout().flush().unwrap();
        while let Err(_) = memory.attach() {
            sleep(Duration::from_secs(2));
        }

        print!("\r{}", " ".repeat(100));
        println!("\rAttached to MBAA.exe");
        io::stdout().flush().unwrap();
        loop {
            match memory.poll() {
                Ok(state) => {
                    if tx.send(state).is_err() {
                        eprintln!("Receiver dropped, shutting down memory thread.");
                        return;
                    }
                },
                Err(e) => {
                    if !memory.is_running() {
                        println!("Game closed.");
                        break;
                    }
                    eprintln!("Lost connection: {:?}", e);
                    memory.detach();
                    break;
                }
            }
            sleep(Duration::from_millis(16));
        }
    }
}

fn validator_thread(rx: Receiver<GameState>) {
    let mut validator = Validator::new();


    for state in rx {
        match validator.validate(state) {
            Ok(validity) => match validity {
                Validity::Invalid(reason) => {
                    println!("Invalid game state: {}", reason);
                    break;
                },
                Validity::MatchFinished(result) => {
                    print!("\r{}", " ".repeat(200));
                    println!("Match finished: {:?}", result);
                },
                _ => {}
            },
            Err(e) => {
                eprintln!("Validator error: {:?}", e);
                break;
            }
        }
    }
}
