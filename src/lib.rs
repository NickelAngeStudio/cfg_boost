//! Hello

use ts::{ generate_target_content, generate_meta_content, generate_match_content};
use proc_macro::{TokenStream};

/// Errors enumeration
mod errors;

/// config.toml fetch functions
mod config;

/// Arms structure and functions
mod arm;

/// Tokenstream generator functions
mod ts;

/// Syntax tree
mod syntax;


#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {

    // Generate content from target_cfg! macro source.
    generate_target_content(item)

}


#[proc_macro]
pub fn match_cfg(item: TokenStream) -> TokenStream {

    // Generate content for match_cfg! macro.
    generate_match_content(item)

}



#[proc_macro_attribute]
pub fn meta_cfg(attr: TokenStream, item: TokenStream) -> TokenStream {

    // Generate attribute content.
    generate_meta_content(attr, item)

}