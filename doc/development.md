# rs-core development notes

# Why rust
The use of rust for this plugin is probably overkill and might even be less performant than a fully
lua based plugin. I'm writing this in rust + lua (and that one required vimscript file) for my own
sake since I want to get more comfy with the language.

# Design
lua frontend that handles keybinds, calling of rust functions.

rust backend that handles interaction with sqlite cache

## Communication between ends

### initialization
lua frontend ---- gives path were the cache sqlite database should be ----> rust backend
lua frontend -- provides the name of the notes directory --->
- the rust backend then handles the case of if the database does not exist/is out of sync.
