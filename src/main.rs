mod game;
mod memory;

use chrono::prelude::Local;

use crate::memory::manager;
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    loop {
        if let Ok(memory) = manager::MemoryManager::new() {
            write("\rAttached to MBAA.exe");

            /*
             memory.poll -> GameState
             validate(GameState)
             valid -> continue
             invalid -> stop, send InvalidGameState or GameInterrupted
             */
            while let Ok(_) = memory.poll() {
                sleep(Duration::from_millis(16));
            }
        } else {
            write("\rWaiting for MBAA.exe");
        }

        sleep(Duration::from_secs(2));
    }

    // let mut sys = System::new_with_specifics(
    //     RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    // );

    // 'outer: loop {
    //     let pid;
    //     if let Some(x) = get_melty(&mut sys) {
    //         pid = x;
    //     } else {
    //         print!("\rWaiting for MBAA.exe");
    //         io::stdout().flush();
    //         pid = 0;

    //         sleep(Duration::from_millis(2000));
    //         continue 'outer;
    //     }

    //     unsafe {
    //         let process: HANDLE =
    //             OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
    //                 .expect("Failed to open process.");
    //         println!("Found MBAA.exe \nWaiting to get in-game.\n");

    //         'inner: loop {
    //             unsafe {
    //                 if let Some(mode) = read_u32(process, GAME_MODE_ADDR) {
    //                     let local_time = Local::now();
    //                     println!("[{}]", local_time.format("%Y-%m-%d %H:%M:%S"));

    //                     match mode {
    //                         1 => println!("GameMode: {:?}", GameState::InGame),
    //                         5 => println!("GameMode: {:?}", GameState::Retry),
    //                         20 => println!("GameMode: {:?}", GameState::CharSelect),
    //                         _ => println!("GameMode: {}", mode),
    //                     };

    //                     if let Some(c) = read_u32(process, P1_CHARACTER_ADDR) {
    //                         let moon = get_moon(read_u32(process, P1_MOON_SELECTOR_ADDR));

    //                         if let Some(chara) = Character::try_from(c).ok() {
    //                             println!("P1 Char: {:?} {:?}", moon, chara);
    //                         }
    //                     }
    //                     if let Some(c) = read_u32(process, P2_CHARACTER_ADDR) {
    //                         let moon = get_moon(read_u32(process, P2_MOON_SELECTOR_ADDR));

    //                         if let Some(chara) = Character::try_from(c).ok() {
    //                             println!("P2 Char: {:?} {:?}", moon, chara);
    //                         }
    //                     }
    //                 } else {
    //                     break 'inner;
    //                 }
    //             }
    //             println!("");
    //             sleep(Duration::from_millis(1000));
    //         }

    //         CloseHandle(process);
    //     }
    // }
}

pub fn write(msg: &str) {
    print!("\r\x1b[2K{}", msg);
    let _ = std::io::stdout().flush();
}

// pub fn get_melty(mut sys: &mut System) -> Option<u32> {
//     sys.refresh_processes(ProcessesToUpdate::All, true);

//     if let Some(p) = sys.processes_by_exact_name("MBAA.exe".as_ref()).next() {
//         Some(p.pid().as_u32())
//     } else {
//         None
//     }
// }

// fn get_moon(value: Option<u32>) -> Moon {
//     match value {
//         Some(0) => Moon::Crescent,
//         Some(1) => Moon::Full,
//         Some(2) => Moon::Half,
//         _ => Moon::None,
//     }
// }

// unsafe fn read_u32(process: HANDLE, address: usize) -> Option<u32> {
//     let mut buffer: u32 = 0;
//     let mut bytes_read: usize = 0;

//     let result = ReadProcessMemory(
//         process,
//         address as *const _,
//         &mut buffer as *mut _ as *mut _,
//         std::mem::size_of::<u32>(),
//         Some(&mut bytes_read),
//     );

//     if result.is_ok() && bytes_read == size_of::<u32>() {
//         Some(buffer)
//     } else {
//         None
//     }
// }
