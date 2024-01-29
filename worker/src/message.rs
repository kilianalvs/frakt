use serde::{de::value::Error, Serialize};
use shared::messages_types::{FragmentRequest, FragmentResult, FragmentTask};
//use shared::messages_types::{FragmentRequest, FragmentResult, FragmentTask};

// Définir un trait pour représenter les messages
pub trait Message : Serialize + Clone {
    fn serialize(&self) -> Result<String, serde_json::Error>;
}

impl Message for FragmentRequest {
    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl Message for FragmentResult {
    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl Message for FragmentTask {
    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}