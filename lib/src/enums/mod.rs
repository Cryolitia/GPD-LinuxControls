use clap::ValueEnum;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use serialize_display_adapter_macro_derive::PrettyJsonSerializeDisplayAdapter;

pub mod hid_usage_id;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum KeyboardMouse {
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    AButton,
    BButton,
    XButton,
    YButton,
    LeftStickUp,
    LeftStickDown,
    LeftStickLeft,
    LeftStickRight,
    LeftStickPush,
    RightStickPush,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum BackButton {
    Left1,
    Left2,
    Left3,
    Left4,
    Right1,
    Right2,
    Right3,
    Right4,
}

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    ValueEnum,
    Debug,
    TryFromPrimitive,
    IntoPrimitive,
    Default,
    Serialize,
    Deserialize,
    PrettyJsonSerializeDisplayAdapter,
)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum Vibrate {
    #[default]
    Disable = 0,
    Light = 1,
    Heavy = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum DeadZone {
    LeftCenter,
    LeftBorder,
    RightCenter,
    RightBorder,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum BackButtonDelay {
    Left1,
    Left2,
    Left3,
    Right1,
    Right2,
    Right3,
}
