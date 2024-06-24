use clap::Parser;

/// A Regex Validator, made with Rust.
///
/// Convert help/man texts into their regex counterpart.
/// The translation is used to validate command-line inputs.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// The input pattern to translate.
    /// Must be a valid help/usage format.
    #[arg(short, long)]
    pub input: String,
    /// format used for translation
    #[arg(short, long)]
    pub format: Option<FormatType>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum FormatType {
    MAN,
    PSQL,
}
