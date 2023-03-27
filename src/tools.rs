use proc_macro::TokenStream;

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
