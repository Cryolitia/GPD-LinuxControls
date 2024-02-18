use serde::{Deserialize, Serialize};
use serialize_display_adapter_macro_derive::SerializeDisplayAdapter;
use crate::controls_field::hid_usage_id_u8::HIDUsageIDu8;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct BackButtonConfig {
    pub left: SpecificBackButtonConfig,
    pub right: SpecificBackButtonConfig
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct SpecificBackButtonConfig {
    pub first: HIDUsageIDu8,
    pub second: HIDUsageIDu8,
    pub third: HIDUsageIDu8,
    pub forth: HIDUsageIDu8,
    pub first_delay: u8,
    pub second_delay: u8,
    pub third_delay: u8,
}