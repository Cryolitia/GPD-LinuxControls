use serde::{Deserialize, Serialize};

use serialize_display_adapter_macro_derive::SerializeDisplayAdapter;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct DeadZonesConfig {
    pub left: SpecificDeadZone,
    pub right: SpecificDeadZone,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct SpecificDeadZone {
    pub center: i8,
    pub border: i8,
}
