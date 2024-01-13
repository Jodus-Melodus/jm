use std::{env, path::PathBuf, process::Command, io::{self, Write, BufReader, BufRead}, fs::{self, File, OpenOptions}};

pub fn get_current_directory() -> PathBuf {
    if let Ok(current_dir) = env::current_dir() {
        current_dir
    } else {
        PathBuf::from(".")
    }
}

pub fn remove_directory(directory_path:PathBuf) {
    if let Err(err) = fs::remove_dir_all(directory_path.clone()) {
        eprintln!("Error deleting directory: {}", err);
    } else {
        println!("Removed directory {:?}", directory_path.display());
    }
}

pub fn remove_file(file_path:PathBuf) {
    if let Err(err) = fs::remove_file(file_path.clone()) {
        eprintln!("Error deleting file: {}", err);
    } else {
        println!("Removed file {:?}", file_path.display());
    }
}

pub fn create_directory(directory_name:&str) {
    if let Err(err) = fs::create_dir(directory_name) {
        eprintln!("Error creating directory: {}", err);
    } else {
        println!("Directory '{}' created successfully", directory_name);
    }
}

pub fn create_file(file_name:&str) {
    match File::create(file_name) {
        Ok(_) => {
            println!("File '{}' created successfully", file_name);
        }
        Err(err) => {
            eprintln!("Error creating file: {}", err);
        }
    }
}

pub fn get_files_and_directories_of_current_path(current_path:PathBuf) -> Vec<String> {
    let mut files_and_directories = Vec::new();

    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_name = entry.file_name();
                files_and_directories.push(entry_name.to_string_lossy().to_string())
            }
        }
    }
    files_and_directories
}

pub fn write_file(content: Vec<&str>, path: PathBuf) -> usize {
    let mut file = OpenOptions::new().create(true).append(true).open(path).expect("Failed to create file");
    let bytes = file.write(content.join(" ").as_bytes()).expect("Failed to write to file");

    bytes
}

pub fn read_file(path:PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let content: Vec<String> = BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    content
}

pub fn readline(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim_end().to_string()
}

pub fn clear() {
    if cfg!(windows) {
        Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .expect("Failed to clear screen");
    }
}

pub fn print_directory(current_path:PathBuf) {
    let files_and_directories = get_files_and_directories_of_current_path(current_path.clone());
    println!("\n{}:", current_path.display());
    for file_or_directory in &files_and_directories {
        if file_or_directory.contains('.') {
            println!("  {}", file_or_directory);
        } else {
            println!("  {}\\", file_or_directory);
        }
    }
    println!();
}

pub fn change_directory(command_arguments:Vec<&str>, current_path:PathBuf) -> PathBuf {
    let mut new_path = current_path.clone();
    if !command_arguments.is_empty() {
        match command_arguments[0] {
            "." => print_directory(current_path),
            ".." => {
                new_path = new_path.parent().unwrap().to_path_buf();
            },
            _ => {
                let available_files_and_direcories = get_files_and_directories_of_current_path(current_path);
                println!("{:?}", available_files_and_direcories);
                println!("> {}", command_arguments[0]);
                if available_files_and_direcories.contains(&command_arguments[0].to_string()) {
                    new_path.push(command_arguments[0])
                }
            }
        }
    };
    new_path
}