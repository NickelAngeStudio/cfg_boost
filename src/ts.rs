use std::{env, path::Path, fs};

use proc_macro::{TokenStream, Group, Delimiter, TokenTree};

use crate::{syntax::{Node, SyntaxTreeNode}, arm::TargetArm, errors::CfgBoostError, TargetMacroSource};

/// Macro that create cfg_attr for items attributes from syntax tree.
macro_rules! format_doc {
    ($tree:expr) => {
        format!("#[cfg_attr(docsrs, doc(cfg({})))]", $tree.to_string())
    }
}

/// Macro that create cfg targets from syntax tree.
macro_rules! format_cfg {
    ($tree:expr) => {
        format!("#[cfg({})]", $tree.to_string())
    }
}


/// Generate content from matching arm and macro source.
#[inline(always)]
pub(crate) fn generate_match_content(stream: TokenStream,  source : TargetMacroSource) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Extract target arms
    let arms = TargetArm::extract(stream.clone());

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
                cumul_tree = SyntaxTreeNode::all_node(SyntaxTreeNode::all_node(left.clone(), SyntaxTreeNode::not_node(right.clone())), syntax_tree),
            _ => {},    // we don't talk about those Bruno.
        }
        
        // 2.3. Create cfg header from cumulative
        let cfg_ts = format_cfg!(cumul_tree).parse::<TokenStream>().unwrap();

        // 2.4. Add to content according to target source.
        match source {
            TargetMacroSource::TargetMacro => {
                // 2.4.1. Split item into vector of items
                let items = split_items(arm.content.clone());

                // 2.4.2. For each item in vector of items
                for item in items {
                    // 2.4.2.1. Add cfg header.
                    content.extend(cfg_ts.clone()); 

                    // 2.4.2.2. Add item to content
                    content.extend(item);
                }
            },
            TargetMacroSource::MatchMacro => {
                // 2.4.1. Add cfg header.
                content.extend(cfg_ts.clone()); 

                // 2.4.2. Add braced content
                content.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace, arm.content.clone()))));
            },
        }
    }

    // 3. Add braces around content if from MatchMacro
    match source {
        TargetMacroSource::MatchMacro => content = TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace, content))),
        _ => {}
    }

    // 4. Return content.
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

/// Generate documented content with target labels.
/// 
/// Target labels are added only if [package.metadata.docs.rs] is in Cargo.toml.
#[inline(always)]
pub(crate) fn generate_documented_content(stream: TokenStream) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

     // 1. Extract target arms
     let arms = TargetArm::extract(stream.clone());

    if get_if_docrs_from_cache() {  // If we generate target labels

        // 2. For each arm
        for arm in arms {

            // 3. Generate syntax tree
            let syntax_tree = generate_syntax_tree(&arm);
    
            // 4. Create cfg_attr header
            let attr_ts = format_doc!(syntax_tree).parse::<TokenStream>().unwrap();

            // 5. Split item into vector of items
            let items = split_items(stream.clone());

            // 6. For each item in vector of items
            for item in items {
                // 6.1. Add attr header.
                content.extend(attr_ts.clone()); 

                // 6.2. Add item to content
                content.extend(item);
            }

        }

    } else {
        // 2. For each arm
        for arm in arms {
            // 3. Add content to arm
            content.extend(arm.content);
        }

    }


    // Return content generated
    content

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

/// Key value of cargo.toml caching.
const CFG_BOOST_CARGO_CACHE : &str = "CFG_BOOST_ATTR_DOC_SET";

/// Tag to search in Cargo.toml
const CFG_BOOST_DOCRS_TAG : &str = "[package.metadata.docs.rs]";

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
            let str_path =  format!("{}/{}", env::var("CARGO_MANIFEST_DIR").unwrap(), "Cargo.toml");
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

    // 1. Generate syntax tree from attributes
    let syntax_tree = SyntaxTreeNode::generate(attr.clone());

    // 2. Add #[cfg] to content
    content.extend(format_cfg!(syntax_tree).parse::<TokenStream>().unwrap());

    // 3. Is Cargo.toml set up for target labels? If true, add cfg_attr header.
    if cfg!(doc) && get_if_docrs_from_cache() {  
        content.extend(format_doc!(syntax_tree).parse::<TokenStream>().unwrap());
    }

    // 4. Add item to content.
    content.extend(item);

    // 5. Write content to stream
    content        

}