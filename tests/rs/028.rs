// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 028 : Documentation with [package.metadata.docs.rs].
use cfg_boost::{ target_cfg, meta_cfg };



/**************
 * TARGET_CFG *
 **************/
target_cfg!{
	#[cfg(unix)] => {
		/// This struct is for unix only
        pub struct UnixOnly {
            a : u64
        }
	},
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

    }
}

/**************
 * meta_cfg *
 **************/
#[meta_cfg(linux)]
/// This struct is for linux only
pub struct LinuxOnly2 {
    a : u64
}

#[meta_cfg(windows)]
/// This struct is for Windows only
pub struct WindowsOnly2 {
    a : u64
}

#[meta_cfg(!linux)]
/// This function is NOT for linux
pub fn NotLinux() {
}





fn main() {
    println!("Test 028 completed!");
}
