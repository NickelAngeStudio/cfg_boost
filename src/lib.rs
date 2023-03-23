//! Hello

// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

use target::{TargetAttributeOption, TargetMatchOption, generate_target_cfg_content, generate_cfg_target_content, TargetGroup};
use proc_macro::{TokenStream};
use syntax::{SyntaxTreeNode};
use tools::{extract_modifier};

use crate::target::cfg_target_attr_panic_message;

/// Target relative function and struct
mod target;

/// Tools and functions
mod tools;

/// Errors enumeration
mod errors;

/// Syntax tree
mod syntax;

#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {

    // Panic string that accumulate panic result message.
    let mut panic_str = String::new();

    // 1. Extract modifiers options from item
    let (modifiers, item) = extract_modifier(item);

    // 2. Extract target groups
    let tg = TargetGroup::extract(item.clone());

    // 3. Generate options from modifiers and groups
    let options = TargetMatchOption::new(modifiers.clone(), &tg);

    // 4. Push options modifiers into panic string
    if options.is_panic_result {   // Push options 
        panic_str.push_str(&format!("\nTARGET_CFG\nOPTIONS       {}", options.to_string()));
    }

    // 5. Generate content according to debug only or not.
    let content = if let Some(true) = options.debug_only {
            if cfg!(debug_assertions) {     // Generate content only if debug.
                generate_target_cfg_content(&tg, &options, &mut panic_str)
            } else {
                TokenStream::default()      // Empty token stream since it's debug only.
            }
        } else {
            generate_target_cfg_content(&tg, &options, &mut panic_str)
        };   

    // 6. If panic result, panic with content.
    if options.is_panic_result {   
        panic!("{}\n\nCONTENT\n{}", panic_str, content.to_string());
    }
   
    // 7. Return content
    content

}



#[proc_macro_attribute]
pub fn cfg_target(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. Extract modifiers from attributes
    let (modifiers, attr) = extract_modifier(attr.clone());

    // 2. Generate options from modifiers
    let options = TargetAttributeOption::new(modifiers.clone());

    // 3. Generate syntax tree from content
    let syntax_tree = SyntaxTreeNode::generate(attr.clone());

    // 4. Generate content according to debug only or not.
    let content = if let Some(true) = options.debug_only {
        if cfg!(debug_assertions) {     // Generate content only if debug.
            generate_cfg_target_content(&options, syntax_tree.clone(), item)
        } else {
            TokenStream::default()      // Empty token stream since it's debug only.
        }
    } else {
        generate_cfg_target_content(&options, syntax_tree.clone(), item)
    };   

    // 5. Panic with content if #
    if options.is_panic_result {
        panic!("{}", cfg_target_attr_panic_message(attr.clone(), &options, syntax_tree.clone(), content.clone()));
    }

    // 6. Push content
    content

}