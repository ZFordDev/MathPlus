#[cfg(windows)]
pub fn play_click_sound() {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC, SND_NODEFAULT};
    use windows::core::PCWSTR;

    let alias: Vec<u16> = OsStr::new("SystemAsterisk")
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let _ = PlaySoundW(
            PCWSTR(alias.as_ptr()),
            None,
            SND_ALIAS | SND_ASYNC | SND_NODEFAULT,
        );
    }
}

#[cfg(not(windows))]
pub fn play_click_sound() {}
