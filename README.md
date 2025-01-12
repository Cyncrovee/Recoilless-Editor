# Recoilless Editor
A free and open source TUI text editor

## Usage
### WARNING (IMPORTANT)
When resizing the window with the program open, it may flicker! Be careful when using it if you have photosensitive epilepsy and/or a similar condition.
### Prerequisites
- Rust installed on the system
### Running
- CD into the repository, and then run
```shell
cargo run -- "file-path"
```
- Replace "file-path" with the path to the file to edit. Speech marks should only be needed if the file path contains one or more spaces

## Features:
- A status bar showing some basic information about the editor/file
- Line numbers
- Modifier key based keybinds (refer to the keybind-reference.md file)

## Misc Info
This is a [Ratatui] app made based off the [Hello World template].

[Ratatui]: https://ratatui.rs
[Hello World Template]: https://github.com/ratatui/templates/tree/main/hello-world

## License

Copyright (c) Cyncrovee <86919717+Cyncrovee@users.noreply.github.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
