use std::env::{args, current_dir};
use std::fs;

// Get the first CLI arg and runs the appropriate code
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

// Get cli argument(s) and return the file path if it is found
pub fn get_file_path(is_boot: bool) -> String {
    let local_file_path: String;
    let args: Vec<String> = args().collect();
    if args[1] == "-p" || args[1] == "--path" {
        local_file_path = args[2].to_string();
        match is_boot {
            true => {
                test_file(&local_file_path);
            }
            false => {
                // Pass
            }
        }
        return local_file_path;
    }
    else if args[1] == "-n" || args[1] == "--name" {
        let dir_path = current_dir().expect("Couldn't fetch current directory");
        let dir_path_string = dir_path.into_os_string().into_string().expect("Failed to convert current directory into String");
        local_file_path = dir_path_string + std::path::MAIN_SEPARATOR_STR + args[2].to_string().as_str();
        match is_boot {
            true => {
                test_file(&local_file_path);
            }
            false => {
                // Pass
            }
        }
        return local_file_path;
    }
    else {
        let dir_path = current_dir().expect("Couldn't fetch current directory");
        let dir_path_string = dir_path.into_os_string().into_string().expect("Failed to convert current directory into String");
        local_file_path = dir_path_string + std::path::MAIN_SEPARATOR_STR + args[1].to_string().as_str();
        match is_boot {
            true => {
                test_file(&local_file_path);
            }
            false => {
                // Pass
            }
        }
        return local_file_path;
    }
}

// Test to see if the file can be found, if not, the program will exit
pub fn test_file(file_path: &String) {
    let file_read = fs::exists(&file_path).expect("File not found");
    match file_read {
        true => {
            // Pass
        }
        false => {
            println!("Couldn't find the file! Try using -h OR --help");
            std::process::exit(0);
        }
    }
}

// Shows the help message
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