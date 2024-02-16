use std::num::IntErrorKind;
use colored::Colorize;
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;
use gpd_linuxcontrols::controls_field::hid_usage_id_u8::HIDUsageIDu8;
use crate::cli::HIDUsageIDArgs;
use crate::helper::Radix::{Decimal, Hexadecimal};

lazy_static! {
    static ref DECIMAL: Regex = Regex::new(r"^\d+$").unwrap();
    static ref HEXADECIMAL: Regex = Regex::new(r"^(0x)?[abcdefABCDEF\d]+$").unwrap();
}

pub(crate) enum Radix {
    Decimal,
    Hexadecimal,
}

pub(crate) fn parse_hex(s: &str) -> Result<u8, String> {
    return if DECIMAL.is_match(s) {
        u8_from_str_radix(s, Decimal)
    } else if HEXADECIMAL.is_match(s) {
        u8_from_str_radix(s.trim_start_matches("0x"), Hexadecimal)
    } else {
        Err(format!("invalid digit found in string {}", s))
    };
}

fn u8_from_str_radix(s: &str, n: Radix) -> Result<u8, String> {
    return match u8::from_str_radix(s, match n {
        Radix::Decimal => 10,
        Radix::Hexadecimal => 16
    }) {
        Ok(n) => Ok(n),
        Err(e) => match e.kind() {
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => Err(format!("{} is not in {}..={}", s, match n {
                Decimal => "0",
                Hexadecimal => "0x00"
            }, match n {
                Decimal => "255",
                Hexadecimal => "0xFF"
            })),
            _ => Err(e.to_string())
        }
    };
}

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
    pub(crate) fn validate_dead_zones(&self, value: i8) -> Result<i8, String> {
        if (-10i8..=10i8).contains(&value) {
            return Ok(value);
        }
        if self.force {
            warn!("Dead zones value {} not in range -10..=10, continue with {}", value, "--force".italic());
            return Ok(value);
        }
        return Err(format!("Dead zones value {} not in range -10..=10, use {} to continue", value, "--force".italic()));
    }

    pub(crate) fn validate_delay(&self, value: u8) -> Result<u8, String> {
        if value == 0 || value == 100 {
            return Ok(value);
        }
        if self.force {
            warn!("Dead zones value {} not in range -10..=10, continue with {}", value, "--force".italic());
            return Ok(value);
        }
        return Err(format!("Dead zones value {} not in range -10..=10, use {} to continue", value, "--force".italic()));
    }
}
