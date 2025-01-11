use std::{env, fs, io::{self, Write}};
use tui_textarea::TextArea;

// Get cli argument(s) and return the file path
pub fn get_file_path() -> String {
    let local_file_path: String;
    let args: Vec<String> = env::args().collect();
    if args[1] == "-p" || args[1] == "--path" {
        local_file_path = args[2].to_string();
        return local_file_path;
    }
    else if args[1] == "-n" || args[1] == "--name" {
        let dir_path = env::current_dir().expect("Couldn't fetch current directory");
        let dir_path_string = dir_path.into_os_string().into_string().expect("Failed to convert current directory into String");
        local_file_path = dir_path_string + std::path::MAIN_SEPARATOR_STR + args[2].to_string().as_str();
        return local_file_path;
    }
    else {
        let dir_path = env::current_dir().expect("Couldn't fetch current directory");
        let dir_path_string = dir_path.into_os_string().into_string().expect("Failed to convert current directory into String");
        local_file_path = dir_path_string + std::path::MAIN_SEPARATOR_STR + args[1].to_string().as_str();
        return local_file_path;
    }
}


// Save the current file (if is_modified is true)
pub fn save_file(is_modified: &bool, file_path: &String, input_area: &mut TextArea) {
    match is_modified {
        true => {
            let mut writer = io::BufWriter::new(fs::File::create(&file_path).expect("Failed to create BufWriter!"));
            for l in input_area.lines() {
                writer.write_all(l.as_bytes()).expect("Error when writing to file!");
                writer.write_all(b"\n").expect("Error when writing to file!");
            }
            drop(writer);
        }
        false => {
            // Pass
        }
    }
}