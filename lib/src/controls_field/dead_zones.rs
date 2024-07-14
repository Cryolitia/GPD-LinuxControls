use serde::{Deserialize, Serialize};

use serialize_display_adapter_macro_derive::PrettyJsonSerializeDisplayAdapter;

#[derive(
    Debug, Copy, Clone, Default, Serialize, Deserialize, PrettyJsonSerializeDisplayAdapter,
)]
#[serde(default)]
pub struct DeadZonesConfig {
    pub left: SpecificDeadZone,
    pub right: SpecificDeadZone,
}

#[derive(
    Debug, Copy, Clone, Default, Serialize, Deserialize, PrettyJsonSerializeDisplayAdapter,
)]
#[serde(default)]
pub struct SpecificDeadZone {
    pub center: i8,
    pub border: i8,
}
