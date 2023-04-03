// Test 021 : Predefined predicates.
use cfg_boost::{ target_cfg, match_cfg, meta_cfg };

/**************
 * TARGET_CFG *
 **************/
target_cfg!{
    foo:ar => { pub fn tar_foo() {} },  // ar : target architecture
    foo:tf => { pub fn ttf_foo() {} },  // tf : target architecture feature
    foo:os => { pub fn tos_foo() {} },  // os : target operating system
    foo:fm => { pub fn tfm_foo() {} },  // fm : target family
    foo:ev => { pub fn tev_foo() {} },  // ev : target environment
    foo:ed => { pub fn ted_foo() {} },  // ed : target endian
    foo:pw => { pub fn tpw_foo() {} },  // pw : target pointer width
    foo:vn => { pub fn tvn_foo() {} },  // vn : target vendor    
    foo:at => { pub fn tat_foo() {} },  // at : target has atomic
    foo:pn => { pub fn tpn_foo() {} },  // pn : panic feature
    foo:ft => { pub fn tft_foo() {} },  // ft : feature
}

/*************
 * MATCH_CFG *
 *************/
// ar : target architecture
fn match_foo() -> String {
    match_cfg!{
        foo:ar => String::from("tar_foo"),  // ar : target architecture
        foo:tf => String::from("ttf_foo"),  // tf : target architecture feature
        foo:os => String::from("tos_foo"),  // os : target operating system
        foo:fm => String::from("tfm_foo"),  // fm : target family
        foo:ev => String::from("tev_foo"),  // ev : target environment
        foo:ed => String::from("ted_foo"),  // ed : target endian
        foo:pw => String::from("tpw_foo"),  // pw : target pointer width
        foo:vn => String::from("tvn_foo"),  // vn : target vendor    
        foo:at => String::from("tat_foo"),  // at : target has atomic
        foo:pn => String::from("tpn_foo"),  // pn : panic feature
        foo:ft => String::from("tft_foo"),  // ft : feature
        _ => String::from("021")
    }
}

/**************
 * meta_cfg *
 **************/
#[meta_cfg(foo:ar | foo:tf | foo:os | foo:fm | foo:ev | foo:ed | foo:pw | foo:vn | foo:at | foo:pn | foo:ft )]
fn cfg_dismissed() -> String {
    String::from("dismissed!")
}
#[meta_cfg(!(foo:ar | foo:tf | foo:os | foo:fm | foo:ev | foo:ed | foo:pw | foo:vn | foo:at | foo:pn | foo:ft ))]
fn cfg_foo() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", "Test", match_foo(), cfg_foo());
}
