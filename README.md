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
Build and execute with cargo : `cargo run -- <args>`  
Example : `cargo run -- -i "cp [OPTION]... [-T] SOURCE DEST"`

Run the tool manually (must be built first) : `./regexor <args>`  
Example : `./regexor -i "cp [OPTION]... [-T] SOURCE DEST"`

## Todo
- [x] First minimal working version (learn Rust strucutre, tests, ...)  
- [ ] handle more complex token (repeatable args, choice delimiter, ...)  
- [ ] interactive mode ?
- [ ] add other definitions (postgreSql)  
