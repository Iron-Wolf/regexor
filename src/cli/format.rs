/// Source grammar of the usage string being translated.
#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// `man(1)` / `--help` style, e.g. `cmd [-abc] [FILE]...`.
    Man,
    /// `PostgreSQL` `psql` meta-command style.
    Psql,
}
