mod linux;
mod mac;
mod win;

use std::format;

pub use linux::LinuxConfig;
pub use mac::MacConfig;
pub use win::WinConfig;

pub trait Platform {
    fn search_ndk_root_path() -> Option<String>;
    fn determine_ndk_path(&self) -> String;
    fn setup_config(&self, ndk_path: &str);

    fn ask_rpath() -> String {
        use std::io::{stdin, stdout, Write};

        let mut user_input = String::new();
        println!("Can't find NDK root path.");
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

    fn download_ndk() {}
}

pub struct ToolSetConfig<'a> {
    target: &'a str,
    ar: &'a str,
    linker: &'a str,
}

impl<'a> ToolSetConfig<'a> {
    pub fn new(target: &'a str, ar: &'a str, linker: &'a str) -> Self {
        ToolSetConfig { target, ar, linker }
    }

    pub fn toolset_format(&self) -> String {
        format!(
            r#"
[target.{}]
ar = "{}"
linker = "{}"
"#,
            self.target, self.ar, self.linker
        )
    }
}

pub struct CargoConfig<'a> {
    sets: Vec<ToolSetConfig<'a>>,
}

impl<'a> CargoConfig<'a> {
    pub fn new(tool_sets: Vec<ToolSetConfig<'a>>) -> Self {
        CargoConfig { sets: tool_sets }
    }

    fn content(&self) -> String {
        let mut config_content = String::new();

        for set in &self.sets {
            config_content.push_str(set.toolset_format().as_str());
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
