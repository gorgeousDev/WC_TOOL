use colored::Colorize;
const UNITS: &[&str] = &["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

#[derive(Debug, Clone)]
pub struct DirInfo {
    pub path: String,
    pub size: u64,
}
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
}

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

impl DirInfo {
    pub fn print_dir_info(&self) {
        let (size_float, unit_idx) = format_size(self.size);
        println!(
            "{} {} {} {:.2} {}",
            "📁".bright_yellow(),
            "Directory:".yellow(),
            self.path.blue().bold(),
            size_float,
            UNITS[unit_idx].bright_green()
        )
    }
}

impl FileInfo {
    pub fn print_file_info(&self) {
        let (size_float, unit_idx) = format_size(self.size);
        println!(
            "{} {} {} {:.2} {}",
            "📄".bright_cyan(),
            "File:".cyan(),
            self.path.blue().bold(),
            size_float,
            UNITS[unit_idx].bright_green()
        )
    }
}
