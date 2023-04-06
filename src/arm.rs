use proc_macro::{TokenStream, TokenTree, Delimiter, Punct, Group, Ident, Span};

use crate::{errors::CfgBoostError, config::{DOC_ALIAS, is_cfg_boost_autodoc, if_docsrs_enabled}, syntax::{SyntaxTreeNode}, CfgBoostMacroSource};

/// Target arm separator
pub(crate) const ARM_SEPARATOR : char = ',';

/// Target attribute => content separator
pub(crate) const CONTENT_SEPARATOR_0 : char = '=';
pub(crate) const CONTENT_SEPARATOR_0_VALUE : u8 = 1;
pub(crate) const CONTENT_SEPARATOR_1 : char = '>';
pub(crate) const CONTENT_SEPARATOR_1_VALUE : u8 = 2;

/// Wild card _ arm symbol
pub(crate) const WILDCARD_ARM : char = '_';
pub(crate) const WILDCARD_ARM_STR : &str = "_";

/// Legacy branch detector
pub(crate) const LEGACY_ARM: char = '#';


/// Enumeration of possible arm types
#[derive(Debug, Clone, Copy)]
pub(crate) enum TargetArmType {
    /// Simplified syntax arm.
    Simplified,

    /// Legacy syntax arm
    Legacy,

    /// Wildcard arm at the end.
    Wildcard,
}

/// Struct used that contains an arm type, it's attributes and content.
#[derive(Clone)]
pub(crate) struct TargetArm {
    pub arm_type : TargetArmType,   // Arm type
    pub arm_ts : TokenStream,       // Left side tokenstream
    pub pred_ts : TokenStream,      // Predicates tokenstream
    pub cfg_ts : TokenStream,       // Tokenstream for #[cfg]
    pub attr_ts : TokenStream,      // Tokenstream for #[cfg_attr]
    pub content : TokenStream,      // Right side tokenstream
}

impl ToString for TargetArm {
    /// Transform self into string.
    fn to_string(&self) -> String {
        format!("arm_type : {:?}, arm_ts : {}, pred_ts : {}, cfg_ts : {}, attr_ts : {},  content : {}", 
            self.arm_type, self.arm_ts.to_string(), self.pred_ts.to_string(), self.cfg_ts.to_string(), self.attr_ts.to_string(), self.content.to_string())
    }
}

impl TargetArm {
    /// Create a new empty normal arm.
    pub fn new() -> TargetArm {
        TargetArm { arm_type : TargetArmType::Simplified, arm_ts : TokenStream::new(), pred_ts : TokenStream::new(), cfg_ts : TokenStream::new(), attr_ts : TokenStream::new(), content : TokenStream::new() }
    }

    /// Extract target arms into a vector from macro source.
    /// 
    /// Panic
    /// Will panic if no Wildcard arm inserted.
    pub fn extract(source : TokenStream, macro_src : CfgBoostMacroSource) -> Vec<TargetArm> {

        // Vector of all arms
        let mut arms : Vec<TargetArm> = Vec::new();

        // Arm used to extract attr and content.
        let mut arm = TargetArm::new();

        // Separator flags
        let mut separator : u8 = 0;

        // 1. Extract Tokens from source
        for token in source {
            if separator < CONTENT_SEPARATOR_0_VALUE + CONTENT_SEPARATOR_1_VALUE {  // Extract for left side (attributes)
                Self::extract_attributes(&mut arm, token, &mut separator);
            } else {    // Extract for right side (content)
                Self::extract_content(&mut arm, &mut arms, token, &mut separator, macro_src);
            }
        }

        // 2. Add last arm if it were not added (missing `,` at last entry is not an error.)
        if separator >= CONTENT_SEPARATOR_0_VALUE + CONTENT_SEPARATOR_1_VALUE {
            Self::add_arm(&mut arms, &mut arm, macro_src);
        }

        // 3. Verify arms integrity before returning them
        Self::verify_arms_integrity(macro_src, &mut arms);

        // 4. Return arms vector
        arms

    }

