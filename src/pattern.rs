use proc_macro::TokenStream;



pub struct TargetGroup {
    attr : TokenStream,
    item : TokenStream,
}

impl ToString for TargetGroup {
    fn to_string(&self) -> String {
        format!("attr : {}, item : {}", self.attr.to_string(), self.item.to_string())
    }
}

impl TargetGroup {
    pub fn new(attr : TokenStream, item : TokenStream) -> TargetGroup {
        TargetGroup { attr, item }
    }
    /// Extract target group
    pub fn extract(source : TokenStream) -> Vec<TargetGroup> {

        // Tell is next block is attributes
        let mut is_block_attr : bool = true;
        let mut tg : Vec<TargetGroup> = Vec::new();

        let mut attr : TokenStream = TokenStream::new();

        for token in source {

            match token {
                proc_macro::TokenTree::Group(grp) => {
                    println!("GRP={}", grp.to_string());
                    println!("SP={:?}", grp.span());
                    println!("CS={:?}", proc_macro::Span::call_site());
                    println!("MS={:?}", proc_macro::Span::mixed_site());
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
