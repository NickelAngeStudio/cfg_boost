#![cfg_attr(docsrs, feature(doc_cfg))]

pub fn inner_cfg() {

    let a = {
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
        {
            10
        }
        #[cfg(all(not(any(target_os = "linux", target_os = "windows", target_os = "macos")), any(target_os = "ios", target_os = "android")))]
        {
            20
        }
        #[cfg(all(not(any(target_os = "linux", target_os = "windows", target_os = "macos")), not(any(target_os = "ios", target_os = "android"))))]
        {
            30
        }
    };

    println!("{}", a);

}

fn main() {
    inner_cfg();
}
