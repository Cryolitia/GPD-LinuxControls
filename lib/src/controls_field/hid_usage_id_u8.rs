use std::fmt;
use std::fmt::Formatter;
use crate::enums::hid_usage_id::HIDUsageID;

#[derive(Copy, Clone, Default)]
pub struct HIDUsageIDu8 {
    id: u8
}

impl fmt::Debug for HIDUsageIDu8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", HIDUsageID::try_from(self.id).map_or_else(|e| format!("{:#X}", self.id), |v| format!("{:?}", v)))
    }
}

impl From<u8> for HIDUsageIDu8 {
    fn from(value: u8) -> Self {
        HIDUsageIDu8 {
            id: value
        }
    }
}

impl From<HIDUsageIDu8> for u8 {
    fn from(value: HIDUsageIDu8) -> Self {
        value.id
    }
}

impl From<HIDUsageID> for HIDUsageIDu8 {
    fn from(value: HIDUsageID) -> Self {
        HIDUsageIDu8 {
            id: value.into()
        }
    }
}