    /// Add arm to arms vector according to macro source.
    #[inline(always)]
    fn add_arm(arms : &mut Vec<TargetArm>, arm : &mut TargetArm, macro_src : CfgBoostMacroSource){

        // 1. Extend pred_ts according to arm type
        arm.pred_ts.extend(match arm.arm_type {
            TargetArmType::Simplified => {
                if arm.arm_ts.is_empty() {  // Arms ts must not be empty
                    panic!("{}", CfgBoostError::EmptyArm.message(""));
                } 
                let syntax_tree = SyntaxTreeNode::generate(arm.arm_ts.clone()); // Simplified predicates comes from syntax tree
                syntax_tree.to_string().parse::<TokenStream>().unwrap()
            },
            TargetArmType::Legacy => {
                if arm.arm_ts.is_empty() {  // Arms ts must not be empty
                    panic!("{}", CfgBoostError::EmptyArm.message(""));
                }
                Self::extract_legacy_predicates(arm.arm_ts.clone())
            },
            TargetArmType::Wildcard => TokenStream::new(),  // Wildcard pred_ts stay empty
        });

        // 2. Generate cfg_ts and attr_ts according to macro source
        match macro_src{
            CfgBoostMacroSource::TargetMacro => {
                arm.cfg_ts.extend(Self::generate_target_cfg_ts(Self::set_default_doc(arm.pred_ts.clone())));
                arm.attr_ts.extend(Self::generate_target_attr_ts(arm.pred_ts.clone()));
            },
            CfgBoostMacroSource::MatchMacro => arm.cfg_ts.extend(Self::generate_match_cfg_ts(arm.pred_ts.clone(), arms)),
        }

        // 3. Add arm to arms vector
        arms.push(arm.clone());

    }

