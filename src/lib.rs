#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
//! <div style="float:right;width:200px;height:80px;"><iframe src="https://github.com/sponsors/NickelAngeStudio/button" title="Sponsor NickelAngeStudio" height="32" width="200" style=" border: 0; border-radius: 6px;"></iframe><a href="https://github.com/NickelAngeStudio/cfg_boost/wiki"><button style="width:200px;height:32px;background-color: #1f883d;border: none;color: white;padding: 0px;text-align: center;border-radius: 6px;text-decoration: none;display: inline-block;font-size: 16px;margin: 0px;">Wiki</button></a></div>
//! 
//! cfg_boost provides a [revamped syntax and macros](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax) 
//! to easily manage all `#[cfg]` [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
//! predicates and parameters in one package.
//! 
//! See [features](https://github.com/NickelAngeStudio/cfg_boost/wiki/Features) to get the full list of features like aliases, attributes, automatic dependency tag documentation and more.
//!
//! ## Example
//! **Transform this :**
//! ```ignore
//! #[cfg(any(doc, any(target_os = "linux", target_os = "macos", target_os = "windows")))]
//! #[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
//! pub mod desktop_mod;
//! 
//! #[cfg(any(doc, any(target_os = "linux", target_os = "macos", target_os = "windows")))]
//! #[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
//! pub use desktop_mod::Struct as Struct;
//! 
//! #[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
//! #[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
//! pub mod mobile_mod;
//! 
//! #[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
//! #[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
//! pub use mobile_mod::Struct1 as Struct1;
//! 
//! #[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
//! #[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
//! pub use mobile_mod::Struct2 as Struct2;
//! 
//! #[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
//! #[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
//! pub fn mobile_only_fn() {}
//! ```
//! 
//! **Into this :**
//! ```ignore
//! target_cfg!{
//!     desktop => {
//!         pub mod desktop_mod;
//!         pub use desktop_mod::Struct as Struct;
//!     },
//!     mobile => {
//!         pub mod mobile_mod;
//!         pub use mobile_mod::Struct1 as Struct1;
//!         pub use mobile_mod::Struct2 as Struct2;
//!         pub fn mobile_only_fn() {}
//!     }
//! }
//! ```
//! 
//! [Get more examples on the wiki.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Examples)
use arm::TargetArm;
use proc_macro::{TokenStream, TokenTree, Group, Delimiter};

/// Errors enumeration
mod errors;

/// config.toml fetch functions
mod config;

/// Arms structure and functions
mod arm;

/// Syntax tree
mod syntax;

/// Proc macro source enumeration to determinate matching macro source.
#[derive(Clone, Copy)]
pub(crate) enum CfgBoostMacroSource {
    /// Call come from target_cfg! macro.
    TargetMacro,

    /// Call come from match_cfg! macro.
    MatchMacro,
}

/// Procedural macro used to declare resource and item outside function.
/// 
/// ## Description
/// target_cfg! use a pattern syntax like [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) 
/// to define conditional compilation outside a function. One-to-many arms can be defined and contrary to [match_cfg!], **any matching arm WILL be included**
/// and not all cases are covered with a [wildcard](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern).
/// 
/// Because this behaviour is different from [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html), 
/// target_cfg! **WILL PANIC** if used in function (use [match_cfg!] inside function instead).
/// 
/// **target_cfg! has no runtime cost.**
/// 
/// ## Syntax
/// ```ignore
/// target_cfg!{
///     !? alias* (| &)? !? value:pred* => {},+
///     #[cfg(legacy_syntax)] => {},+    // target_cfg! also support legacy syntax
/// }
/// ```
/// [More details on syntax here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax)
/// 
/// ## Documentation
/// target_cfg! always wrap arm with `doc | (arm)` if `doc` is not defined in the arm (even for legacy syntax). This allow `cargo doc` to always generate documentation of each arm. 
/// This feature can be deactivated. [More details here](https://github.com/NickelAngeStudio/cfg_boost/wiki/Documentation)
/// 
/// **BONUS :** target_cfg! can also generate those dependency tags. 
/// <img src="https://github.com/NickelAngeStudio/cfg_boost/raw/main/img/tag.png?raw=true" width="600" height="160"><br>
/// [More details here](https://github.com/NickelAngeStudio/cfg_boost/wiki/Documentation)
/// 
/// ## Example
/// **This**
/// ```ignore
/// /// This function is not for windows
/// #[cfg(any(doc, not(windows)))]
/// pub fn not_for_windows() {
/// }
/// 
/// /// This function is not for windows again
/// #[cfg(any(doc, not(windows)))]
/// pub fn not_for_windows_again() {
/// }
/// 
/// #[cfg(any(doc, all(target_arch="x86", target_feature="sse4.1")))]
/// pub fn thirty_two_bits() {
/// }
/// 
/// #[cfg(all(not(doc), any(feature="myfeature1", feature="myfeature2")))]
/// pub struct undocumented_featured {
/// }
/// 
/// #[cfg(any(target_os="ios", target_os="android"))]
/// compile_error!("Mobile not supported");
/// ```
/// **becomes**
/// ```ignore
/// target_cfg!{
///     #[cfg(not(windows))] => {     // Legacy syntax example.
///         /// This function is not for windows
///         pub fn not_for_windows() {
///         }
/// 
///         /// This function is not for windows again
///         pub fn not_for_windows_again() {
///         }
///     },
///     x86:ar & sse4.1:tf => {
///         pub fn thirty_two_bits() {
///         }
///     },
///     !doc & (myfeature1:ft | myfeature2:ft) => {
///         pub struct undocumented_featured {
///         }
///     },
///     mobile => compile_error!("Mobile not supported"),
/// }
/// ```
/// [More examples here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Examples)
#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Extract target arms
    let arms = TargetArm::extract(item.clone(), CfgBoostMacroSource::TargetMacro);

    // 2. For each arm
    for arm in arms {

        // 2.1. Split item into vector of items
        let items = syntax::split_items(arm.content.clone());

        // 2.2. For each item in vector of items
        for item in items {
            // 2.2.1. Add cfg header.
            content.extend(arm.cfg_ts.clone()); 

            // 2.2.2. Add cfg_attr
            content.extend(arm.attr_ts.clone());

            // 2.2.3. Add item to content
            content.extend(item);
        }
    }

    // 3. Return content.
    content

}


