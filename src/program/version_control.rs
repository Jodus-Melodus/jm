use std::path::PathBuf;

use super::file_handeling::create_directory;

pub fn init() {
    create_directory("jm-vc");
}
