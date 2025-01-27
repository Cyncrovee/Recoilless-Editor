use std::{fs, path::Path};

use color_eyre::{eyre::Ok, Result};
use ratatui::{layout::{self, Rect}, style::Style, text::Text, widgets::{self, Block, Borders, Paragraph}, DefaultTerminal};
use tui_textarea::{Input, TextArea, Key, CursorMove};

mod cli_handler;
mod file_handler;

use cli_handler::{boot_arg, get_file_path};
use file_handler::{get_file_size, save_file, parse_config, convert_extension};

struct StatusBarStruct<'a> {
    status_area: Rect,
    status_paragraph: Paragraph<'a>,
    status_text: Text<'a>,
    cursor_line: usize,
    cursor_row: usize,
    status_content: String,
    cursor_seperator: &'a str,
    seperator: &'a str
}

fn main() -> Result<()> {
    boot_arg();
    let test = get_file_path();
    drop(test);
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let config = parse_config();
    let mut linenumber = "empty".to_string();
    match config.get("main", "linenumber") {
        Some(_) => {
            linenumber = config.get("main", "linenumber").unwrap();
        }
        None => {
            //
        }
    };

    // Set editor mode variables
    let mut is_edit_mode = true;
    let mut editor_mode = "Ovr";
    // Get file path, file size and file type
    let file_path = get_file_path();
    let mut file_size = file_handler::get_file_size(&file_path);
    let mut file_type: &str = Path::new(&file_path).extension().unwrap().to_str().unwrap();
    // Convert file extension if applicable
    file_type = convert_extension(file_type);
    // Initialise StatusBarStruct
    let mut status_bar = StatusBarStruct {
        status_area: Rect::new(0, 0, 0, 0),
        status_paragraph: widgets::Paragraph::new("").alignment(ratatui::layout::Alignment::Left),
        status_text: "".into(),
        cursor_line: std::usize::MIN,
        cursor_row: std::usize::MIN,
        status_content: "".into(),
        cursor_seperator: ":",
        seperator: " | "
    };

    // Declare input_area and it's block/styling
    let mut input_area: TextArea = TextArea::default();
    match linenumber.as_str() {
        "false" => {
            // Pass
        }
        &_ => {
            input_area.set_line_number_style(Style::default().fg(ratatui::style::Color::LightCyan));
        }
    }
    input_area.set_block(
        Block::default()
            .title(file_path.clone())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
    );

    // Get contents from file and add them to the input_area
    let file_contents = fs::read_to_string(&file_path);
    input_area.insert_str(file_contents.expect("Failed to unwrap file contents"));
    
    // Declare a bool that will be true when input_area.input(input); is called (see the input events below)
    // And be false after saving (except when saving and quitting)
    let mut is_modified = false;

    // Main loop to draw widgets and handle key inputs
    loop {
        terminal.draw(|frame| {
            frame.render_widget(&input_area, frame.area());
            status_bar.status_area = Rect::new(0, frame.area().bottom(), 1000, 1);
            frame.render_widget(&status_bar.status_paragraph, status_bar.status_area.clamp(frame.area()));
        })?;
        // Get key input(s) and run appropriate functions for said input, or input it to the text area
        match is_edit_mode {
            false => {
                match crossterm::event::read()?.into() {
                    Input { key: Key::Esc, .. } => {
                        is_edit_mode = true;
                        editor_mode = "Ovr"
                    },
                    Input { key: Key::Char('a'), ctrl: true, .. } => {
                        input_area.select_all();
                    },
                    input => {
                        // Add input to input_area
                        input_area.input(input);
                        // Change is_modified to true, in case a change was made to the input_area
                        is_modified = true;
                    }
                }
            }
            true => {
                match crossterm::event::read()?.into() {
                    // Exit program
                    Input { key: Key::End, .. } => break Ok(()),
                    Input { key: Key::Backspace, ctrl: true, alt: true, .. } => break Ok(()),
                    // Go to insert mode
                    Input { key: Key::Char('i'), .. } => {
                        is_edit_mode = false;
                        editor_mode = "Ins";
                    },
                    // Save file
                    Input { key: Key::Char('s'), ctrl: true, alt: false, .. } => {
                        save_file(&is_modified, &file_path, &mut input_area);
                        file_size = get_file_size(&file_path);
                        is_modified = false;
                    },
                    // Save file and exit
                    Input { key: Key::Char('s'), ctrl: true, alt: true, .. } => {
                        save_file(&is_modified, &file_path, &mut input_area);
                        break Ok(());
                    },
                    // General movement (hjkl, arrow keys)
                    Input { key: Key::Char('h'), .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Back);
                    },
                    Input { key: Key::Char('j'), ctrl: false, alt: false, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Down);
                    },
                    Input { key: Key::Char('k'), .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Up);
                    },
                    Input { key: Key::Char('l'), ctrl: false, alt: false, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Forward);
                    },
                    Input { key: Key::Left, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Back);
                    },
                    Input { key: Key::Down, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Down);
                    },
                    Input { key: Key::Up, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Up);
                    },
                    Input { key: Key::Right, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Forward);
                    },
                    // Delete char
                    Input { key: Key::Char('c'), ctrl: true, alt: true, ..} => {
                        input_area.delete_next_char();
                    }
                    // Move around by word
                    Input { key: Key::Char('w'), ctrl: true, alt: false, ..} => {
                        input_area.move_cursor(CursorMove::WordForward);
                    }
                    Input { key: Key::Char('w'), ctrl: false, alt: true, ..} => {
                        input_area.move_cursor(CursorMove::WordBack);
                    }
                    // Delete word
                    Input { key: Key::Char('w'), ctrl: true, alt: true, ..} => {
                        input_area.delete_next_word();
                    }
                    // Move around by line
                    Input { key: Key::Char('l'), ctrl: true, alt: false, ..} => {
                        input_area.move_cursor(CursorMove::Up);
                    }
                    Input { key: Key::Char('l'), ctrl: false, alt: true, ..} => {
                        input_area.move_cursor(CursorMove::Down);
                    }
                    // Delete line
                    Input { key: Key::Char('l'), ctrl: true, alt: true, ..} => {
                        input_area.move_cursor(CursorMove::Head);
                        input_area.delete_line_by_end();
                    }
                    // Make a newline
                    Input { key: Key::Char('n'), ctrl: true, alt: false, ..} => {
                        input_area.move_cursor(CursorMove::End);
                        input_area.insert_newline();
                    },
                    Input { key: Key::Char('n'), ctrl: false, alt: true, ..} => {
                        input_area.move_cursor(CursorMove::Up);
                        input_area.move_cursor(CursorMove::End);
                        input_area.insert_newline();
                    }
                    // Jump to start/end of line
                    Input { key: Key::Char('e'), ctrl: true, alt: false, ..} => {
                        input_area.move_cursor(CursorMove::Head);
                    }
                    Input { key: Key::Char('e'), ctrl: false, alt: true, ..} => {
                        input_area.move_cursor(CursorMove::End);
                    }
                    // Jump to start/end of line
                    Input { key: Key::Char('p'), ctrl: true, alt: false, ..} => {
                        input_area.move_cursor(CursorMove::ParagraphBack);
                    }
                    Input { key: Key::Char('p'), ctrl: false, alt: true, ..} => {
                        input_area.move_cursor(CursorMove::ParagraphForward);
                    }
                    // Jump to start/end of file
                    Input { key: Key::Char('j'), ctrl: true, alt: false, ..} => {
                        input_area.move_cursor(CursorMove::Top);
                    }
                    Input { key: Key::Char('j'), ctrl: false, alt: true, ..} => {
                        input_area.move_cursor(CursorMove::Bottom);
                    }
                    // Undo
                    Input { key: Key::Char('u'), ..} => {
                        input_area.undo();
                    }
                    // Redo
                    Input { key: Key::Char('r'), ..} => {
                        input_area.redo();
                    }
                    // Paste
                    Input { key: Key::Char('p'), ..} => {
                        input_area.paste();
                    }
                    _input => {
                        // Change is_modified to true, in case a change was made to the input_area
                        is_modified = true;
                    }
                }
            }
        }
        // Update the status bar
        status_bar.cursor_line = &input_area.cursor().0 + 1;
        status_bar.cursor_row = &input_area.cursor().1 + 1;
        status_bar.status_content = format!("{cursor_line}{cursor_seperator}{cursor_row}{seperator}{editor_mode}{seperator}{file_type}{seperator}{file_size}",
            cursor_line = &status_bar.cursor_line, cursor_row = &status_bar.cursor_row, cursor_seperator = &status_bar.cursor_seperator, seperator = &status_bar.seperator);
        status_bar.status_text = Text::from(status_bar.status_content);
        status_bar.status_paragraph = widgets::Paragraph::new(status_bar.status_text)
            .alignment(layout::Alignment::Left);
    }
}
