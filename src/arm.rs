use proc_macro::{TokenStream, TokenTree, Delimiter};

use crate::{errors::CfgBoostError, config::{DOC_ALIAS, is_cfg_boost_autodoc, if_docsrs_enabled}, syntax::{SyntaxTreeNode, AND_SYMBOL, OR_SYMBOL, NEGATIVE_SYMBOL}, CfgBoostMacroSource};

#[allow(unused_imports)]
use crate::config::{get_release_modifier_behaviour, ReleaseModifierBehaviour};

/// Target arm separator
pub(crate) const ARM_SEPARATOR : char = ',';

/// Target attribute => content separator
pub(crate) const CONTENT_SEPARATOR_0 : char = '=';
pub(crate) const CONTENT_SEPARATOR_1 : char = '>';

/// Wild card _ arm symbol
pub(crate) const WILDCARD_ARM : char = '_';
pub(crate) const WILDCARD_ARM_STR : &str = "_";

/// Legacy arm detector
pub(crate) const LEGACY_ARM: char = '#';

/// Activate arm modifier
pub(crate) const MODIFIER_ACTIVATE: char = '+';
pub(crate) const MODIFIER_ACTIVATE_VALUE: &str = "all()";   // Is always true

/// Deactivate arm modifier
pub(crate) const MODIFIER_DEACTIVATE: char = '-';
pub(crate) const MODIFIER_DEACTIVATE_VALUE: &str = "any()";   // Is always false

/// Panic arm modifier used to see arm parameters
pub(crate) const MODIFIER_PANIC: char = '@'; 



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

/// Enumeration of possible arm modifiers
#[derive(Debug, Clone, Copy)]
pub(crate) enum TargetArmModifier {
    /// No modifier on target arm
    None,

    /// Activate (+) modifier.
    Activate,

    /// Deactivate (-) modifier.
    Deactivate,

    /// Panic (@) modifier.
    Panic,
}

/// Struct used that contains an arm type, it's attributes and content.
#[derive(Clone)]
pub(crate) struct TargetArm {
    pub arm_type : TargetArmType,   // Arm type
    pub modifier : TargetArmModifier,   // Arm modifier
    pub arm_ts : TokenStream,       // Left side tokenstream
    pub pred_ts : TokenStream,      // Predicates tokenstream
    pub cfg_ts : TokenStream,       // Tokenstream for #[cfg]
    pub attr_ts : TokenStream,      // Tokenstream for #[cfg_attr]
    pub content : TokenStream,      // Right side tokenstream
}

impl ToString for TargetArm {
    /// Transform self into string.
    fn to_string(&self) -> String {
        format!("\nArm : {}\nSyntax : {:?}\nModifier : {:?}\nPredicates : {}\n#[cfg()] : {}\n#[cfg_attr()] : {}\nContent : {}\n", 
        self.arm_ts.to_string(), self.arm_type, self.modifier, self.pred_ts.to_string(), self.cfg_ts.to_string(), self.attr_ts.to_string(), self.content.to_string())
    }
}

