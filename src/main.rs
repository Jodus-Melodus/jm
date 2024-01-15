use std::path::PathBuf;
use program::{jm::Jm, file_handeling::{readline, clear, create_file}};
mod program;

fn main() {
    let file_path = PathBuf::from(readline("Enter file path > "));
    let mut texteditor;

    if !file_path.exists() {
        create_file(file_path.to_str().unwrap());    
    }
    texteditor = Jm::new(file_path);

    loop {
        clear();
        texteditor.clone().display();
        texteditor = texteditor.clone().command();
    }

}
