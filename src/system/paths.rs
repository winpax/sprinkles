use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[allow(clippy::enum_variant_names, dead_code)]
/// This is a non-exhaustive list CSIDLs for Windows defined paths
pub enum WindowsPath {
    CommonAppData,
    AppData,
    LocalAppData,
}

impl WindowsPath {
    pub fn as_csidl(self) -> u32 {
        use windows::Win32::UI::Shell::{CSIDL_APPDATA, CSIDL_COMMON_APPDATA, CSIDL_LOCAL_APPDATA};

        match self {
            WindowsPath::CommonAppData => CSIDL_COMMON_APPDATA,
            WindowsPath::AppData => CSIDL_APPDATA,
            WindowsPath::LocalAppData => CSIDL_LOCAL_APPDATA,
        }
    }

    #[cfg(not(windows))]
    pub fn to_path(&self) -> windows::core::Result<PathBuf> {
        unimplemented!()
    }

    #[cfg(windows)]
    pub fn into_path(self) -> Option<PathBuf> {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};

        use windows::Win32::{
            Foundation::{HWND, MAX_PATH},
            UI::Shell::SHGetSpecialFolderPathW,
        };

        let mut buf = [0u16; MAX_PATH as usize];
        let success = unsafe {
            #[allow(clippy::cast_possible_wrap)]
            SHGetSpecialFolderPathW(HWND::default(), &mut buf, self.as_csidl() as i32, true)
                .as_bool()
        };

        if success {
            let string = OsString::from_wide(&buf);

            let utf8_string = string.to_string_lossy();
            let trimmed = utf8_string.trim_end_matches('\0');

            Some(PathBuf::from(trimmed))
        } else {
            None
        }
    }
}
