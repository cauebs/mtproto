use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Clone)]
pub struct Payload {
    pub data: Vec<u8>,
}

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut seq = serializer.serialize_struct("hu3", 42)?;
        for byte in &self.data {
            seq.serialize_field("hu3", byte)?;
        }
        seq.end()
    }
}

struct PayloadVisitor;

impl<'de> serde::de::Visitor<'de> for PayloadVisitor {
    type Value = Payload;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a stream of bytes")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Payload { data: v.into() })
    }
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PayloadVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnencryptedHeader {
    pub auth_key_id: u64,
    pub message_id: u64,
    pub message_data_length: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnencryptedMessage {
    pub header: UnencryptedHeader,
    pub message_data: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionHeader {
    pub auth_key_id: u64,
    pub msg_key: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub header: EncryptionHeader,
    pub encrypted_data: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadHeader {
    pub salt: u64,
    pub session_id: u64,
    pub message_id: u64,
    pub seq_no: u32,
    pub message_data_length: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedData {
    pub header: PayloadHeader,
    pub message_data: Payload,
}
