// Import from standard
use std::{fs, path::Path};

// Import from crates
use color_eyre::{eyre::Ok, Result};
use ratatui::{
    layout::{self, Rect},
    style::{Modifier, Style},
    text::Text,
    widgets::{self, Block, Borders, Paragraph},
    DefaultTerminal,
};
use tui_textarea::{CursorMove, Input, Key, TextArea};

// Mod external files
mod cli_handler;
mod config_handler;
mod file_handler;

// Get functions from external files
use cli_handler::{boot_arg, get_file_path};
use config_handler::run_config;
use file_handler::{convert_extension, get_file_size, save_file};

// Setup the struct which will be used for the status bar
struct StatusBarStruct<'a> {
    status_area: Rect,
    status_paragraph: Paragraph<'a>,
    status_text: Text<'a>,
    cursor_line: usize,
    cursor_row: usize,
    last_command: &'a str,
    status_content: String,
    cursor_seperator: &'a str,
    seperator: &'a str,
}

fn setup(
    input_area: TextArea,
    status_bar: StatusBarStruct,
    is_ovr_mode: bool,
    editor_mode: &str,
    is_modified: bool,
    file_path: String,
    file_size: String,
    file_type: &str,
) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(
        terminal,
        input_area,
        status_bar,
        is_ovr_mode,
        editor_mode,
        is_modified,
        file_path,
        file_size,
        file_type,
    );
    ratatui::restore();
    result
}

// This function sets all the variables, widgets and styling that will be used by the run() function
fn main() {
    boot_arg();
    let test = get_file_path();
    drop(test);

    // Set editor mode variables
    let is_ovr_mode = true;
    let editor_mode = "Ovr";
    // Get file path, file size and file type
    let file_path = get_file_path();
    let file_size = file_handler::get_file_size(&file_path);
    let mut file_type: &str = Path::new(&file_path).extension().unwrap().to_str().unwrap();
    // Convert file extension if applicable
    file_type = convert_extension(file_type);
    // Initialise StatusBarStruct
    let status_bar = StatusBarStruct {
        status_area: Rect::new(0, 0, 0, 0),
        status_paragraph: widgets::Paragraph::new("").alignment(ratatui::layout::Alignment::Left),
        status_text: "".into(),
        cursor_line: std::usize::MIN,
        cursor_row: std::usize::MIN,
        last_command: "",
        status_content: "".into(),
        cursor_seperator: ":",
        seperator: " | ",
    };

    // Declare input_area and it's block/styling
    let mut input_area: TextArea = TextArea::default();
    input_area.set_block(
        Block::default()
            .title(file_path.clone())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded),
    );

    // Get contents from file and add them to the input_area
    let file_contents = fs::read_to_string(&file_path);
    input_area.insert_str(file_contents.expect("Failed to unwrap file contents"));

    // Declare a bool that will be true when input_area.input(input); is called (see the input events below)
    // And be false after saving (except when saving and quitting)
    let is_modified = false;

    // Apply user config
    run_config(&mut input_area);

    // Continue to setup()
    let _ = setup(
        input_area,
        status_bar,
        is_ovr_mode,
        editor_mode,
        is_modified,
        file_path.clone(),
        file_size,
        file_type,
    );
}

