mod command;
mod config;
mod downloader;
mod platform;
mod unarchiver;

use command::{CommandOptions, CommandState};
use platform::Platform;

#[cfg(test)]
mod downloader_test;
#[cfg(test)]
mod unarchiver_test;

fn main() {
    let cmd_opts = match CommandState::new() {
        CommandState::Options(command) => command,
        CommandState::ExitWithPrint => return,
    };

    let platform = platform(cmd_opts);
    let ndk_path = platform.determine_ndk_root();
    println!("ndk_path : {:?}", &ndk_path);

    //TODO: download toolset
    platform.setup_config(ndk_path.unwrap().as_path());
    // rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
}

#[cfg(target_os = "windows")]
fn platform(cmd_opts: CommandOptions) -> impl Platform {
    platform::WinConfig::new(Some(cmd_opts))
}

#[cfg(target_os = "macos")]
fn platform(cmd_opts: CommandOptions) -> impl Platform {
    platform::MacConfig::new(Some(cmd_opts))
}

#[cfg(target_os = "linux")]
fn platform(cmd_opts: CommandOptions) -> impl Platform {
    platform::LinuxConfig::new(Some(cmd_opts))
}
