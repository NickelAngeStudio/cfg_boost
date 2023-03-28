// Syntax tree used to generate configuration from TokenStream.

use std::{rc::Rc, env, process::Command};
use proc_macro::{TokenStream, TokenTree};

use crate::{errors::CfgBoostError};

/// SyntaxTreeNode in a RC 
pub(crate) type Node = Rc<SyntaxTreeNode>;

/// Key value of rustc conditional configuration flag for retrieval.
const RUSTC_CFG_FLAGS : &str = "RUSTC_CFG_FLAGS";

/// Negative symbol
const NEGATIVE_SYMBOL : char = '!';

/// Symbol for AND.
const AND_SYMBOL : char = '&';

/// Symbol for OR.
const OR_SYMBOL : char = '|';

/// Syntax tree node used to parse attribute tokens.
#[derive(Debug)]
pub(crate) enum SyntaxTreeNode {
    /// A wildcard Tree node.
    WILDCARD(bool),

    /// A Not node
    NOT(Node),

    /// A or(|) operation
    ANY(Node, Node),

    /// A and(&) operation
    ALL(Node, Node),

    /// End leaf of the tree
    LEAF(String)
}

impl ToString for SyntaxTreeNode {
    /// Write the node as string. The format will be the same as used with #[cfg()].
    fn to_string(&self) -> String {
        match self {
            SyntaxTreeNode::NOT(node) => format!("NOT({})", node.to_string()),
            SyntaxTreeNode::ANY(left_node, right_node) => format!("ANY({},{})", left_node.to_string(), right_node.to_string()),
            SyntaxTreeNode::ALL(left_node, right_node) => format!("ALL({},{})", left_node.to_string(), right_node.to_string()),
            SyntaxTreeNode::LEAF(label) => 
                match parse_cfg_predicate(&label.as_str()) {
                    Ok(predicate) => format!("{}", predicate),
                    Err(err) => panic!("{}", err.message(label)),
                },
            SyntaxTreeNode::WILDCARD(_) => String::from("{ empty }"),
        }
    }
}

impl SyntaxTreeNode {
    /// Create an empty SyntaxTreeNode with evaluation
    pub fn empty(value : bool) -> Node {
        Rc::new(SyntaxTreeNode::WILDCARD(value))
    }

    /// Evaluate tree nodes and get if configuration is compiled.
    pub fn evaluate(&self) -> bool {

        match self {
            // Evaluate with childs nodes
            SyntaxTreeNode::NOT(node) => ! node.evaluate(),
            SyntaxTreeNode::ANY(left_node, right_node) => left_node.evaluate() || right_node.evaluate(),
            SyntaxTreeNode::ALL(left_node, right_node) => left_node.evaluate() && right_node.evaluate(),
            
            // Match label to see if enabled or not.
            SyntaxTreeNode::LEAF(label) => match parse_cfg_predicate(&label.as_str()) {
                Ok(label) => is_predicate_in_cfg(&label),
                Err(err) => panic!("{}", err.message(label)),
            },
            SyntaxTreeNode::WILDCARD(value) => *value,  // Empty node already has predefined value.
        }
    }

    /// Generate a SyntaxTreeNode from token stream.
    pub(crate) fn generate(stream : TokenStream) -> Node {

        match split_tokenstream_at_operator(stream.clone()){
            // Means we have to split 
            Some((operator, left, right)) => 
                match operator {
                    AND_SYMBOL => {    // ALL node
                        return Rc::new(SyntaxTreeNode::ALL(Self::generate(left), Self::generate(right)));
                    }
                    OR_SYMBOL => {    // ANY node
                        return Rc::new(SyntaxTreeNode::ANY(Self::generate(left), Self::generate(right)));
                    },
                    _ =>  panic!("{}", CfgBoostError::InvalidCharacter(String::from(operator)).message(&stream.to_string())),
                },
            // No split. Must evaluate if not node, etc...
            None => {
                // Is NOT node?
                let (symbol, content) = extract_negative_symbol(stream.clone());
                
                if is_not_node(symbol) {
                    // Create a NOT node
                    return Rc::new(SyntaxTreeNode::NOT(Self::generate(content)));
                } else {

                    // Extract group
                    match extract_group(content.clone()) {
                        Some(group) => return Self::generate(group),
                        None => {
                            // Verify that node isn't empty.
                            if content.to_string().eq("") { // Make sure node isn't empty
                                panic!("{}", CfgBoostError::EmptyNode.message(&content.to_string()));
                            }

                            match content.to_string().find(":"){
                                Some(pos) => {    // End LEAF reached
                                    match content.to_string()[..pos].trim().find(" "){    // Make sure node doesn't contains spaces.
                                        Some(_) => panic!("{}", CfgBoostError::MissingOperator.message(&content.to_string())),
                                        None => {},
                                    }

                                    return Rc::new(SyntaxTreeNode::LEAF(content.to_string()));
                                },
                                None => {   // Unwrap alias
                                    match content.to_string().find(" "){    // Make sure node doesn't contains spaces.
                                        Some(_) => panic!("{}", CfgBoostError::MissingOperator.message(&content.to_string())),
                                        None => {},
                                    }
                                    match parse_alias_from_label(&content.to_string()) {
                                        Ok(alias) => Self::generate(alias.parse().unwrap()),
                                        Err(err) => panic!("{}", err.message(&stream.to_string())),
                                    }
                                },
                            }
                        },
                    }
                }
            },
        }

    }

}

