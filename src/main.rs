
mod structs;
use crate::structs::Cli;
use clap::Parser;

fn main() {
    let cli_arg = Cli::parse();
    let mut regex_out = String::from("");
    
    println!("{:?}", cli_arg.format);
    for (_i, c) in cli_arg.input.chars().enumerate() {
        print!("{c}");
        regex_out.push(c);
    }
    
    println!();
    //println!("called with {:?}", cli_arg.pattern);

}
