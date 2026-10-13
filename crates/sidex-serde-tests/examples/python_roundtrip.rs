//! Decode Python integration-test output with generated Rust types and write it back.

use std::error::Error;
use std::io::Read;

use sidex::Decode;
use sidex::Encode;
use sidex::formats::json;
use sidex::types::JsonValue;
use sidex_serde_tests::generated::inline;
use sidex_serde_tests::generated::serialization;

fn main() -> Result<(), Box<dyn Error>> {
    let typ = std::env::args().nth(1).ok_or("missing fixture type")?;
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text)?;
    let encoded = match typ.as_str() {
        "Encodings" => roundtrip::<serialization::Encodings>(&text)?,
        "OptionalFields" => roundtrip::<serialization::OptionalFields>(&text)?,
        "Box" => roundtrip::<serialization::Box<JsonValue>>(&text)?,
        "Internal" => roundtrip::<serialization::Internal>(&text)?,
        "Adjacent" => roundtrip::<serialization::Adjacent>(&text)?,
        "External" => roundtrip::<serialization::External>(&text)?,
        "Implicit" => roundtrip::<serialization::Implicit>(&text)?,
        "InlineEnvelope" => roundtrip::<inline::Envelope>(&text)?,
        "InlineNested" => roundtrip::<inline::Nested>(&text)?,
        "InlineEmptyEnvelope" => roundtrip::<inline::EmptyEnvelope>(&text)?,
        "InlineWrapped" => roundtrip::<inline::WrappedEnvelope>(&text)?,
        "InlineGeneric" => roundtrip::<inline::InlineRecord<inline::Payload>>(&text)?,
        "InlineGenericWrapped" => roundtrip::<inline::InlineRecord<inline::PayloadAlias>>(&text)?,
        "InlineGenericCase" => roundtrip::<inline::GenericCase<inline::Payload>>(&text)?,
        "InlineCrossSchema" => roundtrip::<inline::CrossSchema>(&text)?,
        "InlineRenamed" => roundtrip::<inline::RenamedEnvelope>(&text)?,
        "InlineInternal" => roundtrip::<inline::Internal>(&text)?,
        "InlineAdjacent" => roundtrip::<inline::Adjacent>(&text)?,
        "InlineExternal" => roundtrip::<inline::External>(&text)?,
        "InlineImplicit" => roundtrip::<inline::Implicit>(&text)?,
        _ => return Err(format!("unknown fixture type `{typ}`").into()),
    };
    println!("{encoded}");
    Ok(())
}

/// Round-trips one document through Sidex's typed JSON reader and writer.
fn roundtrip<T: Decode + Encode>(text: &str) -> Result<String, Box<dyn Error>> {
    let value = json::from_str::<T>(text)?;
    Ok(json::to_string(&value)?)
}
