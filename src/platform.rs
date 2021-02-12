mod linux;
mod mac;
mod win;

use std::{
    self,
    collections::HashSet,
    fmt, format,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub use linux::LinuxConfig;
pub use mac::MacConfig;
pub use win::WinConfig;

pub trait Platform {
    fn search_ndk_root() -> Option<PathBuf>;
    fn determine_ndk_root(&self) -> PlatformResult<PathBuf>;
    fn targets(&self) -> &HashSet<TargetPlatform>;
    fn setup_config(self, ndk_root: &Path, proj_root: Option<PathBuf>);

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
                .unwrap_or(SystemTime::UNIX_EPOCH);

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

    fn download_ndk() {}
}

#[derive(PartialEq, Eq, Hash)]
pub struct PlatformToolset {
    target: &'static str,
    ar: String,
    linker: String,
}

impl PlatformToolset {
    pub fn new(target: &'static str, ar: String, linker: String) -> Self {
        PlatformToolset { target, ar, linker }
    }

    pub fn clone_with_ndk_root(self, ndk_root: &Path) -> PlatformResult<Self> {
        let root = ndk_root.to_str().ok_or(PlatformError::WrongPathName)?;
        Ok(PlatformToolset {
            target: self.target,
            ar: format!("{}/{}", root, self.ar()),
            linker: format!("{}/{}", root, self.linker()),
        })
    }

    pub fn format_target(&self) -> String {
        format!(
            r#"
[target.{}]
ar = "{}"
linker = "{}"
"#,
            self.target(),
            self.ar(),
            self.linker()
        )
    }

    pub fn target(&self) -> &str {
        self.target
    }

    pub fn ar(&self) -> &str {
        self.ar.as_str()
    }

    pub fn linker(&self) -> &str {
        self.linker.as_str()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TargetPlatform {
    Aarch64(PlatformToolset),
    Armv7(PlatformToolset),
    I686(PlatformToolset),
}

impl TargetPlatform {
    pub fn add_ndk_root(self, root_path: &Path) -> PlatformResult<Self> {
        match self {
            TargetPlatform::Aarch64(aarch64) => Ok(TargetPlatform::Aarch64(
                aarch64.clone_with_ndk_root(root_path)?,
            )),
            TargetPlatform::Armv7(armv7) => {
                Ok(TargetPlatform::Armv7(armv7.clone_with_ndk_root(root_path)?))
            }
            TargetPlatform::I686(i686) => {
                Ok(TargetPlatform::I686(i686.clone_with_ndk_root(root_path)?))
            }
        }
    }

    pub fn to_platform_toolset(&self) -> &PlatformToolset {
        match &self {
            TargetPlatform::Aarch64(aarch64) => aarch64,
            TargetPlatform::Armv7(armv7) => armv7,
            TargetPlatform::I686(i686) => i686,
        }
    }
}

pub struct ConfigWriter<'a> {
    toolsets: &'a HashSet<TargetPlatform>,
}

impl<'a> ConfigWriter<'a> {
    pub fn new(toolsets: &'a HashSet<TargetPlatform>) -> Self {
        ConfigWriter { toolsets }
    }

    fn content(&self) -> String {
        let mut config_content = String::new();

        for target_toolset in self.toolsets {
            let toolset = target_toolset.to_platform_toolset();
            config_content.push_str(toolset.format_target().as_str());
        }
        config_content
    }

    pub fn write(self, proj_root: Option<PathBuf>) {
        use std::fs;
        use std::io::Write;

        let file_name = ".cargo/config";
        let path = proj_root
            .map(|mut root| {
                root.push("/");
                root.push(file_name);
                root
            })
            .unwrap_or(PathBuf::from(file_name));
        path.parent().and_then(|parent_path| {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path).ok()
            } else {
                Some(())
            }
        });

        let mut file = fs::File::create(file_name).expect("file error");

        let content = self.content();
        file.write_all(content.as_bytes()).unwrap();
    }
}

type PlatformResult<T> = Result<T, PlatformError>;

#[derive(Debug)]
pub enum PlatformError {
    ToolsetDoesNotExist,
    WrongPathName,
}

impl fmt::Display for PlatformError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlatformError::ToolsetDoesNotExist => write!(formatter, "ToolsetDoesNotExist"),
            PlatformError::WrongPathName => write!(formatter, "WrongPathName"),
        }
    }
}
