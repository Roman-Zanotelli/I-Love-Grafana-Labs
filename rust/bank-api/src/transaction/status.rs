use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
pub enum TStatus {
    CONFIRMED,
    PENDING,
    DENIED
}