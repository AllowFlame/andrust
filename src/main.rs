mod downloader;
mod platform;
mod unarchiver;

use platform::Platform;

#[cfg(test)]
mod downloader_test;
#[cfg(test)]
mod unarchiver_test;

fn main() {
    let mut platform = platform();
    let ndk_path = platform.determine_ndk_path();

    //TODO: download toolset
    let ndk_path = ndk_path.unwrap();
    platform.setup_config(ndk_path.as_str());
}

#[cfg(target_os = "windows")]
fn platform() -> impl Platform {
    platform::WinConfig::new()
}

#[cfg(target_os = "macos")]
fn platform() -> impl Platform {
    platform::MacConfig
}

#[cfg(target_os = "linux")]
fn platform() -> impl Platform {
    platform::LinuxConfig
}
