use std::process::ExitCode;

use clap::Parser;

use regexor::cli::Cli;
use regexor::translate::translate;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match translate(&cli.input, cli.format) {
        Ok(regex) => {
            println!("{regex}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
