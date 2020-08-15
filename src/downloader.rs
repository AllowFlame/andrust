use hyper::client::HttpConnector;
use hyper::Uri;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;

pub struct Downloader {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Default for Downloader {
    fn default() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);
        Downloader { client }
    }
}

impl Downloader {
    #[tokio::main]
    pub async fn download(&self, uri: Uri, file_name: &str) -> hyper::Result<()> {
        let resp = (&self.client).get(uri).await?;
        let body = resp.into_body();
        Downloader::save_body(body, file_name).await;
        hyper::Result::Ok(())
    }

    /*
    use downloader::Downloader;

    let downloader = Downloader::default();
    let _ = downloader.download(
        "https://dl.google.com/android/repository/android-ndk-r21b-windows-x86_64.zip"
            .parse()
            .unwrap(),
        "ndk.zip",
    );
    */
    pub async fn save_body(mut body: Body, file_name: &str) {
        use hyper::body::HttpBody;
        use std::fs;
        use std::io::{stdout, Error, ErrorKind, Write};
        use std::path::PathBuf;

        use crossterm::{cursor, QueueableCommand};

        let path = PathBuf::from(file_name);
        path.parent().and_then(|parent_path| {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path).ok()
            } else {
                Some(())
            }
        });

        let mut stdout = stdout();
        let _ = stdout.queue(cursor::SavePosition);

        let mut file = fs::File::create(file_name).expect("file error");
        'stream: while let Some(piece) = body.data().await {
            let save_result = piece
                .map_err(|_err| Error::new(ErrorKind::Other, "response body chunk error"))
                .and_then(|chunk| file.write_all(&chunk));

            let _ = file.metadata().map(|meta| {
                let _ = stdout.write(format!("size : {}", meta.len()).as_bytes());
                let _ = stdout.queue(cursor::RestorePosition);
                let _ = stdout.flush();
            });

            match save_result {
                Ok(_) => continue,
                Err(_err) => break 'stream,
            }
        }
    }
}
