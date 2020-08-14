mod downloader;
mod platform;

use downloader::Downloader;
use platform::Platform;

fn main() {
    // let platform = platform();
    // platform.setup();
}

#[cfg(target_os = "windows")]
fn platform() -> impl Platform {
    platform::WinConfig
}

#[cfg(target_os = "macos")]
fn platform() -> impl Platform {
    platform::MacConfig
}
