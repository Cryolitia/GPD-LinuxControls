use clap::ValueEnum;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use serialize_display_adapter_macro_derive::SerializeDisplayAdapter;

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
    EnumIter,
    Serialize,
    Deserialize,
    SerializeDisplayAdapter,
)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum HIDUsageID {
    #[default]
    NoEvent = 0x00,
    ErrorRollOver = 0x01,
    PostFail = 0x02,
    ErrorUndefined = 0x03,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    Digit1 = 0x1E,
    Digit2 = 0x1F,
    Digit3 = 0x20,
    Digit4 = 0x21,
    Digit5 = 0x22,
    Digit6 = 0x23,
    Digit7 = 0x24,
    Digit8 = 0x25,
    Digit9 = 0x26,
    Digit0 = 0x27,
    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    HyphenMinus = 0x2D,
    EqualsSign = 0x2E,
    LeftSquareBracket = 0x2F,
    RightSquareBracket = 0x30,
    ReverseSolidus = 0x31,
    Europe1 = 0x32,
    Semicolon = 0x33,
    Apostrophe = 0x34,
    GraveAccent = 0x35,
    Comma = 0x36,
    FullStop = 0x37,
    Solidus = 0x38,
    CapsLock = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    RightArrow = 0x4F,
    LeftArrow = 0x50,
    DownArrow = 0x51,
    UpArrow = 0x52,
    NumLock = 0x53,
    KeypadSolidus = 0x54,
    KeypadAsterisk = 0x55,
    KeypadHyphenMinus = 0x56,
    KeypadPlusSign = 0x57,
    KeypadEnter = 0x58,
    Keypad1 = 0x59,
    Keypad2 = 0x5A,
    Keypad3 = 0x5B,
    Keypad4 = 0x5C,
    Keypad5 = 0x5D,
    Keypad6 = 0x5E,
    Keypad7 = 0x5F,
    Keypad8 = 0x60,
    Keypad9 = 0x61,
    Keypad0 = 0x62,
    KeypadFullStop = 0x63,
    Europe2 = 0x64,
    Application = 0x65,
    Power = 0x66,
    KeypadEqualsSign = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6A,
    F16 = 0x6B,
    F17 = 0x6C,
    F18 = 0x6D,
    F19 = 0x6E,
    F20 = 0x6F,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Execute = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7A,
    Cut = 0x7B,
    Copy = 0x7C,
    Paste = 0x7D,
    Find = 0x7E,
    Mute = 0x7F,
    VolumeUp = 0x80,
    VolumeDown = 0x81,
    LockingCapsLock = 0x82,
    LockingNumLock = 0x83,
    LockingScrollLock = 0x84,
    KeypadComma = 0x85,
    KeypadEqualSign = 0x86,
    International1 = 0x87,
    International2 = 0x88,
    International3 = 0x89,
    International4 = 0x8A,
    International5 = 0x8B,
    International6 = 0x8C,
    International7 = 0x8D,
    International8 = 0x8E,
    International9 = 0x8F,
    LANG1 = 0x90,
    LANG2 = 0x91,
    LANG3 = 0x92,
    LANG4 = 0x93,
    LANG5 = 0x94,
    LANG6 = 0x95,
    LANG7 = 0x96,
    LANG8 = 0x97,
    LANG9 = 0x98,
    AlternateErase = 0x99,
    SysReq = 0x9A,
    Cancel = 0x9B,
    Clear = 0x9C,
    Prior = 0x9D,
    Return = 0x9E,
    Separator = 0x9F,
    Out = 0xA0,
    Oper = 0xA1,
    ClearAgain = 0xA2,
    CrSelProp = 0xA3,
    ExSel = 0xA4,
    Keypad00 = 0xB0,
    Keypad000 = 0xB1,
    ThousandsSeparator = 0xB2,
    DecimalSeparator = 0xB3,
    CurrencyUnit = 0xB4,
    CurrencySubUnit = 0xB5,
    KeypadLeftParenthesis = 0xB6,
    KeypadRightParenthesis = 0xB7,
    KeypadLeftCurlyBracket = 0xB8,
    KeypadRightCurlyBracket = 0xB9,
    KeypadTab = 0xBA,
    KeypadBackspace = 0xBB,
    KeypadA = 0xBC,
    KeypadB = 0xBD,
    KeypadC = 0xBE,
    KeypadD = 0xBF,
    KeypadE = 0xC0,
    KeypadF = 0xC1,
    KeypadXOR = 0xC2,
    KeypadCircumflexAccent = 0xC3,
    KeypadPercentSign = 0xC4,
    KeypadLessThanSign = 0xC5,
    KeypadGreaterThanSign = 0xC6,
    KeypadAmpersand = 0xC7,
    KeypadDoubleAmpersand = 0xC8,
    KeypadVerticalLine = 0xC9,
    KeypadDoubleVerticalLine = 0xCA,
    KeypadColon = 0xCB,
    KeypadNumberSign = 0xCC,
    KeypadSpace = 0xCD,
    KeypadCommercialAt = 0xCE,
    KeypadExclamationMark = 0xCF,
    KeypadMemoryStore = 0xD0,
    KeypadMemoryRecall = 0xD1,
    KeypadMemoryClear = 0xD2,
    KeypadMemoryAdd = 0xD3,
    KeypadMemorySubtract = 0xD4,
    KeypadMemoryMultiply = 0xD5,
    KeypadMemoryDivide = 0xD6,
    KeypadAddOrSubtract = 0xD7,
    KeypadClear = 0xD8,
    KeypadClearEntry = 0xD9,
    KeypadBinary = 0xDA,
    KeypadOctal = 0xDB,
    KeypadDecimal = 0xDC,
    KeypadHexadecimal = 0xDD,
    KeyboardLeftControl = 0xE0,
    KeyboardLeftShift = 0xE1,
    KeyboardLeftAlt = 0xE2,
    KeyboardLeftGUI = 0xE3,
    KeyboardRightControl = 0xE4,
    KeyboardRightShift = 0xE5,
    KeyboardRightAlt = 0xE6,
    KeyboardRightGUI = 0xE7,

    //GPD defined HID Usage ID extensions:
    MouseUp = 0xE8,
    MouseDown = 0xE9,
}
