// Test 024 : Predefined aliases.
use cfg_boost::{ target_cfg, match_cfg, meta_cfg };

/**************
 * TARGET_CFG *
 **************/
target_cfg!{
    linux => { pub fn tar_foo() {} },       // linux
    unix => { pub fn ttf_foo() {} },        // unix
    windows => { pub fn tos_foo() {} },     // windows
    macos => { pub fn tfm_foo() {} },       // macos
    android => { pub fn tev_foo() {} },     // android
    ios => { pub fn ted_foo() {} },         // ios
    wasm => { pub fn tpw_foo() {} },        // wasm
    desktop => { pub fn tvn_foo() {} },     // desktop
    mobile => { pub fn tat_foo() {} },      // mobile
}

/*************
 * MATCH_CFG *
 *************/
// ar : target architecture
fn match_foo() -> String {
    match_cfg!{
        linux => String::from("tar_foo"),  // linux
        unix => String::from("ttf_foo"),  // unix
        windows => String::from("tos_foo"),  // windows
        macos => String::from("tfm_foo"),  // macos
        android => String::from("tev_foo"),  // android
        ios => String::from("ted_foo"),  // ios
        wasm => String::from("tpw_foo"),  // wasm
        desktop => String::from("tvn_foo"),  // desktop
        mobile => String::from("tat_foo"),  // mobile
        _ => String::from("024")
    }
}



/**************
 * meta_cfg *
 **************/
#[meta_cfg(linux | unix | windows | macos | android | ios | wasm | desktop | mobile)]
fn cfg_foo() -> String {
    String::from("Test 024 completed!")
}

#[meta_cfg(!(linux | unix | windows | macos | android | ios | wasm | desktop | mobile))]
fn cfg_dismissed() -> String {
    String::from("dismissed!")
}


fn main() {
    match_foo();
    println!("{}", cfg_foo());
}
