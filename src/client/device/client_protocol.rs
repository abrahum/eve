use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone)]
pub enum ClientProtocol {
    IPad,         // 0
    AndroidPhone, // 1
    AndroidWatch, // 2
    MacOS,        // 3
    QiDian,       // 4
}

impl<'de> Deserialize<'de> for ClientProtocol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ProtocolVistor)
    }
}

impl Serialize for ClientProtocol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            &ClientProtocol::IPad => serializer.serialize_u64(0),
            &ClientProtocol::AndroidPhone => serializer.serialize_u64(1),
            &ClientProtocol::AndroidWatch => serializer.serialize_u64(2),
            &ClientProtocol::MacOS => serializer.serialize_u64(3),
            &ClientProtocol::QiDian => serializer.serialize_u64(4),
        }
    }
}

struct ProtocolVistor;

impl<'de> Visitor<'de> for ProtocolVistor {
    type Value = ClientProtocol;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expect 0 to 4 represent protocol type")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(ClientProtocol::IPad),
            1 => Ok(ClientProtocol::AndroidPhone),
            2 => Ok(ClientProtocol::AndroidWatch),
            3 => Ok(ClientProtocol::MacOS),
            4 => Ok(ClientProtocol::QiDian),
            _ => Err(E::custom("expect 0 to 4 represent protocol type")),
        }
    }
}
