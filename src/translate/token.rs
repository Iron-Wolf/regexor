/// One lexical unit of a usage string.
pub enum Token {
    /// `[ ... ]` — optional group, holding the tokens parsed from inside it.
    Optional(Vec<Token>),
    /// `-abc` — a run of short flags.
    Flags(String),
    /// Bare word — literal text, or an UPPERCASE placeholder.
    Text(String),
}
