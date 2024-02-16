use std::fmt::Debug;
use std::process::exit;
use clap::Parser;
use log::{debug, error};
use gpd_linuxcontrols::enums::hid_usage_id::HIDUsageID;
use gpd_linuxcontrols::protocol::connect;
use gpd_linuxcontrols::protocol::function::{read_all, read_checksum, read_config, read_firmware_version, save, write_config};
use strum::IntoEnumIterator;
use gpd_linuxcontrols::controls_field::back_button::BackButtonConfig;
use gpd_linuxcontrols::controls_field::ControlsConfig;
use gpd_linuxcontrols::controls_field::dead_zones::DeadZonesConfig;
use gpd_linuxcontrols::controls_field::keyboard_mouse::KeyboardMouseConfig;
use gpd_linuxcontrols::enums::{BackButton, DeadZone};
use gpd_linuxcontrols::LoadArray;
use gpd_linuxcontrols::protocol::raw::{get_report, set_report};

use crate::cli::{Commands, KeyboardMouseArgs, RawCommand, ReadCommand, ResetCommand, WriteCommand};
use crate::helper::RangeValidator;

mod cli;
mod helper;

fn main() {
    let args = cli::Cli::parse();
    env_logger::Builder::new().filter_level(args.verbose.log_level_filter()).init();
    debug!("{args:?}");

    let device = connect().unwrap_or_else(|error| {
        error!("{}", error);
        exit(1);
    });

    match args.command {
        Commands::Read { read_command } => {
            (|| -> Result<String, String> {
                let result: Result<Box<dyn Debug>, String> = match read_command {
                    ReadCommand::Firmware =>
                        read_firmware_version(&device).map(|v| -> Box<dyn Debug> { Box::new(v) }),
                    ReadCommand::Checksum =>
                        read_checksum(&device).map(|v| -> Box<dyn Debug> { Box::new(v) }),
                    other => {
                        let config = read_all(&device)?;
                        let result: Box<dyn Debug> = match other {
                            ReadCommand::All => Box::new(config),
                            ReadCommand::KeyboardMouse => Box::new(config.keyboard_mouse),
                            ReadCommand::BackButton => Box::new(config.back_button),
                            ReadCommand::Vibrate => Box::new(config.vibrate),
                            ReadCommand::DeadZones => Box::new(config.dead_zones),
                            ReadCommand::BackButtonDelay => Box::new(config.back_button),
                            _ => panic!("Never reach!")
                        };
                        Ok(result)
                    }
                };
                return result.map(|v| format!("{:#?}", v));
            })().map_or_else(|e| {
                error!("{}", e);
                exit(1);
            }, |v| {
                println!("{}", v);
                exit(0)
            });
        }
        Commands::Write {
            write_command,
            force
        } => {
            (|| -> Result<(), String> {
                let mut config = read_config(&device)?;
                let validator = RangeValidator {
                    force
                };
                match write_command {
                    WriteCommand::KeyboardMouse(args) => {
                        match args.key {
                            gpd_linuxcontrols::enums::KeyboardMouse::DPadUp => config.keyboard_mouse.directional_pad.up = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::DPadDown => config.keyboard_mouse.directional_pad.down = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::DPadLeft => config.keyboard_mouse.directional_pad.left = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::DPadRight => config.keyboard_mouse.directional_pad.right = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::AButton => config.keyboard_mouse.a = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::BButton => config.keyboard_mouse.b = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::XButton => config.keyboard_mouse.x = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::YButton => config.keyboard_mouse.y = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::LeftStickUp => config.keyboard_mouse.left_stick.up = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::LeftStickDown => config.keyboard_mouse.left_stick.down = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::LeftStickLeft => config.keyboard_mouse.left_stick.left = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::LeftStickRight => config.keyboard_mouse.left_stick.right = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::LeftStickPush => config.keyboard_mouse.left_stick.push = args.value.into(),
                            gpd_linuxcontrols::enums::KeyboardMouse::RightStickPush => config.keyboard_mouse.right_stick_push = args.value.into(),
                        }
                    }
                    WriteCommand::BackButton(args) => {
                        match args.key {
                            BackButton::Left1 => config.back_button.left.first = args.value.into(),
                            BackButton::Left2 => config.back_button.left.second = args.value.into(),
                            BackButton::Left3 => config.back_button.left.third = args.value.into(),
                            BackButton::Left4 => config.back_button.left.forth = args.value.into(),
                            BackButton::Right1 => config.back_button.right.first = args.value.into(),
                            BackButton::Right2 => config.back_button.right.second = args.value.into(),
                            BackButton::Right3 => config.back_button.right.third = args.value.into(),
                            BackButton::Right4 => config.back_button.right.forth = args.value.into(),
                        }
                    }
                    WriteCommand::Vibrate(args) => config.vibrate = args.value,
                    WriteCommand::DeadZone(args) => {
                        match args.key {
                            DeadZone::LeftCenter => config.dead_zones.left.center = validator.validate_dead_zones(args.value)?,
                            DeadZone::LeftBorder => config.dead_zones.left.border = validator.validate_dead_zones(args.value)?,
                            DeadZone::RightCenter => config.dead_zones.right.center = validator.validate_dead_zones(args.value)?,
                            DeadZone::RightBorder => config.dead_zones.right.border = validator.validate_dead_zones(args.value)?,
                        }
                    }
                    WriteCommand::BackButtonDelay(args) => {
                        match args.key {
                            gpd_linuxcontrols::enums::BackButtonDelay::Left1 => validator.validate_delay(args.value)?,
                            gpd_linuxcontrols::enums::BackButtonDelay::Left2 => validator.validate_delay(args.value)?,
                            gpd_linuxcontrols::enums::BackButtonDelay::Left3 => validator.validate_delay(args.value)?,
                            gpd_linuxcontrols::enums::BackButtonDelay::Right1 => validator.validate_delay(args.value)?,
                            gpd_linuxcontrols::enums::BackButtonDelay::Right2 => validator.validate_delay(args.value)?,
                            gpd_linuxcontrols::enums::BackButtonDelay::Right3 => validator.validate_delay(args.value)?,
                        };
                    }
                }
                write_config(&device, config)?;
                Ok(())
            })().err().inspect(|e| {
                error!("{}", e);
                exit(1);
            });
        }
        Commands::HIDUsageID => {
            HIDUsageID::iter().for_each(|i| {
                println!("{:32}{:#X}", format!("{:?}", i), <HIDUsageID as Into<u8>>::into(i))
            })
        }
        Commands::Reset { reset_command } => {
            (|| -> Result<(), String> {
                let mut config = read_config(&device)?;
                match reset_command {
                    ResetCommand::KeyboardMouse => {
                        config.keyboard_mouse = KeyboardMouseConfig::default();
                    }
                    ResetCommand::BackButton => {
                        config.back_button = BackButtonConfig::default();
                    }
                    ResetCommand::DeadZone => {
                        config.dead_zones = DeadZonesConfig::default();
                    }
                    ResetCommand::All => {
                        config = ControlsConfig::default();
                    }
                }
                write_config(&device, config)?;
                save(&device)?;
                Ok(())
            })().err().inspect(|e| {
                error!("{}", e);
                exit(1);
            });
        }
        Commands::Raw { raw_command } => {
            match raw_command {
                RawCommand::SetReport { data } => {
                    let mut load: [u8; 33] = [0; 33];
                    hex::decode_to_slice(data.trim_start_matches("0x"), &mut load as &mut [u8]).err().inspect(|e| {
                        error!("{}", e);
                        exit(1);
                    });
                    set_report(&device, load).err().inspect(|e| {
                        error!("{}", e);
                        exit(1);
                    });
                }
                RawCommand::GetReport => {
                    get_report(&device).map_or_else(|e| {
                        error!("{}", e);
                        exit(1);
                    }, |v| {
                        println!("{:#X}", <[u8; 65] as Into<LoadArray<65>>>::into(v))
                    });
                }
            }
        }
    };
}
