use std::fmt::{Debug, format, Formatter};
use serde::{Deserialize, Serialize, Serializer};
use serialize_display_adapter_macro_derive::SerializeDisplayAdapter;
use crate::controls_field::back_button::BackButtonConfig;
use crate::controls_field::dead_zones::DeadZonesConfig;
use crate::controls_field::keyboard_mouse::KeyboardMouseConfig;
use crate::enums::Vibrate;

pub mod keyboard_mouse;
pub mod back_button;
pub mod dead_zones;
pub mod hid_usage_id_u8;

#[derive(Copy, Clone, Debug, Serialize, SerializeDisplayAdapter)]
pub struct ControlsField {
    pub firmware_version: FirmwareVersion,
    pub keyboard_mouse: KeyboardMouseConfig,
    pub back_button: BackButtonConfig,
    pub vibrate: Vibrate,
    pub dead_zones: DeadZonesConfig,
    pub checksum: Checksum,
}

#[derive(Copy, Clone, Debug, SerializeDisplayAdapter)]
pub struct Checksum {
    value: u64
}

impl From<u64> for Checksum {
    fn from(value: u64) -> Self {
        return Checksum {
            value
        }
    }
}

impl Serialize for Checksum {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(format!("{:#X}", self.value).as_str())
    }
}

#[derive(Copy, Clone, Debug, Serialize, SerializeDisplayAdapter)]
pub struct FirmwareVersion{
    pub gamepad_firmware: Version,
    pub keyboard_mouse_firmware: Version,
}

#[derive(Copy, Clone, Debug, SerializeDisplayAdapter)]
pub struct Version {
    pub major_version: u8,
    pub minor_version: u8,
}

impl Serialize for Version {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(format!("{}.{}", self.major_version, self.minor_version).as_str())
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, SerializeDisplayAdapter)]
#[serde(default)]
pub struct ControlsConfig {
    pub keyboard_mouse: KeyboardMouseConfig,
    pub back_button: BackButtonConfig,
    pub vibrate: Vibrate,
    pub dead_zones: DeadZonesConfig,
}

impl From<ControlsField> for ControlsConfig {
    fn from(value: ControlsField) -> Self {
        return ControlsConfig {
            keyboard_mouse: value.keyboard_mouse,
            back_button: value.back_button,
            vibrate: value.vibrate,
            dead_zones: value.dead_zones,
        }
    }
}