/// Extract a group from token stream
#[inline(always)]
fn extract_group(stream : TokenStream) -> Option<TokenStream> {
    for t in stream {
        match t {
            TokenTree::Group(group) => return Some(group.stream()),
            _ => {},
        }
    }
    None
}

/// Extract ! at the beginning of node.
/// 
/// Returns a pair containing ! tokenstream and the rest of the stream without !.
#[inline(always)]
pub(crate) fn extract_negative_symbol(stream: TokenStream) -> (TokenStream, TokenStream) {

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
                        NEGATIVE_SYMBOL => symbol.extend(TokenStream::from(t)),
                        _ => panic!("{}", CfgBoostError::InvalidCharacter(String::from(punc.as_char())).message(&stream.to_string())),
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

    (symbol, content)

}

/// Verify if node is a NOT node, or not
#[inline(always)]
fn is_not_node(symbol : TokenStream) -> bool{

    for t in symbol {
        match t {
            proc_macro::TokenTree::Punct(punc) => {
                match punc.as_char() {
                    NEGATIVE_SYMBOL => return true,
                    _ => {},
                }
            },
            _ => {},
        }
    }

    // Default value for documentation
    false
}

/// Split a token stream at specified token tree
#[inline(always)]
pub(crate) fn split_tokenstream_at_operator(stream : TokenStream) -> Option<(char, TokenStream, TokenStream)> {

    // Used to munch tokens on the left
    let mut left = TokenStream::new();

    // Used to munch tokens on the right
    let mut right = TokenStream::new();

    // Operator used to separate tokenstream
    let mut operator: char = '?';


    for t in stream.clone() {

        match operator {
            '?' => {
                match t.clone() {
                    proc_macro::TokenTree::Punct(symbol) => {
                        match symbol.as_char() {
                                AND_SYMBOL => {    // ALL node
                                    operator = AND_SYMBOL
                                }
                                OR_SYMBOL => {    // ANY node
                                    operator = OR_SYMBOL
                                }
                                // Valid ignored characters
                                NEGATIVE_SYMBOL | '_' | '-' | ' ' | ':' | '.' => left.extend(TokenStream::from(t)),    // Munch tokens in left hand
                                    
                                _ => {
                                    //err illegal
                                    panic!("{}", CfgBoostError::InvalidCharacter(String::from(symbol.as_char())).message(&stream.to_string()));
                                },
                            }
                        },
                    _ => left.extend(TokenStream::from(t)),    // Munch tokens in left hand
                }
            },
            _ => right.extend(TokenStream::from(t)),    // Munch tokens in right hand
        }        
    }

    match operator {
        '?' => None,    // No split happened
        _ => Some((operator, left, right)), // Split happened

    }

}


/// Parse tokens to generate configuration predicate.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::InvalidConfigurationPredicate]) if predicate not defined.
#[inline(always)]
pub fn parse_cfg_predicate(tokens : &str) -> Result<String, CfgBoostError> {

    // 1. Extract label and predicate from tokens
    match tokens.find(":") {
        Some(position) => {
            let label = tokens[0..position].trim();
            let cfg_opt = tokens[position + 1..].trim();

            // 2. Try to match environment variable to see if predicate was defined in config.toml.
            match env::var(format!("target_cfg_predicate-{}", cfg_opt)) {
                Ok(cfg_value) => Ok(String::from(cfg_value.replace("{}", label))),
                Err(_) => match cfg_opt {   // 2.2 Try to match default predicate
                        // Default configuration predicates
                        "ar" => Ok(format!("target_arch = \"{}\"", label)),
                        "tf" => Ok(format!("target_feature = \"{}\"", label)),
                        "os" => Ok(format!("target_os = \"{}\"", label)),
                        "fm" => Ok(format!("target_family = \"{}\"", label)),
                        "ev" => Ok(format!("target_env = \"{}\"", label)),
                        "ed" => Ok(format!("target_endian = \"{}\"", label)),
                        "pw" => Ok(format!("target_pointer_width = \"{}\"", label)),
                        "vn" => Ok(format!("target_vendor = \"{}\"", label)),
                        "at" => Ok(format!("target_has_atomic = \"{}\"", label)),
                        "pn" => Ok(format!("panic = \"{}\"", label)),
                        "ft" => Ok(format!("feature = \"{}\"", label)),
        
                        // Not found, raise error.
                        _ => Err(CfgBoostError::InvalidConfigurationPredicate(String::from(cfg_opt))),
                    },
            }
        },

        // Should never happen but good to have in hand
        None => Err(CfgBoostError::InvalidConfigurationPredicate(String::from(tokens))),
    } 

}


