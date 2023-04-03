// Test 022 : Custom predicates.
use cfg_boost::{ target_cfg, match_cfg, meta_cfg };

/**************
 * TARGET_CFG *
 **************/
target_cfg!{
    foo:c1 => { pub fn tar_foo() {} },  // c1
    foo:c2 => { pub fn ttf_foo() {} },  // c2
    foo:c3 => { pub fn tos_foo() {} },  // c3
    foo:c4 => { pub fn tfm_foo() {} },  // c4
    foo:c5 => { pub fn tev_foo() {} },  // c5
    foo:c6 => { pub fn ted_foo() {} },  // c6
    foo:c7 => { pub fn tpw_foo() {} },  // c7
    foo:c8 => { pub fn tvn_foo() {} },  // c8
    foo:c9 => { pub fn tat_foo() {} },  // c9
    foo:really_long_predicate_and_i_mean_really_longgggggggggg => { pub fn tpn_foo() {} },  // really_long_predicate_and_i_mean_really_longgggggggggg
    foo:x => { pub fn tft_foo() {} },  // x
}

/*************
 * MATCH_CFG *
 *************/
// ar : target architecture
fn match_foo() -> String {
    match_cfg!{
        foo:c1 => String::from("tar_foo"),  // c1
        foo:c2 => String::from("ttf_foo"),  // c2
        foo:c3 => String::from("tos_foo"),  // c3
        foo:c4 => String::from("tfm_foo"),  // c4
        foo:c5 => String::from("tev_foo"),  // c5
        foo:c6 => String::from("ted_foo"),  // c6
        foo:c7 => String::from("tpw_foo"),  // c7
        foo:c8 => String::from("tvn_foo"),  // c8
        foo:c9 => String::from("tat_foo"),  // c9
        foo:really_long_predicate_and_i_mean_really_longgggggggggg => String::from("tpn_foo"),  // really_long_predicate_and_i_mean_really_longgggggggggg
        foo:x => String::from("tft_foo"),  // x
        _ => String::from("022")
    }
}



/**************
 * meta_cfg *
 **************/
#[meta_cfg(foo:c1 | foo:c2 | foo:c3 | foo:c4 | foo:c5 | foo:c6 | foo:c7 | foo:c8 | foo:c9 | foo:really_long_predicate_and_i_mean_really_longgggggggggg | foo:x)]
fn cfg_dismissed() -> String {
    String::from("dismissed!")
}

#[meta_cfg(!(foo:c1 | foo:c2 | foo:c3 | foo:c4 | foo:c5 | foo:c6 | foo:c7 | foo:c8 | foo:c9 | foo:really_long_predicate_and_i_mean_really_longgggggggggg | foo:x))]
fn cfg_foo() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", twild_foo(), match_foo(), cfg_foo());
}
