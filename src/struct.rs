use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The input pattern to translate
    pub input: String,
    /// format used for translation
    pub format: Option<FormatType>
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum FormatType{
    Man,
    Pgsql
}