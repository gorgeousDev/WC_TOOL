use std::{fs::File, io::Read, os::unix::fs::MetadataExt, path::Path};

use colored::Colorize;

use crate::constants::UNITS;

pub struct FileType {
    path: String,
    pub size: u64,
    pub word_count: usize,
    pub line_count: usize,
    pub character_count: usize,
}
pub fn count_words(content: &str) -> usize {
    return content.split_whitespace().count();
}

pub fn count_lines(content: &str) -> usize {
    return content.lines().count();
}

pub fn count_characters(content: &str) -> usize {
    return content.chars().count();
}

impl FileType {
    pub fn new(path: &str) -> Option<FileType> {
        let path_obj = Path::new(path);
        match File::open(path_obj) {
            Ok(mut file) => match file.metadata() {
                Ok(content) => {
                    let mut content_str = String::new();
                    match file.read_to_string(&mut content_str) {
                        Ok(_) => {
                            let words = count_words(&content_str);
                            let lines = count_lines(&content_str);
                            let chars = count_characters(&content_str);
                            return Some(FileType {
                                path: String::from(path),
                                size: content.size(),
                                word_count: words,
                                line_count: lines,
                                character_count: chars,
                            });
                        }
                        Err(_) => {
                            return None;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    return None;
                }
            },
            Err(_) => None,
        }
    }
    pub fn print_info(&self) {
        let (size_float, unit_idx) = crate::constants::format_size(self.size);
        println!(
            "{} {} {}: {:.2} {}",
            "📄".bright_cyan(),
            "File:".cyan(),
            self.path.blue().bold(),
            size_float,
            UNITS[unit_idx].bright_green()
        )
    }

    pub fn print_words_count(&self) {
        let word_word = if self.word_count == 1 {
            "Word"
        } else {
            "Words"
        };

        println!(
            "{} {} {}: {} {}",
            "📄".bright_cyan(),
            "File:".cyan(),
            self.path.blue().bold(),
            if self.word_count == 0 {
                "No".red().bold()
            } else {
                format!("{}", self.word_count).white()
            },
            word_word.green()
        )
    }
    pub fn print_line_count(&self) {
        let line_word = if self.line_count == 1 {
            "Line"
        } else {
            "Lines"
        };
        println!(
            "{} {} {}: {} {}",
            "📄".bright_cyan(),
            "File:".cyan(),
            self.path.blue().bold(),
            if self.line_count == 0 {
                "No".red().bold()
            } else {
                format!("{}", self.line_count).white()
            },
            line_word.green()
        )
    }
    pub fn print_chars_count(&self) {
        let c_word = if self.line_count == 1 {
            "Character"
        } else {
            "Characters"
        };
        println!(
            "{} {} {}: {} {}",
            "📄".bright_cyan(),
            "File:".cyan(),
            self.path.blue().bold(),
            if self.character_count == 0 {
                "No".red().bold()
            } else {
                format!("{}", self.character_count).white()
            },
            c_word.green()
        )
    }

    pub fn print_all_info(&self) {
        let word_word = if self.word_count == 1 {
            "Word"
        } else {
            "Words"
        };

        let c_word = if self.line_count == 1 {
            "Character"
        } else {
            "Characters"
        };
        let line_word = if self.line_count == 1 {
            "Line"
        } else {
            "Lines"
        };
        println!(
            "{} {} {}: {} {}, {} {}, {} {}",
            "📄".bright_cyan(),
            "File:".cyan(),
            self.path.blue().bold(),
            if self.character_count == 0 {
                "No".red().bold()
            } else {
                format!("{}", self.character_count).white()
            },
            c_word.green(),
            if self.word_count == 0 {
                "No".red().bold()
            } else {
                format!("{}", self.word_count).white()
            },
            word_word.green(),
            if self.line_count == 0 {
                "No".red().bold()
            } else {
                format!("{}", self.line_count).white()
            },
            line_word.green(),
        )
    }
}