    /// Generate #[cfg] tokenstream for target_cfg!.
    /// Return ts created.
    #[inline(always)]
    fn generate_target_cfg_ts(pred_ts : TokenStream) -> TokenStream {

        let mut cfg_ts = TokenStream::new();

        // 1. Add # char
        cfg_ts.extend(TokenStream::from(TokenTree::from(Punct::new(LEGACY_ARM, proc_macro::Spacing::Joint))));

        // 2. Create cfg(predicates) in a temporary tokenstream
        let mut cfg_pred = TokenStream::new();
        cfg_pred.extend(TokenStream::from(TokenTree::from(Ident::new("cfg", Span::call_site()))));
        cfg_pred.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, pred_ts))));

        // 3. Add cfg_predicates to cfg_ts
        cfg_ts.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Bracket, cfg_pred))));

        // 4. Return tokenstream
        cfg_ts

    }

    /// Generate #[cfg_attr] tokenstream for target_cfg!.
    /// Return ts created.
    #[inline(always)]
    fn generate_target_attr_ts(pred_ts : TokenStream) -> TokenStream {

        let mut attr_ts = TokenStream::new();

        if if_docsrs_enabled() {    // Only is docsrs is enabled
            // 1. Add # char
            attr_ts.extend(TokenStream::from(TokenTree::from(Punct::new(LEGACY_ARM, proc_macro::Spacing::Joint))));

            // 2. Create cfg(predicates) in a temporary tokenstream
            let mut cfg_pred = TokenStream::from(TokenTree::from(Ident::new("cfg", Span::call_site())));
            cfg_pred.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, pred_ts))));

            // 3. Create docsrs, doc() wrapping
            let mut doc_pred = TokenStream::from(TokenTree::from(Ident::new("docsrs", Span::call_site())));
            doc_pred.extend(TokenStream::from(TokenTree::from(Punct::new(ARM_SEPARATOR, proc_macro::Spacing::Alone))));
            doc_pred.extend(TokenStream::from(TokenTree::from(Ident::new("doc", Span::call_site()))));
            doc_pred.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, cfg_pred))));

            // 4. Create cfg_attr( wrapping
            let mut cfg_pred = TokenStream::from(TokenTree::from(Ident::new("cfg_attr", Span::call_site())));
            cfg_pred.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, doc_pred))));

            // 5. Add cfg_predicates to attr_ts
            attr_ts.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Bracket, cfg_pred))));
        }

        // 6. Return tokenstream
        attr_ts

    }

    /// Generate #[cfg] tokenstream for match_cfg!.
    /// Return ts created.
    #[inline(always)]
    fn generate_match_cfg_ts(pred_ts : TokenStream, arms : &Vec<TargetArm>) -> TokenStream {

        let mut cfg_ts = TokenStream::new();

        // 1. Add # char
        cfg_ts.extend(TokenStream::from(TokenTree::from(Punct::new(LEGACY_ARM, proc_macro::Spacing::Joint))));

        // 2. Create exclusive, pred
        let mut ex_content = Self::extract_match_cfg_ts_from_arms(arms);
        ex_content.extend(TokenStream::from(TokenTree::from(Punct::new(ARM_SEPARATOR, proc_macro::Spacing::Alone))));  // Add ,
        ex_content.extend(pred_ts);

        // 3. Wrap exclusive arms and pred
        let mut ex_ts = TokenStream::from(TokenTree::from(Ident::new("all", Span::call_site())));
        ex_ts.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, ex_content))));
        
        // 4. Create cfg(predicates) in a temporary tokenstream
        let mut cfg_pred = TokenStream::from(TokenTree::from(Ident::new("cfg", Span::call_site())));
        cfg_pred.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, ex_ts))));

        // 5. Add cfg_predicates to cfg_ts
        cfg_ts.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Bracket, cfg_pred))));

        // 6. Return tokenstream
        cfg_ts

    }

    /// Generate #[cfg] exclusive tokenstream from arms for match_cfg
    /// Return ts created.
    #[inline(always)]
    fn extract_match_cfg_ts_from_arms(arms : &Vec<TargetArm>) -> TokenStream{

        // 1. Generate pred_ts
        let mut pred_ts = TokenStream::new();
        for arm in arms {   // Extract pred_ts of each arm and wrap them in not()
            pred_ts.extend(TokenStream::from(TokenTree::from(Ident::new("not", Span::call_site()))));   // Add not ident
            pred_ts.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, arm.pred_ts.clone()))));    // Add pred_ts in group
            pred_ts.extend(TokenStream::from(TokenTree::from(Punct::new(ARM_SEPARATOR, proc_macro::Spacing::Alone))));  // Add , at the end
        }


        // 2. Add predicates with parenthesis in all
        let mut ex_ts = TokenStream::from(TokenTree::from(Ident::new("all", Span::call_site())));
        ex_ts.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, pred_ts))));
        
        
        // 3. Return tokenstream
        ex_ts
        
    }

    /// Add default doc tokenstream to attributes if not present for legacy syntax.
    /// Return ts created.
    #[inline(always)]
    fn set_default_doc(pred_ts : TokenStream) -> TokenStream {

        // Only if env setting is true
        if is_cfg_boost_autodoc() {

            if Self::is_set_attr_autodoc(pred_ts.clone()) { // If already set, change nothing
                pred_ts
            } else {
                // 1. Inside any()
                let mut any_content = TokenStream::from(TokenTree::from(Ident::new(DOC_ALIAS.0, Span::call_site())));
                any_content.extend(TokenStream::from(TokenTree::from(Punct::new(ARM_SEPARATOR, proc_macro::Spacing::Alone))));
                any_content.extend(pred_ts);

                // 2. Wrap any()
                let mut doc_ts = TokenStream::from(TokenTree::from(Ident::new("any", Span::call_site())));
                doc_ts.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Parenthesis, any_content))));

                // 3. Return new pred_ts
                doc_ts
            }
        } else {    // Make no changes
            pred_ts
        }

    }

    /// Return true if item has doc set.
    #[inline(always)]
    fn is_set_attr_autodoc(attr : TokenStream) -> bool {
        for token in attr.clone() {
            match token {
                TokenTree::Ident(ident) => {
                    if ident.to_string().as_str().eq(DOC_ALIAS.0) {
                        return true;
                    }
                },
                _ => {},
            }
        }

        false
    }

    /// Extract tokens for attributes.
    #[inline(always)]
    fn extract_attributes(arm : &mut TargetArm, token : TokenTree, separator : &mut u8) {

        match token.clone() {
            TokenTree::Ident(ident) => match ident.to_string().as_str() {   // Verify if branch is wildcard with Ident
                WILDCARD_ARM_STR => {
                    if arm.arm_ts.is_empty() {    // Branch is a wildcard.
                        arm.arm_type = TargetArmType::Wildcard;
                    } else {
                        arm.arm_ts.extend(TokenStream::from(token));
                    }
                },  

                _ => arm.arm_ts.extend(TokenStream::from(token)),

            },
            TokenTree::Punct(punct) => match punct.as_char() {      // Verify syntax key symbol
                LEGACY_ARM => {
                    arm.arm_type = TargetArmType::Legacy;
                    arm.arm_ts.extend(TokenStream::from(token));
                },
                CONTENT_SEPARATOR_0 => {
                    if *separator != 0 {    // Content separator error.
                        panic!("{}", CfgBoostError::ContentSeparatorError.message(""));
                    }
                    *separator +=  CONTENT_SEPARATOR_0_VALUE;
                },
                CONTENT_SEPARATOR_1 => {
                    if *separator != CONTENT_SEPARATOR_0_VALUE {    // Content separator error.
                        panic!("{}", CfgBoostError::ContentSeparatorError.message(""));
                    }
                    *separator +=  CONTENT_SEPARATOR_1_VALUE;
                },
                _ => arm.arm_ts.extend(TokenStream::from(token)),
            },


            _ => arm.arm_ts.extend(TokenStream::from(token)), // Add token to attributes
        }
    }


    /// Extract tokens for content.
    #[inline(always)]
    fn extract_content(arm : &mut TargetArm, arms : &mut Vec<TargetArm>, token : TokenTree, separator : &mut u8, macro_src : CfgBoostMacroSource) {

        match token.clone() {
            TokenTree::Group(grp) => arm.content.extend(match grp.delimiter() {
                proc_macro::Delimiter::Brace => {
                    if arm.content.is_empty() { // Add group stream only if content is empty.
                        grp.stream()  // Extract group stream
                    } else {
                        TokenStream::from(token) // Add token to content
                    }
                },
                _ => TokenStream::from(token), // Add token to content
            }),
            TokenTree::Punct(punct) => match punct.as_char() {
            
                ARM_SEPARATOR => {
                    // Add arm to vector.
                    Self::add_arm(arms, arm, macro_src);
    
                    // Reset arm
                    *arm = TargetArm::new();
    
                    // Reset separator
                    *separator = 0;
                },
                // Add content to arm
                _ => arm.content.extend(TokenStream::from(token)),
    
            },
             // Add content to arm
            _ => arm.content.extend(TokenStream::from(token)),
        }

    }


    /// This function make sure arms are verified for error.
    #[inline(always)]
    fn verify_arms_integrity(macro_src : CfgBoostMacroSource, arms: &mut Vec<TargetArm>) {

        // Verify missing comma
        Self::verify_missing_comma(arms);

        // Verify Wildcard arm according to macro source and that single macro has only 1 arm.
        match macro_src {
            CfgBoostMacroSource::TargetMacro => {
                if Self::has_wild_arm(&arms){  // Single macro doesn't accept wildcard arms!
                    panic!("{}", CfgBoostError::WildcardArmOnTarget.message(""));
                }

                // // If any arm is inside a function, panic!
                for arm in arms {
                    if Self::is_inside_function(arm) {  
                        panic!("{}", CfgBoostError::TargetInFunction.message(""));
                    }
                }
            },
            _ => {  
                if !Self::has_wild_arm(&arms){  // Make sure a wildcard arm is written.
                    panic!("{}", CfgBoostError::WildcardArmMissing.message(""));
                }
            }
        }

        

    }

    /// Scan for missing comma separator and panic! if any.
    #[inline(always)]
    fn verify_missing_comma(arms: &mut Vec<TargetArm>) {
        for arm in arms {
            // Upper level tokenstream shouldn't have =>, indicate missing `,`
            let mut separator:u8 = 0;
            for token in arm.content.clone() {
                match token {
                    TokenTree::Punct(punct) => match punct.as_char(){
                        CONTENT_SEPARATOR_0 => {
                            separator += CONTENT_SEPARATOR_0_VALUE;
                        },
                        CONTENT_SEPARATOR_1 => {
                            if separator == CONTENT_SEPARATOR_0_VALUE {     // Missing `,`
                                panic!("{}", CfgBoostError::ArmSeparatorMissing.message(""));
                            }
                        },
                        _ => separator = 0,
                    },
                    _ => separator = 0,
                }
            }
        }
    }

    /// Returns true if arm of macro is inside a function.
    /// 
    /// This function tries to detect `let` and flow of control keywords to determine if inside or not.
    /// 
    /// Since accuracy isn't 100%, it isn't used to validate that match_cfg! is inside a function. Only to detect if target_cfg! is.
    #[inline(always)]
    fn is_inside_function(arm: &TargetArm) -> bool {

        for t in arm.content.clone() {
            match t {
                proc_macro::TokenTree::Ident(ident) => match ident.to_string().as_str() {
                    "let" | "if" | "else" | "loop" | "break" | "while" | "for" | "match" | "println" | "panic"   => return true,    // Those keyword are only found inside functions.
                    _ => {},
                },
                _ => {},
            }
        }

        false
    }

    /// Returns true if a Wild arm is in arms vector.
    #[inline(always)]
    fn has_wild_arm(arms : &Vec<TargetArm>) -> bool {

        let mut has_wc = false;

        for arm in arms {
            if has_wc { // Panic since wildcard arm isn't last.
                panic!("{}", CfgBoostError::WildcardArmNotLast.message(""));
            }
            match arm.arm_type {
                TargetArmType::Wildcard => has_wc = true,
                _ => {},
            }
        }

        // If no match, return false
        has_wc
    }


    /// Extract legacy predicates from legacy syntax
    #[inline(always)]
    fn extract_legacy_predicates(legacy : TokenStream) -> TokenStream {

        for token in legacy.clone() {
            match token {
                TokenTree::Group(grp) => match grp.delimiter() {
                    Delimiter::Parenthesis => return grp.stream(),
                    Delimiter::Bracket => return Self::extract_legacy_predicates(grp.stream()),
                    _ => {},
                },
                _ => {}
            }
        }

        panic!("{}", CfgBoostError::LegacySyntaxError.message(legacy.to_string().as_str()));  // Panic since legacy isn't formatted correctlys

    }



}