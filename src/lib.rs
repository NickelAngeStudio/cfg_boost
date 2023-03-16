// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;
use syntax::SyntaxTreeNode;

/// Syntax tree
mod syntax;

/// Syntax parsing functions
mod parse;

#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {
    println!("item: \"{}\"", item.to_string());

    "fn answer() -> u32 { 42 }".parse().unwrap()
}


#[proc_macro_attribute]
pub fn cfg_target(attr: TokenStream, item: TokenStream) -> TokenStream {

    #[cfg(debug_assertions)]    // Debug only, print cfg_target result.
    {
        print_cfg_target_result(attr.clone(), item.clone());
    }

    // 1. Generate cfg token stream
    let mut ts = generate_cfg_ts_from_attr(attr, true);

    // 2. Add item
    ts.extend(item);

    // 3. Return token stream
    ts


}

/// Generate cfg_target TokenStream from attributes. 
/// 
/// cfg_attr must be true for items but false for block.
#[inline(always)]
fn generate_cfg_ts_from_attr(attr: TokenStream, cfg_attr : bool) -> TokenStream {

    // 1. Generate syntax tree from attributes
    let syntax_tree = SyntaxTreeNode::generate_tree(&attr.to_string());

    // 2. Generate attributes header from tree.
    let header = syntax_tree.as_ref().borrow().target_cfg_to_string();

    // 3. Generate new header as TokenStream
    if cfg_attr {
        format!("#[cfg(any(doc, {}))]\n#[cfg_attr(docsrs, doc(cfg({})))]", header, header).parse::<TokenStream>().unwrap()
    } else {
        format!("#[cfg(any(doc, {}))]", header).parse::<TokenStream>().unwrap()
    }
    
}

/// Print cfg_target proc macro result. (Debug only)
#[cfg(debug_assertions)]
#[inline(always)]
fn print_cfg_target_result(attr: TokenStream, item: TokenStream){

    // 1. Generate syntax tree from attributes
    let syntax_tree = SyntaxTreeNode::generate_tree(&attr.to_string());

    // 2. Print tree structure
    println!("\x1b[94m*** TREE NODES START ***\x1b[0m");
    println!("\x1b[93mATTR = [{:?}]\x1b[0m", &attr.to_string());
    SyntaxTreeNode::print_syntax_tree(syntax_tree.clone());

    println!("\x1b[95mCFG = [{}]\x1b[0m", syntax_tree.as_ref().borrow().target_cfg_to_string());
    println!("\x1b[94m*** TREE NODES START ***\x1b[0m");

    // 1. Generate cfg token stream
    let mut ts = generate_cfg_ts_from_attr(attr, true);

    // 2. Add item
    ts.extend(item);

    // 3. Print result
    println!("\x1b[94m*** RESULT START ***\x1b[0m\n{}\n\x1b[94m*** RESULT END ***\x1b[0m", ts.to_string());

}