use std::{env, fs, io::{self, Write}};

use color_eyre::{eyre::Ok, Result};
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use ratatui::{style::Style, widgets::{Block, Borders}, DefaultTerminal};
use tui_textarea::{Input, TextArea, Key};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    // Declare widget(s) and their styling
    let mut input_area: TextArea = TextArea::default();
    input_area.set_line_number_style(Style::default());
    input_area.set_block(
        Block::default()
            .title("Recoilless Editor")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
    );
    
    // Get cli argument(s) and set the file path to the args[1]
    // Then get contents from file and add them to the input_area
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();
    let file_contents = fs::read_to_string(file_path.clone());
    input_area.insert_str(file_contents.unwrap());

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
            Input { key: Key::Char('s'), ctrl: true, ..} => {
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
            input => {
                input_area.input(input);
                is_modified = true;
            }
        }
    }
}