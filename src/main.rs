
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let cli_arg = Cli::parse();

    // display cliArg properties
    // println!("{:?}", cliArg);

    println!("called with {}/{}", cli_arg.pattern, cli_arg.path.display());

}
