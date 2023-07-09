# Regexor
A Regex Validator, made with Rust.  
Convert help/man text to their regex counterpart. Use this translation to validate command-line

## Install
Rust :
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

VScode extensions : 
- rust-analyzer
- CodeLLDB

## Build & Execute
`cargo run -- "cp [OPTION]... [-T] SOURCE DEST"`

## Todo
[x] First minimal working version (learn Rust strucutre, tests, ...)  
[ ] handle more complex token (repeatable args, choice delimiter, ...)  
[ ] add other definitions (postgreSql)  