/// Parse label to generate alias content.
/// 
/// Error(s)
/// Returns Err([TargetCfgError::AliasNotFound]) if alias not defined.
#[inline(always)]
pub fn parse_alias_from_label(label : &str) -> Result<String, CfgBoostError> {

    // 1. Try to match environment variable to see if it was defined in config.toml.
    match env::var(format!("target_cfg-{}", label)) {
        Ok(alias) => Ok(alias.clone()),     
        Err(_e) => {
            // 2. Try to match predefined alias
            match label {
                // Predefined OS aliases
                "linux" => Ok(String::from("linux:os")),
                "unix" => Ok(String::from("unix:fm")),
                "windows" => Ok(String::from("windows:fm")),
                "macos" => Ok(String::from("macos:os")),
                "android" => Ok(String::from("android:os")),
                "ios" => Ok(String::from("ios:os")),
                "wasm" => Ok(String::from("wasm:fm")),

                // Predefined platform aliases
                "desktop" => Ok(String::from("linux:os | windows:os | macos:os")),
                "mobile" => Ok(String::from("android:os | ios:os")),

                // Not found, raise error.
                _ => Err(CfgBoostError::AliasNotFound(String::from(label))),
            }
        },
    }

}

/// Get the conditional configuration flag set by rustc.
/// 
/// To prevent fetching it each time, the result is cached in environment variable `RUSTC_CFG_FLAGS`.
/// 
/// Reference(s)
/// <https://crates.io/crates/rustc-cfg>
#[inline(always)]
pub(crate) fn get_rustc_print_cfg() -> String {

    match env::var(RUSTC_CFG_FLAGS) {
        Ok(value) => {
            value
        },
        Err(_) => {
            // 1. Fetch content from command
            let cmd = Command::new("rustc").arg("--print").arg("cfg").output();

            match cmd {
                Ok(output) => {
                    let out_str = String::from_utf8(output.stdout);

                    match out_str {
                        Ok(out_str) => {
                            // 2. Save content in environment variable `RUSTC_CFG_FLAGS`.
                            env::set_var(RUSTC_CFG_FLAGS, out_str.clone());

                            // 3. Return content from command
                            out_str
                        },
                        Err(_) => panic!("{}", CfgBoostError::RustcConditionalCfgError.message("")),

                    }
                }
                Err(_) =>  panic!("{}", CfgBoostError::RustcConditionalCfgError.message("")),
            }
        }
    }
}

/// Returns True if predicate is in rustc configuration
#[inline(always)]
pub(crate) fn is_predicate_in_cfg(predicate : &String) -> bool {
    // Remove spaces from predicate
    let trimmed = predicate.replace(" ", "");

    // Verify in env variable first
    match is_predicate_in_env(&trimmed) {
        Some(value) => value,
        // Then look in rustc config.
        None => match get_rustc_print_cfg().find(&trimmed) {
            Some(_) => true,
            None => false,
        },
    }
}

/// Returns Some(True) if predicate is in environment variable.
/// 
/// Returns None if no env variable.
#[inline(always)]
pub(crate) fn is_predicate_in_env(predicate : &String) -> Option<bool> {

    // 1. Extract predicate from value
    let cleaned = predicate.replace("\"", "");  // Remove ""
    let mut split = cleaned.split("=");

    match split.next() {
        Some(label) => {
            match split.next() {
                Some(value) => 
                // 2. Get env variable of predicate in CARGO_CFG
                match env::var(format!("CARGO_CFG_{}", label.to_uppercase())) {
                    Ok(env_value) => match env_value.find(value){
                        Some(_) => Some(true),
                        None => Some(false),
                    },
                    Err(_) => { 
                        // 3. Get env variable of predicate in CARGO_FEATURE
                        let label = label.replace("-", "_"); // ... name of the feature uppercased and having - translated to _
                        match env::var(format!("CARGO_FEATURE_{}", label.to_uppercase())) {
                            Ok(_) => Some(true),
                            Err(_) => {
                                // 4. Try label directly as is.
                                match env::var(format!("{}", label)) {
                                    Ok(env_value) => match env_value.find(value){
                                        Some(_) => Some(true),
                                        None => Some(false),
                                    },
                                    Err(_) => {
                                        // 5. Try label as uppercase.
                                        match env::var(format!("{}", label.to_uppercase())) {
                                            Ok(env_value) => match env_value.find(value){
                                                Some(_) => Some(true),
                                                None => Some(false),
                                            },
                                            Err(_) => {
                                                // 6. Try label as lowercase.
                                                match env::var(format!("{}", label.to_lowercase())) {
                                                    Ok(env_value) => match env_value.find(value){
                                                        Some(_) => Some(true),
                                                        None => Some(false),
                                                    },
                                                    Err(_) => None, 
                                                }
                                            }, 
                                        }
                                    }, 
                                }
                            }, 
                        }
                    },
                },
                None => panic!("{}", CfgBoostError::InvalidPredicateFormat.message(predicate)),
            }
        }
        None => panic!("{}", CfgBoostError::InvalidPredicateFormat.message(predicate)),
    }    
}