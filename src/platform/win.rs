use std::{
    self,
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};

use super::{
    super::command::CommandOptions, ConfigWriter, Platform, PlatformError, PlatformResult,
    PlatformToolset, TargetPlatform,
};

pub struct WinConfig {
    targets: HashSet<TargetPlatform>,
    cmd_opts: Option<CommandOptions>,
}

impl Platform for WinConfig {
    fn search_ndk_root() -> Option<PathBuf> {
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
            .map(|ndk_root| PathBuf::from(ndk_root.as_str()))
    }

    fn determine_ndk_root(&self) -> PlatformResult<PathBuf> {
        let input_ndk_root = self
            .cmd_opts
            .as_ref()
            .and_then(|cmd_opt| cmd_opt.ndk_root());

        if input_ndk_root.is_some()
            && WinConfig::does_toolsets_exist(input_ndk_root.unwrap(), self.targets())
        {
            let verified_ndk_root = input_ndk_root.unwrap().to_path_buf();
            return Ok(verified_ndk_root);
        } else {
            println!("input ndk root is not verified, ndk root candidates are being searched.");
        }

        WinConfig::search_ndk_root()
            .or_else(|| {
                let user_input_path = WinConfig::ask_ndk_root();
                let path = PathBuf::from(user_input_path.as_str());
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            })
            .and_then(|path| {
                let toolsets = self.targets();
                let does_exist = WinConfig::does_toolsets_exist(path.as_path(), toolsets);
                if does_exist {
                    Some(path)
                } else {
                    None
                }
            })
            .ok_or(PlatformError::ToolsetDoesNotExist)
    }

    fn targets(&self) -> &HashSet<TargetPlatform> {
        &self.targets
    }

    fn setup_config(self, ndk_root: &Path, proj_root: Option<PathBuf>) {
        use std::iter::FromIterator;

        let toolsets = self
            .targets
            .into_iter()
            .map(|target| target.add_ndk_root(ndk_root))
            .filter(|target_adding_result| target_adding_result.is_ok())
            .map(|filtered_target| filtered_target.unwrap());
        let toolsets = HashSet::from_iter(toolsets);

        let writer = ConfigWriter::new(&toolsets);
        writer.write(proj_root);
    }

    fn ask_ndk_root() -> String {
        use std::io::{stdin, stdout, Write};

        let mut user_input = String::new();
        println!(r#"Can't find NDK root path. System variable "NDK_TOOL_ROOT" is not set."#);
        print!("Please enter NDK root path: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a correct string");
        if let Some('\n') = user_input.chars().next_back() {
            user_input.pop();
        }
        if let Some('\r') = user_input.chars().next_back() {
            user_input.pop();
        }
        println!("You typed: {}", user_input);

        user_input
    }

    fn does_toolsets_exist(ndk_root: &Path, platform_toolsets: &HashSet<TargetPlatform>) -> bool {
        let mut does_all_exist = true;
        for target_toolset in platform_toolsets {
            let toolsets = target_toolset.to_platform_toolset();
            let ndk_root = match ndk_root.to_str() {
                Some(path) => path,
                None => return false,
            };
            let ar_path = format!("{}/{}", ndk_root, toolsets.ar());
            let linker_path = format!("{}/{}", ndk_root, toolsets.linker());

            does_all_exist = does_all_exist
                && Path::new(ar_path.as_str()).exists()
                && Path::new(linker_path.as_str()).exists();

            if !does_all_exist {
                break;
            }
        }
        does_all_exist
    }

    fn get_latest_folder_name(root_path: &str) -> Option<String> {
        let root_path = Path::new(root_path);
        if !root_path.is_dir() {
            return None;
        }

        let mut latest_folder = None;
        for entry in std::fs::read_dir(root_path).ok()? {
            let entry = entry.ok()?;
            let entry_mod_time = entry.metadata().ok()?.modified().ok()?;

            let latest_folder_mod_time = latest_folder
                .as_ref()
                .and_then(|folder_name: &String| {
                    Path::new(folder_name.as_str())
                        .metadata()
                        .ok()?
                        .modified()
                        .ok()
                })
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

            if latest_folder_mod_time < entry_mod_time {
                latest_folder = entry
                    .path()
                    .as_path()
                    .to_str()
                    .map(|folder_name| folder_name.to_owned());
            }
        }

        latest_folder
    }

    //TODO: implement download logic
    fn download_ndk() {}
}

impl Default for WinConfig {
    fn default() -> Self {
        let toolsets = WinConfig::get_toolsets();
        WinConfig {
            targets: toolsets,
            cmd_opts: None,
        }
    }
}

impl WinConfig {
    pub fn new(cmd_opts: Option<CommandOptions>) -> Self {
        let toolsets = WinConfig::get_toolsets();
        WinConfig {
            targets: toolsets,
            cmd_opts,
        }
    }

    fn get_toolsets() -> HashSet<TargetPlatform> {
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

        toolsets
    }
}
