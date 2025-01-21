use std::env::args;
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
pub fn get_file_path() -> String {
    let args: Vec<String> = args().collect();
    let input_path = args[1].to_string();
    match fs::exists(&input_path).unwrap() {
        true => {
            let full_input_path = std::path::absolute(&input_path).unwrap();
			return full_input_path.into_os_string().into_string().unwrap();
            // return input_path_global;
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
    println!("To open a file in Recoilless Editor, you can simply add the name or path to the file as the first argument");
	println!("This works from the current working directory or the absoloute path");
	println!("");
	println!("When running with cargo, you can add arguments like this:");
	println!("cargo run -- example_arg");
	println!("");
	println!("To exit the program without saving, press the 'end' key or Ctrl + Alt + Backspace, possibly near the page down/home/del keys");
	println!("To exit the program with saving, press the Ctrl + Alt + s keys");
	println!("For more keybinds, refer to the keybind reference file or run the program with -k or --keys as the first argument");
    println!("------------------------------------------------------------------------");
    std::process::exit(0);
}

// Shows the keybinds
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
    println!("End: Exit program");
    println!("Ctrl + Alt + Backspace: Exit program");
    println!("- Ctrl + s: Save file");
    println!("- Ctrl + Alt + s: Save file and exit program");
    println!("");
    println!("------------------------------------------------------------------------");
    std::process::exit(0);
}
