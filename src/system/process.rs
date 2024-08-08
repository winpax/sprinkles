use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use windows::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    },
};

fn get_compare_string(exe_file: &[u16]) -> String {
    let os_string = OsString::from_wide(exe_file);
    let utf8_string = os_string.to_string_lossy();
    let trimmed = utf8_string.trim_end_matches('\0');

    dbg!(trimmed.to_string())
}

pub unsafe fn find_running_process(process: &str) -> windows::core::Result<bool> {
    let mut proc_running = false;

    let h_process_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? };

    if h_process_snap == INVALID_HANDLE_VALUE {
        proc_running = false;
    } else {
        #[allow(clippy::cast_possible_truncation)]
        let mut pe32 = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if unsafe { Process32FirstW(h_process_snap, &mut pe32) }.is_ok() {
            let compare = get_compare_string(&pe32.szExeFile);

            if compare == process {
                proc_running = true;
            } else {
                while unsafe { Process32NextW(h_process_snap, &mut pe32) }.is_ok() {
                    let compare = get_compare_string(&pe32.szExeFile);

                    if compare == process {
                        proc_running = true;
                        break;
                    }
                }
            }
            unsafe { CloseHandle(h_process_snap)? };
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
        assert!(result.is_ok(), "Finding process returned an error");
        assert!(result.unwrap(), "Could not find running process");
    }
}
