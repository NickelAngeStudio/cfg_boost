
## Getting started in 2 steps
To get this type of requirement doc :
![Tokio requirement](https://i.stack.imgur.com/LWiN5.png)

### 1. Update Cargo.toml

In cargo.toml, add the following at the end of the file :
> [package.metadata.docs.rs]
> all-features = true
> rustdoc-args = ["--cfg", "docsrs"]

### 2. Update lib.rs and/or main.rs

In lib.rs and/or main.rs, first line of code should be :
> #![cfg_attr(docsrs, feature(doc_cfg))]

### Testing doc locally

To test documentation locally, use this command :
> RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features

### Reference(s)
https://stackoverflow.com/questions/61417452/how-to-get-a-feature-requirement-tag-in-the-documentation-generated-by-cargo-do
