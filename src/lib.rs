// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

use option::{TargetAttributeOption, TargetMatchOption};
use proc_macro::{TokenStream};
use syntax::{SyntaxTreeNode, Node};
use tools::{extract_symbol, split_items, TargetGroup, TargetActiveCounter};


/// Target symbol options
mod option;

/// Tools and functions
mod tools;

/// Errors enumeration
mod errors;

/// Syntax tree
mod syntax;

#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // Debug string that accumulate debug message.
    let mut debug_str = String::new();

    // 1. Extract symbol options from item
    let (symbol, item) = extract_symbol(item, true);

    // 2. Extract target groups
    let tg = TargetGroup::extract(item.clone());

    // 3. Generate options from symbol and groups
    let options = TargetMatchOption::new(symbol.clone(), &tg);

    // 4. Create active arms counter
    let mut arm_cpt = TargetActiveCounter::new(&options);

    if options.is_debug {   // Push options 
        debug_str.push_str(&format!("\nTARGET_CFG DEBUG\nOPTIONS[{}]", options.to_string()));
    }

    // 5. For each group
    for g in tg {

        // 5.1. Extract symbol from attributes 
        let (symbol_attr, attr) = extract_symbol(g.attr.clone(), true);

        // 5.2. Generate attributes options from symbol and match options
        let opt_attr = TargetAttributeOption::from_match(&options, symbol_attr.clone());

        // 5.3. Generate syntax tree from attributes
        let syntax_tree = SyntaxTreeNode::generate(attr.clone());

        if options.is_debug {   // Push attr debug 
            debug_str.push_str(&format!("\n{}", target_cfg_attr_debug_message(attr.clone(), &opt_attr, syntax_tree.clone())));
        }
        

        // 5.4. Evaluate if value is overriden.
        match opt_attr.always_this {
            Some(is_activated) => {
                if is_activated {
                    content.extend(target_cfg_activate_arm(&mut arm_cpt, &options, syntax_tree.clone(), g.item));
                }
            },
            // 5.5 Evaluate syntax_tree
            None => if options.allow_doc || syntax_tree.evaluate() {
                content.extend(target_cfg_activate_arm(&mut arm_cpt, &options, syntax_tree.clone(), g.item));

            },
        }
    }

    if options.is_debug {   // If debug, panic with content.
        panic!("{}\n\nCONTENT\n{}", debug_str, content.to_string());
    }
   
    // Return content
    content

}


/// Activate a target_cfg! arm from parameters.
#[inline(always)]
pub(crate) fn target_cfg_activate_arm(arm_cpt : &mut TargetActiveCounter, options : &TargetMatchOption, syntax_tree : Node, item: TokenStream) -> TokenStream {
    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Increment active arms.
    arm_cpt.inc();

    // 2. Verify if macro is inside
    if options.is_inner_macro {
        // 2.1. Add item as is in content
        content.extend(item);
    } else {
        // 2.2. Create attr header
        let attr_ts = format!("#[cfg_attr(docsrs, doc(cfg({})))]", syntax_tree.to_string()).parse::<TokenStream>().unwrap();

        // 2.3. Split item into vector of items
        let items = split_items(item.clone());

        // 2.4.. For each item in vector of items
        for item in items {
            // 2.4.1. Add attr header.
            content.extend(attr_ts.clone()); 

            // 2.4.2. Add item to content
            content.extend(item);
        }
    }

    // 3. Return content generated
    content

}

#[proc_macro_attribute]
pub fn cfg_target(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. Extract symbol from attributes
    let (symbol, attr) = extract_symbol(attr.clone(), true);

    // 2. Generate options from symbol
    let options = TargetAttributeOption::new(symbol.clone());

    // 3. Generate syntax tree from content
    let syntax_tree = SyntaxTreeNode::generate(attr.clone());

    // 4. Evaluate if value is overriden.
    match options.always_this {
        // Value is overriden, use provided value
        Some(is_activated) => if is_activated {
            cfg_target_activate(attr, &options, syntax_tree.clone(), item)
        } else {
            cfg_target_deactivate(attr, &options, syntax_tree.clone())
        },
        // Value not overriden, evaluate doc and tree
        None => if options.allow_doc || syntax_tree.evaluate() {
            cfg_target_activate(attr, &options, syntax_tree.clone(), item)
        } else {
            cfg_target_deactivate(attr, &options, syntax_tree.clone())
        },
    }

}

/// Activate tokenstream.
/// 
/// Panic(s)
/// Will print debug value if debug is activated.
#[inline(always)]
pub(crate) fn cfg_target_activate(attr: TokenStream,options : &TargetAttributeOption, syntax_tree : Node, item : TokenStream) -> TokenStream{
    let mut content = TokenStream::new();

    if options.allow_doc {
        // 1. Extend cfg_attr header for documentation
        content.extend(format!("#[cfg_attr(docsrs, doc(cfg({})))]", syntax_tree.to_string()).parse::<TokenStream>().unwrap());
    }

    // 2. Add item to content
    content.extend(item);

    // 3. Panic with content if debug
    if options.is_debug {
        panic!("{}", cfg_target_attr_debug_message(attr.clone(), &options, syntax_tree.clone(), content.clone()));
    }

    // 4. Write content to stream
    content        
}

/// Deactivate tokenstream.
/// 
/// Panic(s)
/// Will print debug value if debug is activated.
#[inline(always)]
pub(crate) fn cfg_target_deactivate(attr: TokenStream,options : &TargetAttributeOption, syntax_tree : Node) -> TokenStream{
    // 1. Panic with content if debug
    if options.is_debug {
        panic!("{}", cfg_target_attr_debug_message(attr.clone(), &options, syntax_tree.clone(), TokenStream::default()));
    }

    // 2. Write empty tokenstream.
    TokenStream::default()
}

/// This function create the debug message for attributes.
#[inline(always)]
pub(crate) fn cfg_target_attr_debug_message(attr: TokenStream, options : &TargetAttributeOption, syntax_tree : Node, content : TokenStream) -> String {
    format!("\nATTR[{}]\nOPTIONS[{}]\nTO_STRING[{}]\nLEAF_EVAL[{}]\nTREE_EVAL[{}]\nEVAL_OVER[{:?}]\nCONTENT[{}]",
    attr.to_string(), options.to_string(), syntax_tree.to_string(), syntax_tree.leaf_node_eval_to_string(), syntax_tree.evaluate(), options.always_this,
    content.to_string())
}

/// This function create the debug message for attributes.
#[inline(always)]
pub(crate) fn target_cfg_attr_debug_message(attr: TokenStream, options : &TargetAttributeOption, syntax_tree : Node) -> String {
    format!("\nATTR[{}]\nOPTIONS[{}]\nTO_STRING[{}]\nLEAF_EVAL[{}]\nTREE_EVAL[{}]\nEVAL_OVER[{:?}]",
    attr.to_string(), options.to_string(), syntax_tree.to_string(), syntax_tree.leaf_node_eval_to_string(), syntax_tree.evaluate(), options.always_this)
}