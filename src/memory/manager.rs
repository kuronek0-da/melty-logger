use sysinfo::{Process, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
use windows::{
    Win32::Foundation::{CloseHandle, HANDLE},
    Win32::System::Diagnostics::Debug::ReadProcessMemory,
    Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
};

use super::addresses;
use crate::write;

pub struct MemoryManager {
    sys: System,
    process: HANDLE,
}

// TODO: actually make this an error
#[derive(Debug)]
pub enum MemoryError {
    ProcessNotFound,
    OpenProcessFailed,
    ReadFailed,
}

impl MemoryManager {
    pub fn new() -> Result<Self, MemoryError> {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything())
        );

        // let pid = find_melty_pid(&mut sys)?;

        let process = MemoryManager::attach(&mut sys)?;

        Ok(Self {
            sys,
            process
        })
    }

    pub fn refresh(&mut self) -> Result<(), MemoryError> {
        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        let process = MemoryManager::attach(&mut self.sys)?;
        self.process = process;
        Ok(())
    }

    pub fn poll(&self) -> Result<(), MemoryError> {
        unsafe {
            let state_value = read_u32(self.process, addresses::GAME_MODE_ADDR)?;

            if let Some(state) = addresses::GameMode::try_from(state_value).ok() {
                write(&format!("\rCurrent state: {:?}", state));
                Ok(())
            } else {
                write(&format!("\rCurrent state: unknown"));
                Ok(())
            }
        }
    }

    fn attach(mut sys: &mut System) -> Result<HANDLE, MemoryError> {
        sys.refresh_processes(ProcessesToUpdate::All, true);
        if let Some(p) = sys.processes_by_exact_name("MBAA.exe".as_ref()).next() {
            let pid = p.pid().as_u32();

            unsafe {
                OpenProcess(
                    PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                    false,
                    pid,
                ).map_err(|_| MemoryError::OpenProcessFailed)
            }
        } else {
            Err(MemoryError::ProcessNotFound)
        }
    }
}

unsafe fn read_u32(process: HANDLE, address: usize) -> Result<u32, MemoryError> {
    let mut buffer: u32 = 0;
    let mut bytes_read: usize = 0;

    let result = ReadProcessMemory(
        process,
        address as *const _,
        &mut buffer as *mut _ as *mut _,
        std::mem::size_of::<u32>(),
        Some(&mut bytes_read),
    );

    if result.is_ok() && bytes_read == size_of::<u32>() {
        Ok(buffer)
    } else {
        Err(MemoryError::ReadFailed)
    }
}

fn find_melty_pid(mut sys: &mut System) -> Result<u32, MemoryError> {
    sys.refresh_processes(ProcessesToUpdate::All, true);
    if let Some(p) = sys.processes_by_exact_name("MBAA.exe".as_ref()).next() {
        Ok(p.pid().as_u32())
    } else {
        Err(MemoryError::ProcessNotFound)
    }
}