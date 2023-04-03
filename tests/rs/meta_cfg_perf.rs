#![cfg_attr(docsrs, feature(doc_cfg))]

use cfg_boost::meta_cfg;

#[meta_cfg(desktop)]
pub fn desktop_fn(){
    println!("Completed!")
}

#[meta_cfg(mobile)]
pub fn mobile_fn(){
    println!("Completed!")
}

fn main() {
    desktop_fn();
}
