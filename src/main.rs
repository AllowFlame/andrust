mod command;
mod downloader;
mod platform;
mod unarchiver;

use command::{Command, CommandState};
use platform::Platform;

#[cfg(test)]
mod downloader_test;
#[cfg(test)]
mod unarchiver_test;

fn main() {
    let command = match CommandState::new() {
        CommandState::Options(command) => command,
        CommandState::ExitWithPrint => return,
    };

    let platform = platform();
    let ndk_path = platform.determine_ndk_root();
    println!("ndk_path : {:?}", &ndk_path);

    //TODO: download toolset
    // let ndk_path = ndk_path.unwrap();
    // platform.setup_config(ndk_path.as_str());
    // rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
}

#[cfg(target_os = "windows")]
fn platform() -> impl Platform {
    platform::WinConfig::new()
}

#[cfg(target_os = "macos")]
fn platform() -> impl Platform {
    platform::MacConfig::new()
}

#[cfg(target_os = "linux")]
fn platform() -> impl Platform {
    platform::LinuxConfig::new()
}
