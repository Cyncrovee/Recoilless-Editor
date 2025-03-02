# Recoiless Editor Keybinds
Below is an overview of some of the keybinds for the editor (not all are included here, but this should cover most of the commonly used ones).

There are some duplicates (i.e two keybinds that do the same thing), just due to the built-in keybinds in tui-textarea that I'm not sure if I want to remove. However the differences between overview mode and insert mode should make this less of a problem.

## Modes
- i: Switch to insert mode (when in overview mode)
- Esc: Switch to overview mode (when in insert mode)

## Movement (Overview Mode)
- hjkl/arrow keys: Move left, down, up and right
- Space: Move right
- Backspace: Move left
- Ctrl + w: Move forward by word
- Alt + w: Move backward by word
- Ctrl + l: Move forward by line
- Alt + l: Move backward by line
- Ctrl + e: Jump to start of line
- Alt + e: Jump to end of line
- Ctrl + Shift + E: Jump to start of line and enter insert mode
- Alt + Shift + E: Jump to end of line and enter insert mode
- Ctrl + p: Jump forward by paragraph
- Alt + p: Jump back by paragraph
- Ctrl + j: Jump to start of file
- Alt + j: Jump to end of file

## Editing (Overview Mode)
- Ctrl + Alt + c: Delete character
- Ctrl + Alt + w: Delete word (forward)
- Ctrl + Alt + p: Delete paragraph (forward)
- Ctrl + Alt + l: Delete line
- Ctrl + n : Make a new line above current line
- Alt + n : Make a new line below current line
- u: Undo
- r: Redo
- p: Paste

## Editor (Overview Mode)
- End: Exit program
- Ctrl + Alt + Backspace: Exit program
- Ctrl + s: Save file
- Ctrl + Alt + s: Save file and exit program