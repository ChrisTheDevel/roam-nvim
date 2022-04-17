# rs-core development notes

# Why rust
The use of rust for this plugin is probably overkill and might even be less performant than a fully lua based plugin. I'm writing this in rust + lua (and that one required vimscript file) for my own sake since I want to get more comfy with the language.

# Design
lua frontend that handles keybinds, calling of rust functions, hiding away yaml metadata in markdown buffers.

rust backend that handles interaction with sqlite cache

## Communication between ends

### initialization
lua frontend ---- gives path were the cache sqlite database should be ----> rust backend
lua frontend -- provides the name of the notes directory --->
- the rust backend then handles the case of if the database does not exist/is out of sync.

## Structure of structs
All interactions with the database will be through some handle provided by the diesel crate. The easiest way to expose these interactions with the rest of the library will be by placing this handle in a struct.

## Features
- parse note dir and save all link information

# TODO
- [ ] implement struct that wraps the database.
- [ ] implement function that returns all links of document
- [ ] implement function that returns all files in notes directory together with their atime and mtime (accessed and modified).

# Questions
## How should I handle schema migrations?
The easiest way would probably be just throw away the old database, create a new one and then repopulate with the data.
Since the main source of the data ought to be the documents rather than the database this should be safe to do.
