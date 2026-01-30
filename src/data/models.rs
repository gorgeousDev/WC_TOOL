// use colored::Colorize;

// #[derive(Debug, Clone)]
// pub struct DirInfo {
//     pub path: String,
//     pub size: u64,
//     pub entry_count: usize,
// }
// #[derive(Debug, Clone)]
// pub struct FileInfo {
//     pub path: String,
//     pub size: u64,
// }

// impl DirInfo {
//     pub fn print_dir_info(&self) {
//         let (size_float, unit_idx) = format_size(self.size);
//         println!(
//             "{} {} {} {:.2} {}, {} Entries",
//             "📁".bright_yellow(),
//             "Directory:".yellow(),
//             self.path.blue().bold(),
//             size_float,
//             UNITS[unit_idx].bright_green(),
//             self.entry_count
//         )
//     }
// }

// impl FileInfo {
//     pub fn print_file_info(&self) {
//         let (size_float, unit_idx) = format_size(self.size);
//         println!(
//             "{} {} {} {:.2} {}",
//             "📄".bright_cyan(),
//             "File:".cyan(),
//             self.path.blue().bold(),
//             size_float,
//             UNITS[unit_idx].bright_green()
//         )
//     }
// }
