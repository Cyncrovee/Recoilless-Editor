use std::env::{args, current_dir};
use std::fs;

// Get the first CLI arg and runs the appropriate code
pub fn boot_arg() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(_) => {
            match args[1].as_str() {
                "-h" | "--help" => {
                    show_help();
                }
                "-k" | "--keys" => {
                    show_keybinds();
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

fn show_keybinds() {
    println!("------------------------------------------------------------------------");
    println!("");
    println!("Below is an overview of some of the keybinds for the editor (not all are included here, but this should cover most of the commonly used ones).");
    println!("There are some duplicates (i.e two keybinds that do the same thing), just due to the built-in keybinds in tui-textarea that I'm not sure if I want to remove. However the differences between edit mode and insert mode should make this less of a problem.");
    println!("");
    println!("Modes:");
    println!("- i: Switch to insert mode (when in edit mode)");
    println!("- Esc: Switch to edit mode (when in insert mode)");
    println!("");
    println!("Movement (Edit Mode)");
    println!("- hjkl/arrow keys: Move left, down, up and right");
    println!("- Ctrl + w: Move forward by word");
    println!("- Alt + w: Move backward by word");
    println!("- Ctrl + l: Move forward by line");
    println!("- Alt + l: Move backward by line");
    println!("- Ctrl + e: Jump to start of line");
    println!("- Alt + e: Jump to end of line");
    println!("- Ctrl + p: Jump back by paragraph");
    println!("- Alt + p: Jump forward by paragraph");
    println!("- Ctrl + j: Jump to start of file");
    println!("- Alt + j: Jump to end of file");
    println!("");
    println!("Editing (Edit Mode)");
    println!("- Ctrl + Alt + w: Delete word");
    println!("- Ctrl + Alt + l: Delete line");
    println!("- Ctrl + n :Make a new line below current line");
    println!("- Alt + n :Make a new line above current line");
    println!("- u: Undo");
    println!("- r: Redo");
    println!("- p: Paste");
    println!("");
    println!("Editor (Edit Mode)");
    println!("- Ctrl + home: Exit program");
    println!("- Ctrl + s: Save file");
    println!("- Ctrl + Alt + s: Save file and exit program");
    println!("");
    println!("------------------------------------------------------------------------");
    std::process::exit(0);
}