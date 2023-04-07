// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 031 : Disabling documentation even with [package.metadata.docs.rs].
use cfg_boost::{ target_cfg, meta_cfg };



/**************
 * TARGET_CFG *
 **************/
target_cfg!{
	#[cfg(all(not(doc), unix))] => {
		/// This struct is for unix only
        pub struct UnixOnly {
            a : u64
        }
	},
    !doc & linux => { 
        /// This struct is for linux only
        pub struct LinuxOnly {
            a : u64
        }
    },     
    !doc & windows => { 
        /// This function is for windows only
        pub fn windows_only(){

        }
    },
    !doc & macos => { 
        /// This documentation is hidden
        #[doc(hidden)]
        pub fn hidden_doc(){

        }
    },
    !doc & android => { 
        /// Android only function.
        pub fn android_only(){

        }
    },

}


target_cfg!{
    !doc & (x86_64:ar & sse2:tf) => { 
        /// This function is x64 sse2 only
        pub fn x64sse2(){

        }
    },     
    !doc & wasm => { 
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
        !doc & linux => {
            /// Create new JohnDoe
            pub fn new() -> JohnDoe{
                JohnDoe { speech : String::from("linux") }
            }

            /// Make John Doe talk
            pub fn talk(&self) {
                println!("{}", self.speech);
            }

        },
        !doc & windows => {
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
#[meta_cfg(!doc & linux)]
/// This struct is for linux only
pub struct LinuxOnly2 {
    a : u64
}

#[meta_cfg(!doc & windows)]
/// This struct is for Windows only
pub struct WindowsOnly2 {
    a : u64
}

#[meta_cfg(!doc & !linux)]
/// This function is NOT for linux
pub fn NotLinux() {
}





fn main() {
    println!("Test 031 completed!");
}
