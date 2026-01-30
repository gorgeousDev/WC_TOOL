pub const UNITS: &[&str] = &["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

pub fn format_size(mut size: u64) -> (f64, usize) {
    let mut named_size = 0;
    let mut size_float = size as f64;

    while size >= 1024 && named_size < UNITS.len() - 1 {
        size /= 1024;
        size_float /= 1024.0;
        named_size += 1;
    }
    (size_float, named_size)
}