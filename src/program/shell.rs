use std::{path::PathBuf, process::exit};

use super::{file_handeling::{create_file, create_directory, get_current_directory, change_directory, clear, print_directory, read_file, readline, write_file, remove_directory, remove_file}, version_control::init};

fn run_command(command_arguments:Vec<&str>, current_path:PathBuf) -> PathBuf {
    let mut new_path = current_path.clone();
    if !command_arguments.is_empty() {
        match command_arguments[0] {
            "cd" | "chdir" => {new_path = change_directory(command_arguments[1..].to_vec(), current_path)},
            "mkdir" | "md" => {if command_arguments.len() > 2 {create_directory(command_arguments[1])}},
            "mkfile" | "mf" => {if command_arguments.len() > 2 {create_file(command_arguments[1])}},
            "cls" | "clear" => clear(),
            "dir" | "ls" => print_directory(current_path),
            "read" => {
                if command_arguments.len() == 2 {
                    let mut file_path = current_path.clone();
                    file_path.push(PathBuf::from(command_arguments[1]));
                    println!("{}", read_file(file_path).join("\n"));
                };
            },
            "write" => {
                if command_arguments.len() > 2 {
                    let mut file_path = current_path.clone();
                    file_path.push(command_arguments[1]);

                    let bytes = write_file(command_arguments[2..].to_vec(), file_path);
                    println!("{:?} Bytes written.", bytes);
                }
            },
            "rd" => {
                if command_arguments.len() == 2 {
                    let mut directory_path = current_path.clone();
                    directory_path.push(command_arguments[1]);

                    remove_directory(directory_path);
                }
            },
            "rf" => {
                if command_arguments.len() == 2 {
                    let mut file_path = current_path.clone();
                    file_path.push(command_arguments[1]);

                    remove_file(file_path);
                }
            },
            "echo" => println!("{}", command_arguments[1..].join(" ")),
            "exit" | "quit" => exit(0),
            "init" => init(),
            "help" => {
                println!("
cd [path]                           Change the current directory to a specified folder
chdir [path]                        Change the current directory to a specified folder
clear                               Clear the console screen
cls                                 Clear the console screen
dir                                 List all files and directories in the current directory
echo                                Print text to the screen
exit                                Exit the console
help                                Display this message
init                                Initualizes the current directory as a jm-vc directory
ls                                  List all files and directories in the current directory
md [path]                           Creates a new directory
mf [path]                           Creates a new file
mkdir [path]                        Creates a new directory
mkfile [path]                       Creates a new file
quit                                Exit the console
rd [path]                           Removes the specified directory
read [path]                         Read the contents of a file and print to screen
rf [path]                           Removes the specified file
write [path] [content]              Write to the end of a file and auto creates if not exist
                ");
            },
            "" => (),
            _ => println!("{:?} is not a valid command.", command_arguments[0])
        }
    };
    new_path
}

pub fn shell() {
    let mut current_path = get_current_directory();
    println!("Welcome to the JM Shell! You are in {:?}", &current_path.display());

    loop {
        let command = readline("jm> ");
        current_path = run_command(command.split(' ').collect(), current_path);
    }
}
