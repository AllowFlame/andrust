use std::{
    collections::HashSet,
    env, format,
    path::{Path, PathBuf},
};

use super::{
    super::command::CommandOptions, super::downloader, ConfigWriter, Platform, PlatformError,
    PlatformResult, PlatformToolset, TargetPlatform,
};

pub struct LinuxConfig {
    targets: HashSet<TargetPlatform>,
    cmd_opts: Option<CommandOptions>,
}

impl Platform for LinuxConfig {
    fn search_ndk_root() -> Option<PathBuf> {
        env::var("NDK_TOOL_ROOT")
            .or(env::var("ANDROID_HOME")
                .map(|sdk_root| format!("{}/ndk-bundle", sdk_root.as_str())))
            .or(env::var("HOME")
                .map(|home_path| format!("{}/tools/Android/sdk/ndk-bundle", home_path.as_str())))
            .ok()
            .and_then(|path| {
                if Path::new(path.as_str()).exists() {
                    Some(path)
                } else {
                    None
                }
            })
            .or_else(|| {
                env::var("ANDROID_HOME").ok().and_then(|path| {
                    let ndk_root = format!("{}/ndk", path.as_str());
                    LinuxConfig::get_latest_folder_name(ndk_root.as_str())
                })
            })
            .map(|ndk_root| PathBuf::from(ndk_root.as_str()))
    }

    fn determine_ndk_root(&self) -> PlatformResult<PathBuf> {
        let input_ndk_root = self
            .cmd_opts
            .as_ref()
            .and_then(|cmd_opt| cmd_opt.ndk_root());

        if input_ndk_root.is_some()
            && LinuxConfig::does_toolsets_exist(input_ndk_root.unwrap(), self.targets())
        {
            let verified_ndk_root = input_ndk_root.unwrap().to_path_buf();
            return Ok(verified_ndk_root);
        } else {
            println!("input ndk root is not verified, ndk root candidates are being searched.");
        }

        let root_path = LinuxConfig::search_ndk_root()
            .or_else(|| {
                let user_input_path = LinuxConfig::ask_ndk_root();
                let path = PathBuf::from(user_input_path.as_str());
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            })
            .and_then(|path| {
                let toolsets = self.targets();
                let does_exist = LinuxConfig::does_toolsets_exist(path.as_path(), toolsets);
                if does_exist {
                    Some(path)
                } else {
                    None
                }
            });

        //FIXME: following code is for downloading
        // let download = || {
        //     use crate::downloader::{BuildPlatformConfig, Downloader};

        //     let config = BuildPlatformConfig::default();
        //     let downloader = Downloader::default();
        //     let _ = downloader.download(config.download_url().unwrap().parse().unwrap(), "ndk.zip");

        //     env::var("HOME")
        //         .map(|home_path| format!("{}/tools", home_path.as_str()))
        //         .ok()
        // };

        root_path.ok_or(PlatformError::ToolsetDoesNotExist)
    }

    fn targets(&self) -> &HashSet<TargetPlatform> {
        &self.targets
    }

    fn setup_config(self, root_path: &Path) {
        use std::iter::FromIterator;

        let toolsets = self
            .targets
            .into_iter()
            .map(|target| target.add_ndk_root(root_path))
            .filter(|target_adding_result| target_adding_result.is_ok())
            .map(|filtered_target| filtered_target.unwrap());
        let toolsets = HashSet::from_iter(toolsets);

        let writer = ConfigWriter::new(&toolsets);
        writer.write(None);
    }

    //TODO: implement download logic
    fn download_ndk() {
        let downloader = downloader::Downloader::default();
        let _ = downloader.download(
            "https://dl.google.com/android/repository/android-ndk-r21b-linux-x86_64.zip"
                .parse()
                .unwrap(),
            "ndk.zip",
        );
    }
}

impl LinuxConfig {
    pub fn new(cmd_opts: Option<CommandOptions>) -> Self {
        let toolsets = LinuxConfig::get_toolsets();
        LinuxConfig {
            targets: toolsets,
            cmd_opts,
        }
    }

    fn get_toolsets() -> HashSet<TargetPlatform> {
        let aarch64_ar = "{}/toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android-ar";
        let aarch64_linker =
            "{}/toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android21-clang.cmd";
        let aarch64 = PlatformToolset::new(
            "aarch64-linux-android",
            aarch64_ar.to_owned(),
            aarch64_linker.to_owned(),
        );

        let armv7_ar = "{}/toolchains/llvm/prebuilt/windows-x86_64/bin/arm-linux-androideabi-ar";
        let armv7_linker =
            "{}/toolchains/llvm/prebuilt/windows-x86_64/bin/armv7a-linux-androideabi16-clang.cmd";
        let armv7 = PlatformToolset::new(
            "armv7-linux-androideabi",
            armv7_ar.to_owned(),
            armv7_linker.to_owned(),
        );

        let i686_ar = "{}/toolchains/llvm/prebuilt/windows-x86_64/bin/i686-linux-android-ar";
        let i686_linker =
            "{}/toolchains/llvm/prebuilt/windows-x86_64/bin/i686-linux-android16-clang.cmd";
        let i686 = PlatformToolset::new(
            "i686-linux-android",
            i686_ar.to_owned(),
            i686_linker.to_owned(),
        );

        let mut toolsets = HashSet::new();
        toolsets.insert(TargetPlatform::Aarch64(aarch64));
        toolsets.insert(TargetPlatform::Armv7(armv7));
        toolsets.insert(TargetPlatform::I686(i686));

        toolsets
    }
}
