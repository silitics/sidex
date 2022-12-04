#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

// use serde::Serialize;

pub mod de;
pub mod ser;

// struct F32(f32);

// impl Serialize for F32 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         if serializer.is_human_readable() {
//             if self.0.is_nan() {
//                 serializer.serialize_str("NaN")
//             } else if self.0.is_infinite() {
//                 if self.0.is_sign_negative() {
//                     serializer.serialize_str("-Infinity")
//                 } else {
//                     serializer.serialize_str("+Infinity")
//                 }
//             } else {
//                 serializer.serialize_f32(self.0)
//             }
//         } else {
//             serializer.serialize_f32(self.0)
//         }
//     }
// }
