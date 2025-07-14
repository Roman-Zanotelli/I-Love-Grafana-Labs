use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum TAction {
    CONFIRM, //POST Only
    DENY, //POST Only
    SEND(#[serde(deserialize_with = "greater_than_zero")] i32),
    RECV(#[serde(deserialize_with = "greater_than_zero")] i32),
}

pub fn greater_than_zero<'de, D>(deser: D) -> Result<i32, D::Error> where D: serde::Deserializer<'de> {
    let v = i32::deserialize(deser)?;
    if v <= 0 {
        Err(serde::de::Error::custom("amount must be > 0"))
    } else {
        Ok(v)
    }
}

impl TAction {
    pub fn as_str(&self) -> &str {
        match self {
            TAction::CONFIRM => "confirm",
            TAction::DENY => "deny",
            TAction::SEND(_) => "send",
            TAction::RECV(_) => "recv",
        }
    }
}