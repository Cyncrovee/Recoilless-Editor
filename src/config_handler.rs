use configparser::ini::Ini;
use dirs::home_dir;
use ratatui::style::Style;
use tui_textarea::TextArea;

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
    let mut linenumber = "empty".to_string();
    let mut hardtab= "empty".to_string();
    // Get linenumber from config if applicable
    match config.get("main", "linenumber") {
        Some(_) => {
            linenumber = config.get("main", "linenumber").unwrap();
        }
        None => {
            //
        }
    };
    // Get hardtab from config if applicable
    match config.get("main", "hardtab") {
        Some(_) => {
            hardtab = config.get("main", "hardtab").unwrap();
        }
        None => {
            //
        }
    };

    match config.get("main", "linenumber") {
        Some(_) => {
            linenumber = config.get("main", "linenumber").unwrap();
        }
        None => {
            //
        }
    };
    // Get hardtab from config if applicable
    match config.get("main", "hardtab") {
        Some(_) => {
            hardtab = config.get("main", "hardtab").unwrap();
        }
        None => {
            //
        }
    };

    match hardtab.as_str() {
        "true" => {
            input_area.set_hard_tab_indent(true);
        }
        &_ => {
            // Pass
        }
    }
    match linenumber.as_str() {
        "false" => {
            // Pass
        }
        &_ => {
            input_area.set_line_number_style(Style::default().fg(ratatui::style::Color::LightCyan));
        }
    }
}