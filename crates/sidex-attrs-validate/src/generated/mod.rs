/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for the validation extension. Regenerated from `lib/validate/schemas/attrs.sidex`."]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]

    /// One validation rule attached to a field.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct FieldRule {
        pub expr: TokensValue,
        #[serde(default)]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub code: ::std::option::Option<::std::string::String>,
    }

    /// One validation rule attached to a record.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct RecordRule {
        pub expr: TokensValue,
        #[serde(default)]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub code: ::std::option::Option<::std::string::String>,
    }

    /// One validation rule attached to a wrapper.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct WrapperRule {
        pub expr: TokensValue,
        #[serde(default)]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub code: ::std::option::Option<::std::string::String>,
    }

    /// One validation rule attached to an opaque type.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct OpaqueRule {
        pub expr: TokensValue,
        #[serde(default)]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub code: ::std::option::Option<::std::string::String>,
    }

    /// Captured `{ … }`-form attribute body — text + span.
    ///
    /// Mirrors the JSON shape produced by the typed-attrs parser for fields
    /// of type `core::attrs::Tokens` (see `sidex-core::attrs_parser`). Kept
    /// hand-written here to avoid bootstrapping the validate-attrs codegen
    /// through `core::attrs::Tokens` itself.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct TokensValue {
        pub text: ::std::string::String,
        #[serde(default)]
        pub span: ::std::option::Option<Span>,
    }

    /// Source span for a captured tokens body.
    #[derive(Clone, Copy, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Span {
        pub src: usize,
        pub start: usize,
        pub end: usize,
    }
}
