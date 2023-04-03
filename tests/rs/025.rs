// Test 025 : Custom aliases missing.
use cfg_boost::{ target_cfg, match_cfg, meta_cfg };

/**************
 * TARGET_CFG *
 **************/
target_cfg!{
    pig => { pub fn tar_foo() {} },       // pig
    dog => { pub fn ttf_foo() {} },        // dog
    cow => { pub fn tos_foo() {} },     // cow
    parastratiosphecomyia_stratiosphecomyioides => { pub fn tfm_foo() {} },       // parastratiosphecomyia_stratiosphecomyioides
    mosquito => { pub fn tev_foo() {} },     // mosquito
    frog => { pub fn ted_foo() {} },         // frog
    lion => { pub fn tpw_foo() {} },        // lion
    fish => { pub fn tvn_foo() {} },     // fish
    b => { pub fn tat_foo() {} },      // b
}

/*************
 * MATCH_CFG *
 *************/
fn match_foo() -> String {
    match_cfg!{
        pig => String::from("tar_foo"),  // pig
        dog => String::from("ttf_foo"),  // dog
        cow => String::from("tos_foo"),  // cow
        parastratiosphecomyia_stratiosphecomyioides => String::from("tfm_foo"),  // parastratiosphecomyia_stratiosphecomyioides
        mosquito => String::from("tev_foo"),  // mosquito
        frog => String::from("ted_foo"),  // frog
        lion => String::from("tpw_foo"),  // lion
        fish => String::from("tvn_foo"),  // fish
        b => String::from("tat_foo"),  // b
        _ => String::from("025")
    }
}



/**************
 * meta_cfg *
 **************/
#[meta_cfg(pig | dog | cow | parastratiosphecomyia_stratiosphecomyioides | mosquito | frog | lion | fish | b)]
fn cfg_dismissed() -> String {
    String::from("dismissed!")
}


#[meta_cfg(!(pig | dog | cow | parastratiosphecomyia_stratiosphecomyioides | mosquito | frog | lion | fish | b))]
fn cfg_foo() -> String {
    String::from("Test 024 completed!")
}



fn main() {
    match_foo();
    println!("{} {} {}", twild_foo(), match_foo(), cfg_foo());
}
