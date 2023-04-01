use std::{env, path::Path, fs};

use proc_macro::{TokenStream, Group, Delimiter, TokenTree};

use crate::{syntax::{Node, SyntaxTreeNode}, arm::TargetArm, errors::CfgBoostError, parse::DOC_ALIAS};

// Constants
pub const DOC_HIDDEN_TAG : &str = "#[doc(hidden)]";                 // Tag used to detect if code is hidden. (Don't generate cfg_attr)
const CFG_BOOST_CARGO_CACHE : &str = "CFG_BOOST_ATTR_DOC_SET";      // Key value of cargo.toml caching.
const CFG_BOOST_DOCRS_TAG : &str = "[package.metadata.docs.rs]";    // Tag to search in Cargo.toml
const CARGO_MANIFEST_DIR : &str = "CARGO_MANIFEST_DIR";             // Cargo manifest dir key
const CARGO_MANIFEST_NAME : &str = "Cargo.toml";                    // Cargo manifest file name

/// Macro that create cfg_attr for items attributes from syntax tree.
macro_rules! format_doc {
    ($tree:expr) => {
        format!("#[cfg_attr(docsrs, doc(cfg({})))]", $tree.to_string())
    }
}

/// Macro that create cfg targets from syntax tree. Doc is ALWAYS allowed.
macro_rules! format_cfg {
    ($tree:expr) => {
        format!("#[cfg({})]", $tree.to_string())
    }
}

/// Proc macro source enumeration to determinate matching macro source.
#[derive(Clone, Copy)]
pub(crate) enum CfgBoostMacroSource {
    /// Call come from target_cfg! macro.
    TargetMacro,

    /// Call come from match_cfg! macro.
    MatchMacro,

    /// Call come from single_cfg! macro.
    SingleMacro,
}


/// Generate content from matching arm and macro source.
#[inline(always)]
pub(crate) fn generate_target_content(stream: TokenStream,  source : CfgBoostMacroSource) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Extract target arms
    let arms = TargetArm::extract(stream.clone(), source);

    // 2. Create cumulative tree that guard branchs
    let mut cumul_tree = SyntaxTreeNode::empty_node();

    // 2. For each arm
    for arm in arms {

        // 2.1. Generate syntax tree from attributes according to branch type.
        let syntax_tree = generate_syntax_tree(&arm);

        // 2.2. Update cumulative tree with syntax tree.
        match cumul_tree.as_ref() {
            SyntaxTreeNode::EMPTY => cumul_tree = SyntaxTreeNode::all_node(SyntaxTreeNode::empty_node(), syntax_tree.clone()),  // Init cumulative tree
            SyntaxTreeNode::ALL(left, right) =>     // Cumulative tree is ALWAYS ALL after init
                // Previous right node becomes negative and syntax nodes are added with all.
                cumul_tree = SyntaxTreeNode::all_node(SyntaxTreeNode::all_node(left.clone(), SyntaxTreeNode::not_node(right.clone())), syntax_tree.clone()),
            _ => {},    // we don't talk about those Bruno.
        }
        
        // 2.3. Add to content according to target source.
        content.extend(match source {
            CfgBoostMacroSource::MatchMacro => generate_match_macro_content(&arm, cumul_tree.clone()),
            _ => generate_target_macro_content(&arm, cumul_tree.clone(), syntax_tree.clone()),
        });
    }

    // 3. Return content.
    content

}

/// Generate Tokenstream for target_cfg! macro from TargetArm, cumulative and syntax nodes.
#[inline(always)]
fn generate_target_macro_content(arm : &TargetArm, cumul_tree : Node, syntax_tree : Node) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 2.3. Create cfg header from cumulative
    let cfg_ts = format_cfg!(cumul_tree.clone()).parse::<TokenStream>().unwrap();

    // 2.4.1. Split item into vector of items
    let items = split_items(arm.content.clone());

    // 2.4.2. For each item in vector of items
    for item in items {
        // 2.4.2.1. Add cfg header.
        content.extend(cfg_ts.clone()); 

        // 2.4.2.2. Add cfg_attr if not hidden and arm.is_doc is Some(true)
        match arm.is_doc{
            Some(is_doc) => if is_doc && !is_item_hidden(item.clone()) && get_if_docrs_from_cache() {  
                content.extend(format_doc!(match arm.arm_type {
                    crate::arm::TargetArmType::Normal => syntax_tree.clone(),       // Normal arm uses Syntax tree for cfg_attr
                    crate::arm::TargetArmType::Wildcard => cumul_tree.clone(),      // Wildcard uses cumulative tree
                }).parse::<TokenStream>().unwrap());
            },
            None => {}, // None are not generated
        }
        
        // 2.4.2.3. Add item to content
        content.extend(item);
    }

    // 3. Return content generated
    content

}

/// Generate Tokenstream for match_cfg! macro from TargetArm and cumulative node.
#[inline(always)]
fn generate_match_macro_content(arm : &TargetArm, cumul_tree : Node) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Add cfg header.
    content.extend(format_cfg!(cumul_tree).parse::<TokenStream>().unwrap()); 

    // 2. Add braced content
    content.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace, arm.content.clone()))));

    // 3. Return content TokenStream
    content
}

