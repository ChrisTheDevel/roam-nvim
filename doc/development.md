# roam-nvim development notes

# Design
lua frontend that exposes rust functions.
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

# controll flow for cache updates

# Questions
## How should I handle schema migrations?
use the diesel migrations crate. It will handle updates to the schema automatically.
