use clap::ValueEnum;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use serialize_display_adapter_macro_derive::SerializeDisplayAdapter;

use crate::enums::hid_usage_id::HIDUsageID;
use crate::parse_hex;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, SerializeDisplayAdapter)]
pub struct HIDUsageIDu8 {
    id: u8,
}

impl From<u8> for HIDUsageIDu8 {
    fn from(value: u8) -> Self {
        HIDUsageIDu8 { id: value }
    }
}

impl From<HIDUsageIDu8> for u8 {
    fn from(value: HIDUsageIDu8) -> Self {
        value.id
    }
}

impl From<HIDUsageID> for HIDUsageIDu8 {
    fn from(value: HIDUsageID) -> Self {
        HIDUsageIDu8 { id: value.into() }
    }
}

impl Serialize for HIDUsageIDu8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match HIDUsageID::try_from(self.id) {
            Ok(v) => v.serialize(serializer),
            Err(_) => serializer.serialize_str(format!("{:#X}", self.id).as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for HIDUsageIDu8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        HIDUsageID::from_str(&s, true).map_or_else(
            |_| -> Result<Self, D::Error> { parse_hex(&s).map_err(|e| D::Error::custom(e)) },
            |v| -> Result<Self, _> { Ok(v.into()) },
        )
    }
}