fn run(
    mut terminal: DefaultTerminal,
    mut input_area: TextArea,
    mut status_bar: StatusBarStruct,
    mut is_ovr_mode: bool,
    mut editor_mode: &str,
    mut is_modified: bool,
    file_path: String,
    mut file_size: String,
    file_type: &str,
) -> Result<()> {
    // Main loop to draw widgets and handle key inputs
    loop {
        terminal.draw(|frame| {
            frame.render_widget(&input_area, frame.area());
            status_bar.status_area = Rect::new(0, frame.area().bottom(), 1000, 1);
            frame.render_widget(
                &status_bar.status_paragraph,
                status_bar.status_area.clamp(frame.area()),
            );
        })?;
        // Get key input(s) and run appropriate functions for said input, or input it to the text area
        match is_ovr_mode {
            false => {
                match crossterm::event::read()?.into() {
                    Input { key: Key::Esc, .. } => {
                        is_ovr_mode = true;
                        editor_mode = "Ovr";
                        input_area
                            .set_cursor_style(Style::default().bg(ratatui::style::Color::Reset));
                        input_area.set_cursor_style(
                            Style::default()
                                .fg(ratatui::style::Color::Reset)
                                .add_modifier(Modifier::REVERSED),
                        );
                    }
                    Input {
                        key: Key::Char('a'),
                        ctrl: true,
                        ..
                    } => {
                        input_area.select_all();
                    }
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
                    // Exit program, either via end key or Ctrl + Alt + Backspace
                    Input { key: Key::End, .. } => break Ok(()),
                    Input {
                        key: Key::Backspace,
                        ctrl: true,
                        alt: true,
                        ..
                    } => break Ok(()),
                    // Go to insert mode
                    Input {
                        key: Key::Char('i'),
                        ..
                    } => {
                        is_ovr_mode = false;
                        editor_mode = "Ins";
                        input_area.set_cursor_style(
                            Style::default().bg(ratatui::style::Color::LightCyan),
                        );
                        input_area.set_cursor_style(
                            Style::default()
                                .fg(ratatui::style::Color::LightCyan)
                                .add_modifier(Modifier::REVERSED),
                        );
                    }
                    // Save file
                    Input {
                        key: Key::Char('s'),
                        ctrl: true,
                        alt: false,
                        ..
                    } => {
                        save_file(&is_modified, &file_path, &mut input_area);
                        file_size = get_file_size(&file_path);
                        is_modified = false;
                    }
                    // Save file and exit
                    Input {
                        key: Key::Char('s'),
                        ctrl: true,
                        alt: true,
                        ..
                    } => {
                        save_file(&is_modified, &file_path, &mut input_area);
                        break Ok(());
                    }
                    // General movement (hjkl, arrow keys)
                    Input {
                        key: Key::Char('h'),
                        ..
                    } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Back);
                        status_bar.last_command = "| h";
                    }
                    Input {
                        key: Key::Char('j'),
                        ctrl: false,
                        alt: false,
                        ..
                    } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Down);
                        status_bar.last_command = "| j";
                    }
                    Input {
                        key: Key::Char('k'),
                        ..
                    } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Up);
                        status_bar.last_command = "| k";
                    }
                    Input {
                        key: Key::Char('l'),
                        ctrl: false,
                        alt: false,
                        ..
                    } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Forward);
                        status_bar.last_command = "| l";
                    }
                    Input { key: Key::Left, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Back);
                    }
                    Input { key: Key::Down, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Down);
                    }
                    Input { key: Key::Up, .. } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Up);
                    }
                    Input {
                        key: Key::Right, ..
                    } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Forward);
                    }
                    Input {
                        key: Key::Char(' '),
                        ..
                    } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Forward);
                        status_bar.last_command = "| >";
                    }
                    Input {
                        key: Key::Backspace,
                        ..
                    } => {
                        input_area.cancel_selection();
                        input_area.move_cursor(CursorMove::Back);
                        status_bar.last_command = "| <";
                    }
                    // Delete char
                    Input {
                        key: Key::Char('c'),
                        ctrl: true,
                        alt: true,
                        ..
                    } => {
                        input_area.delete_next_char();
                        status_bar.last_command = "| DEL-CHAR";
                    }
                    // Move around by word
                    Input {
                        key: Key::Char('w'),
                        ctrl: true,
                        alt: false,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::WordForward);
                        status_bar.last_command = "| WORD-FOR";
                    }
                    Input {
                        key: Key::Char('w'),
                        ctrl: false,
                        alt: true,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::WordBack);
                        status_bar.last_command = "| WORD-BACK";
                    }
                    // Delete word (forward)
                    Input {
                        key: Key::Char('w'),
                        ctrl: true,
                        alt: true,
                        ..
                    } => {
                        input_area.delete_next_word();
                        status_bar.last_command = "| DEL-WORD";
                    }
                    // Move around by line
                    Input {
                        key: Key::Char('l'),
                        ctrl: true,
                        alt: false,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::Down);
                        status_bar.last_command = "| LINE-FOR";
                    }
                    Input {
                        key: Key::Char('l'),
                        ctrl: false,
                        alt: true,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::Up);
                        status_bar.last_command = "| LINE-BACK";
                    }
                    // Delete line
                    Input {
                        key: Key::Char('l'),
                        ctrl: true,
                        alt: true,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::Head);
                        input_area.delete_line_by_end();
                        status_bar.last_command = "| DEL-LINE";
                    }
                    // Make a newline
                    Input {
                        key: Key::Char('n'),
                        ctrl: true,
                        alt: false,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::Up);
                        input_area.move_cursor(CursorMove::End);
                        input_area.insert_newline();
                        status_bar.last_command = "| NEW-LINE-UP";
                    }
                    Input {
                        key: Key::Char('n'),
                        ctrl: false,
                        alt: true,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::End);
                        input_area.insert_newline();
                        status_bar.last_command = "| NEW-LINE-DOWN";
                    }
                    // Jump to start/end of line
                    Input {
                        key: Key::Char('e'),
                        ctrl: true,
                        alt: false,
                        shift: false,
                    } => {
                        input_area.move_cursor(CursorMove::Head);
                        status_bar.last_command = "| JUMP-LINE-START";
                    }
                    Input {
                        key: Key::Char('e'),
                        ctrl: false,
                        alt: true,
                        shift: false,
                    } => {
                        input_area.move_cursor(CursorMove::End);
                        status_bar.last_command = "| JUMP-LINE-END";
                    }
                    // Jump to start/end of line and enter insert mode
                    Input {
                        key: Key::Char('E'),
                        ctrl: true,
                        alt: false,
                        shift: true,
                    } => {
                        input_area.move_cursor(CursorMove::Head);
                        status_bar.last_command = "| JUMP-LINE-START";

                        is_ovr_mode = false;
                        editor_mode = "Ins";
                        input_area.set_cursor_style(
                            Style::default().bg(ratatui::style::Color::LightCyan),
                        );
                        input_area.set_cursor_style(
                            Style::default()
                                .fg(ratatui::style::Color::LightCyan)
                                .add_modifier(Modifier::REVERSED),
                        );
                    }
                    Input {
                        key: Key::Char('E'),
                        ctrl: false,
                        alt: true,
                        shift: true,
                    } => {
                        input_area.move_cursor(CursorMove::End);
                        status_bar.last_command = "| JUMP-LINE-END";

                        is_ovr_mode = false;
                        editor_mode = "Ins";
                        input_area.set_cursor_style(
                            Style::default().bg(ratatui::style::Color::LightCyan),
                        );
                        input_area.set_cursor_style(
                            Style::default()
                                .fg(ratatui::style::Color::LightCyan)
                                .add_modifier(Modifier::REVERSED),
                        );
                    }
                    // Jump to start/end of paragraph
                    Input {
                        key: Key::Char('p'),
                        ctrl: true,
                        alt: false,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::ParagraphForward);
                        status_bar.last_command = "| JUMP-PAR-FOR";
                    }
                    Input {
                        key: Key::Char('p'),
                        ctrl: false,
                        alt: true,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::ParagraphBack);
                        status_bar.last_command = "| JUMP-PAR-BACK";
                    }
                    // Delete paragraph (forward)
                    Input {
                        key: Key::Char('p'),
                        ctrl: true,
                        alt: true,
                        ..
                    } => {
                        input_area.start_selection();
                        input_area.move_cursor(CursorMove::ParagraphForward);
                        input_area.cut();
                        input_area.cancel_selection();
                        status_bar.last_command = "| DEL-PAR-FOR";
                    }
                    // Jump to start/end of file
                    Input {
                        key: Key::Char('j'),
                        ctrl: true,
                        alt: false,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::Top);
                        status_bar.last_command = "| JUMP-FILE-START";
                    }
                    Input {
                        key: Key::Char('j'),
                        ctrl: false,
                        alt: true,
                        ..
                    } => {
                        input_area.move_cursor(CursorMove::Bottom);
                        status_bar.last_command = "| JUMP-FILE-END";
                    }
                    // Undo
                    Input {
                        key: Key::Char('u'),
                        ..
                    } => {
                        input_area.undo();
                        status_bar.last_command = "| UNDO";
                    }
                    // Redo
                    Input {
                        key: Key::Char('r'),
                        ..
                    } => {
                        input_area.redo();
                        status_bar.last_command = "| REDO";
                    }
                    // Paste
                    Input {
                        key: Key::Char('p'),
                        ..
                    } => {
                        input_area.paste();
                        status_bar.last_command = "| PASTE";
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
        status_bar.status_content = format!("{cursor_line}{cursor_seperator}{cursor_row}{seperator}{editor_mode}{seperator}{file_type}{seperator}{file_size}{last_command}",
            cursor_line = &status_bar.cursor_line, cursor_row = &status_bar.cursor_row, last_command = &status_bar.last_command, cursor_seperator = &status_bar.cursor_seperator, seperator = &status_bar.seperator);
        status_bar.status_text = Text::from(status_bar.status_content);
        status_bar.status_paragraph =
            widgets::Paragraph::new(status_bar.status_text).alignment(layout::Alignment::Left);
    }
}
