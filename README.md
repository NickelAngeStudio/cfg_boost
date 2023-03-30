
# cfg_boost

Revamped syntax to easily manage all #[cfg] parameters. Provides pattern matching like [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) thus the first matching arm is evaluated and all possibility are covered.

See [Features Wiki](https://github.com/NickelAngeStudio/cfg_boost/wiki/Features) to get the full list of features like aliases, attributes, automatic requirement tags documentation and more.

## Example
**Make this :**
```
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod macos_mod;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use macos_mod::Struct as Struct;

#[cfg(target_os = "windows")]
pub mod windows_mod;

#[cfg(target_os = "windows")]
pub use windows_mod::Struct1 as Struct1;

#[cfg(target_os = "windows")]
pub use windows_mod::Struct2 as Struct2;

#[cfg(target_os = "windows")]
pub fn windows_only_fn() {}
```

**Look like this :**
```
target_cfg!{
    macos | ios => {
        pub mod macos_mod;
        pub use macos_mod::Struct as Struct;
    },
    windows => {
        pub mod windows_mod;
        pub use windows_mod::Struct1 as Struct1;
        pub use windows_mod::Struct2 as Struct2;
        pub fn windows_only_fn() {}
    },
    _ => compile_error!{"Platform not supported"},
}
```

See [Examples wiki](https://github.com/NickelAngeStudio/cfg_boost/wiki/Examples) for more use cases.


## Installation
Execute this command in your Rust project folder.
```
cargo add cfg_boost
```

## Dependencies
cfg_boost has no dependencies and only use stable features.

## Question?
See [cfg_boost wiki](https://github.com/NickelAngeStudio/cfg_boost/wiki), it contains a **LOT** of information.

&nbsp;
---

*Sponsor me via [GitHub Sponsors](https://github.com/sponsors/NickelAngeStudio) and get your sponsor royalty tier.*
