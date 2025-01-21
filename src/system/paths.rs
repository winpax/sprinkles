use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
#[allow(clippy::enum_variant_names, dead_code)]
/// This is a non-exhaustive list CSIDLs for Windows defined paths
pub enum Paths {
    /// System wide application data
    ///
    /// `CommonAppData` on Windows
    CommonAppData,
    /// Persistent application data for the current user
    ///
    /// `AppData` on Windows
    AppData,
    /// Non-persistent application data for the current user
    ///
    /// `LocalAppData` on Windows
    LocalAppData,
}

#[cfg(windows)]
impl Paths {
    pub(crate) fn as_csidl(self) -> u32 {
        use windows::Win32::UI::Shell::{CSIDL_APPDATA, CSIDL_COMMON_APPDATA, CSIDL_LOCAL_APPDATA};

        match self {
            Paths::CommonAppData => CSIDL_COMMON_APPDATA,
            Paths::AppData => CSIDL_APPDATA,
            Paths::LocalAppData => CSIDL_LOCAL_APPDATA,
        }
    }

    pub fn into_path(self) -> Option<PathBuf> {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};

        use windows::Win32::{Foundation::MAX_PATH, UI::Shell::SHGetSpecialFolderPathW};

        let mut buf = [0u16; MAX_PATH as usize];
        let success = unsafe {
            #[allow(clippy::cast_possible_wrap)]
            SHGetSpecialFolderPathW(None, &mut buf, self.as_csidl() as i32, true).as_bool()
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

#[cfg(not(windows))]
impl Paths {
    #[allow(clippy::unused_self)]
    pub fn into_path(self) -> Option<PathBuf> {
        use std::env;

        match self {
            Paths::CommonAppData => Some(PathBuf::from("/usr/share")),
            Paths::AppData => env::var_os("XDG_DATA_HOME").map(PathBuf::from).or_else(|| {
                env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share"))
            }),
            Paths::LocalAppData => env::var_os("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache"))),
        }
    }
}
