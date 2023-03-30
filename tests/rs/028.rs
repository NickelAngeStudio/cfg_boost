// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 028 : Documentation with [package.metadata.docs.rs].
use cfg_boost::{ target_cfg, cfg_target };



/**************
 * TARGET_CFG *
 **************/
target_cfg!{
    linux => { 
        /// This struct is for linux only
        pub struct LinuxOnly {
            a : u64
        }
    },     
    windows => { 
        /// This function is for windows only
        pub fn windows_only(){

        }
    },
    macos => { 
        /// This documentation is hidden
        #[doc(hidden)]
        pub fn hidden_doc(){

        }
    },
    android => { 
        /// Android only function.
        pub fn android_only(){

        }
    },


    _ => { 
        /// This enum is only Wildcard
        pub enum Wildcard{
            Wild1,
            Wild2
        }
    },
}


target_cfg!{
    x86_64:ar & sse2:tf => { 
        /// This function is x64 sse2 only
        pub fn x64sse2(){

        }
    },     
    wasm => { 
        /// This function is for web assembly
        pub fn wasm_only(){

        }
    },
    _ => { },
}


/// Test struct with multiple implementation of the same name.
pub struct JohnDoe {
    speech : String,
}

impl JohnDoe {
    target_cfg!{
        linux => {
            /// Create new JohnDoe
            pub fn new() -> JohnDoe{
                JohnDoe { speech : String::from("linux") }
            }

            /// Make John Doe talk
            pub fn talk(&self) {
                println!("{}", self.speech);
            }

        },
        windows => {
            /// Create new JohnDoe
            pub fn new() -> JohnDoe{
                JohnDoe { speech : String::from("windows") }
            }

            /// Make John Doe talk
            pub fn talk(&self) {
                println!("{}", self.speech);
            }

        },
        _ {
            /// Create new JohnDoe
            pub fn new() -> JohnDoe{
                JohnDoe { speech : String::from("unknown") }
            }

            /// Make John Doe talk
            pub fn talk(&self) {
                println!("{}", self.speech);
            }
        }

    }
}

/**************
 * CFG_TARGET *
 **************/
#[cfg_target(linux)]
/// This struct is for linux only
pub struct LinuxOnly2 {
    a : u64
}

#[cfg_target(windows)]
/// This struct is for Windows only
pub struct WindowsOnly2 {
    a : u64
}

#[cfg_target(!linux)]
/// This function is NOT for linux
pub fn NotLinux() {
}





fn main() {
    println!("Test 028 completed!");
}
