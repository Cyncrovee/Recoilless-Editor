use std::{
    fs,
    io::{self, Write},
};
use tui_textarea::TextArea;

// Fetches the file's size
pub fn get_file_size(file_path: &String) -> String {
    let mut file_size = fs::File::open(&file_path)
        .expect("Failed to open file in get_file_size")
        .metadata()
        .expect("Failed to get file metadata in get_file_size")
        .len()
        .to_string();
    file_size.push_str(" Bytes Saved ");
    return file_size;
}

// Save the current file (if is_modified is true)
pub fn save_file(is_modified: &bool, file_path: &String, input_area: &mut TextArea) {
    match is_modified {
        true => {
            let mut writer = io::BufWriter::new(
                fs::File::create(&file_path).expect("Failed to create BufWriter!"),
            );
            for l in input_area.lines() {
                writer
                    .write_all(l.as_bytes())
                    .expect("Error when writing to file!");
                writer
                    .write_all(b"\n")
                    .expect("Error when writing to file!");
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
        // Configuration Files
        "cfg" => {
            file_type = "CFG File";
            return file_type;
        }
        "json" => {
            file_type = "JSON File";
            return file_type;
        }
        "toml" => {
            file_type = "TOML File";
            return file_type;
        }
        "yaml" => {
            file_type = "YAML FILE";
            return file_type;
        }
        "ini" => {
            file_type = "INI File";
            return file_type;
        }
        "csv" => {
            file_type = "Comma Seperated Values File";
            return file_type;
        }
        // Git Specific Files
        "gitignore" => {
            file_type = "Git Ignore File";
            return file_type;
        }
        "gitattributes" => {
            file_type = "Git Attributes File";
            return file_type;
        }
        // UI/Markup Files
        "md" => {
            file_type = "Markdown File";
            return file_type;
        }
        "xml" => {
            file_type = "XML File";
            return file_type;
        }
        "xaml" => {
            file_type = "XAML File";
            return file_type;
        }
        "axaml" => {
            file_type = "AXAML File";
            return file_type;
        }
        "html" => {
            file_type = "HTML File";
            return file_type;
        }
        "xhtml" => {
            file_type = "XHTML File";
            return file_type;
        }
        "css" => {
            file_type = "CSS File";
            return file_type;
        }
        // Script Files
        "sh" => {
            file_type = "Shell Script";
            return file_type;
        }
        "ps1" => {
            file_type = "Powershell Script";
            return file_type;
        }
        "bat" => {
            file_type = "Batch File";
            return file_type;
        }
        // Programmins Source Files
        "c" => {
            file_type = "C Source File";
            return file_type;
        }
        "h" => {
            file_type = "C/C++ Source File";
            return file_type;
        }
        "cpp" | "C" | "cc" | "cxx" | "c++" | "H" | "hh" | "hpp" | "hxx" | "h++" | "cppm"
        | "ixx" => {
            file_type = "C++ Source File";
            return file_type;
        }
        "r" => {
            file_type = "R Source File";
            return file_type;
        }
        "scala" | "sc" => {
            file_type = "Scala Source File";
            return file_type;
        }
        "ml" => {
            file_type = "OCaml File";
            return file_type;
        }
        "mll" => {
            file_type = "OCamllex File";
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
        "fs" => {
            file_type = "F# Source File";
            return file_type;
        }
        "hs" => {
            file_type = "Haskell";
            return file_type;
        }
        "erl" => {
            file_type = "Erlang Source File";
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
        "swift" => {
            file_type = "Swift Souce File";
            return file_type;
        }
        "dart" => {
            file_type = "Dart Source File";
            return file_type;
        }
        "pl" => {
            file_type = "Perl Source File";
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
