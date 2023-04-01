use std::env;

use crate::errors::CfgBoostError;

// Key used to fetch custom predicate
pub(crate) const ENV_KEY_PREDICATE : &str = "target_cfg_predicate-";

// Key used to fetch custom aliases
pub(crate) const ENV_KEY_ALIAS : &str = "target_cfg-";

// Linux alias and value
pub(crate) const LINUX_ALIAS : &str = "linux";
pub(crate) const LINUX_ALIAS_VALUE : &str = "linux:os";

// Unix alias and value
pub(crate) const UNIX_ALIAS : &str = "unix";
pub(crate) const UNIX_ALIAS_VALUE : &str = "unix:_";

// Windows alias and value
pub(crate) const WINDOWS_ALIAS : &str = "windows";
pub(crate) const WINDOWS_ALIAS_VALUE : &str = "windows:_";

// Macos alias and value
pub(crate) const MACOS_ALIAS : &str = "macos";
pub(crate) const MACOS_ALIAS_VALUE : &str = "macos:os";

// Android alias and value
pub(crate) const ANDROID_ALIAS : &str = "android";
pub(crate) const ANDROID_ALIAS_VALUE : &str = "android:os";

// Ios alias and value
pub(crate) const IOS_ALIAS : &str = "ios";
pub(crate) const IOS_ALIAS_VALUE : &str = "ios:os";

// Wasm alias and value
pub(crate) const WASM_ALIAS : &str = "wasm";
pub(crate) const WASM_ALIAS_VALUE : &str = "wasm:_";

// Doc alias and value
pub(crate) const DOC_ALIAS : &str = "doc";
pub(crate) const DOC_ALIAS_VALUE : &str = "doc:_";

// Desktop alias and value
pub(crate) const DESKTOP_ALIAS : &str = "desktop";
pub(crate) const DESKTOP_ALIAS_VALUE : &str = "linux:os | windows:_ | macos:os";

// Mobile alias and value
pub(crate) const MOBILE_ALIAS : &str = "mobile";
pub(crate) const MOBILE_ALIAS_VALUE : &str = "android:os | ios:os";

// Predicate placeholder
pub(crate) const PREDICATE_PLACEHOLDER : &str = "{}";

// Target architecture predicate
pub(crate) const TARGET_ARCH_PREDICATE : &str = "ar";
pub(crate) const TARGET_ARCH_PREDICATE_VALUE : &str = "target_arch = \"{}\"";

// Target feature predicate
pub(crate) const TARGET_FEATURE_PREDICATE : &str = "tf";
pub(crate) const TARGET_FEATURE_PREDICATE_VALUE : &str = "target_feature = \"{}\"";

// Target os predicate
pub(crate) const TARGET_OS_PREDICATE : &str = "os";
pub(crate) const TARGET_OS_PREDICATE_VALUE : &str = "target_os = \"{}\"";

// Target family predicate
pub(crate) const TARGET_FAMILY_PREDICATE : &str = "fm";
pub(crate) const TARGET_FAMILY_PREDICATE_VALUE : &str = "target_family = \"{}\"";

// Target environment predicate
pub(crate) const TARGET_ENV_PREDICATE : &str = "ev";
pub(crate) const TARGET_ENV_PREDICATE_VALUE : &str = "target_env = \"{}\"";

// Target endian predicate
pub(crate) const TARGET_ENDIAN_PREDICATE : &str = "ed";
pub(crate) const TARGET_ENDIAN_PREDICATE_VALUE : &str = "target_endian = \"{}\"";

// Target pointer width predicate
pub(crate) const TARGET_PW_PREDICATE : &str = "pw";
pub(crate) const TARGET_PW_PREDICATE_VALUE : &str = "target_pointer_width = \"{}\"";

// Target vendor predicate
pub(crate) const TARGET_VENDOR_PREDICATE : &str = "vn";
pub(crate) const TARGET_VENDOR_PREDICATE_VALUE : &str = "target_vendor = \"{}\"";

// Target has atomic predicate
pub(crate) const TARGET_ATOMIC_PREDICATE : &str = "at";
pub(crate) const TARGET_ATOMIC_PREDICATE_VALUE : &str = "target_has_atomic = \"{}\"";

// Panic predicate
pub(crate) const PANIC_PREDICATE : &str = "pn";
pub(crate) const PANIC_PREDICATE_VALUE : &str = "panic = \"{}\"";

// Feature predicate
pub(crate) const FEATURE_PREDICATE : &str = "ft";
pub(crate) const FEATURE_PREDICATE_VALUE : &str = "feature = \"{}\"";

