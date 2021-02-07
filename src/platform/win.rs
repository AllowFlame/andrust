use std::{
    self,
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};

use super::{
    ConfigWriter, Platform, PlatformError, PlatformResult, PlatformToolset, TargetPlatform,
};

pub struct WinConfig {
    targets: HashSet<TargetPlatform>,
}

impl Platform for WinConfig {
    fn search_ndk_root() -> Option<String> {
        env::var("NDK_TOOL_ROOT")
            .ok()
            .and_then(|ndk_root| {
                if Path::new(ndk_root.as_str()).exists() {
                    Some(ndk_root)
                } else {
                    None
                }
            })
            .or_else(|| {
                env::var("ANDROID_HOME").ok().and_then(|sdk_root| {
                    let ndk_root = format!("{}/ndk", sdk_root.as_str());
                    WinConfig::get_latest_folder_name(ndk_root.as_str())
                })
            })
    }

    fn determine_ndk_root(&self) -> PlatformResult<String> {
        let ndk_root: Option<String> = WinConfig::search_ndk_root()
            .or_else(|| {
                let path = WinConfig::ask_ndk_root();
                if Path::new(path.as_str()).exists() {
                    Some(path)
                } else {
                    None
                }
            })
            .and_then(|path| {
                let toolsets = self.targets();
                let does_exist = WinConfig::does_toolsets_exist(path.as_str(), toolsets);
                if does_exist {
                    Some(path)
                } else {
                    None
                }
            });

        match ndk_root {
            Some(path) => Result::Ok(path),
            None => Result::Err(PlatformError::ToolsetDoesNotExist),
        }
    }

    fn targets(&self) -> &HashSet<TargetPlatform> {
        &self.targets
    }

    fn setup_config(self, ndk_root: &str, proj_root: Option<PathBuf>) {
        use std::iter::FromIterator;

        let toolsets = self
            .targets
            .into_iter()
            .map(|target| target.add_ndk_root(ndk_root));
        let toolsets = HashSet::from_iter(toolsets);

        let writer = ConfigWriter::new(&toolsets);
        writer.write(None);
    }
}

impl WinConfig {
    pub fn new() -> Self {
        let aarch64_ar = "toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android-ar.exe";
        let aarch64_linker =
            "toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android21-clang.cmd";
        let aarch64 = PlatformToolset::new(
            "aarch64-linux-android",
            aarch64_ar.to_owned(),
            aarch64_linker.to_owned(),
        );

        let armv7_ar = "toolchains/llvm/prebuilt/windows-x86_64/bin/arm-linux-androideabi-ar.exe";
        let armv7_linker =
            "toolchains/llvm/prebuilt/windows-x86_64/bin/armv7a-linux-androideabi16-clang.cmd";
        let armv7 = PlatformToolset::new(
            "armv7-linux-androideabi",
            armv7_ar.to_owned(),
            armv7_linker.to_owned(),
        );

        let i686_ar = "toolchains/llvm/prebuilt/windows-x86_64/bin/i686-linux-android-ar.exe";
        let i686_linker =
            "toolchains/llvm/prebuilt/windows-x86_64/bin/i686-linux-android16-clang.cmd";
        let i686 = PlatformToolset::new(
            "i686-linux-android",
            i686_ar.to_owned(),
            i686_linker.to_owned(),
        );

        let mut toolsets = HashSet::new();
        toolsets.insert(TargetPlatform::Aarch64(aarch64));
        toolsets.insert(TargetPlatform::Armv7(armv7));
        toolsets.insert(TargetPlatform::I686(i686));

        WinConfig { targets: toolsets }
    }
}
