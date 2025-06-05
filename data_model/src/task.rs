use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Done
}

// Strings cannout use Copy because they are on the heap so they use Clone
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub status: TaskStatus
}