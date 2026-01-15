use colored::{ColoredString, Colorize};

use std::path::Path;

pub struct Program {
    flags_vec: Vec<String>,
    error_word: ColoredString,
}

impl Program {
    pub fn new() -> Self {
        Program {
            flags_vec: vec!["-f".to_string(), "-h".to_string()],
            error_word: "ERROR: ".red().bold(),
        }
    }

    fn handle_path(&self, file_path: &String) {
        if !Path::new(file_path).exists() {
            eprintln!("{} File '{}' does not exist", self.error_word, file_path);
            return;
        }

        match std::fs::File::open(file_path) {
            Ok(file) => match file.metadata() {
                Ok(metadata) => {
                    let file_size_str = self.handle_size(metadata.len());
                    println!("{}", file_size_str);
                }
                Err(_) => eprintln!("{}Failed to read file metadata", self.error_word),
            },
            Err(_) => eprintln!("{}Failed to open file", self.error_word),
        }
    }

    fn handle_size(&self, mut size: u64) -> String {
        let units = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        let mut named_size = 0;
        let mut size_float = size as f64;

        while size >= 1024 && named_size < units.len() - 1 {
            size /= 1024;
            size_float /= 1024.0;
            named_size += 1;
        }

        format!("File size: {:.2} {}", size_float, units[named_size].green())
    }

    fn print_help(&self) {
        println!("{}", "wc_tool - Word and File Count Utility\n".green());
        println!("USAGE:");
        println!("    wc_tool [OPTIONS]\n");
        println!("OPTIONS:");
        println!(
            "{}{}",
            "    -f <FILE>".blue(),
            " Specify the file path to analyze"
        );
        println!("                 Example: wc_tool -f test.txt\n");
        println!(
            "{}{}",
            "    -h".blue(),
            "           Display this help message\n"
        );
        println!("EXAMPLES:");
        println!("    wc_tool -f test.txt      Display file size");
        println!("    wc_tool -h               Show this help message");
    }

    pub fn run(&self, args: Vec<String>) {
        if args.len() > 1 {
            let mut i = 1;
            while i < args.len() {
                if self.flags_vec.contains(&args[i]) {
                    if args[i] == "-h" {
                        self.print_help();
                    } else if args[i] == "-f" {
                        if i + 1 < args.len() {
                            self.handle_path(&args[i + 1]);
                            i += 1;
                        } else {
                            eprintln!("{}{}", self.error_word, " -f requires a file path");
                        }
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