impl TargetArm {
    /// Create a new empty normal arm.
    pub fn new() -> TargetArm {
        TargetArm { arm_type : TargetArmType::Simplified, modifier:TargetArmModifier::None, arm_ts : TokenStream::new(), pred_ts : TokenStream::new(), cfg_ts : TokenStream::new(), attr_ts : TokenStream::new(), content : TokenStream::new() }
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
                    Self::extract_content(&mut arm, &mut arms, token, &mut left_side);
                }
            }
        }

        // 2. Add last arm if it were not added (missing `,` at last entry is not an error.)
        if !left_side {
            Self::add_arm(&mut arms, &mut arm, &mut left_side);
        }

        // 3. Verify arms integrity.
        Self::verify_arms_integrity(macro_src, &mut arms);

        // 4. Generate arms predicates
        Self::generate_arms_predicate(macro_src, &mut arms);

        // 5. Panic! for arms with @
        Self::panic_arms(&arms);

        // 6. Return arms vector
        arms

    }

    /// Panic for arms with @.
    #[inline(always)] 
    fn panic_arms(arms : &Vec<TargetArm>) {
        
        // Get arm with panic modifier
        let arms = arms.iter().filter(|arm| match arm.modifier {
            TargetArmModifier::Panic => true,
            _ => false,
        });

        // Create arms message
        let mut message = String::new();
        for arm in arms {
            message.push_str(&arm.to_string());
        }

        // Panic! if message length > 0
        if message.len() > 0 {
            panic!("\n*** Macro panicked because some arm have the `{}` modifier ***\n{}", MODIFIER_PANIC, message);
        }

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
                MODIFIER_PANIC => {
                    if !arm.arm_ts.is_empty() {
                        panic!("{}", CfgBoostError::ModifierNotFirst.message(""));  // Modifier is not first character
                    }
                    arm.modifier = TargetArmModifier::Panic;
                },
                MODIFIER_ACTIVATE => {
                    if !arm.arm_ts.is_empty() {
                        panic!("{}", CfgBoostError::ModifierNotFirst.message(""));  // Modifier is not first character
                    }
                    // Debug behaviour. Activate arm.
                    #[cfg(debug_assertions)]
                    {
                        arm.modifier = TargetArmModifier::Activate;
                    }
                    // Release behaviour. Panic or Ignore.
                    #[cfg(not(debug_assertions))]
                    {
                        match get_release_modifier_behaviour() {
                            ReleaseModifierBehaviour::Panic => panic!("{}", CfgBoostError::ModifierPanicRelease.message("")),  // Modifier release panic
                            _ => {},    // Just ignore it
                        }

                    }
                },
                MODIFIER_DEACTIVATE => {
                    if !arm.arm_ts.is_empty() {
                        panic!("{}", CfgBoostError::ModifierNotFirst.message(""));  // Modifier is not first character
                    }
                    // Debug behaviour. Activate arm.
                    #[cfg(debug_assertions)]
                    {
                        arm.modifier = TargetArmModifier::Deactivate;
                    }
                    // Release behaviour. Panic or Ignore.
                    #[cfg(not(debug_assertions))]
                    {
                        match get_release_modifier_behaviour() {
                            ReleaseModifierBehaviour::Panic => panic!("{}", CfgBoostError::ModifierPanicRelease.message("")),  // Modifier release panic
                            _ => {},    // Just ignore it
                        }

                    }

                },
                LEGACY_ARM => {
                    if !arm.arm_ts.is_empty() {
                        panic!("{}", CfgBoostError::MixedSyntaxError.message(""));  // Mixed syntax error
                    }
                    arm.arm_type = TargetArmType::Legacy;
                    arm.arm_ts.extend(TokenStream::from(token));
                },
                NEGATIVE_SYMBOL | AND_SYMBOL | OR_SYMBOL => {   // Verify if mixed syntax
                    match arm.arm_type{
                        TargetArmType::Legacy => panic!("{}", CfgBoostError::MixedSyntaxError.message("")),  // Mixed syntax error
                        _ => arm.arm_ts.extend(TokenStream::from(token)),
                    }

                },
                ARM_SEPARATOR => panic!("{}", CfgBoostError::ContentSeparatorMissing.message("")),  // Arm content separator error
                CONTENT_SEPARATOR_0 | CONTENT_SEPARATOR_1 => {},    // Ignore tokens
                _ => arm.arm_ts.extend(TokenStream::from(token)),
            },
            _ => arm.arm_ts.extend(TokenStream::from(token)), // Add token to attributes
        }
    }


    /// Extract tokens for content.
    #[inline(always)]
    fn extract_content(arm : &mut TargetArm, arms : &mut Vec<TargetArm>, token : TokenTree, left_side : &mut bool) {

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
                ARM_SEPARATOR => Self::add_arm(arms, arm, left_side),    // Add arm to vector.
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
    fn add_arm(arms : &mut Vec<TargetArm>, arm : &mut TargetArm, left_side : &mut bool){
        // 1. Add arm to arms vector
        arms.push(arm.clone());

        // 2. Reset arm and separator
        *arm = TargetArm::new();
        *left_side = true;

    }

    /// Generate arms predicate used to generate configuration tokenstream.
    #[inline(always)]
    fn generate_arms_predicate(macro_src : CfgBoostMacroSource, arms : &mut Vec<TargetArm>){

        // Each macro has different predicates behaviour
        match macro_src {
            CfgBoostMacroSource::TargetMacro => {
                // For each arm
                arms.iter_mut().for_each(|arm| {
                    // 1. Generate predicate_ts
                    arm.pred_ts.extend(Self::generate_pred_ts(arm.arm_type, arm.arm_ts.clone()));

                    // 2. Generate cfg_ts with doc according to modifier
                    arm.cfg_ts.extend(Self::generate_target_cfg_ts(Self::set_default_doc(arm.pred_ts.clone()), arm.modifier));

                    // 3. Generate attr_ts
                    arm.attr_ts.extend(Self::generate_target_attr_ts(arm.pred_ts.clone()));
                });

            },
            CfgBoostMacroSource::MatchMacro => {
                // Debug behaviour. Set modifiers.
                #[cfg(debug_assertions)]
                {
                    // If contains activated arm, deactivate all others
                    if arms.iter().filter(|arm| match arm.modifier{
                        TargetArmModifier::Activate => true,
                        _ => false,
                    }).count() > 0 {
                        arms.iter_mut().for_each(|arm| match arm.modifier {
                            TargetArmModifier::Activate => {},
                            _ => arm.modifier = TargetArmModifier::Deactivate ,
                        });
                    }
                }

                // Used to accumulate tokenstream for condition. Default is all() == true
                let mut cumul_ts = MODIFIER_ACTIVATE_VALUE.parse::<TokenStream>().unwrap();

                // For each arm
                arms.iter_mut().for_each(|arm| {
                    // 1. Generate predicate_ts
                    arm.pred_ts.extend(Self::generate_pred_ts(arm.arm_type, arm.arm_ts.clone()));

                    // 2. Generate pred_ts from cumulatives tokenstream according to arm type
                    let pred_ts = format!("all({},{})", cumul_ts, arm.pred_ts.clone()).parse::<TokenStream>().unwrap();

                    // 3. Generate cfg_ts according to modifier and pred_ts
                    arm.cfg_ts.extend(Self::generate_target_cfg_ts(pred_ts.clone(), arm.modifier));

                    // 4. Cumulate tokenstream for arm exclusivity.
                    cumul_ts.extend(format!(", not({})", { // Wish I could use match_cfg! here =(
                        #[cfg(debug_assertions)]
                        {
                            match arm.modifier {
                                TargetArmModifier::Activate => MODIFIER_ACTIVATE_VALUE.parse::<TokenStream>().unwrap(),
                                TargetArmModifier::Deactivate => MODIFIER_DEACTIVATE_VALUE.parse::<TokenStream>().unwrap(),
                                _ => arm.pred_ts.clone(),
                            }
                        }
                        #[cfg(not(debug_assertions))]
                        {
                            arm.pred_ts.clone()
                        }
                    }).parse::<TokenStream>().unwrap());
                });
            },
        }
    }

    /// Generate #[cfg] tokenstream for target_cfg!.
    /// Return ts created.
    #[inline(always)]
    fn generate_target_cfg_ts(pred_ts : TokenStream, modifier : TargetArmModifier) -> TokenStream {

        let mut pred_str = pred_ts.to_string();

        // Debug behaviour. Generate cfg_ts according to modifier
        #[cfg(debug_assertions)]
        {
            match modifier{
                TargetArmModifier::Activate => pred_str = String::from(MODIFIER_ACTIVATE_VALUE),
                TargetArmModifier::Deactivate => pred_str = String::from(MODIFIER_DEACTIVATE_VALUE),
                _ => {}
            }
        }

        format!("#[cfg({})]", pred_str).parse::<TokenStream>().unwrap()
    }


    /// Generate predicate tokenstream for arm.
    /// Return ts created.
    #[inline(always)]
    fn generate_pred_ts(arm_type : TargetArmType, arm_ts : TokenStream) -> TokenStream {

        match arm_type{
            TargetArmType::Simplified => {
                let syntax_tree = SyntaxTreeNode::generate(arm_ts); // Simplified predicates comes from syntax tree
                syntax_tree.to_string().parse::<TokenStream>().unwrap()
            },
            TargetArmType::Legacy => {
                Self::extract_legacy_predicates(arm_ts)
            },
            TargetArmType::Wildcard => MODIFIER_ACTIVATE_VALUE.parse::<TokenStream>().unwrap(),  // Wildcard pred_ts is MODIFIER_ACTIVATE_VALUE
        }

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

    /// Add default doc tokenstream to attributes if not present for legacy syntax.
    /// Return ts created.
    #[inline(always)]
    fn set_default_doc(pred_ts : TokenStream) -> TokenStream {

        // Only if env setting is true
        if is_cfg_boost_autodoc() {
            if Self::is_set_attr_autodoc(pred_ts.clone()) { // If already set, change nothing
                pred_ts
            } else {
                format!("any({}, {})", DOC_ALIAS, pred_ts).parse::<TokenStream>().unwrap()
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
                    if ident.to_string().as_str().eq(DOC_ALIAS) {
                        return true;
                    }
                },
                TokenTree::Group(grp) => return Self::is_set_attr_autodoc(grp.stream()),
                _ => {}               
            }
        }

        false
    }


    /// This function ensure arms integrity by checking for errors.
    /// 
    /// Errors verified :
    /// CfgBoostError::EmptyArm
    /// CfgBoostError::WildcardArmOnTarget
    /// CfgBoostError::TargetInFunction
    /// CfgBoostError::WildcardArmMissing
    /// CfgBoostError::MatchModifierMoreThanOneActivate
    /// CfgBoostError::MatchDeactivatedWildArm
    #[inline(always)]
    fn verify_arms_integrity(macro_src : CfgBoostMacroSource, arms: &mut Vec<TargetArm>) {

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
                    if arm.arm_ts.is_empty() {  // Arms ts must not be empty
                        panic!("{}", CfgBoostError::EmptyArm.message(""));
                    } 
                }
            },
            _ => {  
                if !Self::has_wild_arm(&arms){  // Make sure a wildcard arm is written.
                    panic!("{}", CfgBoostError::WildcardArmMissing.message(""));
                }

                // Test for more than 1 activated arm
                let mut activated:usize  = 0;

                arms.iter().for_each(|arm| match arm.modifier {
                    TargetArmModifier::Activate => activated += 1,  // Increment activated arms
                    TargetArmModifier::Deactivate => match arm.arm_type{
                        TargetArmType::Wildcard => panic!("{}", CfgBoostError::MatchDeactivatedWildArm.message("")),    // Wildcard arm cannot be deativated
                        _ => {},
                    },
                    _ => {},
                });

                if activated > 1 {  // Cannot have more than 1 activated in match_cfg!
                    panic!("{}", CfgBoostError::MatchModifierMoreThanOneActivate.message(""));
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