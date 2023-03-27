use crate::arm::{ARM_SEPARATOR, CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1, WILDCARD_BRANCH};

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

    /// Happens when having an empty arm.
    EmptyArm,

    /// Happens when wildcard arm _ is not the last.
    WildcardArmNotLast,

    /// Happens when a separator `,` is missing between arms.
    ArmSeparatorMissing,

    /// Happens when a content separator `=>` is malformed.
    ContentSeparatorError,

    /// Happens when wildcard arm is not set.
    WildcardArmMissing,
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
            TargetCfgError::EmptyArm => format!("No attributes in arm detected for content\n```\n{}\n```\n", tokens),
            TargetCfgError::WildcardArmNotLast => format!("Wildcard branch `_` must ALWAYS be the last branch."),
            TargetCfgError::ArmSeparatorMissing => format!("Arm syntax incorrect. Are you missing a separator `{}` between arms?", ARM_SEPARATOR),
            TargetCfgError::ContentSeparatorError => format!("Arm syntax incorrect. Is your arm separator `{}{}` syntax Ok?", CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1),
            TargetCfgError::WildcardArmMissing => format!("Ensure that all possible cases are being handled by adding a match arm with a wildcard pattern `{}`", WILDCARD_BRANCH),
        }
    }
}