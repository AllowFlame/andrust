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

    pub async fn save_body(mut body: Body, file_name: &str) {
        use hyper::body::HttpBody;
        use std::fs;
        use std::io::Write;
        use std::io::{Error, ErrorKind};
        use std::path::PathBuf;

        let path = PathBuf::from(file_name);
        path.parent().and_then(|parent_path| {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path).ok()
            } else {
                Some(())
            }
        });

        let mut file = fs::File::create(file_name).expect("file error");
        'stream: while let Some(piece) = body.data().await {
            let save_result = piece
                .map_err(|_err| Error::new(ErrorKind::Other, "response body chunk error"))
                .and_then(|chunk| file.write_all(&chunk));

            match save_result {
                Ok(_) => continue,
                Err(_err) => break 'stream,
            }
        }
    }
}
