/// Possible target_cfg errors.
pub enum TargetCfgError {
    /// Missing operator (happens when a leaf contains a space)
    MissingOperator,

    /// Empty node due to missing variable.
    EmptyNode,

    /// Invalid character used
    InvalidCharacter(char),

    /// Alias written is not found
    AliasNotFound(String),

    /// Invalid configuration predicate
    InvalidConfigurationPredicate(String),

    /// Cannot fetch rustc conditional configuration
    RustcConditionalCfgError,

    /// Happens when a predicate has an invalid format
    InvalidPredicateFormat,

    /// Happens when more than 1 branch is active and target_cfg is exclusive.
    TargetCfgIsExclusive,

    /// Happens when having an empty branch without * modifier.
    EmptyBranch,

    /// Happens when exclusive branch _ is not the last branch.
    ExclusiveBranchNotLast,
}

/// Error message implementation.
impl TargetCfgError {
    pub fn message(&self, tokens : &str) -> String {
        match self {
            TargetCfgError::EmptyNode =>  format!("Empty node generated from attributes. Are you missing a statement between separator?"),
            TargetCfgError::InvalidCharacter(c) => format!("Invalid character `{}` for `{:?}`.", c, tokens),
            TargetCfgError::MissingOperator => format!("Operator `&` or '|' missing for `{:?}`. Target must not contain space.", tokens),
            TargetCfgError::AliasNotFound(alias) => format!("Alias `{}` has no match! Is it added in config.toml as `target_cfg-{}`?", alias, alias),
            TargetCfgError::InvalidConfigurationPredicate(cfg_prd) => format!("Configuration predicate `{}` has no match! Is it added in config.toml as `target_cfg_predicate-{}`?", cfg_prd, cfg_prd),
            TargetCfgError::RustcConditionalCfgError => format!("Cannot fetch rustc conditional configuration!"),
            TargetCfgError::InvalidPredicateFormat => format!("Invalid predicate format for `{:?}`.", tokens),
            TargetCfgError::TargetCfgIsExclusive => format!("More than 1 active branch in target_cfg!. If it is desired behaviour, add modifier `!$` at the begining of macro.\nSee https://github.com/NickelAngeStudio/target_cfg/wiki/Syntax for more informations."),
            TargetCfgError::EmptyBranch => format!("Empty branch detected for content \n```\n{}\n```\nYou can allow empty branch with `*` or `!*` modifier.", tokens),
            TargetCfgError::ExclusiveBranchNotLast => format!("Exclusive branch `_` must ALWAYS be the last branch."),
        }
    }
}