use proc_macro::TokenStream;

use crate::{errors::TargetCfgError, tools::TargetGroup};


/// cfg_target attributes options.
/// 
/// They are detected by the tail symbols ?, @ and *. They are deactivated using !.
/// 
/// Panic(s)
/// Will panic is any other symbol that !, ?, @, * are used.
pub(crate) struct TargetAttributeOption {
    /// Flag that tell if documentation is allowed. cfg!(doc) by default, can be always true with ? or always false with !?.
    pub allow_doc : bool,

    /// Flag that tell if debug symbol `@` has been used. Can be cancelled by !@.
    pub is_debug : bool,

    /// Flag that say that condition is always true. Uses symbol `*`.
    /// None means that it isn't set to force a particular value.
    pub always_this : Option<bool>,
}

impl ToString for TargetAttributeOption {
    fn to_string(&self) -> String {
        format!("allow_doc : `{:?}`, is_debug : `{:?}`, always_this : `{:?}`", self.allow_doc, self.is_debug, self.always_this )
    }
}

impl TargetAttributeOption {
    /// Create a new attribute options from symbol tokenstream.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn new(symbol: TokenStream) -> TargetAttributeOption{

        let mut allow_doc = cfg!(doc);
        let mut is_debug : bool = false;
        let mut always_this : Option<bool> = None;

        // Not flag. Activated by !, consumed by other symbol.
        let mut is_not = false;

        for t in symbol.clone() {
            match t {
                proc_macro::TokenTree::Punct(punc) => {
                    match punc.as_char() {
                        '!' => is_not = true,   // Activate is_not flag
                        '@' => {    // Debug node
                            is_debug = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        '?' => {    // Documentation
                            allow_doc = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        '*' => {    // Always true
                            always_this = Some(!is_not && true);
                            is_not = false; // Consume is_not flag
                        },
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(punc.as_char()).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetAttributeOption{ allow_doc, is_debug, always_this }

    }

    /// Create a new attribute options from symbol tokenstream and parent target_cfg match.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn from_match(match_opt : &TargetMatchOption, symbol: TokenStream) -> TargetAttributeOption {

        let mut allow_doc = match_opt.allow_doc;
        let mut is_debug : bool = match_opt.is_debug;   // Copy default value from match
        let mut always_this : Option<bool> = None;

        // Not flag. Activated by !, consumed by other symbol.
        let mut is_not = false;

        for t in symbol.clone() {
            match t {
                proc_macro::TokenTree::Punct(punc) => {
                    match punc.as_char() {
                        '!' => is_not = true,   // Activate is_not flag
                        '@' => {    // Debug node
                            if ! match_opt.is_debug {
                                is_debug = !is_not && true; 
                            }
                            is_not = false; // Consume is_not flag                           
                        },
                        '?' => {    // Documentation
                            allow_doc = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        '*' => {    // Always true
                            always_this = Some(!is_not && true);
                            is_not = false; // Consume is_not flag
                        },
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(punc.as_char()).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetAttributeOption{ allow_doc, is_debug, always_this }

    }
}

/// target_cfg matching options.
/// 
/// They are detected by the tail symbols ?, @ and $. They are deactivated using !.
/// 
/// Panic(s)
/// Will panic is any other symbol that !, ?, @, $ are used.
pub(crate) struct TargetMatchOption {
    /// Flag that tell if documentation is allowed. cfg!(doc) by default, can be always true with ? or always false with !?.
    /// This flag can be overriden locally by TargetAttributeOption if ? is defined.
    pub allow_doc : bool,

    /// Flag that tell if debug symbol `@` has been used. Can be cancelled by !@.
    /// If true, CANNOT be overriden locally by TargetAttributeOption !@.
    pub is_debug : bool,

    /// Flag that say that target_cfg arms are exclusives. $ is used to tell it is.
    /// If 2 or more arms are true, it will panic!. If it is intended, use !$ to be inclusive.
    pub is_exclusive : bool,

    /// Flag that determine if macro is called from inside a function or not.
    pub is_inner_macro : bool,
}

impl ToString for TargetMatchOption {
    fn to_string(&self) -> String {
        format!("allow_doc : `{:?}`, is_debug : `{:?}`, is_exclusive : `{:?}`, is_inner_macro : `{:?}`", 
            self.allow_doc, self.is_debug, self.is_exclusive, self.is_inner_macro )
    }
}

impl TargetMatchOption {
    /// Create a new match options from symbol tokenstream and target group.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn new(symbol: TokenStream, tg :  &Vec<TargetGroup>) -> TargetMatchOption{

        let mut allow_doc = cfg!(doc);
        let mut is_debug = false;
        let mut is_exclusive  = true;

        // Not flag. Activated by !, consumed by other symbol.
        let mut is_not = false;

        for t in symbol.clone() {
            match t {
                proc_macro::TokenTree::Punct(punc) => {
                    match punc.as_char() {
                        '!' => is_not = true,   // Activate is_not flag
                        '@' => {    // Debug node
                            is_debug = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        '?' => {    // Documentation
                            allow_doc = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        '$' => {    // Exclusive
                            is_exclusive = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(punc.as_char()).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetMatchOption{ allow_doc, is_debug, is_exclusive, is_inner_macro : Self::is_inner_macro(tg) }

    }

    /// Returns true if macro was called inside a function.
    /// 
    /// This function tries to detect `let` and flow of control keywords to determine if inner or not.
    fn is_inner_macro(tg : &Vec<TargetGroup>) -> bool {

        for g in tg  {
            for t in g.item.clone() {
                match t {
                    proc_macro::TokenTree::Ident(ident) => match ident.to_string().as_str() {
                        "let" | "if" | "else" | "loop" | "break" | "while" | "for" | "match" | "println"   => return true,    // Those keyword are only found inside functions.
                        _ => {},
                    },
                    _ => {},
                }
                
            }
        }

        false
    }
}