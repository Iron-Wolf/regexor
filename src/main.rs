
mod structs;
mod utils;

use std::process::exit;

use crate::structs::*;
use crate::utils::reg_utils::*;
use crate::utils::string_utils::*;
use clap::Parser;

// cp [OPTION]... [-T] SOURCE DEST
// cp [OPTION]... -t RÉPERTOIRE SOURCE...
// ftp [-46AadefginpRtVv] [-N NETRC] [-o OUTPUT]
fn main() {
    let cli_arg = Cli::parse();
    println!("input  : {:?}", cli_arg.input);

    let result = split_and_translate(cli_arg.input);
    println!("output : '{result}'");
    exit(0);
}

fn split_and_translate(cmd_string: String) -> String {
    let parts: Vec<String> = split_string(cmd_string);
    let mut regex_out = String::from("");

    for p in parts {
        // we don't test for closing bracket, because
        // she should always comme in couple with the openning one
        if p.starts_with('[') {
            // optional arguments
            regex_out.push_str("(");
            // recursivly handle text inside brackets
            let p_inside = rem_first_and_last(p.as_str());
            let regex_inside = split_and_translate(p_inside.to_string());
            regex_out.push_str(&regex_inside);
            regex_out.push_str(")? *")
        }
        else if p.starts_with('-') {
            regex_out.push_str("-[");
            for (i, c) in p.chars().enumerate() {
                if i != 0 {
                    regex_out.push(c);
                }
            }
            regex_out.push_str("]+ *");
        }
        else {
            // simple text
            regex_out.push_str(&reg_str(p));
            regex_out.push_str(" *")
        }
    }
    return regex_out;
}


#[cfg(test)]
mod main_tests;