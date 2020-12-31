mod downloader;
mod platform;
mod unarchiver;

use platform::Platform;

#[cfg(test)]
mod downloader_test;
#[cfg(test)]
mod unarchiver_test;

fn main() {
    let platform = platform();
    let ndk_path = platform.determine_ndk_path();
    platform.setup_config(ndk_path.as_str());
}

#[cfg(target_os = "windows")]
fn platform() -> impl Platform {
    platform::WinConfig
}

#[cfg(target_os = "macos")]
fn platform() -> impl Platform {
    platform::MacConfig
}

#[cfg(target_os = "linux")]
fn platform() -> impl Platform {
    platform::LinuxConfig
}
