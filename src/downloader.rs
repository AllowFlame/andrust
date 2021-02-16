use std::cell::{RefCell, RefMut};
use std::fs;
use std::io::{stdout, Stdout, Write};

use serde::{Deserialize, Serialize};

use crossterm::{cursor, QueueableCommand};
use webcraft::{
    hyper::Body, hyper::Request, hyper::Uri, Craft, CraftError, CraftResult, SaveFileObserver,
};

struct DownloadIndicator {
    stdout: RefCell<Stdout>,
}

impl Default for DownloadIndicator {
    fn default() -> Self {
        let mut stdout = stdout();
        let _ = stdout.queue(cursor::SavePosition);

        DownloadIndicator {
            stdout: RefCell::new(stdout),
        }
    }
}

impl SaveFileObserver for DownloadIndicator {
    fn on_save(&self, file: &fs::File) {
        let _ = file.metadata().map(|meta| {
            let stdout = self.stdout.borrow_mut();
            let mut stdout: RefMut<Stdout> = RefMut::map(stdout, |t| t);
            let _ = stdout.write(format!("size : {}", meta.len()).as_bytes());
            let _ = stdout.queue(cursor::RestorePosition);
            let _ = stdout.flush();
        });
    }
}

pub struct Downloader {
    craft: Craft,
}

impl Default for Downloader {
    fn default() -> Self {
        let craft = Craft::default();
        Downloader { craft }
    }
}

impl Downloader {
    #[tokio::main]
    pub async fn download(&self, uri: Uri, file_name: &str) -> CraftResult<()> {
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
            .map_err(|_err| CraftError::HyperConnector)?;

        let body = self
            .craft
            .visit(req, &|response| Result::Ok(response))
            .await?
            .into_body();

        let indicator = DownloadIndicator::default();
        Craft::save_body(body, file_name, Some(Box::new(indicator))).await?;

        Result::Ok(())
    }
}

pub struct BuildPlatformConfig {
    download_url: Option<Vec<String>>,
}

impl Default for BuildPlatformConfig {
    #[cfg(target_os = "windows")]
    fn default() -> Self {
        let windows = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-windows-x86_64.zip"
                .to_owned(),
        ];
        BuildPlatformConfig {
            download_url: Some(windows),
        }
    }

    #[cfg(target_os = "macos")]
    fn default() -> Self {
        let windows = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-darwin-x86_64.zip"
                .to_owned(),
        ];
        BuildPlatformConfig {
            download_url: Some(windows),
        }
    }

    #[cfg(target_os = "linux")]
    fn default() -> Self {
        let windows = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-linux-x86_64.zip".to_owned(),
        ];
        BuildPlatformConfig {
            download_url: Some(windows),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadConfig {
    windows: Option<Vec<String>>,
    macos: Option<Vec<String>>,
    linux: Option<Vec<String>>,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        let windows = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-windows-x86_64.zip"
                .to_owned(),
        ];
        let macos = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-darwin-x86_64.zip"
                .to_owned(),
        ];
        let linux = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-linux-x86_64.zip".to_owned(),
        ];

        DownloadConfig {
            windows: Some(windows),
            macos: Some(macos),
            linux: Some(linux),
        }
    }
}

impl DownloadConfig {
    pub fn windows(&self) -> Option<&str> {
        self.windows.as_ref().and_then(|vec| Some(vec[0].as_str()))
    }

    pub fn macos(&self) -> Option<&str> {
        self.macos.as_ref().and_then(|vec| Some(vec[0].as_str()))
    }

    pub fn linux(&self) -> Option<&str> {
        self.linux.as_ref().and_then(|vec| Some(vec[0].as_str()))
    }
}