/// Generate a syntax tree from arm.
#[inline(always)]
pub(crate) fn generate_syntax_tree(arm : &TargetArm) -> Node {
    match arm.arm_type {
        crate::arm::TargetArmType::Normal => {
            if arm.attr.is_empty() {    // Panic if attributes are empty on normal branch
                panic!("{}", CfgBoostError::EmptyArm.message(&arm.content.to_string()));
            }
            SyntaxTreeNode::generate(arm.attr.clone())
        },
        // If wildcard reached, 
        crate::arm::TargetArmType::Wildcard => SyntaxTreeNode::wildcard_node(),
    }
}

/// Split tokenstream in different [item](https://doc.rust-lang.org/reference/items.html) vector tokenstream.
/// 
/// An item is defined as all tokens until a ; and/or {}.
#[inline(always)]
pub(crate) fn split_items(stream : TokenStream) -> Vec<TokenStream> {

    let mut item = TokenStream::new();
    let mut items : Vec<TokenStream> = Vec::new();

    for t in stream {
        match &t {
            proc_macro::TokenTree::Group(grp) => {
                // Validate if first and last character of group is 
                match grp.delimiter(){
                    proc_macro::Delimiter::Brace => {    // End of item. 
                        item.extend(TokenStream::from(t)); // Add into item tokenstream
                        items.push(item);   // Push item into vector.
                        item = TokenStream::new();  // Reset item tokenstream
                    },
                    _ => item.extend(TokenStream::from(t)), // Add into item tokenstream
                }
            }
            ,
            proc_macro::TokenTree::Punct(punc) => {
                if punc.as_char().eq(&';') { // End of item.
                    item.extend(TokenStream::from(t)); // Add into item tokenstream
                    items.push(item);   // Push item into vector.
                    item = TokenStream::new();  // Reset item tokenstream
                } else {
                    item.extend(TokenStream::from(t)); // Add into item tokenstream
                }
            },
            _ => item.extend(TokenStream::from(t)), // Add into item tokenstream
        }
    }

    items
}

/// Returns True if cfg-attr is generated for documentation labels.
#[inline(always)]
pub(crate) fn get_if_docrs_from_cache() -> bool {
    // 1. Get previous result from cache. 
    match env::var(CFG_BOOST_CARGO_CACHE) {
        Ok(value) => {
            value.eq("true")
        },
        Err(_) => {
            // 2. Read Cargo.toml if no result
            let str_path =  format!("{}/{}", env::var(CARGO_MANIFEST_DIR).unwrap(), CARGO_MANIFEST_NAME);
            let file_path = Path::new(&str_path);

            match fs::read_to_string(file_path){
                Ok(content) => {
                    match content.find(CFG_BOOST_DOCRS_TAG){
                        Some(_) => { 
                            env::set_var(CFG_BOOST_CARGO_CACHE, "true");    // Cache result
                            true
                        },
                        None => {
                            env::set_var(CFG_BOOST_CARGO_CACHE, "false");    // Cache result
                            false
                        },
                    }
                },

                // Cargo.toml not found, return false.
                Err(_) => {
                    env::set_var(CFG_BOOST_CARGO_CACHE, "false");
                    false
                },
            }
        }
    }
}

/// cfg_target attribute macro content generator.
#[inline(always)]
pub(crate) fn generate_attr_content(attr : TokenStream, item : TokenStream) -> TokenStream{

    let mut content = TokenStream::new();

    // 1. Verify and set default doc attribute
    let attr = set_attr_doc(attr);

    // 2. Generate syntax tree from attributes
    let syntax_tree = SyntaxTreeNode::generate(attr.clone());

    // 3. Add #[cfg] to content
    content.extend(format_cfg!(syntax_tree).parse::<TokenStream>().unwrap());

    // 4. Is Cargo.toml set up for target labels and #[doc(hidden)] not set?
    if !is_item_hidden(item.clone()) && get_if_docrs_from_cache() {  
        content.extend(format_doc!(syntax_tree).parse::<TokenStream>().unwrap());
    }

    // 5. Add item to content.
    content.extend(item);

    // 6. Write content to stream
    content        

}

/// Add default doc attribute to attributes if not present.
/// Return ts created.
#[inline(always)]
fn set_attr_doc(attr : TokenStream) -> TokenStream{

    let mut is_set = false;

    // Verify if doc is set
    for token in attr.clone() {
        match token {
            TokenTree::Ident(ident) => {
                if ident.to_string().as_str().eq(DOC_ALIAS.0) {
                    is_set = true;
                }
            },
            _ => {},
        }
    }

    if is_set { // If already set, change nothing
        attr
    } else {
        // 1. Wrap attr in ()
        let grp_ts = TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis,attr))); 

        // 2. Set attr to doc |
        let mut attr = format!("{} |", DOC_ALIAS.0).parse::<TokenStream>().unwrap();

        // 3. Extend with grp_ts
        attr.extend(grp_ts);

        // 4. Return new attributes
        attr
    }

}

/// Return true if item has #[doc(hidden)] tag.
#[inline(always)]
fn is_item_hidden(item : TokenStream) -> bool{
    match item.to_string().find(DOC_HIDDEN_TAG){
        Some(_) => true,
        None => false,
    }
}