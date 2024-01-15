use std::{path::PathBuf, process::exit, fs};
use crate::program::file_handeling::write_file;
use super::file_handeling::{read_file, readline};

#[derive(Clone)]
pub struct Jm {
    file_path: PathBuf,
    lines: Vec<String>,
    cursor: usize,
    number_width: usize,
    saved: bool,
    file_size : u64
}

impl Jm {
    pub fn new(file_path: PathBuf) -> Self {
        let lines = read_file(file_path.clone());
        Jm {
            file_path: file_path.clone(),
            lines: lines.clone(),
            cursor: 0,
            number_width: lines.len().to_string().len(),
            saved: true,
            file_size: fs::metadata(file_path).unwrap().len()
        }
    }

    pub fn display(mut self) {
        self.number_width = self.lines.len().to_string().len();
        let num_width = self.number_width;

        println!("--------------------------------------------------------------------------------------------------");
        println!("| Ln:{} | {} | {} bytes |",
            self.cursor+1,
            if self.saved {"Saved"} else {"Unsaved"},
            self.file_size
        );
        println!("--------------------------------------------------------------------------------------------------");

        for (i, line) in self.lines.iter().enumerate() {
            if i == self.cursor {
                println!(" {:<num_width$}> {}", i+1, line);
            } else {
                println!(" {:<num_width$}| {}", i+1, line);
            }
        }
    }

    pub fn command(mut self) -> Self {
        let num_width = self.number_width;
        println!("--------------------------------------------------------------------------------------------------");
        let cmd = readline(&format!(" {:<num_width$}> ", self.cursor+1));
        self.file_size = fs::metadata(self.clone().file_path).unwrap().len();
        
        match cmd.as_str() {
            "q" => {
                if self.saved {
                    exit(0);
                } else if readline("You have unsaved changes. Do you want to save and exit? (y/n) > ") == "y" {
                    write_file(self.lines.iter().map(|l| l.as_str()).collect::<Vec<&str>>(), self.file_path);
                    println!("Save successful!");
                    readline("Enter to exit");
                    exit(0);
                } else {
                    exit(0);
                }
            },
            ">" => self.lines.push("".to_string()),
            "<" => self.lines.insert(0, "".to_string()),
            "^" => self.lines.insert(self.cursor+1, "".to_string()),
            "mu" => self.cursor = (self.cursor-1) % self.lines.len(),
            "md" => self.cursor = (self.cursor+1) % self.lines.len(),
            "dl" => {self.lines.remove(self.cursor);},
            "ca" => self.lines.clear(),
            "." => self.cursor = self.lines.len()-1,
            "$" => self.cursor = 0,
            "s" => {
                println!("{} bytes written", write_file(self.lines.iter().map(|l| l.as_str()).collect::<Vec<&str>>(), self.clone().file_path));
                readline("Enter to continue");
                self.saved = true;
            },
            "goto" => self.cursor = readline("> ").parse::<usize>().unwrap()-1,
            "?" => {
                println!("
?       Display this msg
q       Quit the editor
>       Append empty line to end
<       Insert empty line at start
^       Insert empty line under current one
mu      Up a line
md      Down a line
dl      Deletes current line
ca      Clears entire file
.       Set cursor at end of file
$       Set cursor at beginning of file
s       Save changes
goto    Go to specific line
");
readline("Enter to continue");
            },
            "" => self.cursor = (self.cursor+1) % self.lines.len(),
            _ => {
                self.lines[self.cursor] = cmd;
                self.saved = false;
                self.cursor = (self.cursor+1) % self.lines.len();
            }
        }
        self
    }
}
