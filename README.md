# Regexor
A Regex Validator, made with Rust.  
Convert help/man texts into their regex counterpart. The translation is used to validate command-line inputs.

## Install
Rust :
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

VScode extensions : 
- rust-analyzer
- CodeLLDB

## Build & Execute
Build and execute the tool with the command : `cargo run -- "<input>"`  
The `<input>` value must be a valid *man* synopsis.

Example : `cargo run -- "cp [OPTION]... [-T] SOURCE DEST"`

## Todo
- [x] First minimal working version (learn Rust strucutre, tests, ...)  
- [ ] handle more complex token (repeatable args, choice delimiter, ...)  
- [ ] add other definitions (postgreSql)  
