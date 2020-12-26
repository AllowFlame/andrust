mod downloader;
mod platform;
mod unarchiver;

use platform::Platform;

fn main() {
    // let platform = platform();
    /*
    let ndk_path = platform.determine_ndk_path();
    platform.setup_config(ndk_path.as_str());
    // */
    unzip();

    /* download sample
    let downloader = downloader::Downloader::default();
    let _ = downloader.download(
        "https://dl.google.com/android/repository/android-ndk-r21b-windows-x86_64.zip"
            .parse()
            .unwrap(),
        "ndk.zip",
    );
    */
}

#[cfg(target_os = "windows")]
fn platform() -> impl Platform {
    platform::WinConfig
}

#[cfg(target_os = "macos")]
fn platform() -> impl Platform {
    platform::MacConfig
}

#[cfg(target_os = "linux")]
fn platform() -> impl Platform {
    platform::LinuxConfig
}

fn unzip() {
    use std::fs;

    // real_main();
    let fname = std::path::Path::new("zip_test/test.zip");
    let file = fs::File::open(&fname).unwrap();
    unarchiver::unzip(&file);
}

fn real_main() -> i32 {
    use std::fs;
    use std::io;

    // let args: Vec<_> = std::env::args().collect();
    // if args.len() < 2 {
    //     println!("Usage: {} <filename>", args[0]);
    //     return 1;
    // }
    // let fname = std::path::Path::new(&*args[1]);
    let fname = std::path::Path::new("target/ndk.zip");
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap(); // <- optimizing point
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    return 0;
}
