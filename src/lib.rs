//! Hello

use ts::{ generate_target_content, generate_attr_content, CfgBoostMacroSource};
use proc_macro::{TokenStream, TokenTree, Group, Delimiter};

/// Errors enumeration
mod errors;

/// Arms structure and functions
mod arm;

/// Tokenstream generator functions
mod ts;

/// Syntax tree
mod syntax;


#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {

    // Generate content from target_cfg! macro source.
    generate_target_content(item, CfgBoostMacroSource::SelectMacro)

}

#[proc_macro]
pub fn match_cfg(item: TokenStream) -> TokenStream {

    // Generate content from match_cfg! macro source and add braces around content.
    TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace,generate_target_content(item, CfgBoostMacroSource::MatchMacro))))

}

#[proc_macro_attribute]
pub fn cfg_target(attr: TokenStream, item: TokenStream) -> TokenStream {

    // Generate attribute content.
    generate_attr_content(attr, item)

}