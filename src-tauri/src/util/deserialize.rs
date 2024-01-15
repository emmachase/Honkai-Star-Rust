use serde::de;

pub fn deserialize_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: f64 = de::Deserialize::deserialize(deserializer)?;
    Ok(s as u8)
}
