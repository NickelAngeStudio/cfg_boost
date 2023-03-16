// Parsing functions used by syntax tree.

use std::{env, cell::RefCell, rc::Rc};

use crate::{syntax::{SyntaxNodeType, SyntaxTreeNode}, errors::SyntaxParseError};

/// Parse tokens to generate configuration predicate.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::InvalidConfigurationPredicate]) if predicate not defined.
#[inline(always)]
pub fn parse_cfg_predicate(tokens : &str) -> Result<String, SyntaxParseError> {

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
                        _ => Err(SyntaxParseError::InvalidConfigurationPredicate(String::from(cfg_opt))),
                    },
            }
        },

        // Should never happen but good to have in hand
        None => Err(SyntaxParseError::InvalidConfigurationPredicate(String::from(tokens))),
    } 

}

/// Parse label to generate alias content.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::AliasNotFound]) if alias not defined.
#[inline(always)]
pub fn parse_alias_from_label(label : &str) -> Result<String, SyntaxParseError> {

    // 1. Try to match environment variable to see if it was defined in config.toml.
    match env::var(format!("target_cfg-{}", label)) {
        Ok(alias) => Ok(alias.clone()),     
        Err(_e) => {
            // 2. Try to match predefined alias
            match label {
                // Predefined OS aliases
                "linux" => Ok(String::from("linux:os")),
                "windows" => Ok(String::from("windows:os")),
                "macos" => Ok(String::from("macos:os")),
                "android" => Ok(String::from("android:os")),
                "ios" => Ok(String::from("ios:os")),
                "wasm" => Ok(String::from("wasm:fm")),

                // Predefined platform aliases
                "desktop" => Ok(String::from("linux:os | windows:os | macos:os")),
                "mobile" => Ok(String::from("android:os | ios:os")),

                // Not found, raise error.
                _ => Err(SyntaxParseError::AliasNotFound(String::from(label))),
            }
        },
    }

}


/// Parse tokens to verify if expression is not.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::EmptyNode]) if tokens are empty.
#[inline(always)]
pub fn parse_is_not(tokens: &str) -> Result<bool, SyntaxParseError> {

    match tokens.chars().nth(0) {
        Some(c) => if c == '!' {
            Ok(true)
        } else {
            Ok(false)
        },
        // This shouldn't happens. Indicate a syntax error.
        None => {
            Err(SyntaxParseError::EmptyNode)
        },
    }

}

/// Parse tokens and strip it of outer parentheses.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::EmptyNode]) if tokens are empty.
/// 
/// Returns Err([SyntaxParseError::MismatchedParentheses]) if parentheses are mismatched.
#[inline(always)]
pub fn parse_strip_parentheses(tokens:&str) -> Result<&str, SyntaxParseError> {

    // Start is different when using !
    match parse_is_not(tokens) {
        Ok(is_not) => {
            // If node negative, index start at 1
            let index =if is_not { 1 } else { 0 };

            // Get occurence of parentheses in the beginning and end
            let start = tokens.find("(");
            let end = tokens.rfind(")");

            match start {
                Some(start) => {
                    // Start parentheses will be at pos 2 only if !, else it will be pos 0.
                    if start <= index * 2 {

                        match end {
                            Some(end) => Ok(&tokens[start + 1..end].trim()),
                            None => Err(SyntaxParseError::MismatchedParentheses(start)),
                        }

                    } else {
                        // No parentheses to strip, move along
                        Ok(&tokens[index..].trim())
                    }
                },
                // No parentheses to strip, move along
                None => Ok(&tokens[index..].trim()),
     }


        },
        Err(err) => Err(err),
    }

}

