use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;
use std::collections::HashMap;

pub fn serialize_cord_hash_map<S, T, K>(
    value: &HashMap<(K, K), T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
    K: ToString,
{
    let mut map = serializer.serialize_map(Some(value.len()))?;

    for (key, value) in value {
        let key = format!("{}x{}", key.0.to_string(), key.1.to_string());

        map.serialize_entry(&key, &value)?;
    }

    map.end()
}
