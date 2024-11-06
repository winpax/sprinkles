use std::path::Path;

pub struct Unix;

impl super::Common for Unix {
    fn symlink_dir(original: impl AsRef<Path>, link: impl AsRef<Path>) -> std::io::Result<()> {
        std::os::windows::fs::symlink_dir(original, link)?;
    }
}
