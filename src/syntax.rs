// Syntax tree used to generate configuration from TokenStream.

use std::{rc::Rc};
use proc_macro::{TokenStream, TokenTree};

use crate::{errors::CfgBoostError, config::{get_cfg_boost_alias, get_cfg_boost_predicate}};

/// SyntaxTreeNode in a RC 
pub(crate) type Node = Rc<SyntaxTreeNode>;

/// Negative symbol
pub(crate) const NEGATIVE_SYMBOL : char = '!';

/// Symbol for AND.
const AND_SYMBOL : char = '&';

/// Symbol for OR.
const OR_SYMBOL : char = '|';


/// Syntax tree node used to parse attribute tokens.
#[derive(Debug)]
pub(crate) enum SyntaxTreeNode {
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
            SyntaxTreeNode::NOT(node) => format!("not({})", node.to_string()),
            SyntaxTreeNode::ANY(left_node, right_node) => format!("any({},{})", left_node.to_string(), right_node.to_string()),
            SyntaxTreeNode::ALL(left_node, right_node) => format!("all({},{})", left_node.to_string(), right_node.to_string()),
            SyntaxTreeNode::LEAF(label) => 
                match get_cfg_boost_predicate(&label.as_str()) {
                    Ok(predicate) => format!("{}", predicate),
                    Err(err) => panic!("{}", err.message(label)),
                },
        }
    }
}

impl SyntaxTreeNode {
    /// Create a NOT SyntaxTreeNode
    pub fn not_node(child : Node) -> Node {
        Rc::new(SyntaxTreeNode::NOT(child.clone()))
    }

    /// Create an ALL SyntaxTreeNode
    pub fn all_node(left : Node, right : Node) -> Node {
        Rc::new(SyntaxTreeNode::ALL(left.clone(), right.clone()))
    }

    /// Create an ANY SyntaxTreeNode
    pub fn any_node(left : Node, right : Node) -> Node {
        Rc::new(SyntaxTreeNode::ANY(left.clone(), right.clone()))
    }

    /// Generate a SyntaxTreeNode from token stream.
    pub(crate) fn generate(stream : TokenStream) -> Node {

        match split_tokenstream_at_operator(stream.clone()){
            // Means we have to split 
            Some((operator, left, right)) => 
                match operator {
                    AND_SYMBOL => {    // ALL node
                        return Self::all_node(Self::generate(left), Self::generate(right));
                    }
                    OR_SYMBOL => {    // ANY node
                        return Self::any_node(Self::generate(left), Self::generate(right));
                    },
                    _ =>  panic!("{}", CfgBoostError::InvalidCharacter(String::from(operator)).message(&stream.to_string())),
                },
            // No split. Must evaluate if not node, etc...
            None => {
                // Is NOT node?
                let (symbol, content) = extract_negative_symbol(stream.clone());
                
                if is_not_node(symbol) {
                    // Create a NOT node
                    return Self::not_node(Self::generate(content));
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
                                    match get_cfg_boost_alias(&content.to_string()) {
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