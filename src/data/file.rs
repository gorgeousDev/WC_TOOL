use std::{fs::File, io::Read, path::Path};

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
    pub fn new(path: &str, read_content: bool) -> Option<FileType> {
        let path_obj = Path::new(path);
        match path_obj.metadata() {
            Ok(metadata) => {
                if read_content {
                    let mut content_str = String::new();
                    if let Ok(mut file) = File::open(path_obj) {
                        if file.read_to_string(&mut content_str).is_ok() {
                            let words = count_words(&content_str);
                            let lines = count_lines(&content_str);
                            let chars = count_characters(&content_str);
                            return Some(FileType {
                                path: path.to_string(),
                                size: metadata.len(),
                                word_count: words,
                                line_count: lines,
                                character_count: chars,
                            });
                        }
                    }

                    return Some(FileType {
                        path: path.to_string(),
                        size: metadata.len(),
                        word_count: 0,
                        line_count: 0,
                        character_count: 0,
                    });
                } else {
                    return Some(FileType {
                        path: path.to_string(),
                        size: metadata.len(),
                        word_count: 0,
                        line_count: 0,
                        character_count: 0,
                    });
                }
            }
            Err(e) => {
                eprintln!(
                    "{} Cannot read file {:?}: {}",
                    "ERROR:".red().bold(),
                    path,
                    e
                );
                None
            }
        }
    }
    pub fn print_info(&self) {
        let (size_float, unit_idx) = crate::constants::format_size(self.size);
        println!(
            "   {} {:<12} {}: {:.2} {}",
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
