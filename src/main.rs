mod model;
mod utils;

use std::process::exit;

use crate::model::cli_struct::*;
use crate::utils::reg_utils::*;
use crate::utils::string_utils::*;
use clap::Parser;

// cp [OPTION]... [-T] SOURCE DEST
// cp [OPTION]... -t RÉPERTOIRE SOURCE...
// ftp [-46AadefginpRtVv] [-N NETRC] [-o OUTPUT]
fn main() {
    let cli_arg = Cli::parse();
    println!("input  : {:?}", cli_arg.input);

    let result = translate_man(cli_arg.input);
    println!("output : {result}");
    exit(0);
}

/// Recursively process the input and translate it to Regex form
fn translate_man(cmd_string: String) -> String {
    let parts: Vec<String> = split_string(cmd_string);
    let mut regex_out = String::from("");

    for p in parts {
        // we don't test for closing bracket, because
        // she should always comme in couple with the opening one
        if p.starts_with('[') {
            // optional arguments
            let mut regex_current = String::from("");
            print!("opt : {p} => ");

            regex_current.push_str("(");
            // recursively handle text inside brackets
            let p_inside = rem_first_and_last(p.as_str());
            let regex_inside = translate_man(p_inside.to_string());
            regex_current.push_str(&regex_inside);
            regex_current.push_str(")? *");

            regex_out.push_str(&*regex_current);
            println!("opt : {p} => {regex_current}")
        } else if p.starts_with('-') {
            // args
            let mut regex_current = String::from("");
            print!("arg : {p} => ");

            regex_current.push_str("-[");
            for (i, c) in p.chars().enumerate() {
                if i != 0 {
                    regex_current.push(c);
                }
            }
            regex_current.push_str("]+ *");

            regex_out.push_str(&*regex_current);
            println!("arg : {p} => {regex_current}")
        } else {
            // simple text
            let mut regex_current = String::from("");
            print!("txt : {p} => ");

            regex_current.push_str(&reg_str(p.clone()));
            regex_current.push_str(" *");

            regex_out.push_str(&*regex_current);
            println!("txt : {p} => {regex_current}")
        }
    }
    return regex_out;
}


#[cfg(test)]
mod main_tests;
