use crate::arm::{ARM_SEPARATOR, CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1, WILDCARD_BRANCH};

/// Possible cfg_boost errors.
pub enum CfgBoostError {
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
impl CfgBoostError {
    pub fn message(&self, tokens : &str) -> String {
        match self {
            CfgBoostError::MissingOperator => format!("Operator `&` or '|' missing for `{:?}`. Target must not contain space.", tokens),
            CfgBoostError::EmptyNode =>  format!("Empty node generated from attributes. Are you missing a statement between separator?"),
            CfgBoostError::InvalidCharacter(c) => format!("Invalid character `{}` for `{:?}`.", c, tokens),
            CfgBoostError::AliasNotFound(alias) => format!("Alias `{}` has no match! Is it added in config.toml as `target_cfg-{}`?", alias, alias),
            CfgBoostError::InvalidConfigurationPredicate(cfg_prd) => format!("Configuration predicate `{}` has no match! Is it added in config.toml as `target_cfg_predicate-{}`?", cfg_prd, cfg_prd),
            CfgBoostError::EmptyArm => format!("Empty arm with no attributes detected!"),
            CfgBoostError::WildcardArmNotLast => format!("Wildcard branch `_` must ALWAYS be the last branch."),
            CfgBoostError::ArmSeparatorMissing => format!("Arm syntax incorrect. Are you missing a separator `{}` between arms?", ARM_SEPARATOR),
            CfgBoostError::ContentSeparatorError => format!("Arm syntax incorrect. Is your arm separator `{}{}` syntax Ok?", CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1),
            CfgBoostError::WildcardArmMissing => format!("Ensure that all possible cases are being handled by adding a match arm with a `{}` wildcard pattern.", WILDCARD_BRANCH),
        }
    }
}