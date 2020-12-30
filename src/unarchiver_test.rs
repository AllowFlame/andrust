use super::unarchiver;

#[test]
fn unzip_test() {
    use std::fs;

    let file_name = std::path::Path::new("zip_test/test.zip");
    let file = fs::File::open(&file_name).unwrap();
    let _ = unarchiver::unzip(&file);
}
