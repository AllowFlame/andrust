mod linux;
mod mac;
mod win;

use std::{collections::HashSet, fmt, format};

pub use linux::LinuxConfig;
pub use mac::MacConfig;
pub use win::WinConfig;

pub trait Platform {
    fn search_ndk_root_path() -> Option<String>;
    fn determine_ndk_path(&self) -> PlatformResult<String>;

    fn targets(&self) -> &HashSet<TargetPlatform>;

    fn setup_config(self, ndk_path: &str);

    fn ask_root_path() -> String {
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

    fn does_toolsets_exist(ndk_path: &str, platform_toolsets: &HashSet<TargetPlatform>) -> bool {
        use std::path::Path;

        let mut does_all_exist = false;
        for target_toolset in platform_toolsets {
            let toolsets = target_toolset.to_platform_toolset();
            let ar_path = format!("{}/{}", ndk_path, toolsets.ar());
            let linker_path = format!("{}/{}", ndk_path, toolsets.linker());

            does_all_exist = does_all_exist
                && Path::new(ar_path.as_str()).exists()
                && Path::new(linker_path.as_str()).exists();

            if !does_all_exist {
                break;
            }
        }
        does_all_exist
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

    pub fn clone_with_root_path(self, root_path: &str) -> Self {
        PlatformToolset {
            target: self.target,
            ar: format!("{}/{}", root_path, self.ar()),
            linker: format!("{}/{}", root_path, self.linker()),
        }
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
    pub fn add_root_path(self, root_path: &str) -> Self {
        match self {
            TargetPlatform::Aarch64(aarch64) => {
                TargetPlatform::Aarch64(aarch64.clone_with_root_path(root_path))
            }
            TargetPlatform::Armv7(armv7) => {
                TargetPlatform::Armv7(armv7.clone_with_root_path(root_path))
            }
            TargetPlatform::I686(i686) => {
                TargetPlatform::I686(i686.clone_with_root_path(root_path))
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

    pub fn write(self) {
        use std::fs;
        use std::io::Write;
        use std::path::PathBuf;

        let file_name = ".cargo/config";
        let path = PathBuf::from(file_name);
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
}

impl fmt::Display for PlatformError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlatformError::ToolsetDoesNotExist => write!(formatter, "ToolsetDoesNotExist"),
        }
    }
}
