use clap::Parser;

use super::format::Format;

/// The command line arguments accepted by the `regexor` binary.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Usage string to translate, e.g. `"ftp [-pinegvd] [HOST]"`.
    #[arg(short, long)]
    pub input: String,

    /// Grammar of the usage string (defaults to `man`).
    #[arg(short, long, value_enum, default_value = "man")]
    pub format: Format,
}
