use std::cell::{RefCell, RefMut};
use std::fs;
use std::io::{stdout, Stdout, Write};

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
        let macos = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-darwin-x86_64.zip"
                .to_owned(),
        ];
        BuildPlatformConfig {
            download_url: Some(macos),
        }
    }

    #[cfg(target_os = "linux")]
    fn default() -> Self {
        let linux = vec![
            "https://dl.google.com/android/repository/android-ndk-r21b-linux-x86_64.zip".to_owned(),
        ];
        BuildPlatformConfig {
            download_url: Some(linux),
        }
    }
}

impl BuildPlatformConfig {
    pub fn download_url(&self) -> Option<&str> {
        self.download_url
            .as_ref()
            .and_then(|vec| Some(vec[0].as_str()))
    }
}
