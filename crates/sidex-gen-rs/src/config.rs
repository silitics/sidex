use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub derive: Vec<String>,
}

// pub struct Config {
//     pub containers: Containers,
// }

// pub struct Containers {
//     pub sequence: String,
//     pub map: String,
// }

// #[derive(Clone)]
// pub struct RustPath(syn::Path);

// impl std::fmt::Display for RustPath {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let path = &self.0;
//         f.write_str(&quote!(#path).to_string())
//     }
// }

// impl std::fmt::Debug for RustPath {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("SynPath").field(&self.to_string()).finish()
//     }
// }

// impl serde::Serialize for RustPath {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(&self.to_string())
//     }
// }

// pub struct SynVisitor<T: syn::parse::Parse>(PhantomData<T>);

// impl<T: syn::parse::Parse> SynVisitor<T> {
//     pub fn new() -> Self {
//         Self(PhantomData)
//     }
// }

// impl<'de, T: syn::parse::Parse> serde::de::Visitor<'de> for SynVisitor<T> {
//     type Value = T;

//     fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         f.write_fmt(format_args!(
//             "Expected Rust code of type {:?}.",
//             std::any::type_name::<T>()
//         ))
//     }

//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         syn::parse_str(v).map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v),
// &self))     }
// }

// impl<'de> serde::Deserialize<'de> for RustPath {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         Ok(RustPath(deserializer.deserialize_str(SynVisitor::new())?))
//     }
// }
