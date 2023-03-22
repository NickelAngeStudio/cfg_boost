use proc_macro::TokenStream;

use crate::{target::TargetMatchOption, errors::TargetCfgError};

/// Extract stream of modifiers at the beginning .
/// 
/// Returns a pair containing modifiers tokenstream and the rest of the stream without modifiers.
#[inline(always)]
pub(crate) fn extract_modifier(stream: TokenStream) -> (TokenStream, TokenStream) {

    // Modifiers Tokenstream
    let mut modifier = TokenStream::new();

    // Content Tokenstream
    let mut content = TokenStream::new();

    // Flag that show if in modifiers right now.
    let mut is_modifier = true;

    for t in stream.clone() {

        if is_modifier {
            match t.clone() {
                proc_macro::TokenTree::Group(grp) => match grp.delimiter() {
                    proc_macro::Delimiter::Bracket => {
                        modifier.extend(grp.stream());  // Extract modifiers
                        is_modifier = false;    // Finised parsing modifiers
                    },
                    _ => {
                        content.extend(TokenStream::from(t));
                        is_modifier = false;    // No modifier in tokenstream
                    },
                },
                _ => {
                    content.extend(TokenStream::from(t));
                    is_modifier = false;    // No modifier in tokenstream
                },
            }
        } else {
            content.extend(TokenStream::from(t));
        }


    }

    // Return modifiers and content
    (modifier, content)
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