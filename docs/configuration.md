# Configuration
Recoilless Editor supports basic configuration of the editor.

The configuration file is located in homedir/.config/recoilless/rcl_config.txt
On Linux this is:
```
/home/username/.config/recoilless/rcl_config.txt
```
On Windows this is:
```
C:\Users\username\.config\recoilless\rcl_config.txt
```

You may need to create the '.config' and 'recoilless' directories if they do not already exist.

The configuration file itself work silimarly to TOML (however keep in mind it is not actually TOML).
Everything should be listed under the [main] section.
For example:
```
[main]
linenumber = false
```
This will disable the line numbers in the editor (which are enabled by default)

Below are all the currently avaiable options for configuration:

| Option     | Values                 |
| ---------- | ---------------------- |
| linenumber | false (default: true)  |