/// Procedural macro used exclusively inside a function.
/// 
/// ## Description
/// match_cfg! use a pattern syntax like [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) 
/// to define conditional compilation in a function.  The first matching arm is evaluated and all possible values must be covered with a [wildcard](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern).
/// 
/// This behaviour is the same as [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html), 
/// thus match_cfg! can be used inside a function (while [target_cfg!] will [panic!]).
/// 
/// **match_cfg! has no runtime cost.**
/// 
/// ## Syntax
/// ```ignore
/// match_cfg!{
///     !? alias* (| &)? !? value:pred* => {},+
///     #[cfg(legacy_syntax)] => {},+    // match_cfg! also support legacy syntax
///     _ => {}+?     // Mandatory wildcard arm
/// };?
/// ```
/// [More details on syntax here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax)
/// 
/// ## Example
/// **This**
/// ```ignore
/// pub fn foo(){
///     let a = {
///         #[cfg(linux)]
///         {
///             10
///         }
///         #[cfg(windows)]
///         {
///             20
///         }
///         #[cfg(all(not(linux), not(windows)))]   // This would be a wildcard arm.
///         {
///             30
///         }
///     };
/// 
///     #[cfg(linux)]
///     {
///         println!("linux={}", a);
///     }
///     #[cfg(windows)]
///     {
///         println!("windows={}", a);
///     }
///     #[cfg(all(not(linux), not(windows)))]   // This would be a wildcard arm.
///     {
///         println!("not linux and not windows={}", a);
///     }
/// }
/// ```
/// **becomes**
/// ```ignore
/// pub fn foo(){
///     let a = match_cfg!{
///         linux => 10,
///         windows => 20,
///         _ => 30     // Last `,` is optional like match
///     };
/// 
///     match_cfg!{
///         linux => println!("linux={}", a),
///         #[cfg(windows)] => println!("windows={}", a),   // Legacy syntax example
///         _ => println!("not linux and not windows={}", a),
///     };
/// }
/// ```
/// [More examples here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Examples)
#[proc_macro]
pub fn match_cfg(item: TokenStream) -> TokenStream {

     // TokenStream that accumulate content
     let mut content = TokenStream::new();

     // 1. Extract target arms
     let arms = TargetArm::extract(item.clone(), CfgBoostMacroSource::MatchMacro);
 
     // 2. For each arm
     for arm in arms {
         // 2.1. Add cfg header.
         content.extend(arm.cfg_ts.clone()); 
 
         // 2.2. Add braced content
         content.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace, arm.content.clone()))));
     }
 
     // 3. Add braces around content then return it.
     TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace,content)))

}


/// Attribute macro like [cfg](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) with [simplified syntax](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax) used for one item.
/// 
/// ## Description
/// meta_cfg work exactly like [cfg](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) but with [simplified syntax](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax).
/// 
/// **meta_cfg has no runtime cost.**
/// 
/// ## Syntax
/// ```ignore
/// #[meta_cfg(!? alias* (| &)? !? value:pred*)]
/// item
/// 
/// #[meta_cfg(#[cfg(legacy_syntax)])]  // meta_cfg also support legacy syntax.
/// item
/// ```
/// [More details on syntax here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax)
/// 
/// ## Documentation
/// meta_cfg always wrap predicate with `doc | (predicates)` if `doc` is not defined. This allow `cargo doc` to always generate documentation. 
/// This feature can be deactivated. [More details here](https://github.com/NickelAngeStudio/cfg_boost/wiki/Documentation)
/// 
/// **BONUS :** meta_cfg can also generate those dependency tags. 
/// <img src="https://github.com/NickelAngeStudio/cfg_boost/raw/main/img/tag.png?raw=true" width="600" height="160"><br>
/// [More details here](https://github.com/NickelAngeStudio/cfg_boost/wiki/Documentation)
/// 
/// ## Example
/// **This**
/// ```ignore
/// #[cfg(any(doc, any(windows, unix, target_os="macos")))]
/// pub fn foo() {}
/// ```
/// **becomes**
/// ```ignore
/// #[meta_cfg(windows | unix | macos)]
/// pub fn foo() {}
/// ```
/// [More examples here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Examples)
#[proc_macro_attribute]
pub fn meta_cfg(attr: TokenStream, item: TokenStream) -> TokenStream {

    // 1. Generate target_cfg! syntax
    let mut stream = attr;
    stream.extend(" => ".parse::<TokenStream>().unwrap());  // Add separator
    stream.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace,item))));   // Add braced content

    // 2. Generate tokenstream with target_cfg! macro
    target_cfg(stream)

}