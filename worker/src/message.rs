use std::any::Any;

use serde::{de::value::Error, Serialize};
use shared::messages_types::{FragmentRequest, FragmentResult, FragmentTask};
//use shared::messages_types::{FragmentRequest, FragmentResult, FragmentTask};

// Définir un trait pour représenter les messages
// #[vtable]
// #[repr(C)]
// #[derive(Debug, Serialize, PartialEq, Deserialize, Clone, Copy)]
//définir enum plutot que trait
pub trait Message {
    fn serialize(&self) -> Result<String, serde_json::Error>;
}

// Implémenter le trait Any pour permettre le downcast dynamique
impl dyn Message {
    fn as_any(&self) -> &dyn Message {
        self
    }
}

// Implémenter le trait pour FragmentRequest
impl Message for FragmentRequest {
    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// Implémenter le trait pour FragmentResult
impl Message for FragmentResult {
    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// Implémenter le trait pour FragmentTask
impl Message for FragmentTask {
    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}