#![cfg(windows)]

use std::{
    ffi::OsString,
    mem::MaybeUninit,
    os::windows::ffi::OsStringExt,
    path::{Path, PathBuf},
};

use windows::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE, MAX_PATH},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, Process32FirstW, Process32NextW,
        MODULEENTRY32W, PROCESSENTRY32W, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
    },
};

struct ModuleIterator {
    h_module_snap: windows::Win32::Foundation::HANDLE,
    me32: MODULEENTRY32W,
    first: bool,
}

impl ModuleIterator {
    fn new(h_module_snap: windows::Win32::Foundation::HANDLE) -> Self {
        Self {
            h_module_snap,
            #[allow(clippy::cast_possible_truncation)]
            me32: MODULEENTRY32W {
                dwSize: std::mem::size_of::<MODULEENTRY32W>() as u32,
                ..Default::default()
            },
            first: true,
        }
    }
}

impl Iterator for ModuleIterator {
    type Item = MODULEENTRY32W;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            unsafe { Module32FirstW(self.h_module_snap, &mut self.me32) }.ok()?;
        } else {
            unsafe { Module32NextW(self.h_module_snap, &mut self.me32) }.ok()?;
        }

        self.first = false;

        Some(self.me32)
    }
}

impl Drop for ModuleIterator {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.h_module_snap).ok() };
    }
}

struct ProcessIterator {
    h_process_snap: windows::Win32::Foundation::HANDLE,
    pe32: PROCESSENTRY32W,
    first: bool,
}

impl ProcessIterator {
    fn new(h_process_snap: windows::Win32::Foundation::HANDLE) -> Self {
        Self {
            h_process_snap,
            #[allow(clippy::cast_possible_truncation)]
            pe32: PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                ..unsafe { MaybeUninit::uninit().assume_init() }
            },
            first: true,
        }
    }
}

impl Iterator for ProcessIterator {
    type Item = PROCESSENTRY32W;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            unsafe { Process32FirstW(self.h_process_snap, &mut self.pe32) }.ok()?;
        } else {
            unsafe { Process32NextW(self.h_process_snap, &mut self.pe32) }.ok()?;
        }

        self.first = false;

        Some(self.pe32)
    }
}

impl Drop for ProcessIterator {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.h_process_snap).ok() };
    }
}

unsafe fn match_process_path(
    pe32: &PROCESSENTRY32W,
    base_path: impl AsRef<Path>,
) -> windows::core::Result<Vec<PathBuf>> {
    let pid = pe32.th32ProcessID;
    let mut paths = Vec::new();

    let h_module_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid)? };

    if h_module_snap == INVALID_HANDLE_VALUE {
        return Ok(Vec::new());
    }

    let module_iterator = ModuleIterator::new(h_module_snap);

    for me32 in module_iterator {
        let path = PathBuf::from(get_compare_string(&me32.szExePath));

        if path.starts_with(&base_path) {
            paths.push(path);
        }
    }

    Ok(paths)
}

fn get_compare_string(exe_file: &[u16]) -> String {
    let os_string = OsString::from_wide(exe_file);
    let utf8_string = os_string.to_string_lossy();
    let trimmed = utf8_string.trim_end_matches('\0');

    dbg!(trimmed.to_string())
}

pub unsafe fn find_running_process(base_dir: &str) -> windows::core::Result<bool> {
    let mut proc_running = false;

    let h_process_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? };

    if h_process_snap == INVALID_HANDLE_VALUE {
        proc_running = false;
    } else {
        let process_iterator = ProcessIterator::new(h_process_snap);

        for pe32 in process_iterator {
            let compare = unsafe { match_process_path(&pe32, base_dir)? };

            if !compare.is_empty() {
                proc_running = true;
                break;
            }
        }
    }

    Ok(proc_running)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test looks for 'cargo.exe', which can only exist on Windows
    #[cfg_attr(not(windows), ignore)]
    fn test_find_running_process() {
        let process = "cargo.exe";
        let result = unsafe { find_running_process(process) };

        match result {
            Err(e) => {
                eprintln!("Finding process returned an error: {e}");
                panic!("Finding process returned an error");
            }
            Ok(false) => panic!("Could not find running process"),
            _ => {}
        }
    }
}