/// Parse tokens and detect any double operator.
/// 
/// Returns Ok(false) if no double operator.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::DoubleOperator]) if any double operator.
#[inline(always)]
pub fn parse_double_operators(tokens:&str) -> Result<bool, SyntaxParseError>{

    match tokens.find("!!"){
        Some(pos) => return Err(SyntaxParseError::DoubleOperator(pos)),
        None => {},
    }

    match tokens.find("&&"){
        Some(pos) => return Err(SyntaxParseError::DoubleOperator(pos)),
        None => {},
    }

    match tokens.find("||"){
        Some(pos) => return Err(SyntaxParseError::DoubleOperator(pos)),
        None => {},
    }

    Ok(false)
}


/// Parse node type (ANY, ALL, LEAF) from tokens.
/// 
/// Returns Ok(SyntaxNodeType) if successul.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::InvalidCharacter]) for invalid characters.
#[inline(always)]
pub fn parse_node_type(tokens:&str)-> Result<SyntaxNodeType, SyntaxParseError> {

    // Counter of closure. If any character is encountered and closure is 0, it means that it is top level.
    let mut closure_count : i32 = 0;

    // Enumerate characters from position from the end
    for p in (0..tokens.len()).rev() {
        let c = tokens.chars().nth(p);
        match c {
            Some(c) => {
                match c {
                    '(' => closure_count -=1,
                    ')' => closure_count +=1,
                    '&' => {
                        // Only if we are at top level
                        if closure_count == 0 {
                            return Ok(SyntaxNodeType::ALL(p));
                        }
                    }
                    '|' => {
                        // Only if we are at top level
                        if closure_count == 0 {
                            return Ok(SyntaxNodeType::ANY(p));
                        }
                    }
                    '!' | '_' | '-' | ' ' | ':' | '.' => {},  // Valids ignored characters
                    _ => {  // Any other characters MUST be alphanumeric.
                        if !c.is_alphanumeric() {
                            return Err(SyntaxParseError::InvalidCharacter(c, p));
                        }
                    }
                }

                // Mismatched if closure is < 0
                if closure_count < 0 {
                    return Err(SyntaxParseError::MismatchedParentheses(p));
                }  
            },
            None => return Err(SyntaxParseError::EmptyNode),
        }

        
    }

    // Mismatched ( if closure is > 0
    if closure_count > 0 {
        return Err(SyntaxParseError::MismatchedParentheses(0));
    }       

    // Node is a leaf
    return Ok(SyntaxNodeType::LEAF(String::from(tokens)));
}


/// Parse a tree leaf from tokens.
/// 
/// Returns leaf parsed.
/// 
/// Panic(s)
/// Will panic! if an operator is missing.
/// Will panic! if an alias is not found.
/// Will panic! if tokens is empty.
#[inline(always)]
pub(crate) fn parse_leaf(tokens: &str) -> Result<Rc<RefCell<SyntaxTreeNode>>, SyntaxParseError> {

    match parse_is_not(tokens){       // If leaf, verify if negative, strip outer parentheses and note symbol.
        Ok(is_not) => {
            match tokens.find("(") {
                Some(_) => {
                    match parse_strip_parentheses(tokens) {
                        Ok(token_strip) => SyntaxTreeNode::generate_syntax_node(token_strip, is_not),
                        Err(err) => Err(err),
                    }
                },
                None => {       // End leaf, return node.
                    let label = String::from(tokens.replace("!", "").trim());

                    if label.matches(" ").count() > 2 { // If it got more than 2 spaces, it's missing an operator
                        panic!("{}", SyntaxParseError::message(&SyntaxParseError::MissingOperator, tokens))
                    } else {
                        // Verify if alias
                        match tokens.find(":") {
                            Some(_) => Ok(SyntaxTreeNode::new(None, None, SyntaxNodeType::LEAF(String::from(label)), is_not)),

                            // Unwrap alias
                            None => 
                                match parse_alias_from_label(&label){
                                    // Alias found, unwrap it.
                                    Ok(alias) => SyntaxTreeNode::generate_syntax_node(&alias, is_not),

                                    // Alias not found, panic!.
                                    Err(err) => panic!("{}", err.message(tokens)),
                                },
                        }
                    }                    
                },
            }
        },
        Err(err) => panic!("{}", err.message(tokens)),
    }
}
    