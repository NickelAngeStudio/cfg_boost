//! Hello

use ts::{ generate_match_content, generate_attr_content};
use proc_macro::{TokenStream};

/// Errors enumeration
mod errors;

/// Arms structure and functions
mod arm;

/// Tokenstream generator functions
mod ts;

/// Syntax tree
mod syntax;

/// Proc macro source enumeration to determinate matching macro source.
pub(crate) enum TargetMacroSource {
    /// Call come from target_cfg! macro.
    TargetMacro,

    /// Call come from match_cfg! macro.
    MatchMacro,
}

#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {

    // Generate content from target_cfg! macro source.
    generate_match_content(item, TargetMacroSource::TargetMacro)

}

#[proc_macro]
pub fn match_cfg(item: TokenStream) -> TokenStream {

    // Generate content from match_cfg! macro source.
    generate_match_content(item, TargetMacroSource::MatchMacro)

}

#[proc_macro_attribute]
pub fn cfg_target(attr: TokenStream, item: TokenStream) -> TokenStream {

    // Generate attribute content.
    generate_attr_content(attr, item)

}