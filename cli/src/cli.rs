use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use gpd_linuxcontrols::controls_field::hid_usage_id_u8::HIDUsageIDu8;
use gpd_linuxcontrols::enums::{BackButton, BackButtonDelay, DeadZone, KeyboardMouse, Vibrate};
use gpd_linuxcontrols::enums::hid_usage_id::HIDUsageID;
use gpd_linuxcontrols::parse_hex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[command(flatten)]
    pub(crate) verbose: Verbosity,
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub(crate) enum Commands {
    #[command(about = "Read config field")]
    Read {
        #[command(subcommand)]
        read_command: ReadCommand,
    },
    #[command(about = "Write config field")]
    Write {
        #[command(subcommand)]
        write_command: WriteCommand,

        #[arg(long, global = true, help = "Ignore value legality check and force write")]
        force: bool,
    },
    #[command(about = "Print HID Usage ID table")]
    HIDUsageID,
    #[command(about = "Reset All config field to 0 / No Function")]
    Reset {
        #[command(subcommand)]
        reset_command: ResetCommand,
    },
    #[command(about = "Directly transfer raw data by SET_REPORT and GET_REPORT")]
    Raw {
        #[command(subcommand)]
        raw_command: RawCommand,
    },
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub(crate) enum ReadCommand {
    #[command(about = "Reset all fields")]
    All,
    #[command(about = "Read all configurable fields")]
    Config,
    #[command(about = "Read Firmware versions")]
    Firmware,
    KeyboardMouse,
    BackButton,
    Vibrate,
    DeadZones,
    BackButtonDelay,
    Checksum,
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub(crate) enum WriteCommand {
    #[command(about = "Write configurable fields as JSON")]
    Config {
        #[command(flatten)]
        args: ConfigJsonArgs
    },
    KeyboardMouse(KeyboardMouseArgs),
    BackButton(BackButtonArgs),
    Vibrate(VibrateArgs),
    DeadZone(DeadZoneArgs),
    BackButtonDelay(BackButtonDelayArgs),
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub(crate) enum ResetCommand {
    KeyboardMouse,
    BackButton,
    DeadZone,
    All,
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub(crate) enum RawCommand {
    #[command(about = "Directly transfer raw data by SET_REPORT")]
    SetReport {
        data: String
    },
    #[command(about = "Directly transfer raw data by GET_REPORT")]
    GetReport,
}

#[derive(Args, Debug, Eq, PartialEq)]
#[group(required = true, multiple = false)]
pub(crate) struct HIDUsageIDArgs {
    #[arg(long, value_parser = parse_hex, next_line_help = true, long_help =
    "USB HID Usage ID in hexadecimal or decimal
https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/translate.pdf
https://www.usb.org/sites/default/files/hut1_21_0.pdf")]
    pub(crate) hex: Option<HIDUsageIDu8>,

    #[arg(long_help = "USB HID Usage ID by name , see details with --help")]
    pub(crate) name: Option<HIDUsageID>,
}

#[derive(Args, Debug, Eq, PartialEq)]
#[group(required = true, multiple = false)]
pub(crate) struct ConfigJsonArgs {
    #[arg(long, help = "Read JSON from a file")]
    pub(crate) file: Option<String>,

    pub(crate) json: Option<String>,
}

#[derive(Parser, Debug, Eq, PartialEq)]
pub(crate) struct KeyboardMouseArgs {
    pub(crate) key: KeyboardMouse,

    #[command(flatten)]
    pub(crate) value: HIDUsageIDArgs,
}

#[derive(Parser, Debug, Eq, PartialEq)]
pub(crate) struct BackButtonArgs {
    pub(crate) key: BackButton,

    #[command(flatten)]
    pub(crate) value: HIDUsageIDArgs,
}

#[derive(Parser, Debug, Eq, PartialEq)]
pub(crate) struct VibrateArgs {
    pub(crate) value: Vibrate,
}

#[derive(Parser, Debug, Eq, PartialEq)]
pub(crate) struct DeadZoneArgs {
    pub(crate) key: DeadZone,
    pub(crate) value: i8,
}

#[derive(Parser, Debug, Eq, PartialEq)]
pub(crate) struct BackButtonDelayArgs {
    pub(crate) key: BackButtonDelay,
    pub(crate) value: u8,
}
