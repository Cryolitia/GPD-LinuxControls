use std::fmt::{Formatter, UpperHex};
use std::num::IntErrorKind;

pub use colored;
pub use lazy_static::lazy_static;
pub use log;
pub use regex;
use regex::Regex;
pub use strum;

use crate::controls_field::hid_usage_id_u8::HIDUsageIDu8;
use crate::Radix::{Decimal, Hexadecimal};

pub mod enums;
pub mod controls_field;
pub mod protocol;

pub struct LoadArray<const N: usize> {
    value: [u8; N],
}

impl<const N: usize> From<[u8; N]> for LoadArray<N> {
    fn from(value: [u8; N]) -> Self {
        return LoadArray {
            value
        };
    }
}

impl<const N: usize> UpperHex for LoadArray<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a: String = self.value.iter().map(|i| {
            format!("{:02X}", i).to_string() + " "
        }).collect();
        return write!(f, "{}", a);
    }
}

lazy_static! {
    static ref DECIMAL: Regex = Regex::new(r"^\d+$").unwrap();
    static ref HEXADECIMAL: Regex = Regex::new(r"^(0x)?[abcdefABCDEF\d]+$").unwrap();
}

enum Radix {
    Decimal,
    Hexadecimal,
}

pub fn parse_hex(s: &str) -> Result<HIDUsageIDu8, String> {
    return if DECIMAL.is_match(s) {
        u8_from_str_radix(s, Decimal)
    } else if HEXADECIMAL.is_match(s) {
        u8_from_str_radix(s.trim_start_matches("0x"), Hexadecimal)
    } else {
        Err(format!("invalid digit found in string {}", s))
    };
}

fn u8_from_str_radix(s: &str, n: Radix) -> Result<HIDUsageIDu8, String> {
    return match u8::from_str_radix(s, match n {
        Radix::Decimal => 10,
        Radix::Hexadecimal => 16
    }) {
        Ok(n) => Ok(n.into()),
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