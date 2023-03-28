use proc_macro::TokenStream;

use crate::errors::CfgBoostError;

/// Target arm separator
pub(crate) const ARM_SEPARATOR : char = ',';

/// Target attribute => content separator
pub(crate) const CONTENT_SEPARATOR_0 : char = '=';
pub(crate) const CONTENT_SEPARATOR_1 : char = '>';

/// Wild card _ arm symbol
pub(crate) const WILDCARD_BRANCH : char = '_';


/// Enumeration of possible arm types
#[derive(Debug, Clone, Copy)]
pub(crate) enum TargetArmType {
    /// Normal arm with predicates.
    Normal,

    /// Wildcard arm at the end.
    Wildcard,
}

/// Struct used that contains an arm type, it's attributes and content.
pub(crate) struct TargetArm {
    pub arm_type : TargetArmType,
    pub attr : TokenStream,
    pub content : TokenStream,
}

impl ToString for TargetArm {
    /// Transform self into string.
    fn to_string(&self) -> String {
        format!("arm_type : {:?}, attr : {}, content : {}", self.arm_type, self.attr.to_string(), self.content.to_string())
    }
}

impl TargetArm {
    /// Create a new empty normal arm.
    pub fn new() -> TargetArm {
        TargetArm { arm_type : TargetArmType::Normal, attr : TokenStream::new(), content : TokenStream::new() }
    }

    /// Extract target arms into a vector.
    /// 
    /// Panic
    /// Will panic if no Wildcard arm inserted.
    pub fn extract(source : TokenStream) -> Vec<TargetArm> {

        // Vector of all arms
        let mut arms : Vec<TargetArm> = Vec::new();

        // Arm used to extract attr and content.
        let mut arm = TargetArm::new();

        // Separator flags
        let mut separator = (false, false);


        for token in source {
            match token.clone() {
                // Verify if we extract group stream.
                proc_macro::TokenTree::Group(grp) => {
                    match grp.delimiter() {
                        proc_macro::Delimiter::Brace => {
                            if arm.content.is_empty() { // Add group only if content is empty.
                                Self::add_ts_to_arm(grp.stream(), &mut arm, &arms, separator);  // Extract group stream
                            } else {
                                Self::add_ts_to_arm(TokenStream::from(token), &mut arm, &arms, separator);
                            }
                        },
                        _ => Self::add_ts_to_arm(TokenStream::from(token), &mut arm, &arms, separator),
                    }                    
                },
                proc_macro::TokenTree::Punct(punct) => match punct.as_char() {
                    ARM_SEPARATOR => {   
                        // Add arm to vector.
                        arms.push(arm);

                        // Reset arm
                        arm = TargetArm::new();

                        // Reset separator
                        separator = (false, false);
                    },
                    CONTENT_SEPARATOR_0 => {
                        if !arm.content.is_empty() { // Indicate a missing arm separator
                            panic!("{}", CfgBoostError::ArmSeparatorMissing.message(""));
                        }
                        separator.0 = true
                    },
                    CONTENT_SEPARATOR_1 => {
                        if !arm.content.is_empty() { // Indicate a missing arm separator
                            panic!("{}", CfgBoostError::ArmSeparatorMissing.message(""));
                        }
                        match arm.arm_type {    // Panic if arm attr is empty.
                            TargetArmType::Normal => if arm.attr.is_empty(){
                                panic!("{}", CfgBoostError::EmptyArm.message(""));
                            },
                            _ => {},
                        }
                        separator.1 = true
                    },
                    // Add content to attr or content
                    _ => Self::add_ts_to_arm(TokenStream::from(token), &mut arm, &arms, separator),

                },
                proc_macro::TokenTree::Ident(ident) => {
                    if !(separator.0 || separator.1) {
                        if ident.to_string().eq(&String::from(WILDCARD_BRANCH)) {
                            arm.arm_type = TargetArmType::Wildcard;    // Branch is a wildcard.
                        } else {// Add token to attr or content.
                            Self::add_ts_to_arm(TokenStream::from(token), &mut arm, &arms, separator);
                        }
                    } else {    // Add token to attr or content.
                        Self::add_ts_to_arm(TokenStream::from(token), &mut arm, &arms, separator);
                    }
                },
                // Add content to attr or content
                _ => Self::add_ts_to_arm(TokenStream::from(token), &mut arm, &arms, separator),
            }
        }

         // Add last wildcard arm if it were not added (missing `,` at last entry is not an error.)
        match arm.arm_type {
            TargetArmType::Wildcard => arms.push(arm),
            _ => if !Self::has_wild_arm(&arms){  // Make sure a wildcard arm is written.
                panic!("{}", CfgBoostError::WildcardArmMissing.message(""));
            },
        }

        arms

    }

    /// Add tokenstream to arm attr or content according to separator.
    #[inline(always)]
    fn add_ts_to_arm(token : TokenStream, arm : &mut TargetArm, arms : &Vec<TargetArm>, separator : (bool, bool)) {
        Self::validate_arms(&arms, separator);    // Validate arm structures for integrity.
        if separator.0 || separator.1 {  // Add to content
            arm.content.extend(TokenStream::from(token));
        } else {    // Add to attributes
            arm.attr.extend(TokenStream::from(token));
        }
    }

    /// Validate arms structure and panic! is any syntax error occurs.
    /// 
    /// Panic(s)
    /// Panic if Wildcard arm isn't last
    /// Panic if there is a missing separator between arms.
    /// Panic if arm separator => incorrect.
    #[inline(always)]
    fn validate_arms(arms : &Vec<TargetArm>, separator : (bool, bool)) {
        if Self::has_wild_arm(&arms) { // Panic since wildcard arm isn't last.
            panic!("{}", CfgBoostError::WildcardArmNotLast.message(""));
        }
        if separator.0 && !separator.1 {    // Separator syntax error.
            panic!("{}", CfgBoostError::ContentSeparatorError.message(""));
        }
        if !separator.0 && separator.1 {    // Separator syntax error.
            panic!("{}", CfgBoostError::ContentSeparatorError.message(""));
        }
    }

    

    /// Returns true if a Wild arm is in arms vector.
    #[inline(always)]
    fn has_wild_arm(arms : &Vec<TargetArm>) -> bool {

        for arm in arms {
            match arm.arm_type {
                TargetArmType::Wildcard => return true,
                _ => {},
            }
        }

        // If no match, return false
        false
    }
}