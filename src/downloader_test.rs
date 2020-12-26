use super::downloader;

#[test]
fn downloader_test() {
    let downloader = downloader::Downloader::default();
    let _ = downloader.download(
        "https://dl.google.com/android/repository/android-ndk-r21b-windows-x86_64.zip"
            .parse()
            .unwrap(),
        "ndk.zip",
    );
}
