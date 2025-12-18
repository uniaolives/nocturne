use serde::Serialize;
use serde_json::{Value, ser::CompactFormatter};
use crate::serialization::error::SerializationError;

/// Recursively sorts JSON objects by key (lexicographical UTF-8)
fn sort_json(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = std::mem::take(map).into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            for (k, mut v) in entries {
                sort_json(&mut v);
                map.insert(k, v);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                sort_json(v);
            }
        }
        _ => {}
    }
}

pub fn to_canonical_json<T: Serialize>(value: &T) -> Result<String, SerializationError> {
    let mut json = serde_json::to_value(value)?;
    sort_json(&mut json);

    let mut out = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut out, CompactFormatter);
    json.serialize(&mut ser)?;

    Ok(String::from_utf8(out)?)
}
