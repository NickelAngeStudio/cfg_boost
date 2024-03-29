use crate::arm::{ARM_SEPARATOR, CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1, WILDCARD_ARM, MODIFIER_ACTIVATE, MODIFIER_DEACTIVATE, MODIFIER_PANIC};

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

    /// Happens when wildcard arm is not set for match_cfg!.
    WildcardArmMissing,

    /// Happens when wildcard arm is set when using single_cfg!
    WildcardArmOnTarget,

    /// Happens when trying to use target_cfg! inside a function.
    TargetInFunction,

    /// Happens when legacy syntax is incorrect
    LegacySyntaxError,

    /// Happens when mixing legacy and simplifier syntax on same arm.
    MixedSyntaxError,

    /// Happens when a separator `=>` is missing between arms.
    ContentSeparatorMissing,

    /// Happens when a modifier `+` or `-` isn't the first character of arm.
    ModifierNotFirst,

    /// Happens when a modifier `+` or `-` is used during release compilation and not set to ignore.
    #[allow(dead_code)]
    ModifierPanicRelease,

    /// Happens when more than 1 modifier `+` in match_cfg!.
    MatchModifierMoreThanOneActivate,

    /// Happens when using modifier `-` on wildcard arm of match_cfg!.
    MatchDeactivatedWildArm,
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
            CfgBoostError::WildcardArmMissing => format!("Ensure that all possible cases are being handled by adding a match arm with a `{}` wildcard pattern.", WILDCARD_ARM),
            CfgBoostError::WildcardArmOnTarget => format!("target_cfg! macro cannot have a `{}` wildcard pattern.", WILDCARD_ARM),
            CfgBoostError::TargetInFunction => format!("target_cfg! macro cannot be used inside a function. Use match_cfg! instead."),
            CfgBoostError::LegacySyntaxError => format!("Legacy syntax error in `{}`.", tokens),
            CfgBoostError::MixedSyntaxError => format!("Legacy syntax and simplified syntax can't be mixed on same arm!"),
            CfgBoostError::ContentSeparatorMissing => format!("Arm content separator `{}{}` missing!", CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1),
            CfgBoostError::ModifierNotFirst => format!("Arm modifiers `{}`, `{}` and `{}` must be the first character of arm!", MODIFIER_ACTIVATE, MODIFIER_DEACTIVATE, MODIFIER_PANIC),
            CfgBoostError::ModifierPanicRelease => format!("Arm modifiers `{}` and `{}` will panic during release compilation by default! This behaviour can be changed. See https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax#six-modifiers", MODIFIER_ACTIVATE, MODIFIER_DEACTIVATE),
            CfgBoostError::MatchModifierMoreThanOneActivate => format!("match_cfg! cannot have more than one `{}` modifier!", MODIFIER_ACTIVATE),
            CfgBoostError::MatchDeactivatedWildArm => format!("match_cfg! cannot deactivate wildcard arm with `{}` modifier!", MODIFIER_DEACTIVATE),
        }
    }
}