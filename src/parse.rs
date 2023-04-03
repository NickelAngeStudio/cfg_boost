use std::env;

use crate::errors::CfgBoostError;

// Contants
pub(crate) const ENV_KEY_PREDICATE : &str = "target_cfg_predicate-";            // Key used to fetch custom predicate
pub(crate) const ENV_KEY_ALIAS : &str = "target_cfg-";                          // Key used to fetch custom aliases
pub(crate) const PREDICATE_PLACEHOLDER : &str = "{}";                           // Predicate placeholder

// Aliases
pub(crate) const LINUX_ALIAS : (&str, &str) = ("linux", "linux:os");            // Linux alias and value
pub(crate) const UNIX_ALIAS :  (&str, &str) = ("unix", "unix:_");               // Unix alias and value
pub(crate) const WINDOWS_ALIAS : (&str, &str) = ("windows", "windows:_");       // Windows alias and value
pub(crate) const MACOS_ALIAS :  (&str, &str) = ("macos", "macos:os");           // Macos alias and value
pub(crate) const ANDROID_ALIAS :  (&str, &str) = ("android", "android:os");     // Android alias and value
pub(crate) const IOS_ALIAS :  (&str, &str) = ("ios", "ios:os");                 // Ios alias and value
pub(crate) const WASM_ALIAS :  (&str, &str) = ("wasm", "wasm:_");               // Wasm alias and value
pub(crate) const DOC_ALIAS :  (&str, &str) = ("doc", "doc:_");                  // Doc alias and value
pub(crate) const TEST_ALIAS :  (&str, &str) = ("test", "test:_");               // Test alias and value
pub(crate) const DESKTOP_ALIAS :  (&str, &str) = ("desktop", "linux:os | windows:_ | macos:os");   // Desktop alias and value
pub(crate) const MOBILE_ALIAS :  (&str, &str) = ("mobile", "android:os | ios:os");  // Mobile alias and value

