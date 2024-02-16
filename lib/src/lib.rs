use std::fmt::{Formatter, UpperHex};

pub mod enums;
pub mod controls_field;
pub mod protocol;

pub struct LoadArray<const N: usize> {
    value: [u8; N]
}

impl<const N: usize> From<[u8; N]> for LoadArray<N> {
    fn from(value: [u8; N]) -> Self {
        return LoadArray {
            value
        }
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