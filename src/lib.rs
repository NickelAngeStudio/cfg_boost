// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{process::Command};
use proc_macro::{TokenStream};
use syntax::SyntaxTreeNode;

use crate::{errors::TargetCfgError, syntax::get_rustc_print_cfg};

/// target_cfg pattern extractor
mod pattern;

/// Errors enumeration
mod errors;

/// Syntax tree
mod syntax;

#[proc_macro]
pub fn target_cfg(_item: TokenStream) -> TokenStream {

    /*
    let tg = TargetGroup::extract(item);

   println!("MODPATH={}", module_path!());

    for t in tg {
        println!("t={}", t.to_string());
    }
    */

    /*
   for token in item {
    match token {
        proc_macro::TokenTree::Group(group) => {
            println!("L1 GROUP[{}]", group.to_string());
            for token2 in group.stream() {
                token2.span().
                match token2 {
                    proc_macro::TokenTree::Group(group) => println!("L2 GROUP[{}]", group.to_string()),
                    proc_macro::TokenTree::Ident(ident) => println!("L2 IDENT[{}]", ident.to_string()),
                    proc_macro::TokenTree::Punct(punc) => println!("L2 PUNCT[{}]", punc.to_string()),
                    proc_macro::TokenTree::Literal(lit) => println!("L2 LITERAL[{}]", lit.to_string()),
                }
            }


        }, //println!("GROUP={}", group.to_string()),
        proc_macro::TokenTree::Ident(ident) => println!("L1 IDENT[{}]", ident.to_string()),
        proc_macro::TokenTree::Punct(punc) => println!("L1 PUNCT[{}]", punc.to_string()),
        proc_macro::TokenTree::Literal(lit) => println!("L1 LITERAL[{}]", lit.to_string()),
    }
    */
    /*
    let thread_join_handle = thread::spawn(move || {
        let cmd = Command::new("cargo").arg("rustc").arg("--lib").arg("--").arg("--print").arg("cfg").output();
        cmd
    });
    // some work here
    let res = thread_join_handle.join();

    match res {
        Ok(cmd) => {
            match cmd{
                Ok(output) => {
                    let out_str = String::from_utf8(output.stdout);

                    match out_str {
                        Ok(cfg_var) => println!("TCFG={}", cfg_var),
                        Err(_) => todo!(),
                    }
                },
                Err(_) => todo!(),
            }
        },
        Err(_) => todo!(),
    }
    */

    // https://crates.io/crates/rustc-cfg
    let cmd = Command::new("rustc").arg("--print").arg("cfg").output();

    match cmd{
        Ok(output) => {
            let out_str = String::from_utf8(output.stdout);

            match out_str {
                Ok(cfg_var) => println!("TCFG={}", cfg_var),
                Err(_) => todo!(),
            }
        },
        Err(_) => todo!(),
    }
    
    //println!("Input={}", stringify!(item.to_string()));


    //target_cfg_parser!{item};

    //println!("{}", item.to_string());

    "fn aaaa() {}".parse().unwrap()
}




#[proc_macro_attribute]
pub fn cfg_target(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. Extract symbol from attributes
    let (symbol, attr) = extract_symbol(attr.clone(), true);

    // 2. Generate syntax tree from content
    let syntax_tree = SyntaxTreeNode::generate(attr.clone());

    // 3 Evaluate syntax tree. Doc always pass except if using !? for no doc.
    if is_doc(symbol.clone()) || syntax_tree.evaluate() {
        let mut content = TokenStream::new();

        if is_doc(symbol.clone()) {
            // 3.1.1. Extend cfg_attr header for documentation
            content.extend(format!("#[cfg_attr(docsrs, doc(cfg({})))]", syntax_tree.to_string()).parse::<TokenStream>().unwrap());
        }

        // 3.1.2 Add item to content
        content.extend(item);
        
        if is_debug(symbol.clone()) {
            // 3.1.3.1. If Debug, panic! with content.
            panic!("\nCFG\n---\n{}\nATTR\n----\n{}\n\nEVAL\n----\n{}\n\nCONTENT\n-------\n{}", get_rustc_print_cfg(), syntax_tree.to_string(), 
                syntax_tree.evaluate().to_string(), content.to_string());
        } else {
            // 3.1.3.2. If not debug write content.
            content
        }
        
    } else {
        if is_debug(symbol.clone()) {
            // 3.2.1. If Debug, panic! with content.
            panic!("\nCFG\n---\n{}\nATTR\n----\n{}\n\nEVAL\n----\n{}\n\nCONTENT\n-------\n{}", get_rustc_print_cfg(), syntax_tree.to_string(), 
                syntax_tree.evaluate().to_string(), "{ removed }");
        } else {
            // 3.2.2. If not debug write empty tokenstream.
            TokenStream::default()
        }
    }

}

// rustc --print cfg


/// Get if debug (@) symbol is activated.
fn is_debug(symbol: TokenStream) -> bool {

    // Not flag. Activated by !, consumed by other symbol.
    let mut is_not = false;

    for t in symbol {
        match t {
            proc_macro::TokenTree::Punct(punc) => {
                match punc.as_char() {
                    '!' => is_not = true,   // Activate is_not flag
                    '@' => {    // Debug node
                        return !is_not && true;
                    },
                    _ => is_not = false,    // Deactivate is_not flag
                }
            },
            _ => {},
        }
    }

    // Not debug if reached here
    false
}

/// Get if documentation (?) symbol is activated. Is true by default.
fn is_doc(symbol: TokenStream) -> bool {

    // Not flag. Activated by !, consumed by other symbol.
    let mut is_not = false;

    for t in symbol {
        match t {
            proc_macro::TokenTree::Punct(punc) => {
                match punc.as_char() {
                    '!' => is_not = true,   // Activate is_not flag
                    '?' => {    // Documentation node
                        return !is_not && true;
                    },
                    _ => is_not = false,    // Deactivate is_not flag
                }
            },
            _ => {},
        }
    }

    // Default value for documentation
    cfg!(doc)
}


/// Extract stream of symbol at the beginning and validate spacing of ! if true.
/// 
/// Returns a pair containing symbol tokenstream and the rest of the stream without symbols.
pub(crate) fn extract_symbol(stream: TokenStream, validate_spacing : bool) -> (TokenStream, TokenStream) {

    // Symbol Tokenstream
    let mut symbol = TokenStream::new();

    // Content Tokenstream
    let mut content = TokenStream::new();

    // Flag that show if in symbol right now.
    let mut is_symbol = true;

    for t in stream.clone() {
        if is_symbol {
            match t.clone() {
                proc_macro::TokenTree::Punct(punc) => {
                    match punc.as_char() {
                        '?' | '@' | '$' => {
                            symbol.extend(TokenStream::from(t))
                        }
                        '!' => { 
                            if validate_spacing {
                                match punc.spacing() {
                                    proc_macro::Spacing::Alone => { // Alone ! are meant to be NOT()
                                        is_symbol = false;
                                        content.extend(TokenStream::from(t));
                                    },
                                    proc_macro::Spacing::Joint => symbol.extend(TokenStream::from(t)), // Joint ! are for symbol.
                                }
                            } else {
                                symbol.extend(TokenStream::from(t))
                            }
                        }
                        _ => {
                            // Illegal character
                            panic!("{}", TargetCfgError::InvalidCharacter(punc.as_char()).message(&stream.to_string()));
                        },
                    }
                    
                },
                _ => {
                    is_symbol = false;
                    content.extend(TokenStream::from(t));
                },
            }
        } else {
            content.extend(TokenStream::from(t));
        }
    }

    return (symbol, content)

}