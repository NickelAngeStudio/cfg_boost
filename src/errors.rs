use crate::modifiers::{TargetActiveComparisonOperator, GUARD_MODIFIER, OVERRIDE_MODIFIER};

/// Possible target_cfg errors.
pub enum TargetCfgError {
    /// Missing operator (happens when a leaf contains a space)
    MissingOperator,

    /// Empty node due to missing variable.
    EmptyNode,

    /// Invalid character used
    InvalidCharacter(String),

    /// Alias written is not found
    AliasNotFound(String),

    /// Invalid configuration predicate
    InvalidConfigurationPredicate(String),

    /// Cannot fetch rustc conditional configuration
    RustcConditionalCfgError,

    /// Happens when a predicate has an invalid format
    InvalidPredicateFormat,

    /// Happens when having an empty branch without * modifier.
    EmptyBranch,

    /// Happens when exclusive branch _ is not the last branch.
    ExclusiveBranchNotLast,

    /// Happens when more than counter comparison with control result in error.
    ActiveBranchCountError(usize, TargetActiveComparisonOperator, usize),

    /// Happens when * or !* are used during a release build.
    GuardBlockAlwaysValueBranchNotDebug,
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
            TargetCfgError::ActiveBranchCountError(cpt, op, control) => format!("Active branch count must be `{}{}`. It is currently {}. This can be changed via target_cfg! modifiers.", op.to_string(), control, cpt),
            TargetCfgError::EmptyBranch => format!("No attributes in branch detected for content \n```\n{}\n```\nYou can allow empty branch with special branch symbol `+` or `_`.", tokens),
            TargetCfgError::ExclusiveBranchNotLast => format!("Exclusive branch `_` must ALWAYS be the last branch."),
            TargetCfgError::GuardBlockAlwaysValueBranchNotDebug => format!("Branch guard prevent value override modifier ({}) for release build.\nYou can deactivate branch guard with `!{}` in target_cfg! modifiers.", OVERRIDE_MODIFIER, GUARD_MODIFIER),
        }
    }
}