#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub fn desktop_fn(){
    println!("Completed!")
}

#[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
pub fn mobile_fn(){
    println!("Completed!")
}

fn main() {
    desktop_fn();
}
