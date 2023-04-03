// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Performance control parameters.

#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub fn desktop_fn(){
    println!("perf_target_ctrl completed!")
}

#[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
pub fn mobile_fn(){
    println!("perf_target_ctrl completed!")
}

fn main() {
    desktop_fn();
}