// Wildcard predicate
pub(crate) const WILDCARD_PREDICATE : &str = "_";
pub(crate) const WILDCARD_PREDICATE_VALUE : &str = PREDICATE_PLACEHOLDER;

/// Parse tokens to generate configuration predicate.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::InvalidConfigurationPredicate]) if predicate not defined.
#[inline(always)]
pub fn parse_cfg_predicate(tokens : &str) -> Result<String, CfgBoostError> {

    // 1. Extract label and predicate from tokens
    match tokens.find(":") {
        Some(position) => {
            let label = tokens[0..position].trim();
            let cfg_opt = tokens[position + 1..].trim();

            // 2. Try to match environment variable to see if predicate was defined in config.toml.
            match env::var(format!("{}{}", ENV_KEY_PREDICATE, cfg_opt)) {
                Ok(cfg_value) => Ok(String::from(cfg_value.replace(PREDICATE_PLACEHOLDER, label))),
                Err(_) => match cfg_opt {   // 2.2 Try to match default predicate
                        // Default configuration predicates
                        TARGET_ARCH_PREDICATE => Ok(String::from(TARGET_ARCH_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_FEATURE_PREDICATE => Ok(String::from(TARGET_FEATURE_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_OS_PREDICATE => Ok(String::from(TARGET_OS_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_FAMILY_PREDICATE => Ok(String::from(TARGET_FAMILY_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_ENV_PREDICATE => Ok(String::from(TARGET_ENV_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_ENDIAN_PREDICATE => Ok(String::from(TARGET_ENDIAN_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_PW_PREDICATE => Ok(String::from(TARGET_PW_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_VENDOR_PREDICATE => Ok(String::from(TARGET_VENDOR_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        TARGET_ATOMIC_PREDICATE => Ok(String::from(TARGET_ATOMIC_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        PANIC_PREDICATE => Ok(String::from(PANIC_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        FEATURE_PREDICATE => Ok(String::from(FEATURE_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
                        WILDCARD_PREDICATE => Ok(String::from(WILDCARD_PREDICATE_VALUE.replace(PREDICATE_PLACEHOLDER, label))),
        
                        // Not found, raise error.
                        _ => Err(CfgBoostError::InvalidConfigurationPredicate(String::from(cfg_opt))),
                    },
            }
        },

        // Should never happen but good to have in hand
        None => Err(CfgBoostError::InvalidConfigurationPredicate(String::from(tokens))),
    } 

}


/// Parse label to generate alias content.
/// 
/// Error(s)
/// Returns Err([TargetCfgError::AliasNotFound]) if alias not defined.
#[inline(always)]
pub fn parse_alias_from_label(label : &str) -> Result<String, CfgBoostError> {

    // 1. Try to match environment variable to see if it was defined in config.toml.
    match env::var(format!("{}{}", ENV_KEY_ALIAS, label)) {
        Ok(alias) => Ok(alias.clone()),     
        Err(_e) => {
            // 2. Try to match predefined alias
            match label {
                // Predefined OS aliases
                LINUX_ALIAS => Ok(String::from(LINUX_ALIAS_VALUE)),
                UNIX_ALIAS => Ok(String::from(UNIX_ALIAS_VALUE)),
                WINDOWS_ALIAS => Ok(String::from(WINDOWS_ALIAS_VALUE)),
                MACOS_ALIAS => Ok(String::from(MACOS_ALIAS_VALUE)),
                ANDROID_ALIAS => Ok(String::from(ANDROID_ALIAS_VALUE)),
                IOS_ALIAS => Ok(String::from(IOS_ALIAS_VALUE)),
                WASM_ALIAS => Ok(String::from(WASM_ALIAS_VALUE)),
                DOC_ALIAS => Ok(String::from(DOC_ALIAS_VALUE)),
                DESKTOP_ALIAS => Ok(String::from(DESKTOP_ALIAS_VALUE)),
                MOBILE_ALIAS => Ok(String::from(MOBILE_ALIAS_VALUE)),

                // Not found, raise error.
                _ => Err(CfgBoostError::AliasNotFound(String::from(label))),
            }
        },
    }

}

/// parse_cfg_predicate unit tests
#[cfg(test)]
mod parse_cfg_predicate_unit_tests {
    use super::{parse_cfg_predicate};

    /// Test parse_cfg_predicate from a pair of (PREDICATE, PREDICATE_VALUE)
    fn test_cfg_predicate(predicate_tested : (&str,&str)){

        // 1. Set predicate argument value
        const ARGUMENT_VALUE: &str = "test_cfg_predicate";
        
        // 2. Format predicate label syntax.
        let pred = format!("{}:{}", ARGUMENT_VALUE, predicate_tested.0);

        // 3. Set predicate control value expected.
        let control = String::from(predicate_tested.1.replace(super::PREDICATE_PLACEHOLDER, ARGUMENT_VALUE));

        // 4. match result of parse_cfg_predicate function.
        match parse_cfg_predicate(pred.as_str()){
            // 4.1. Panic! if result ne control
            Ok(result) => if result.ne(&control){
                panic!("parse_cfg_predicate::{} test error. Expected {}, got {}!", "target_arch_predicate", control, result);
            },

            // 4.2. Error occured, panic!
            Err(err) => panic!("{}", err.message(pred.as_str())),
        }
    }
    
    /// Test parse_cfg_predicate TARGET_ARCH_PREDICATE
    #[test]
    fn target_arch_predicate() {
        test_cfg_predicate((super::TARGET_ARCH_PREDICATE, super::TARGET_ARCH_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_FEATURE_PREDICATE
    #[test]
    fn target_feature() {
        test_cfg_predicate((super::TARGET_FEATURE_PREDICATE, super::TARGET_FEATURE_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_OS_PREDICATE
    #[test]
    fn target_os() {
        test_cfg_predicate((super::TARGET_OS_PREDICATE, super::TARGET_OS_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_FAMILY_PREDICATE 
    #[test]
    fn target_family() {
        test_cfg_predicate((super::TARGET_FAMILY_PREDICATE, super::TARGET_FAMILY_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_ENV_PREDICATE
    #[test]
    fn target_env() {
        test_cfg_predicate((super::TARGET_ENV_PREDICATE, super::TARGET_ENV_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_ENDIAN_PREDICATE
    #[test]
    fn target_endian() {
        test_cfg_predicate((super::TARGET_ENDIAN_PREDICATE, super::TARGET_ENDIAN_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_PW_PREDICATE
    #[test]
    fn target_pw() {
        test_cfg_predicate((super::TARGET_PW_PREDICATE, super::TARGET_PW_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_VENDOR_PREDICATE
    #[test]
    fn target_vendor() {
        test_cfg_predicate((super::TARGET_VENDOR_PREDICATE, super::TARGET_VENDOR_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate TARGET_ATOMIC_PREDICATE
    #[test]
    fn target_atomic() {
        test_cfg_predicate((super::TARGET_ATOMIC_PREDICATE, super::TARGET_ATOMIC_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate PANIC_PREDICATE
    #[test]
    fn panic() {
        test_cfg_predicate((super::PANIC_PREDICATE, super::PANIC_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate FEATURE_PREDICATE
    #[test]
    fn features() {
        test_cfg_predicate((super::FEATURE_PREDICATE, super::FEATURE_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate WILDCARD_PREDICATE
    #[test]
    fn wildcard() {
        test_cfg_predicate((super::WILDCARD_PREDICATE, super::WILDCARD_PREDICATE_VALUE));
    }

    /// Test parse_cfg_predicate custom predicate
    #[test]
    fn custom() {

        // 1. Create list of custom predicate
        let custom_pred : Vec<(&str, &str)> = vec![
            ("c1", "custom1 = \"{}\""),
            ("c2", "custom2 = \"{}\""),
            ("c3", "custom3 = \"{}\""),
            ("c4", "custom4 = \"{}\""),
            ("c5", "custom5 = \"{}\""),
            ("c6", "custom6 = \"{}\""),
            ("c7", "custom7 = \"{}\""),
            ("c8", "custom8 = \"{}\""),
            ("c9", "custom9 = \"{}\""),
            ("really_long_predicate_and_i_mean_really_longgggggggggg", "really_long_predicate_and_i_mean_really_longgggggggggg = \"{}\""),
            ("x", "x = \"{}\"")];

        // 2. Set custom predicate in env.
        for pred in &custom_pred {
            std::env::set_var(format!("{}{}", super::ENV_KEY_PREDICATE, pred.0), pred.1);
        }

        // 3. Test each custom predicates
        for pred in custom_pred {
            test_cfg_predicate((pred.0, pred.1));
        }

    }

    /// Test parse_cfg_predicate errors when predicate not found. Must panic!
    #[test]
    #[should_panic]
    fn error() {
        test_cfg_predicate(("not_found", "not_found=\"{}\""));
    }


}