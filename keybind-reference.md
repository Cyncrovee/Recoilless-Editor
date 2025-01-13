# Recoiless Editor Keybinds
Below is an overview of some of the keybinds for the editor (not all are included here, but this should cover most of the commonly used ones).

There are some duplicates (i.e two keybinds that do the same thing), just due to the built-in keybinds in tui-textarea that I'm not sure if I want to remove. However the differences between edit mode and insert mode should make this less of a problem.

## Modes
- i: Switch to insert mode (when in edit mode)
- Esc: Switch to edit mode (when in insert mode)

## Movement (Edit Mode)
- hjkl/arrow keys: Move left, down, up and right
- Ctrl + w: Move forward by word
- Alt + w: Move backward by word
- Ctrl + l: Move forward by line
- Alt + l: Move backward by line
- Ctrl + e: Jump to start of line
- Alt + e: Jump to end of line
- Ctrl + j: Jump to start of file
- Alt + j: Jump to end of file

## Editing (Edit Mode)
- Ctrl + Alt + w: Delete word
- Ctrl + Alt + l: Delete line
- Ctrl + n :Make a new line below current line
- Alt + n :Make a new line above current line
- u: Undo
- r: Redo
- p: Paste

## Editor (Edit Mode)
- Ctrl + home: Exit program
- Ctrl + s: Save file
- Ctrl + Alt + s: Save file and exit program