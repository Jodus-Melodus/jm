use std::io::stdout;
use std::path::PathBuf;
use crossterm::execute;
use crossterm::terminal::{ScrollDown, ClearType, Clear};


use program::{jm::Jm, file_handeling::{readline, clear}};

mod program;

fn main() {
    let file_path = PathBuf::from(readline("Enter file path > "));
    let mut texteditor = Jm::new(file_path);

    loop {
        clear();
        texteditor.clone().display();
        execute!(stdout(), ScrollDown(1), Clear(ClearType::FromCursorDown));
        texteditor = texteditor.clone().command();
    }

}
