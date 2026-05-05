/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for API stability and lifecycle. Mirrors `lib/api/schemas/attrs.sidex`."]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]

    /// `#[deprecated]` on a top-level definition.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct DefDeprecated {
        #[serde(default)]
        pub since: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }

    /// `#[deprecated]` on a record field.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct FieldDeprecated {
        #[serde(default)]
        pub since: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }

    /// `#[deprecated]` on a variant case.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct VariantDeprecated {
        #[serde(default)]
        pub since: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }

    /// `#[deprecated]` at the top of a schema file.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct SchemaDeprecated {
        #[serde(default)]
        pub since: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }

    /// `#[unstable]` on a top-level definition.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct DefUnstable {
        #[serde(default)]
        pub feature: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub issue: ::std::option::Option<::std::string::String>,
    }

    /// `#[unstable]` on a record field.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct FieldUnstable {
        #[serde(default)]
        pub feature: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub issue: ::std::option::Option<::std::string::String>,
    }

    /// `#[unstable]` on a variant case.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct VariantUnstable {
        #[serde(default)]
        pub feature: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub issue: ::std::option::Option<::std::string::String>,
    }

    /// `#[unstable]` at the top of a schema file.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct SchemaUnstable {
        #[serde(default)]
        pub feature: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub note: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub issue: ::std::option::Option<::std::string::String>,
    }

    /// `#[since]` on a top-level definition.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct DefSince {
        pub version: ::std::string::String,
    }

    /// `#[since]` on a record field.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct FieldSince {
        pub version: ::std::string::String,
    }

    /// `#[since]` on a variant case.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct VariantSince {
        pub version: ::std::string::String,
    }

    /// `#[since]` at the top of a schema file.
    #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct SchemaSince {
        pub version: ::std::string::String,
    }
}
