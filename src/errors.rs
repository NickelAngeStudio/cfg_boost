/// Possible target_cfg errors.
pub enum SyntaxParseError {
    /// !!, && and || are not accepted.
    DoubleOperator(usize),

    /// Mismatched parentheses
    MismatchedParentheses(usize),

    /// Missing operator (happens when a leaf contains a space)
    MissingOperator,

    /// Empty node due to missing variable.
    EmptyNode,

    /// Invalid character used
    InvalidCharacter(char, usize),

    /// Alias written is not found
    AliasNotFound(String),

    /// Invalid configuration predicate
    InvalidConfigurationPredicate(String)
}

/// Error message implementation.
impl SyntaxParseError {
    pub fn message(&self, tokens : &str) -> String {
        match self {
            SyntaxParseError::DoubleOperator(pos) => format!("Invalid double operator for `{:?}` at position {}.", tokens, pos),
            SyntaxParseError::MismatchedParentheses(pos) => format!("Mismatched parentheses for `{:?}` at position {}.", tokens, pos),
            SyntaxParseError::EmptyNode =>  format!("Empty node generated from attributes `{:?}`. Are you missing a statement between separator?", tokens),
            SyntaxParseError::InvalidCharacter(c, pos) => format!("Invalid character `{}` for `{:?}` at position {}.", c, tokens, pos),
            SyntaxParseError::MissingOperator => format!("Operator `&` or '|' missing for `{:?}`.", tokens),
            SyntaxParseError::AliasNotFound(alias) => format!("Alias `{}` has no match! Is it added in config.toml as `target_cfg-{}`?", alias, alias),
            SyntaxParseError::InvalidConfigurationPredicate(cfg_prd) => format!("Configuration predicate `{}` has no match! Is it added in config.toml as `target_cfg_predicate-{}`?", cfg_prd, cfg_prd),
        }
    }
}