use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::exit,
};

pub struct Command {
    root: Option<PathBuf>,
    ndk_home: Option<PathBuf>,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            root: None,
            ndk_home: None,
        }
    }
}

impl Command {
    pub fn new() -> Self {
        let args = Command::parse_args();
        Command::from(args)
    }

    pub fn from(command_map: HashMap<String, String>) -> Self {
        let mut root: Option<PathBuf> = None;
        let mut ndk_home: Option<PathBuf> = None;

        for (opt, obj) in command_map {
            match opt.as_str() {
                "-r" | "--root" => root = Some(PathBuf::from(obj.as_str())),
                "-n" | "--ndk" => ndk_home = Some(PathBuf::from(obj.as_str())),
                "-v" | "--version" => {
                    show_version();
                    break;
                }
                "-h" | "--help" => {
                    show_help();
                    break;
                }
                _ => (),
            }
        }

        Command { root, ndk_home }
    }

    fn parse_args() -> HashMap<String, String> {
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

        if opt != None {
            commands.insert(opt.unwrap(), "".to_owned());
        }

        commands
    }

    pub fn root(&self) -> Option<&Path> {
        self.root.as_ref().map(|root| root.as_path())
    }

    pub fn ndk_home(&self) -> Option<&Path> {
        self.ndk_home.as_ref().map(|home| home.as_path())
    }
}

pub fn show_version() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!(
        r#"andrust {}
        "#,
        VERSION
    );
}

pub fn show_help() {
    println!(
        r#"andrust is a helper to set up android cross compilation configuration for rust project

USAGE:
    andrust [OPTIONS] [OBJECT]

OPTIONS:
    -r, --root              Set rust project root directory, default path is .
    -h, --home              Set NDK home directory
    -v, --version           Prints version information
    -h, --help              Prints help information
    "#
    );
}
