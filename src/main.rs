use std::{env, fs, io::{self, Write}};

use color_eyre::{eyre::Ok, Result};
use ratatui::{style::Style, widgets::{Block, Borders}, DefaultTerminal};
use tui_textarea::{Input, TextArea, Key, CursorMove};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    // Set file path
    let file_path = get_file_path();

    // Declare widget(s) and their styling
    let mut input_area: TextArea = TextArea::default();
    input_area.set_line_number_style(Style::default().fg(ratatui::style::Color::LightCyan));
    input_area.set_block(
        Block::default()
            .title(file_path.clone())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
    );

    // Get contents from file and add them to the input_area
    let file_contents = fs::read_to_string(file_path.clone());
    input_area.insert_str(file_contents.expect("Failed to unwrap file contents"));
    // Declare a bool that will change depending on if the file is modified
    let mut is_modified = false;

    // Main loop to draw widgets and handle key inputs
    loop {
        terminal.draw(|frame| {
            frame.render_widget(&input_area, frame.area());
        })?;
        // Get key input(s) and run appropriate functions for said input, or input it to the text area
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, ..} => break Ok(()),
            Input { key: Key::Char('a'), ctrl: true, ..} => {
                input_area.select_all();
            },
            // Save file
            Input { key: Key::Char('s'), ctrl: true, alt: false, ..} => {
                match is_modified {
                    true => {
                        let mut writer = io::BufWriter::new(fs::File::create(file_path.clone())?);
                        for l in input_area.lines(){
                            writer.write_all(l.as_bytes())?;
                            writer.write_all(b"\n")?;
                        }
                        is_modified = false;
                        drop(writer);
                    }
                    false => {
                        // Pass
                    }
                }
            },
            // Save file and quit
            Input { key: Key::Char('s'), ctrl: true, alt: true, ..} => {
                match is_modified {
                    true => {
                        let mut writer = io::BufWriter::new(fs::File::create(file_path.clone())?);
                        for l in input_area.lines() {
                            writer.write_all(l.as_bytes())?;
                            writer.write_all(b"\n")?;
                        }
                        drop(writer);
                        break Ok(());
                    }
                    false => {
                        // Pass
                    }
                }
            }
            // Paste
            Input { key: Key::Char('p'), ctrl: true, ..} => {
                input_area.paste();
            }
            // Make a newline
            Input { key: Key::Char('n'), ctrl: true, alt: false, ..} => {
                input_area.move_cursor(CursorMove::End);
                input_area.insert_newline();
            },
            Input { key: Key::Char('n'), alt: true, ctrl: false, ..} => {
                input_area.move_cursor(CursorMove::Up);
                input_area.move_cursor(CursorMove::End);
                input_area.insert_newline();
            }
            // Move around by word
            Input { key: Key::Char('w'), ctrl: true, alt: false, ..} => {
                input_area.move_cursor(CursorMove::WordForward);
            }
            Input { key: Key::Char('w'), alt: true, ctrl: false, ..} => {
                input_area.move_cursor(CursorMove::WordBack);
            }
            // Delete word
            Input { key: Key::Char('w'), alt: true, ctrl: true, ..} => {
                input_area.delete_next_word();
            }
            // Move around by line
            Input { key: Key::Char('l'), ctrl: true, alt: false, ..} => {
                input_area.move_cursor(CursorMove::Up);
            }
            Input { key: Key::Char('l'), alt: true, ctrl: false, ..} => {
                input_area.move_cursor(CursorMove::Down);
            }
            // Delete line
            Input { key: Key::Char('l'), alt: true, ctrl: true, ..} => {
                input_area.move_cursor(CursorMove::Head);
                input_area.delete_line_by_end();
            }
            // Jump to start/end of line
            Input { key: Key::Char('e'), ctrl: true, alt: false, ..} => {
                input_area.move_cursor(CursorMove::Head);
            }
            Input { key: Key::Char('e'), alt: true, ctrl: false, ..} => {
                input_area.move_cursor(CursorMove::End);
            }
            // Jump to start/end of file
            Input { key: Key::Char('j'), ctrl: true, alt: false, ..} => {
                input_area.move_cursor(CursorMove::Top);
            }
            Input { key: Key::Char('j'), alt: true, ctrl: false, ..} => {
                input_area.move_cursor(CursorMove::Bottom);
            }
            input => {
                input_area.input(input);
                is_modified = true;
            }
        }
    }
}

// Get cli argument(s) and return the file path
fn get_file_path() -> String {
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