# WC_tool 
$!! -- by GorgeousDev -- !!$



`wc_tool` is a Rust-based command-line tool to analyze files and directories. It can display:

- File size
- Word count
- Line count
- Character count
- Directory summary (size, number of entries)
- Recursive directory information  

It works safely with both text and binary files, showing warnings for unreadable files or directories without causing the program to panic.

---

## 📦 USAGE

- Download the release.
- Use args in command line to see it as below:

```bash
wc_tool [OPTIONS]
⚙️ OPTIONS
Flag	Description
-f <FILE>	Display basic file info (size)
-w <FILE>	Display word count of the file
-l <FILE>	Display line count of the file
-m <FILE>	Display character count of the file
-i <FILE>	Display all info (size, words, lines, characters)
-d <DIR>	Display simple info of a directory (size & entry count)
-da / -ad	Display all info of a directory recursively (recursive traversal)
-h	Display this help message

💡 EXAMPLES
wc_tool -f test.txt       # Display file size
wc_tool -w test.txt       # Display word count
wc_tool -l test.txt       # Display line count
wc_tool -m test.txt       # Display character count
wc_tool -i test.txt       # Display all info for file
wc_tool -d my_dir         # Display simple info for directory
wc_tool -da my_dir        # Display all info recursively for directory
wc_tool -h                # Show this help message
```
## 🔧 FEATURES

- Text File Analysis:

- Counts words, lines, characters, and file size.

- Handles empty or binary files gracefully.

- Directory Analysis:

- Summarizes total size of all files inside.

- Counts number of entries (files and subdirectories).

- Recursive traversal available (-da / -ad).

- Safe Handling:

- Ignores unreadable files or directories, showing a warning instead of crashing.

- When scanning directories, words/lines/characters for files are set to zero (size only).

- Flexible Usage:

  - Can analyze a single file or an entire directory.

  - Works for all types of files, including binary files.


