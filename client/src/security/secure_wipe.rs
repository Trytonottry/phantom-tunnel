use zeroize::Zeroize;

fn secure_wipe(path: &str) {
    let mut data = std::fs::read(path).unwrap();
    data.zeroize();
    std::fs::write(path, &data).ok();
    std::fs::remove_file(path).ok();
}