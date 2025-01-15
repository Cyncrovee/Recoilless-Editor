use std::{env, fs, io::{self, Write}};
use tui_textarea::TextArea;

pub fn boot_arg() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(_) => {
            match args[1].as_str() {
                "-h" | "--help"=> {
                    show_help();
                }
                _ => {
                    // Pass
                }
            }
        }
        None => show_help(),
    }
    drop(args);
}


fn show_help() {
    println!("------------------------------------------------------------------------");
    println!("To open a file in Recoilless Editor, you can use the arguments listed below:");
    println!("");
    println!("  -n OR --name filename");
    println!("      Replace filename with the name and extension of the file you are trying to open if applicable");
    println!("  -p OR --path filepath");
    println!("      Replace filepath with the full path, name, and extension of the file you are trying to open if applicable");
    println!("");
    println!("You can also open a file by simply adding the file name/extension as the first flag. Both this and -n/--name work based off the current working directory, so you can also input a file path from that directory.");
    println!("------------------------------------------------------------------------");
    std::process::exit(0);
}

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