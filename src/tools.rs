use proc_macro::TokenStream;

use crate::{option::TargetMatchOption, errors::TargetCfgError};


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
                        _ => symbol.extend(TokenStream::from(t)),
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

/// Split tokenstream in different [item](https://doc.rust-lang.org/reference/items.html) vector tokenstream.
/// 
/// An item is defined as all tokens until a ; and/or {}.
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

/// Keep track of active target_cfg arms.
pub(crate) struct TargetActiveCounter {
    is_exclusive : bool,

    counter : usize,
}

impl TargetActiveCounter {
    /// Create a new TargetActiveCounter referencing is_exclusive option.
    pub fn new(options : &TargetMatchOption) -> TargetActiveCounter {
        TargetActiveCounter { is_exclusive : options.is_exclusive, counter: 0 }
    }

    /// Increment active counter.
    /// 
    /// Panic(s)
    /// Will panic if counter > 1 and is_exclusive is true.
    pub fn inc(&mut self) {
        self.counter = self.counter + 1;

        if self.is_exclusive && self.counter > 1 {  // Panic if exclusive and more than 1 arm active.
            panic!("{}", TargetCfgError::TargetCfgIsExclusive.message(""));
            
        }
    }
}

/// Struct used to contain an item with attributes.
pub(crate) struct TargetGroup {
    pub attr : TokenStream,
    pub item : TokenStream,
}

impl ToString for TargetGroup {
    /// Transform self into string.
    fn to_string(&self) -> String {
        format!("attr : {}, item : {}", self.attr.to_string(), self.item.to_string())
    }
}

impl TargetGroup {
    /// Create a new TargetGroup from attributes and item.
    pub fn new(attr : TokenStream, item : TokenStream) -> TargetGroup {
        TargetGroup { attr, item }
    }
    /// Extract target groups into a vector.
    pub fn extract(source : TokenStream) -> Vec<TargetGroup> {

        // Tell is next block is attributes
        let mut is_block_attr : bool = true;
        let mut tg : Vec<TargetGroup> = Vec::new();

        let mut attr : TokenStream = TokenStream::new();

        for token in source {

            match token {
                proc_macro::TokenTree::Group(grp) => {
                    if is_block_attr {
                        attr = grp.stream();
                    } else {
                        tg.push(TargetGroup::new(attr, grp.stream()));
                        attr = TokenStream::new();
                    }
                    is_block_attr = !is_block_attr;

                },
                _ => {},
            }
        }

        tg
    }
}
