use configparser::ini::Ini;
use dirs::home_dir;
use ratatui::style::Style;
use tui_textarea::{CursorMove, TextArea};

pub fn parse_config() -> Ini {
    let lesser_config_path = "\\.config\\recoilless\\rcl_config.txt";
    let mut config_path = home_dir().unwrap().into_os_string();
    // Join (lesser_)config_path into one var
    config_path.push(&lesser_config_path);
    let mut config_main = Ini::new();
    let _config = config_main.load(&config_path);
    return config_main;
}

pub fn run_config(input_area: &mut TextArea) {
    let config = parse_config();
    let mut linenumber = "true".to_string();
    let mut hardtab= "false".to_string();
    let mut _tablength_string= "empty".to_string();
    let mut tablength_int: u8 = 0;
    let mut cursorstart = "true".to_string();
    // Get linenumber from config if applicable
    match config.get("main", "linenumber") {
        Some(_) => {
            linenumber = config.get("main", "linenumber").unwrap();
        }
        None => {
            // Pass
        }
    };
    match config.get("main", "cursorstart") {
        Some(_) => {
            cursorstart = config.get("main", "cursorstart").unwrap();
        }
        None => {
            // Pass
        }
    }
    // Get hardtab from config if applicable
    match config.get("main", "hardtab") {
        Some(_) => {
            hardtab = config.get("main", "hardtab").unwrap();
        }
        None => {
            // Pass
        }
    };
    // Get tab length from config if applicable
    match config.get("main", "tablength") {
        Some(_) => {
            _tablength_string = config.get("main", "tablength").unwrap();
            match _tablength_string.parse::<u8>() {
                Ok(ok_res) => tablength_int = ok_res,
                Err(_) => {
                    // Pass
                }
            }
        }
        None => {
            // Pass
        }
    };

    // Set config options if able/applicable
    match linenumber.as_str() {
        "false" => {
            // Pass
        }
        &_ => {
            input_area.set_line_number_style(Style::default().fg(ratatui::style::Color::LightCyan));
        }
    }
    match cursorstart.as_str() {
        "false" => {
            input_area.move_cursor(CursorMove::End);
            input_area.move_cursor(CursorMove::Bottom);
        }
        &_ => {
            input_area.move_cursor(CursorMove::Head);
            input_area.move_cursor(CursorMove::Top);
        }
    }
    match hardtab.as_str() {
        "true" => {
            input_area.set_hard_tab_indent(true);
        }
        &_ => {
            // Pass
        }
    }
    match tablength_int {
        tablength_int if tablength_int >= 1 => {
            input_area.set_tab_length(tablength_int);
        }
        0 => {
            input_area.set_tab_length(4);
        }
        _ => {
            input_area.set_tab_length(4);
        }
    }
}