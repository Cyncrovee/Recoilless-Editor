use std::{fs, io::{self, Write}};
use tui_textarea::TextArea;

// Fetches the file's size
pub fn get_file_size(file_path: &String) -> String {
    let mut file_size = fs::File::open(&file_path).expect("Failed to open file in get_file_size").metadata().expect("Failed to get file metadata in get_file_size").len().to_string();
    file_size.push_str(" Bytes Saved ");
    return file_size;
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

// Convert to file extension into something more readable
pub fn convert_extension(mut file_type: &str) -> &str {
    match file_type {
        "txt" => {
            file_type = "Text File";
            return file_type;
        }
        "md" => {
            file_type = "Markdown File";
            return file_type;
        }
        "cfg" => {
            file_type = "CFG File";
            return file_type;
        }
        "XML" => {
            file_type = "XML File";
            return file_type;
        }
        "XAML" => {
            file_type = "XAML File";
            return file_type;
        }
        "AXAML" => {
            file_type = "AXAML File";
            return file_type;
        }
        "JSON" => {
            file_type = "JSON File";
            return file_type;
        }
        "toml" => {
            file_type = "TOML File";
            return file_type;
        }
        // Commented out due to confusion over c/c++ file extensions
        /*"c" => {
            file_type = "C Source File";
            return file_type;
        }*/
        "sh" => {
            file_type = "Shell Script";
            return file_type;
        }
        "gd" => {
            file_type = "GDScript Source File";
            return file_type;
        }
        "rs" => {
            file_type = "Rust Source File";
            return file_type;
        }
        "zig" => {
            file_type = "Zig Source File";
            return file_type;
        }
        "cs" => {
            file_type = "C# Source File";
            return file_type;
        }
        "py" => {
            file_type = "Python Source File";
            return file_type;
        }
        "java" => {
            file_type = "Java Source File";
            return file_type;
        }
        "go" => {
            file_type = "GOLANG Source File";
            return file_type;
        }
        "lua" => {
            file_type = "Lua Source File";
            return file_type;
        }
        "cr" => {
            file_type = "Crystal Source File";
            return file_type;
        }
        "hx" => {
            file_type = "Haxe Source File";
            return file_type;
        }
        "dart" => {
            file_type = "Dart Source File";
            return file_type;
        }
        "rb" => {
            file_type = "Ruby Source File";
            return file_type;
        }
        "php" => {
            file_type = "PHP Source File";
            return file_type;
        }
        "js" => {
            file_type = "JavaScript Source File";
            return file_type;
        }
        "ts" => {
            file_type = "TypeScript Source File";
            return file_type;
        }
        &_ => {
            return file_type;
        }
    }
}