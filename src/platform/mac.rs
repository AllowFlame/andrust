use std::collections::HashSet;
use std::env;
use std::format;

use super::{CargoConfig, Platform, TargetPlatformToolset, ToolSetConfig};

pub struct MacConfig;

impl Platform for MacConfig {
    fn search_ndk_root_path() -> Option<String> {
        use std::path::Path;

        env::var("NDK_TOOL_ROOT")
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

    fn determine_ndk_path(&self) -> String {
        let root_path = MacConfig::search_ndk_root_path().or_else(|| {
            use std::path::Path;

            let path = MacConfig::ask_root_path();
            if Path::new(path.as_str()).exists() {
                Some(path)
            } else {
                None
            }
        });

        //TODO: ask downloading
        match root_path {
            Some(path) => path,
            None => "".to_owned(),
        }
    }

    fn setup_toolsets() -> HashSet<TargetPlatformToolset> {
        let aarch64_ar = "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar";
        let aarch64_linker =
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang";
        let aarch64 = TargetPlatformToolset::Aarch64(
            "aarch64-linux-android",
            aarch64_ar.to_owned(),
            aarch64_linker.to_owned(),
        );

        let armv7_ar = "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar";
        let armv7_linker =
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi16-clang";
        let armv7 = TargetPlatformToolset::Armv7(
            "armv7-linux-androideabi",
            armv7_ar.to_owned(),
            armv7_linker.to_owned(),
        );

        let i686_ar = "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android-ar";
        let i686_linker =
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android16-clang";
        let i686 = TargetPlatformToolset::I686(
            "i686-linux-android",
            i686_ar.to_owned(),
            i686_linker.to_owned(),
        );

        let mut toolsets = HashSet::new();
        toolsets.insert(aarch64);
        toolsets.insert(armv7);
        toolsets.insert(i686);
        toolsets
    }

    fn setup_config(&self, root_path: &str) {
        let aarch64_ar = format!(
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar",
            root_path
        );
        let aarch64_linker = format!(
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang",
            root_path
        );
        let aarch64 = ToolSetConfig::new(
            "aarch64-linux-android",
            aarch64_ar.as_str(),
            aarch64_linker.as_str(),
        );

        let armv7_ar = format!(
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar",
            root_path
        );
        let armv7_linker = format!(
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi16-clang",
            root_path
        );
        let armv7 = ToolSetConfig::new(
            "armv7-linux-androideabi",
            armv7_ar.as_str(),
            armv7_linker.as_str(),
        );

        let i686_ar = format!(
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android-ar",
            root_path
        );
        let i686_linker = format!(
            "{}/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android16-clang",
            root_path
        );
        let i686 = ToolSetConfig::new("i686-linux-android", i686_ar.as_str(), i686_linker.as_str());

        let mut toolsets = Vec::new();
        toolsets.push(aarch64);
        toolsets.push(armv7);
        toolsets.push(i686);

        let config = CargoConfig::new(toolsets);
        config.write();
    }
}
