#![doc = include_str!("../README.md")]

pub use sidex_serde as serde;

pub mod interfaces;

/// Macro for including bundles build with [`sidex_build_rs`].
///
/// [`sidex_build_rs`]: https://docs.rs/sidex_build_rs/latest/sidex_build_rs/
///
///
/// ## Examples
///
/// To include a bundle named `bundle_name` as a module with the same name, use the
/// following macro invocation:
///
/// ```ignore
/// sidex::include_bundle!(pub bundle_name);
/// ```
///
/// Here, `pub` specifies that the generated module should be public. Without it, the
/// generated module would be private.
///
/// Using the `as` keyword, it is also possible to give the generated module a different
/// name than the bundle name. For instance, the following macro invocation includes the
/// bundle named `bundle_name` as a module named `mod_name`:
///
/// ```ignore
/// sidex::include_bundle!(pub bundle_name as mod_name);
/// ```
///
/// Additional attributes for the module can be given using the standard Rust syntax. For
/// instance:
///
/// ```ignore
/// sidex::include_bundle! {
///     #[cfg(feature = "include-bundle")]
///     bundle_name as mod_name
/// }
/// ```
#[macro_export]
macro_rules! include_bundle {
    ($(#[$meta:meta])* $vis:vis $bundle_name:ident) => {
        $crate::include_bundle!($(#[$meta])* $vis $bundle_name as $bundle_name);
    };
    ($(#[$meta:meta])* $vis:vis $bundle_name:ident as $mod_name:ident) => {
        $(#[$meta])*
        $vis mod $mod_name {
            include!(concat!(
                env!("OUT_DIR"),
                "/sidex-bundles/",
                stringify!($bundle_name),
                "/mod.rs"
            ));
        }
    };
}
