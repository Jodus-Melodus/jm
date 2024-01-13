use std::path::PathBuf;

use super::file_handeling::{create_directory, read_file};

pub fn init() {
    create_directory("jm-vc");
}

pub fn compare_files(path_of_file1:PathBuf, path_of_file2:PathBuf) -> Vec<String> {
    let mut file1_contents = read_file(path_of_file1);
    let mut file2_contents = read_file(path_of_file2);

    let mut changes: Vec<String> = Vec::new();
    

    while file1_contents.len() != file2_contents.len() {
        if file1_contents.len() < file2_contents.len() {
            file1_contents.push("".to_string());
        } else {
            file2_contents.push("".to_string())
        }
    }

    for i in 0..file1_contents.len() {
        if file1_contents[i] != file2_contents[i] {
            changes.push(format!("Line {}:\n\t{} => \n\t{}", i, file1_contents[i], file2_contents[i]))
        }
    }
    
    changes
}