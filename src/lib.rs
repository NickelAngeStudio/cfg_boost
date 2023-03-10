// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Blabla

/// test target macro
#[macro_export]
macro_rules! target_cfg {

    /*
    ( $($target:tt -> $elements:tt)+ ) => {



    };
    */

    () => {};

    /*
    ( [$target:tt] -> { $element:item }  ) => {

        
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux",target_os = "windows" ))))]
        $element

        

    };
    */

    ( $cfg:meta => $element:item ) => {
        #[cfg(any(doc, all( $cfg )))]
        $element
    };

    ( $target:tt -> { $($elements:item)+ }  ) => {

        //let $ret = $crate::target_cfg_header!{[$target]};
        //#[cfg(any(doc, all( stringify!($target) )))]
        //$element

        // Continue munching
        //target_cfg!{[$target] -> { $($elements)* }}

        $(
            target_cfg!{target_os="linux" => $elements }
        )+
        

    };

    ( $($target:tt -> { $($elements:item)+ })+ ) => {

        $(
            // Continue munching
            target_cfg!{$target -> { $($elements)+ }}
        )+

    };

    /*
    (trace $name:ident; $($tail:tt)*) => {
        {
            println!(concat!(stringify!($name), " = {:?}"), $name);
            mixed_rules!($($tail)*);
        }
    };
     */


    /*
    ( $($target:tt -> ( $($elements:item)+ ))+ ) => {
        
        //$(#[$attr])*
        //#[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = $target_os,$(target_os = $target_extra, )* ))))]
        $( $target
        $(
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux",target_os = "windows" ))))]
        $elements
        )+
        )+

        //target_os = "linux", target_os = "windows", target_os = "macos"
    };

    ( $($target:tt ->  $element:block )+ ) => {

        
        
        //$(#[$attr])*
        //#[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = $target_os,$(target_os = $target_extra, )* ))))]
        $( #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux",target_os = "windows" ))))]
            {
            $element
            }
        )+
        

        //target_os = "linux", target_os = "windows", target_os = "macos"
    };
*/
    

    

}

#[doc(hidden)]
#[macro_export]
macro_rules! target_cfg_header {

    // Entry point
    ( [$target:tt] ) => {
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux",target_os = "windows" ))))]

    }

}

target_cfg!{ 
    [!wasm & (linux | windows | macos)] -> {
        /// My comment
        /// ```
        /// example
        /// ```
        pub fn my_func(aaa : u32) -> u64 {

            println!("This is my body!");

            /*
            target_cfg!{
                [!wasm & linux] -> {
                    return 3;
                }
                [!wasm & windows] -> {
                    return 4;
                }
            }
            */

            2
        }
    }

    [!wasm & (linux | windows | macos)] -> {
        /// My comment3
        /// ```
        /// example
        /// ```
        pub fn my_func3(aaa : u32) -> u64 {

            println!("This is my body!");

            2

        }
    }
}

/*
target_cfg!{ 
    [linux] -> (
        pub mod test1 {

        }

        pub mod test2 {

        }
    )
}
*/

#[cfg(test)]
mod test {
    use crate::my_func;

    #[test]
    fn test() {
        println!("Je suis ");
        my_func(5);
    }
}