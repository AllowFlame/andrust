use std::env;
use std::format;

use super::{CargoConfig, Platform, ToolSetConfig};

pub struct MacConfig;

impl Platform for MacConfig {
    fn search_rpath() -> Option<String> {
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
        let root_path = MacConfig::search_rpath().or_else(|| {
            use std::path::Path;

            let path = MacConfig::ask_rpath();
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
