use gpd_linuxcontrols::colored::Colorize;
use gpd_linuxcontrols::controls_field::hid_usage_id_u8::HIDUsageIDu8;
use gpd_linuxcontrols::log::warn;

use crate::cli::HIDUsageIDArgs;

impl From<HIDUsageIDArgs> for HIDUsageIDu8 {
    fn from(value: HIDUsageIDArgs) -> Self {
        let mut result: HIDUsageIDu8 = Default::default();
        value.hex.inspect(|v| result = *v);
        value.name.inspect(|v| result = (*v).into());
        result
    }
}

pub(crate) struct RangeValidator {
    pub(crate) force: bool,
}

impl RangeValidator {
    pub(crate) fn validate_dead_zones(&self, value: i8, field: &str) -> Result<i8, String> {
        if (-10i8..=10i8).contains(&value) {
            return Ok(value);
        }
        if self.force {
            warn!("Dead zones value {} for field DeadZones::{} not in range -10..=10, continue with {}", value, field, "--force".italic());
            return Ok(value);
        }
        return Err(format!("Dead zones value {} for field DeadZones::{} not in range -10..=10, use {} to continue", value, field, "--force".italic()));
    }

    pub(crate) fn validate_delay(&self, value: u8, field: &str) -> Result<u8, String> {
        if value == 0 || value == 100 {
            return Ok(value);
        }
        if self.force {
            warn!("Dead zones value {} for field BackButton::{}Delay not 0(0x00) or 100(0x64), continue with {}", value, field, "--force".italic());
            return Ok(value);
        }
        return Err(format!("Dead zones value {} for field BackButton::{}Delay not 0(0x00) or 100(0x64), use {} to continue", value, field, "--force".italic()));
    }
}
