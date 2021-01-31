use std::collections::HashMap;

pub struct Command {
    root: Option<String>,
    name: Option<String>,
    package: Option<String>,
    version: Option<String>,
    offset: Option<i32>,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            root: None,
            name: None,
            package: None,
            version: None,
            offset: None,
        }
    }
}

impl Command {
    pub fn from(command_map: HashMap<String, String>) -> Option<Self> {
        let mut root: Option<String> = None;
        let mut name: Option<String> = None;
        let mut package: Option<String> = None;
        let mut version: Option<String> = None;
        let mut offset: Option<i32> = None;

        for (opt, obj) in command_map {
            match opt.as_str() {
                "-r" | "--root" => root = Some(obj),
                "-n" | "--name" => name = Some(obj),
                "-p" | "--package" => package = Some(obj),
                "-v" | "--version" => version = Some(obj),
                "-o" | "--offset" => offset = Some(obj.as_str().parse::<i32>().unwrap_or(0)),
                _ => (),
            }
        }

        if name == None && package == None && version == None && offset == None {
            return None;
        }

        Some(Command {
            root,
            name,
            package,
            version,
            offset,
        })
    }

    pub fn root(&self) -> Option<&String> {
        self.root.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn package(&self) -> Option<&String> {
        self.package.as_ref()
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn offset(&self) -> Option<i32> {
        self.offset
    }

    pub fn _consume_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn consume_package(&mut self, package: String) {
        self.package = Some(package);
    }

    pub fn consume_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn consume_offset(&mut self, offset: i32) {
        self.offset = Some(offset);
    }
}

pub fn parse_args() -> HashMap<String, String> {
    use std::env;

    let args = env::args();

    let mut commands = HashMap::new();
    let mut opt: Option<String> = None;
    for arg in args {
        if opt == None && arg.as_str().starts_with("-") {
            opt = Some(arg);
        } else if opt != None {
            commands.insert(opt.unwrap(), arg);
            opt = None;
        }
    }

    commands
}

pub fn show_help() {
    println!(
        r#"
andrust is a helper to set cross compilation configuration for rust project
USAGE:
    rn_renamer [OPTIONS] [OBJECT]
OPTIONS:
    -r, --root              Set RN root directory, default path is .
    -n, --name              Set application Name
    -p, --package           Set application package name
    -v, --version           Set version name
    -o, --offset            Set version code offset
    -h, --help              Prints help information
    "#
    );
}