// Predicates
pub(crate) const TARGET_ARCH_PREDICATE :  (&str, &str) = ("ar", "target_arch = \"{}\"");        // Target architecture predicate
pub(crate) const TARGET_FEATURE_PREDICATE :  (&str, &str) = ("tf", "target_feature = \"{}\"");  // Target feature predicate
pub(crate) const TARGET_OS_PREDICATE :  (&str, &str) = ("os", "target_os = \"{}\"");            // Target os predicate
pub(crate) const TARGET_FAMILY_PREDICATE :  (&str, &str) = ("fm", "target_family = \"{}\"");    // Target family predicate
pub(crate) const TARGET_ENV_PREDICATE :  (&str, &str) = ("ev", "target_env = \"{}\"");          // Target environment predicate
pub(crate) const TARGET_ENDIAN_PREDICATE :  (&str, &str) = ("ed", "target_endian = \"{}\"");    // Target endian predicate
pub(crate) const TARGET_PW_PREDICATE :  (&str, &str) = ("pw", "target_pointer_width = \"{}\""); // Target pointer width predicate
pub(crate) const TARGET_VENDOR_PREDICATE :  (&str, &str) = ("vn", "target_vendor = \"{}\"");    // Target vendor predicate
pub(crate) const TARGET_ATOMIC_PREDICATE :  (&str, &str) = ("at", "target_has_atomic = \"{}\"");    // Target has atomic predicate
pub(crate) const PANIC_PREDICATE :  (&str, &str) = ("pn", "panic = \"{}\"");                    // Panic predicate
pub(crate) const FEATURE_PREDICATE :  (&str, &str) = ("ft", "feature = \"{}\"");                // Feature predicate
pub(crate) const WILDCARD_PREDICATE :  (&str, &str) = ("_", PREDICATE_PLACEHOLDER);             // Wildcard predicate

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
                        val if TARGET_ARCH_PREDICATE.0.eq(val)  => Ok(String::from(TARGET_ARCH_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_FEATURE_PREDICATE.0.eq(val) => Ok(String::from(TARGET_FEATURE_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_OS_PREDICATE.0.eq(val) => Ok(String::from(TARGET_OS_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_FAMILY_PREDICATE.0.eq(val) => Ok(String::from(TARGET_FAMILY_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_ENV_PREDICATE.0.eq(val) => Ok(String::from(TARGET_ENV_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_ENDIAN_PREDICATE.0.eq(val) => Ok(String::from(TARGET_ENDIAN_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_PW_PREDICATE.0.eq(val) => Ok(String::from(TARGET_PW_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_VENDOR_PREDICATE.0.eq(val) => Ok(String::from(TARGET_VENDOR_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if TARGET_ATOMIC_PREDICATE.0.eq(val) => Ok(String::from(TARGET_ATOMIC_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if PANIC_PREDICATE.0.eq(val) => Ok(String::from(PANIC_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if FEATURE_PREDICATE.0.eq(val) => Ok(String::from(FEATURE_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
                        val if WILDCARD_PREDICATE.0.eq(val) => Ok(String::from(WILDCARD_PREDICATE.1.replace(PREDICATE_PLACEHOLDER, label))),
        
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
                val if LINUX_ALIAS.0.eq(val) => Ok(String::from(LINUX_ALIAS.1)),
                val if UNIX_ALIAS.0.eq(val) => Ok(String::from(UNIX_ALIAS.1)),
                val if WINDOWS_ALIAS.0.eq(val) => Ok(String::from(WINDOWS_ALIAS.1)),
                val if MACOS_ALIAS.0.eq(val) => Ok(String::from(MACOS_ALIAS.1)),
                val if ANDROID_ALIAS.0.eq(val) => Ok(String::from(ANDROID_ALIAS.1)),
                val if IOS_ALIAS.0.eq(val) => Ok(String::from(IOS_ALIAS.1)),
                val if WASM_ALIAS.0.eq(val) => Ok(String::from(WASM_ALIAS.1)),
                val if DOC_ALIAS.0.eq(val) => Ok(String::from(DOC_ALIAS.1)),
                val if TEST_ALIAS.0.eq(val) => Ok(String::from(TEST_ALIAS.1)),
                val if DESKTOP_ALIAS.0.eq(val) => Ok(String::from(DESKTOP_ALIAS.1)),
                val if MOBILE_ALIAS.0.eq(val) => Ok(String::from(MOBILE_ALIAS.1)),

                // Not found, raise error.
                _ => Err(CfgBoostError::AliasNotFound(String::from(label))),
            }
        },
    }

}

/*************
* UNIT TESTS * 
*************/

/// parse_alias_from_label unit tests
#[cfg(test)]
mod parse_alias_from_label_tests {
    use std::time::Instant;


    /// Test a pair of alias, alias value.
    fn test_parse_alias_from_label(alias : (&str, &str)) {
        match super::parse_alias_from_label(alias.0) {
            Ok(result) => {
                // If result != value, panic!
                if result.ne(alias.1) {
                    panic!("parse_alias_from_label_tests::{} test error. Expected {}, got {}!", "test_parse_alias_from_label", alias.1, result);
                }
            },
            Err(err) => panic!("{}", err.message(alias.0)),
        }
    }

    /// Create vector with all predefined aliases
    fn create_alias_vector() -> Vec<(&'static str, &'static str)>{
        vec![super::LINUX_ALIAS,
            super::UNIX_ALIAS,
            super::WINDOWS_ALIAS,
            super::MACOS_ALIAS,
            super::ANDROID_ALIAS,
            super::IOS_ALIAS,
            super::WASM_ALIAS,
            super::DOC_ALIAS,
            super::TEST_ALIAS,
            super::DESKTOP_ALIAS,
            super::MOBILE_ALIAS]
    }

    /// Test all predefined aliases
    #[test]
    fn predefined() {
        let aliases = create_alias_vector();

        // Test each predefined alias
        for alias in aliases {
            test_parse_alias_from_label(alias);
        }
    }

    /// Test custom aliases
    #[test]
    fn custom() {
        // 1. Create list of custom aliases
        let aliases : Vec<(&str, &str)> = vec![
            ("pig", "foo:c9 | foo:really_long_predicate_and_i_mean_really_longgggggggggg | foo:x"),
            ("dog", "foo:ar | foo:tf | foo:os | foo:fm | foo:ev | foo:ed | foo:pw | foo:vn | foo:at | foo:pn | foo:ft"),
            ("cow", "foo:c3 | foo:c4 | foo:c5 | foo:c6"),
            ("parastratiosphecomyia_stratiosphecomyioides", "foo:vn | foo:at | foo:pn | foo:ft"),
            ("mosquito", "foo:c1 | foo:c2 | foo:c3 | foo:c4 | foo:c5 | foo:c6 | foo:c7 | foo:c8 | foo:c9 | foo:really_long_predicate_and_i_mean_really_longgggggggggg | foo:x"),
            ("frog", "foo:pw | foo:vn | foo:at | foo:c3 | foo:c4 | foo:c5 | foo:c6"),
            ("lion", "foo:os | foo:fm | foo:ev | foo:ed | foo:pw"),
            ("fish", "foo:c8"),
            ("b", "foo:ar | foo:fm")];

        // 2. Set custom aliases in env.
        for alias in &aliases {
            std::env::set_var(format!("{}{}", super::ENV_KEY_ALIAS, alias.0), alias.1);
        }

        // 3. Test each custom alias
        for alias in aliases {
            test_parse_alias_from_label(alias);
        }
    }

    /// Test not found error
    #[test]
    #[should_panic]
    fn error() {
        test_parse_alias_from_label(("not_found", "not_found:os"));
    }

    /// Performance stress test. 
    /// Verify 1 000 000 queries to parse_alias_from_label. 
    /// Should take less than 1 sec on recent computers.
    #[test]
    #[ignore]
    fn stress_performance() {
        // Compile aliases in 1 list
        let aliases = create_alias_vector();

        // Get time started
        let start = Instant::now();

        for i in 0..1000000 {
            // Pick an alias from aliases
            let alias = aliases[i & (aliases.len() - 1)];

            // Test each alias picked
            match super::parse_alias_from_label(alias.0) {
                Ok(result) => {
                    // If result != value, panic!
                    if result.ne(alias.1) {
                        panic!("parse_alias_from_label_tests::{} test error. Expected {}, got {}!", "test_parse_alias_from_label", alias.1, result);
                    }
                },
                Err(err) => panic!("{}", err.message(alias.0)),
            }
        }

        // Get elapsed duration
        let duration = start.elapsed();

        // Assert performance and panic! if took too much time.
        assert!(duration.as_millis() < 1000, "Performance issue. Should be less than 1 sec for 1 000 000 on recent machine!");

    }


}

/// parse_cfg_predicate unit tests
#[cfg(test)]
mod parse_cfg_predicate_unit_tests {
    use std::time::Instant;

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

    /// Create a vector of all predefined predicates
    fn create_predicate_vector() -> Vec<(&'static str, &'static str)>{
         vec![
            super::TARGET_ARCH_PREDICATE,
            super::TARGET_FEATURE_PREDICATE,
            super::TARGET_OS_PREDICATE,
            super::TARGET_FAMILY_PREDICATE,
            super::TARGET_ENV_PREDICATE,
            super::TARGET_ENDIAN_PREDICATE,
            super::TARGET_PW_PREDICATE,
            super::TARGET_VENDOR_PREDICATE,
            super::TARGET_ATOMIC_PREDICATE,
            super::PANIC_PREDICATE,
            super::FEATURE_PREDICATE,
            super::WILDCARD_PREDICATE]
    }
    
    /// Test parse_cfg_predicate WILDCARD_PREDICATE
    #[test]
    fn predefined() {
        let predicates = create_predicate_vector();

        // Test each predefined predicates
        for pred in predicates {
            test_cfg_predicate(pred);
        }
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
            test_cfg_predicate(pred);
        }

    }

    /// Test parse_cfg_predicate errors when predicate not found. Must panic!
    #[test]
    #[should_panic]
    fn error() {
        test_cfg_predicate(("not_found", "not_found=\"{}\""));
    }

    /// Performance stress test. 
    /// Verify 1 000 000 queries to parse_cfg_predicate. 
    /// Should take less than 5 sec on recent computers.
    /// 
    /// Note : Predicate construction take 1 secs.
    #[test]
    #[ignore]
    fn stress_performance() {
        // Compile aliases in 1 list
        let predicates = create_predicate_vector();

        // Get time started
        let start = Instant::now();

        for i in 0..1000000 {
            // 1. Pick a predicate
            let predicate = predicates[i & (predicates.len() - 1)];

            // 2. Format predicate label syntax.
            let label = format!("stress_performance:{}", predicate.0);

            // 3. Set predicate control value expected.
            let control = String::from(predicate.1.replace(super::PREDICATE_PLACEHOLDER, "stress_performance"));

            // 4. match result of parse_cfg_predicate function.
            match parse_cfg_predicate(label.as_str()){
                // 4.1. Panic! if result ne control
                Ok(result) => if result.ne(&control){
                    panic!("parse_cfg_predicate::{} test error. Expected {}, got {}!", "target_arch_predicate", control, result);
                },

                // 4.2. Error occured, panic!
                Err(err) => panic!("{}", err.message(label.as_str())),
            }
        }

        // Get elapsed duration
        let duration = start.elapsed();

        // Assert performance and panic! if took too much time.
        assert!(duration.as_millis() < 5000, "Performance issue. Should be less than 5 secs for 1 000 000 on recent machine!");

    }

}