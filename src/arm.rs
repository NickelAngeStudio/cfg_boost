use proc_macro::{TokenStream, TokenTree, Delimiter, Punct, Group, Ident, Span};

use crate::{errors::CfgBoostError, config::{DOC_ALIAS, is_cfg_boost_autodoc, if_docsrs_enabled}, syntax::{SyntaxTreeNode}, CfgBoostMacroSource};

/// Target arm separator
pub(crate) const ARM_SEPARATOR : char = ',';

/// Target attribute => content separator
pub(crate) const CONTENT_SEPARATOR_0 : char = '=';
pub(crate) const CONTENT_SEPARATOR_1 : char = '>';

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

        // Tell if we are extracting for left_side or not
        let mut left_side = true;

        // Flag for 1st part of separator
        let mut separator  = false;

        // 1. Extract Tokens from source
        for token in source {
            // Handle punct to see if left or right side
            if !Self::handle_arm_separator(&mut arm, token.clone(), &mut left_side, &mut separator) {
                // If token was not handled by separator
                if left_side {  // Extract for left side (attributes)
                    Self::extract_attributes(&mut arm, token);
                } else {    // Extract for right side (content)
                    Self::extract_content(&mut arm, &mut arms, token, &mut left_side, macro_src);
                }
            }
        }

        // 2. Add last arm if it were not added (missing `,` at last entry is not an error.)
        if !left_side {
            Self::add_arm(&mut arms, &mut arm, &mut left_side, macro_src);
        }

        // 3. Verify arms integrity before returning them
        Self::verify_arms_integrity(macro_src, &mut arms);

        // 4. Return arms vector
        arms

    }


    /// Extract tokens for attributes.
    #[inline(always)]
    fn extract_attributes(arm : &mut TargetArm, token : TokenTree) {
        match token.clone() {
            TokenTree::Group(grp) => {
                match arm.arm_type {    // Make sure legacy syntax is correct
                    TargetArmType::Legacy => match grp.delimiter() {
                        Delimiter::Bracket => {},
                        _ => panic!("{}", CfgBoostError::LegacySyntaxError.message(token.to_string().as_str())),  // Panic since legacy isn't formatted correctly
                    },
                    _ => {},
                }
                arm.arm_ts.extend(TokenStream::from(token));
            },
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
                CONTENT_SEPARATOR_0 | CONTENT_SEPARATOR_1 => {},    // Ignore tokens
                _ => arm.arm_ts.extend(TokenStream::from(token)),
            },
            _ => arm.arm_ts.extend(TokenStream::from(token)), // Add token to attributes
        }
    }


    /// Extract tokens for content.
    #[inline(always)]
    fn extract_content(arm : &mut TargetArm, arms : &mut Vec<TargetArm>, token : TokenTree, left_side : &mut bool, macro_src : CfgBoostMacroSource) {

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
                ARM_SEPARATOR => Self::add_arm(arms, arm, left_side, macro_src),    // Add arm to vector.
                _ => arm.content.extend(TokenStream::from(token)),  // Add content to arm
            },
             // Add content to arm
            _ => arm.content.extend(TokenStream::from(token)),
        }

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

        panic!("{}", CfgBoostError::LegacySyntaxError.message(legacy.to_string().as_str()));  // Panic since legacy isn't formatted correctly

    }

    /// Process tokens punctuation and determine if left side or right side.
    /// 
    /// Returns true if token was handled.
    /// 
    /// Panic(s)
    /// Will panic! if arm separator comma is missing.
    #[inline(always)]
    fn handle_arm_separator(arm : &mut TargetArm, token : TokenTree, left_side : &mut bool, separator : &mut bool) -> bool {
        match token.clone() {
            TokenTree::Punct(punct) => {
                match punct.as_char(){
                    CONTENT_SEPARATOR_0 => {
                        if *separator && *left_side {   // Double == in left side
                            panic!("{}", CfgBoostError::ContentSeparatorError.message(""));
                        } else {
                            *separator = true;
                        }
                    },
                    CONTENT_SEPARATOR_1 => {
                        if *separator {
                            *left_side = !*left_side;     // Switch side
                            *separator = false;

                            if *left_side && !arm.content.is_empty() {    // Missing comma `,` arm separator.
                                panic!("{}", CfgBoostError::ArmSeparatorMissing.message(""));
                            }

                            return true;
                        } else {
                            if *left_side { // Missing = before >
                                panic!("{}", CfgBoostError::ContentSeparatorError.message(""));
                            }
                        }                        
                    },
                    _ => *separator = false,    // Reset separator
                }
            },
            _ => *separator = false // Reset separator
        }

        false
    }

    /// Add arm to arms vector according to macro source.
    #[inline(always)]
    fn add_arm(arms : &mut Vec<TargetArm>, arm : &mut TargetArm, left_side : &mut bool, macro_src : CfgBoostMacroSource){

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

        // 4. Reset arm and separator
        *arm = TargetArm::new();
        *left_side = true;

    }

    /// Generate #[cfg] tokenstream for target_cfg!.
    /// Return ts created.
    #[inline(always)]
    fn generate_target_cfg_ts(pred_ts : TokenStream) -> TokenStream {

        format!("#[cfg({})]", pred_ts.to_string()).parse::<TokenStream>().unwrap()

    }

    /// Generate #[cfg_attr] tokenstream for target_cfg!.
    /// Return ts created.
    #[inline(always)]
    fn generate_target_attr_ts(pred_ts : TokenStream) -> TokenStream {

        if if_docsrs_enabled() {    // Only is docsrs is enabled
            format!("#[cfg_attr(docsrs, doc(cfg({})))]", pred_ts.to_string()).parse::<TokenStream>().unwrap()
        } else {
            TokenStream::new()
        }

    }

    /// Generate #[cfg] tokenstream for match_cfg!.
    /// Return ts created.
    #[inline(always)]
    fn generate_match_cfg_ts(pred_ts : TokenStream, arms : &Vec<TargetArm>) -> TokenStream {

        format!("#[cfg(all({},{}))]", Self::extract_match_cfg_ts_from_arms(arms).to_string(), pred_ts.to_string()).parse::<TokenStream>().unwrap()

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
                format!("any({}, {})", DOC_ALIAS.0, pred_ts).parse::<TokenStream>().unwrap()
            }
        } else {    // Make no changes
            pred_ts
        }

    }

    /// Return true if item has doc set.
    #[inline(always)]
    fn is_set_attr_autodoc(attr : TokenStream) -> bool {
        for token in attr.clone() {
            match token.clone() {
                TokenTree::Ident(ident) => {
                    if ident.to_string().as_str().eq(DOC_ALIAS.0) {
                        return true;
                    }
                },
                TokenTree::Group(grp) => return Self::is_set_attr_autodoc(grp.stream()),
                _ => {}               
            }
        }

        false
    }


    /// This function make sure arms are verified for error.
    #[inline(always)]
    fn verify_arms_integrity(macro_src : CfgBoostMacroSource, arms: &mut Vec<TargetArm>) {

        // Verify if target_cfg! contains no wildcard and is not in function and validate that match_cfg! has a wildcard arm.
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

}