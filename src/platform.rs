mod mac;
mod win;

use std::format;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub use win::WinConfig;

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
            r#"[target.{}]
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
        let content = self.content();

        let dest_path = Path::new(".cargo").join("config");
        let mut file = File::create(&dest_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}
