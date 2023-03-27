use std::{env, path::Path, fs};

use proc_macro::{TokenStream, Group, Delimiter, TokenTree};

use crate::{syntax::{Node, SyntaxTreeNode}, arm::TargetArm, errors::TargetCfgError, TargetMacroSource};

/// Macro that create cfg_attr for items attributes from syntax tree.
macro_rules! format_doc_attr {
    ($tree:expr) => {
        format!("#[cfg_attr(docsrs, doc(cfg({})))]", $tree.to_string())
    }
}


/// Generate content from matching arm and macro source.
#[inline(always)]
pub(crate) fn generate_match_content(item: TokenStream,  source : TargetMacroSource) -> TokenStream {
    // 1. Extract target arms
    let arms = TargetArm::extract(item.clone());

    // 2. For each arm
    for arm in arms {

        // 2.1. Generate syntax tree from attributes according to branch type.
        let syntax_tree = match arm.arm_type {
            crate::arm::TargetArmType::Normal => {
                if arm.attr.is_empty() {    // Panic if attributes are empty on normal branch
                    panic!("{}", TargetCfgError::EmptyArm.message(&arm.content.to_string()));
                }
                SyntaxTreeNode::generate(arm.attr.clone())
            },
            // If wildcard reached, 
            crate::arm::TargetArmType::Wildcard => SyntaxTreeNode::empty(true),
        };

        // 2.2. Evaluate syntax tree
        if syntax_tree.evaluate() {
            // 2.3. Generate and return syntax tree content
            return activate_arm(syntax_tree.clone(), arm.content, source);
        }
    }

    // 3. If no match return empty token stream.
    TokenStream::default()
}

/// Activate a matching arm from source macro.
#[inline(always)]
pub(crate) fn activate_arm(syntax_tree : Node, stream: TokenStream, source : TargetMacroSource) -> TokenStream {
    
    match source {
        // If from target_cfg macro.
        TargetMacroSource::TargetMacro => {
            match syntax_tree.as_ref() {
                SyntaxTreeNode::WILDCARD(_) => stream,    // Wildcard are returned as is
                _ => {
                    if get_if_docrs_from_cache() {  // If we generate target labels
                        generate_documented_content(syntax_tree.clone(), stream)
                    } else {    // Return content as is
                        stream
                    }
                },
            }
        },
        TargetMacroSource::MatchMacro => {
            // Wrap content in group delimiter {}
            TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace, stream)))
        },
    }

}

/// Generate documented content with target labels.
fn generate_documented_content(syntax_tree : Node, stream: TokenStream) -> TokenStream {
    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Create cfg_attr header
    let attr_ts = format_doc_attr!(syntax_tree).parse::<TokenStream>().unwrap();

    // 2. Split item into vector of items
    let items = split_items(stream.clone());

    // 3. For each item in vector of items
    for item in items {
        // 3.1. Add attr header.
        content.extend(attr_ts.clone()); 

        // 3.2. Add item to content
        content.extend(item);
    }

    // 4. Return content generated
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
const CFG_EASY_CARGO_CACHE : &str = "CFG_EASY_ATTR_DOC_SET";

/// Tag to search in Cargo.toml
const CFG_EASY_DOCRS_TAG : &str = "[package.metadata.docs.rs]";

/// Returns True if cfg-attr is generated for documentation labels.
#[inline(always)]
pub(crate) fn get_if_docrs_from_cache() -> bool {
    // 1. Get previous result from cache. 
    match env::var(CFG_EASY_CARGO_CACHE) {
        Ok(value) => {
            value.eq("true")
        },
        Err(_) => {
            // 2. Read Cargo.toml if no result
            let str_path =  format!("{}/{}", env::var("CARGO_MANIFEST_DIR").unwrap(), "Cargo.toml");
            let file_path = Path::new(&str_path);

            match fs::read_to_string(file_path){
                Ok(content) => {
                    match content.find(CFG_EASY_DOCRS_TAG){
                        Some(_) => { 
                            env::set_var(CFG_EASY_CARGO_CACHE, "true");    // Cache result
                            true
                        },
                        None => {
                            env::set_var(CFG_EASY_CARGO_CACHE, "false");    // Cache result
                            false
                        },
                    }
                },

                // Cargo.toml not found, return false.
                Err(_) => {
                    env::set_var(CFG_EASY_CARGO_CACHE, "false");
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

    // 2. Is syntax tree evalute to true.
    if syntax_tree.evaluate() {
        // 2.1. Is Cargo.toml set up for target labels? If true, add cfg_attr header.
        if get_if_docrs_from_cache() {  
            content.extend(format_doc_attr!(syntax_tree).parse::<TokenStream>().unwrap());
        }

        // 2.2. Add item to content.
        content.extend(item);
    }

    // 3. Write content to stream
    content        

}