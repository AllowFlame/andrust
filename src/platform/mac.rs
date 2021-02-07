use std::{
    collections::HashSet,
    env, format,
    path::{Path, PathBuf},
};

use super::{
    ConfigWriter, Platform, PlatformError, PlatformResult, PlatformToolset, TargetPlatform,
};

pub struct MacConfig {
    targets: HashSet<TargetPlatform>,
}

impl Platform for MacConfig {
    fn search_ndk_root() -> Option<String> {
        env::var("NDK_TOOL_ROOT")
            .or(env::var("ANDROID_HOME")
                .map(|sdk_root| format!("{}/ndk-bundle", sdk_root.as_str())))
            .or(env::var("HOME")
                .map(|home_path| format!("{}/Library/Android/sdk/ndk-bundle", home_path.as_str())))
            .ok()
            .and_then(|path| {
                if Path::new(path.as_str()).exists() {
                    Some(path)
                } else {
                    None
                }
            })
    }

    fn determine_ndk_root(&self) -> PlatformResult<String> {
        let root_path = MacConfig::search_ndk_root()
            .or_else(|| {
                let path = MacConfig::ask_ndk_root();
                if Path::new(path.as_str()).exists() {
                    Some(path)
                } else {
                    None
                }
            })
            .and_then(|path| {
                let toolsets = self.targets();
                let does_exist = MacConfig::does_toolsets_exist(path.as_str(), toolsets);
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

    fn setup_config(self, root_path: &str, proj_root: Option<PathBuf>) {
        use std::iter::FromIterator;

        let toolsets = self
            .targets
            .into_iter()
            .map(|target| target.add_ndk_root(root_path));
        let toolsets = HashSet::from_iter(toolsets);

        let writer = ConfigWriter::new(&toolsets);
        writer.write(None);
    }
}

impl MacConfig {
    pub fn new() -> Self {
        let aarch64_ar = "toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar";
        let aarch64_linker =
            "toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang";
        let aarch64 = PlatformToolset::new(
            "aarch64-linux-android",
            aarch64_ar.to_owned(),
            aarch64_linker.to_owned(),
        );

        let armv7_ar = "toolchains/llvm/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar";
        let armv7_linker =
            "toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi16-clang";
        let armv7 = PlatformToolset::new(
            "armv7-linux-androideabi",
            armv7_ar.to_owned(),
            armv7_linker.to_owned(),
        );

        let i686_ar = "toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android-ar";
        let i686_linker = "toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android16-clang";
        let i686 = PlatformToolset::new(
            "i686-linux-android",
            i686_ar.to_owned(),
            i686_linker.to_owned(),
        );

        let mut toolsets = HashSet::new();
        toolsets.insert(TargetPlatform::Aarch64(aarch64));
        toolsets.insert(TargetPlatform::Armv7(armv7));
        toolsets.insert(TargetPlatform::I686(i686));

        MacConfig { targets: toolsets }
    }
}
