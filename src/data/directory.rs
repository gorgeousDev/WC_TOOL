use crate::data::file::FileType;
use colored::Colorize;
use std::{fs::read_dir, path::Path};

enum Entry {
    File(FileType),
    Dir(DirectoryType),
}

pub struct DirectoryType {
    size: u64,
    entries: Vec<Entry>,
    path: String,
}

impl DirectoryType {
    pub fn new(path_str: &String) -> Option<DirectoryType> {
        let path = Path::new(path_str);

        match read_dir(path) {
            Ok(directory) => {
                let mut size: u64 = 0;
                let mut entries_list: Vec<Entry> = Vec::new();

                for entry in directory {
                    match entry {
                        Ok(entry) => {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                match DirectoryType::new(&entry_path.display().to_string()) {
                                    Some(entry_dir) => {
                                        size += entry_dir.size;
                                        entries_list.push(Entry::Dir(entry_dir));
                                    }
                                    None => eprintln!(
                                        "Warning: Cannot read subdirectory {:?}",
                                        entry_path
                                    ),
                                }
                            } else if entry_path.is_file() {
                                match FileType::new(&entry_path.display().to_string(), false) {
                                    Some(entry_file) => {
                                        size += entry_file.size;
                                        entries_list.push(Entry::File(entry_file));
                                    }
                                    None => eprintln!("Warning: Cannot read file {:?}", entry_path),
                                }
                            }
                        }
                        Err(e) => eprintln!("Warning: Cannot read entry: {}", e),
                    }
                }

                Some(DirectoryType {
                    size,
                    entries: entries_list,
                    path: path_str.clone(),
                })
            }
            Err(e) => {
                eprintln!(
                    "{} Cannot read directory {:?}: {}",
                    "ERROR:".red().bold(),
                    path_str,
                    e
                );
                None
            }
        }
    }

    pub fn print_dir_simple_info(&self) {
        let (size_float, unit_idx) = crate::constants::format_size(self.size);
        println!(
            "{} {} {} {:.2} {}, {} Entries",
            "📁".bright_yellow(),
            "Directory:".yellow(),
            self.path.blue().bold(),
            size_float,
            crate::constants::UNITS[unit_idx].bright_green(),
            self.entries.len()
        )
    }

    pub fn print_dir_all_info(&self, is_main: bool) {
        if is_main {
            self.print_dir_simple_info();
        }
        for entry in &self.entries {
            match entry {
                Entry::Dir(dir) => {
                    dir.print_dir_simple_info();
                    dir.print_dir_all_info(false)
                }
                Entry::File(file) => file.print_info(),
            }
        }
    }
}
