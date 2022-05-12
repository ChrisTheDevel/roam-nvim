# roam-nvim development notes

# Features
## Plugin overal - all features of plugin
- conceal metadata blocks in markdown files - lua side of plugin
- conceal links - lua side of plugin
- function to create link with promt of filenames - lua side of plugin (promt + keybinds), gest
  list of paths + titles from rust backend, will save link data to backend
- function to create alias - will insert alias field in yaml header (lua frontend), and save alias
  data to backend (rust)
- function to find node with title and navigate to it. -  promt is done in lua with retreival of
  data done in rust.
- function to get all backlinks. - presentation of data done in lua

in general all presentation of data + promts will be done using lua + some chooser plugin
(telescope being a candiadate). The caching of data and retreival of data will be done by the rust
backend.

# Design
lua frontend that exposes rust functions.
rust backend that handles interaction with sqlite cache

## initialization

lua frontend has setup function to which a table of settings is passed.
This table is then merged with default settings table.
- path to sqlite database (will use default .share/data path)
- name of sqlite database (core.db by default)
- extension of files (md by default)
- were the plugins should be active. Only "nodes_dir" initially, might change
- link type, should []() syntax be used or [[]]? []() initially

setup table can then be passsed to rust backend struct

## How to stay in sync

when starting the plugin the notes directory and the database are compared, for the files not in
sync they are synced. When in sync the database is then loaded and turned into an in memory
representation of the graph.

When a file is written a function is called to check out if that file is in sync with the database
and the graph


# Questions
## How should I handle schema migrations?
use the diesel migrations crate. It will handle updates to the schema automatically.
## How should I expose the functionality of the rust library to the lua plugin?
- have backend be a struct with all subcomponents of lib as fields. Then define all functions that
  the backend will expose as methods on that struct.
  - [ ] how can I expose the backend struct using mlua. By using the UserDataType trait.
## How does error handling over the rust->lua border work?
- If the user providse faulty options the rust backend might not initalize properly, how these
  errors be propagated to the user?
