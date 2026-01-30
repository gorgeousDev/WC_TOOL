use crate::data::file::FileType;

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

    pub fn handle_f_flag(&self, path: &String) {
        match FileType::new(path) {
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
        match FileType::new(path) {
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
        match FileType::new(path) {
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
        match FileType::new(path) {
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
        match FileType::new(path) {
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
                            i += 1;
                        } else {
                            eprintln!("{} -f requires a file path", self.error_word);
                        }
                    }
                    // "-d" => {
                    //     if (i + 1) < args.len() {
                    //         match self.handle_dir(&args[i + 1]) {
                    //             Ok(dir) => {
                    //                 dir.print_dir_info();
                    //             }
                    //             Err(e) => eprintln!("{}{}", self.error_word, e),
                    //         }
                    //         i += 1;
                    //     } else {
                    //         eprintln!("{}{}", self.error_word, " -d requires a directory path");
                    //     }
                    // }
                    "-w" | "-W" => {
                        if i + 1 < args.len() {
                            self.handle_w_flag(&args[i + 1]);
                            i += 1;
                        } else {
                            eprintln!("{} -w requires a file path", self.error_word);
                        }
                    }
                    "-l" | "-L" => {
                        if i + 1 < args.len() {
                            self.handle_l_flag(&args[i + 1]);
                            i += 1;
                        } else {
                            eprintln!("{} -l requires a file path", self.error_word);
                        }
                    }

                    "-m" | "-M" => {
                        if i + 1 < args.len() {
                            self.handle_m_flag(&args[i + 1]);
                            i += 1;
                        } else {
                            eprintln!("{} -l requires a file path", self.error_word);
                        }
                    }

                    "-i" | "-I" => {
                        if i + 1 < args.len() {
                            self.handle_i_flag(&args[i + 1]);
                            i += 1;
                        } else {
                            eprintln!("{} -l requires a file path", self.error_word);
                        }
                    }

                    _ => {
                        self.print_help();
                    }
                }
                i += 1;
            }
        } else {
            eprintln!("{} {}", self.error_word, "unavailabe flag");
            self.print_help();
        }
    }
}
