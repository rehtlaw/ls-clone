use std::env::current_dir;
use std::fs::FileType;

use colored::*;

fn main() {
    let pwd: std::path::PathBuf = current_dir().expect("couldn't get pwd");
    println!("{}", pwd.to_str().unwrap());
    for item in pwd.read_dir().expect("couldn't read directory") {
        if let Ok(item) = item {
            let file_type = item.file_type().expect("couldn't get filetype");
            let file_name = item
                .file_name()
                .to_str()
                .expect("couldn't get file name")
                .to_string();

            let colored_name = get_color(file_type, file_name);

            println!("{}", colored_name);
        }
    }
}

fn get_color(ftype: FileType, name: String) -> ColoredString {
    if ftype.is_dir() == true {
        name.cyan()
    } else if ftype.is_symlink() == true {
        name.green()
    } else {
        name.white()
    }
}
