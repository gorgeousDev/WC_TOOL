use crate::data::models::DirInfo;
use crate::data::models::FileInfo;

use colored::{ColoredString, Colorize};

pub struct Program {
    error_word: ColoredString,
}

impl Program {
    pub fn new() -> Self {
        Program {
            error_word: "ERROR: ".red().bold(),
        }
    }

    fn handle_path(&self, file_path: &str) -> Result<FileInfo, std::io::Error> {
        match std::fs::File::open(file_path) {
            Ok(file) => match file.metadata() {
                Ok(metadata) => {
                    let file_size = metadata.len();
                    let file_info = FileInfo {
                        path: file_path.to_string(),
                        size: file_size,
                    };
                    file_info.print_file_info();
                    return Ok(FileInfo {
                        path: file_path.to_string(),
                        size: file_size,
                    });
                }
                Err(_) => eprintln!("{}Failed to read file metadata", self.error_word),
            },
            Err(e) => eprintln!("{}{}", self.error_word, e),
        }
        Ok(FileInfo {
            path: file_path.to_string(),
            size: 0,
        })
    }

    fn print_help(&self) {
        println!("{}", "wc_tool - Word and File Count Utility\n".green());
        println!("USAGE:\n    wc_tool [OPTIONS]\n");
        println!("OPTIONS:");
        println!(
            "    {} {}",
            "-f <FILE>".blue(),
            "Specify the file path to analyze"
        );
        println!("               Example: wc_tool -f test.txt\n");
        println!("    {} {}\n", "-h".blue(), "Display this help message");
        println!("EXAMPLES:");
        println!("    wc_tool -f test.txt      Display file size");
        println!("    wc_tool -h               Show this help message");
    }

    fn handle_dir(&self, path: &str) -> Result<u64, std::io::Error> {
        let mut total_size: u64 = 0;
        let entries = std::fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            let display = entry_path.display().to_string();

            if entry_path.is_dir() {
                match self.handle_dir(&display) {
                    Ok(size) => total_size += size,
                    Err(_) => {}
                }
            } else if entry_path.is_file() {
                match self.handle_path(&display) {
                    Ok(file_info) => {
                        total_size += file_info.size;
                    }
                    Err(e) => println!("{}{}", self.error_word, e),
                }
            }
        }
        let dir_info = DirInfo {
            path: path.to_string(),
            size: total_size,
        };
        dir_info.print_dir_info();
        Ok(total_size)
    }

    pub fn run(&self, args: Vec<String>) {
        if args.len() > 1 {
            let mut i = 1;
            while i < args.len() {
                let flag = &args[i];
                match flag.as_str() {
                    "-h" => self.print_help(),
                    "-f" => {
                        if (i + 1) < args.len() {
                            match self.handle_path(&args[i + 1]) {
                                Ok(_) => {
                                    // Move to the file argument
                                }
                                Err(e) => {
                                    eprintln!("{}{}", self.error_word, e);
                                }
                            }
                            i += 1;
                        } else {
                            eprintln!("{}{}", self.error_word, " -f requires a file path");
                        }
                    }
                    "-d" => {
                        if (i + 1) < args.len() {
                            match self.handle_dir(&args[i + 1]) {
                                Ok(_) => {}
                                Err(e) => eprintln!("{}{}", self.error_word, e),
                            }
                            i += 1;
                        } else {
                            eprintln!("{}{}", self.error_word, " -d requires a directory path");
                        }
                    }

                    _ => {
                        self.print_help();
                    }
                }
                i += 1;
            }
        } else {
            eprintln!(
                "{}{}",
                self.error_word, " cannot run program without arguments\n"
            );
            self.print_help();
        }
    }
}
