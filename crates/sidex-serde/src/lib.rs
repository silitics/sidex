#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod adapter;
pub mod de;
pub mod ser;

pub use adapter::AsBytes;
pub use adapter::AsF32;
pub use adapter::AsF64;
pub use adapter::AsI64;
pub use adapter::AsSelf;
pub use adapter::AsU64;
pub use adapter::DeserializeAs;
pub use adapter::DeserializeAsWrap;
pub use adapter::SerializeAs;
pub use adapter::SerializeAsWrap;
pub use adapter::SidexType;
