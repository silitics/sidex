//! JSON aliases decode through the native codec and Serde without changing writes.

use sidex::formats::json;
use sidex_serde_tests::generated::aliases::Envelope;
use sidex_serde_tests::generated::aliases::Event;
use sidex_serde_tests::generated::aliases::Thread;

/// Old and canonical keys read identically, including optional and inline fields.
#[test]
fn aliases_roundtrip() {
    for key in ["threadId", "chatId", "conversationId"] {
        for title in ["title", "chatName", "oldTitle"] {
            let source = format!(r#"{{"{key}":"t1","{title}":"Title"}}"#);
            let native: Thread = json::from_str(&source).unwrap();
            let serde: Thread = serde_json::from_str(&source).unwrap();
            let expected = r#"{"threadId":"t1","title":"Title"}"#;
            assert_eq!(json::to_string(&native).unwrap(), expected);
            assert_eq!(serde_json::to_string(&serde).unwrap(), expected);
            let inline = format!(r#"{{"{key}":"t1","value":7}}"#);
            let envelope: Envelope<u32> = json::from_str(&inline).unwrap();
            assert_eq!(
                json::to_string(&envelope).unwrap(),
                r#"{"threadId":"t1","value":7}"#
            );
            let event = format!(r#"{{"tag":"Updated","{key}":"t1"}}"#);
            let event: Event = serde_json::from_str(&event).unwrap();
            assert_eq!(
                serde_json::to_string(&event).unwrap(),
                r#"{"tag":"Updated","threadId":"t1"}"#
            );
        }
    }
}

/// Multiple spellings of one field in an input object are rejected.
#[test]
fn duplicate_alias_input() {
    for source in [
        r#"{"threadId":"t1","chatId":"t2"}"#,
        r#"{"chatId":"t1","conversationId":"t2"}"#,
    ] {
        assert!(json::from_str::<Thread>(source).is_err());
        assert!(serde_json::from_str::<Thread>(source).is_err());
    }
}

/// Strict decoding recognizes old keys, including keys in inline payloads.
#[test]
fn strict_alias_input() {
    let value = json::from_str(r#"{"chatId":"t1","oldTitle":"Title","value":7}"#).unwrap();
    let decoded: Envelope<u32> = json::from_value_strict(&value).unwrap();
    assert_eq!(decoded.thread.thread_id, "t1");
}
