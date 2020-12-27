use std::fs;
use std::io;
use std::sync::Arc;

use zip::{read::ZipFile, ZipArchive};

#[tokio::main]
pub async fn unzip(zip_file: &fs::File) -> io::Result<()> {
    let archive = Arc::new(ZipArchive::new(zip_file).unwrap());

    let mut file_write_results = Vec::new();
    for i in 0..archive.len() {
        let arc = Arc::clone(&archive);
        let file_result = extract_zip_file(arc.clone(), i);
        file_write_results.push(file_result)
    }

    let _ = futures::future::join_all(file_write_results).await;

    Result::Ok(())
}

async fn extract_zip_file(archive: Arc<ZipArchive<&fs::File>>, index: usize) -> io::Result<u64> {
    println!("extract_zip_file - i : {}", index);
    let mut archive = Arc::clone(&archive);
    let archive = Arc::make_mut(&mut archive);
    let mut zip_file = archive.by_index(index).unwrap();

    let outpath = zip_file.sanitized_name();
    println!(
        "extract_zip_file - outpath : {}",
        &outpath.as_path().display()
    );

    if zip_file.is_dir() || zip_file.name().ends_with('/') {
        println!("File extracted to \"{}\"", outpath.as_path().display());
        tokio::fs::create_dir_all(&outpath).await.unwrap();
        Ok(0)
    } else {
        println!(
            "File extracted to \"{}\" ({} bytes)",
            outpath.as_path().display(),
            zip_file.size()
        );
        if let Some(path) = outpath.parent() {
            if !path.exists() {
                tokio::fs::create_dir_all(&path).await.unwrap();
            }
        }
        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut zip_file, &mut outfile).unwrap();
        // let mut outfile = tokio::fs::File::create(&outpath).await.map_err(|e| {
        //     println!("error - path: {}, e: {}", &outpath.as_path().display(), e);
        // });
        // tokio::io::copy(&mut zip_file, &mut outfile).await
        Ok(0)
    }
}
