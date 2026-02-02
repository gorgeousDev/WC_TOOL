use crate::data::{directory::DirectoryType, file::FileType};

use colored::{ColoredString, Colorize};

pub struct Program {
    error_word: ColoredString,
}

impl Program {
    pub fn new() -> Self {
        Program {
            error_word: "ERROR:".red().bold(),
        }
    }
    fn print_help(&self) {
        println!(
            "{}",
            "wc_tool - Word and File Count Utility\n".green().bold()
        );
        println!("{}", "USAGE:".yellow().bold());
        println!("    wc_tool [OPTIONS]\n");

        println!("{}", "OPTIONS:".yellow().bold());
        println!(
            "    {:<20} {}",
            "-f/F <FILE>".blue(),
            "Display file info (size, basic info)"
        );
        println!(
            "    {:<20} {}",
            "-w/W <FILE>".blue(),
            "Display word count of the file"
        );
        println!(
            "    {:<20} {}",
            "-l/L <FILE>".blue(),
            "Display line count of the file"
        );
        println!(
            "    {:<20} {}",
            "-m/M <FILE>".blue(),
            "Display character count of the file"
        );
        println!(
            "    {:<20} {}",
            "-i/I <FILE>".blue(),
            "Display all info (size, words, lines, characters) of the file"
        );
        println!(
            "    {:<20} {}",
            "-d/D <DIR>".blue(),
            "Display simple info of the directory (size & entry count)"
        );
        println!(
            "    {:<20} {}",
            "-da/-ad <DIR>".blue(),
            "Display all info of directory recursively"
        );
        println!("    {:<20} {}", "-h".blue(), "Display this help message");

        println!("\n{}", "EXAMPLES:".yellow().bold());
        println!("    wc_tool -f test.txt        Display file size");
        println!("    wc_tool -w test.txt        Display word count");
        println!("    wc_tool -l test.txt        Display line count");
        println!("    wc_tool -m test.txt        Display character count");
        println!("    wc_tool -i test.txt        Display all info for file");
        println!("    wc_tool -d my_dir          Display simple info for directory");
        println!("    wc_tool -da my_dir         Display all info recursively for directory");
        println!("    wc_tool -h                 Show this help message\n");
    }

    pub fn handle_f_flag(&self, path: &String) {
        match FileType::new(path, true) {
            Some(file) => {
                file.print_info();
            }
            None => eprintln!(
                "{} Cannot find the file with the path: {}",
                self.error_word,
                path.green()
            ),
        }
    }

    pub fn handle_w_flag(&self, path: &String) {
        match FileType::new(path, true) {
            Some(file) => {
                file.print_words_count();
            }
            None => eprintln!(
                "{} Cannot find the file with the path: {}",
                self.error_word,
                path.green()
            ),
        }
    }

    pub fn handle_l_flag(&self, path: &String) {
        match FileType::new(path, true) {
            Some(file) => {
                file.print_line_count();
            }
            None => eprintln!(
                "{} Cannot find the file with the path: {}",
                self.error_word,
                path.green()
            ),
        }
    }
    pub fn handle_m_flag(&self, path: &String) {
        match FileType::new(path, true) {
            Some(file) => {
                file.print_chars_count();
            }
            None => eprintln!(
                "{} Cannot find the file with the path: {}",
                self.error_word,
                path.green()
            ),
        }
    }
    pub fn handle_i_flag(&self, path: &String) {
        match FileType::new(path, true) {
            Some(file) => {
                file.print_all_info();
            }
            None => eprintln!(
                "{} Cannot find the file with the path: {}",
                self.error_word,
                path.green()
            ),
        }
    }

    pub fn run(&self, args: Vec<String>) {
        if args.len() > 1 {
            let mut i = 1;
            while i < args.len() {
                let flag = &args[i];
                match flag.as_str() {
                    "-h" => self.print_help(),
                    "-f" | "-F" => {
                        if i + 1 < args.len() {
                            self.handle_f_flag(&args[i + 1]);
                        } else {
                            eprintln!("{} -f requires a file path", self.error_word);
                        }
                        break;
                    }
                    "-d" => {
                        if (i + 1) < args.len() {
                            match DirectoryType::new(&args[i + 1]) {
                                Some(dir) => dir.print_dir_simple_info(),
                                None => eprintln!(
                                    "{} Cannot read the directory: {}",
                                    self.error_word,
                                    &args[i + 1]
                                ),
                            }
                        } else {
                            eprintln!("{}{}", self.error_word, " -d requires a directory path");
                        }
                        break;
                    }
                    "-w" | "-W" => {
                        if i + 1 < args.len() {
                            self.handle_w_flag(&args[i + 1]);
                        } else {
                            eprintln!("{} -w requires a file path", self.error_word);
                        }
                        break;
                    }
                    "-l" | "-L" => {
                        if i + 1 < args.len() {
                            self.handle_l_flag(&args[i + 1]);
                        } else {
                            eprintln!("{} -l requires a file path", self.error_word);
                        }
                        break;
                    }

                    "-m" | "-M" => {
                        if i + 1 < args.len() {
                            self.handle_m_flag(&args[i + 1]);
                        } else {
                            eprintln!("{} -l requires a file path", self.error_word);
                        }
                        break;
                    }

                    "-i" | "-I" => {
                        if i + 1 < args.len() {
                            self.handle_i_flag(&args[i + 1]);
                        } else {
                            eprintln!("{} -l requires a file path", self.error_word);
                        }
                        break;
                    }

                    "-da" | "-ad" => {
                        if (i + 1) < args.len() {
                            match DirectoryType::new(&args[i + 1]) {
                                Some(dir) => dir.print_dir_all_info(true),
                                None => eprintln!(
                                    "{} Cannot read the directory: {}",
                                    self.error_word,
                                    &args[i + 1]
                                ),
                            }
                        } else {
                            eprintln!("{}{}", self.error_word, " -d requires a directory path");
                        }
                        break;
                    }

                    _ => {
                        self.print_help();
                    }
                }
                i += 1;
            }
        } else {
            self.print_help();
        }
    }
}
