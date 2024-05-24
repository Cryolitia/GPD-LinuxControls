use serde::{Deserialize, Serialize};

use serialize_display_adapter_macro_derive::SerializeDisplayAdapter;

use crate::controls_field::hid_usage_id_u8::HIDUsageIDu8;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct KeyboardMouseConfig {
    pub directional_pad: DirectionalPadConfig,
    pub a: HIDUsageIDu8,
    pub b: HIDUsageIDu8,
    pub x: HIDUsageIDu8,
    pub y: HIDUsageIDu8,
    pub left_stick: LeftStickConfig,
    pub right_stick_push: HIDUsageIDu8,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct DirectionalPadConfig {
    pub up: HIDUsageIDu8,
    pub down: HIDUsageIDu8,
    pub left: HIDUsageIDu8,
    pub right: HIDUsageIDu8,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct LeftStickConfig {
    pub up: HIDUsageIDu8,
    pub down: HIDUsageIDu8,
    pub left: HIDUsageIDu8,
    pub right: HIDUsageIDu8,
    pub push: HIDUsageIDu8,
}
