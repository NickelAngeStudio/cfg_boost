use std::time::Instant;

use super::ALIASES;
use super::{get_cfg_boost_predicate, PREDICATES};

/// Test all predefined aliases
#[test]
fn predefined_aliases() {
    // Test each predefined alias
    for alias in ALIASES {
        test_parse_alias_from_label(alias);
    }
}

/// Test custom aliases
#[test]
fn custom_aliases() {
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
fn error_alias() {
    test_parse_alias_from_label(("not_found", "not_found:os"));
}

/// Performance stress test. 
/// Verify 1 000 000 queries to parse_alias_from_label. 
/// Should take less than 1 sec on recent computers.
#[test]
#[ignore]
fn stress_performance_aliases() {
    // Get time started
    let start = Instant::now();

    for i in 0..1000000 {
        // Pick an alias from aliases
        let alias = ALIASES[i & (ALIASES.len() - 1)];

        // Test each alias picked
        match super::get_cfg_boost_alias(alias.0) {
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

/// Test parse_cfg_predicate WILDCARD_PREDICATE
#[test]
fn predefined_predicates() {
    // Test each predefined predicates
    for pred in PREDICATES {
        test_cfg_predicate(pred);
    }
}

/// Test parse_cfg_predicate custom predicate
#[test]
fn custom_predicates() {

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
fn error_predicate() {
    test_cfg_predicate(("not_found", "not_found=\"{}\""));
}

/// Performance stress test. 
/// Verify 1 000 000 queries to parse_cfg_predicate. 
/// Should take less than 5 sec on recent computers.
/// 
/// Note : Predicate construction take 1 secs.
#[test]
#[ignore]
fn stress_performance_predicates() {
    // Get time started
    let start = Instant::now();

    for i in 0..1000000 {
        // 1. Pick a predicate
        let predicate = PREDICATES[i & (PREDICATES.len() - 1)];

        // 2. Format predicate label syntax.
        let label = format!("stress_performance:{}", predicate.0);

        // 3. Set predicate control value expected.
        let control = String::from(predicate.1.replace(super::PREDICATE_PLACEHOLDER, "stress_performance"));

        // 4. match result of parse_cfg_predicate function.
        match get_cfg_boost_predicate(label.as_str()){
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


/************
* FUNCTIONS * 
************/
/// Test a pair of alias, alias value.
fn test_parse_alias_from_label(alias : (&str, &str)) {
    match super::get_cfg_boost_alias(alias.0) {
        Ok(result) => {
            // If result != value, panic!
            if result.ne(alias.1) {
                panic!("parse_alias_from_label_tests::{} test error. Expected {}, got {}!", "test_parse_alias_from_label", alias.1, result);
            }
        },
        Err(err) => panic!("{}", err.message(alias.0)),
    }
}


/// Test parse_cfg_predicate from a pair of (PREDICATE, PREDICATE_VALUE)
fn test_cfg_predicate(predicate_tested : (&str,&str)){

    // 1. Set predicate argument value
    const ARGUMENT_VALUE: &str = "test_cfg_predicate";
    
    // 2. Format predicate label syntax.
    let pred = format!("{}:{}", ARGUMENT_VALUE, predicate_tested.0);

    // 3. Set predicate control value expected.
    let control = String::from(predicate_tested.1.replace(super::PREDICATE_PLACEHOLDER, ARGUMENT_VALUE));

    // 4. match result of parse_cfg_predicate function.
    match get_cfg_boost_predicate(pred.as_str()){
        // 4.1. Panic! if result ne control
        Ok(result) => if result.ne(&control){
            panic!("parse_cfg_predicate::{} test error. Expected {}, got {}!", "target_arch_predicate", control, result);
        },

        // 4.2. Error occured, panic!
        Err(err) => panic!("{}", err.message(pred.as_str())),
    }
}