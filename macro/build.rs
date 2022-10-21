use rustc_version::{version_meta, Channel};

fn main() {
    if matches!(version_meta().unwrap().channel, Channel::Nightly) {
        enable_unstable_features();
    }
}

/// Enables the `use_unstable_features` configuration conditional check.
///
/// # Examples
///
/// ```ignore
/// #![cfg_attr(use_unstable_features, feature(/* ... */))]
///
/// #[cfg(use_unstable_features)]
/// mod foo { /* ... */ }
/// ```
fn enable_unstable_features() {
    println!("cargo:rustc-cfg=use_unstable_features");
}
