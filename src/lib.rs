#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
//! <div style="float:right;width:200px;height:80px;"><iframe src="https://github.com/sponsors/NickelAngeStudio/button" title="Sponsor NickelAngeStudio" height="32" width="200" style=" border: 0; border-radius: 6px;"></iframe><a href="https://github.com/NickelAngeStudio/cfg_boost/wiki"><button style="width:200px;height:32px;background-color: #1f883d;border: none;color: white;padding: 0px;text-align: center;border-radius: 6px;text-decoration: none;display: inline-block;font-size: 16px;margin: 0px;">Wiki</button></a></div>
//! 
//! cfg_boost provides a [revamped syntax and macros](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax) 
//! to easily manage all `#[cfg]` [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
//! predicates and parameters in one package.
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
//! <br>
//! 
//! [Get more examples on the wiki.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Examples)
use ts::{ generate_target_content, generate_meta_content, generate_match_content};
use proc_macro::{TokenStream};

/// Errors enumeration
mod errors;

/// config.toml fetch functions
mod config;

/// Arms structure and functions
mod arm;

/// Tokenstream generator functions
mod ts;

/// Syntax tree
mod syntax;


/// Procedural macro used to declare resource and item outside function.
/// 
/// ## Description
/// target_cfg! use a pattern syntax like [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) 
/// to define conditional compilation. One-to-many arms can be defined and contrary to [match_cfg!], **any matching arm WILL be included.**
/// Because this behaviour is not [hygienic](https://doc.rust-lang.org/reference/macros-by-example.html#hygiene), 
/// target_cfg! **CANNOT** be used in function (use [match_cfg!] inside function).
/// 
/// ## Syntax
/// ```ignore
/// target_cfg!{
///     !? alias* (| &)? !? value:pred* => {},
///     !? alias* (| &)? !? value:pred* => {},
/// }
/// ```
/// [More details on syntax here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Syntax)
/// 
/// ## Documentation
/// target_cfg! always wrap arm with `doc | (arm)` if `doc` is not defined in the arm. This allow `cargo doc` to always generate documentation of each arm. 
/// This feature can be deactivated. [More details here](https://github.com/NickelAngeStudio/cfg_boost/wiki/Documentation)
/// 
/// **BONUS :** target_cfg! can also generate those dependency tags. 
/// <img src="https://github.com/NickelAngeStudio/cfg_boost/raw/main/img/tag.png?raw=true" width="600" height="190"><br>
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
/// compile_error!("Not supported");
/// ```
/// **Becomes this**
/// ```ignore
/// target_cfg!{
///     !windows => {
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
///     mobile => compile_error!("Not supported"),
/// }
/// ```
/// [More examples here.](https://github.com/NickelAngeStudio/cfg_boost/wiki/Examples)
#[proc_macro]
pub fn target_cfg(item: TokenStream) -> TokenStream {

    // Generate content from target_cfg! macro source.
    generate_target_content(item)

}


/// Procedural macro used exclusively inside a function.
#[proc_macro]
pub fn match_cfg(item: TokenStream) -> TokenStream {

    // Generate content for match_cfg! macro.
    generate_match_content(item)

}


/// Attribute macro used for one item.
#[proc_macro_attribute]
pub fn meta_cfg(attr: TokenStream, item: TokenStream) -> TokenStream {

    // Generate attribute content.
    generate_meta_content(attr, item)

}