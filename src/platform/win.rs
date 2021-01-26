use std::{collections::HashSet, env, path::Path};

use super::{
    ConfigWriter, Platform, PlatformError, PlatformResult, PlatformToolset, TargetPlatform,
};

pub struct WinConfig {
    targets: HashSet<TargetPlatform>,
}

impl Platform for WinConfig {
    fn search_ndk_root_path() -> Option<String> {
        env::var("NDK_TOOL_ROOT").ok().and_then(|path| {
            if Path::new(path.as_str()).exists() {
                Some(path)
            } else {
                None
            }
        })
        // .or_else(|| {
        //     env::var("ANDROID_HOME").ok().and_then(|path| {
        //         let ndk_root = format!("{}/ndk", path.as_str());
        //     })
        // })
    }

    fn determine_ndk_path(&self) -> PlatformResult<String> {
        let root_path = WinConfig::search_ndk_root_path()
            .or_else(|| {
                let path = WinConfig::ask_root_path();
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

        match root_path {
            Some(path) => Result::Ok(path),
            None => Result::Err(PlatformError::ToolsetDoesNotExist),
        }
    }

    fn targets(&self) -> &HashSet<TargetPlatform> {
        &self.targets
    }

    fn setup_config(self, root_path: &str) {
        use std::iter::FromIterator;

        let toolsets = self
            .targets
            .into_iter()
            .map(|target| target.add_root_path(root_path));
        let toolsets = HashSet::from_iter(toolsets);

        let writer = ConfigWriter::new(&toolsets);
        writer.write();
    }
}

impl WinConfig {
    pub fn new() -> Self {
        let aarch64_ar = "toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android-ar";
        let aarch64_linker =
            "toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android21-clang.cmd";
        let aarch64 = PlatformToolset::new(
            "aarch64-linux-android",
            aarch64_ar.to_owned(),
            aarch64_linker.to_owned(),
        );

        let armv7_ar = "toolchains/llvm/prebuilt/windows-x86_64/bin/arm-linux-androideabi-ar";
        let armv7_linker =
            "toolchains/llvm/prebuilt/windows-x86_64/bin/armv7a-linux-androideabi16-clang.cmd";
        let armv7 = PlatformToolset::new(
            "armv7-linux-androideabi",
            armv7_ar.to_owned(),
            armv7_linker.to_owned(),
        );

        let i686_ar = "toolchains/llvm/prebuilt/windows-x86_64/bin/i686-linux-android-ar";
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
