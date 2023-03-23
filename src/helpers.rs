use std::path::PathBuf;

pub fn replace_bytes(
    data: &mut [u8],
    replace_at: usize,
    replace_with: &[u8],
    replace_length: usize,
) {
    assert!(data.len() >= replace_at + replace_length);
    let iter = data
        .iter_mut()
        .skip(replace_at)
        .take(replace_length)
        .enumerate();

    for (i, b) in iter {
        *b = replace_with[i];
    }
}

pub fn get_unix_timestamp() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

pub fn truncate_path(pb: &PathBuf) -> String {
    let slice = pb.as_path().to_str().unwrap(); //we know absolute paths are long enough
    format!("...{}", &slice[slice.len() - 15 ..])
}