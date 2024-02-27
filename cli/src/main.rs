use std::fmt::Display;
use std::io::Read;
use std::process::exit;

use clap::Parser;

use gpd_linuxcontrols::controls_field::back_button::BackButtonConfig;
use gpd_linuxcontrols::controls_field::ControlsConfig;
use gpd_linuxcontrols::controls_field::dead_zones::DeadZonesConfig;
use gpd_linuxcontrols::controls_field::keyboard_mouse::KeyboardMouseConfig;
use gpd_linuxcontrols::enums::{BackButton, DeadZone};
use gpd_linuxcontrols::LoadArray;
use gpd_linuxcontrols::log::{debug, error};
use gpd_linuxcontrols::protocol::{connect, disconnect, find, unsafe_detach_kernel_driver};
use gpd_linuxcontrols::protocol::function::{read_all, read_checksum, read_config, read_firmware_version, save, write_config};
use gpd_linuxcontrols::protocol::raw::{get_report, set_report};
use gpd_linuxcontrols::strum::IntoEnumIterator;

use crate::cli::{Commands, KernelDriverCommand, RawCommand, ReadCommand, ResetCommand, WriteCommand};
use crate::helper::RangeValidator;

mod cli;
mod helper;

fn main() {
    let args = cli::Cli::parse();
    let log_level = args.verbose.log_level_filter();
    env_logger::Builder::new().filter_level(log_level).init();
    gpd_linuxcontrols::protocol::set_logger(log_level);
    debug!("{args:?}");

    if matches!(args.command, Commands::HIDUsageID) {
        gpd_linuxcontrols::enums::hid_usage_id::HIDUsageID::iter().for_each(|i| {
            println!("{:32}{:#X}", i.to_string(), <gpd_linuxcontrols::enums::hid_usage_id::HIDUsageID as Into<u8>>::into(i))
        });
        exit(0);
    }

    let mut device = find().unwrap_or_else(|error| {
        error!("{}", error);
        exit(1);
    });

    if let Commands::KernelDriver { kernel_driver_command } = args.command {
        (|| -> Result<(), String> {
            match kernel_driver_command {
                KernelDriverCommand::Detach => {
                    unsafe { unsafe_detach_kernel_driver(device) }
                }
                KernelDriverCommand::Attach => {
                    connect(&mut device)?;
                    disconnect(device)?;
                    Ok(())
                }
            }
        })().map_or_else(|e| {
            error!("{}", e);
            exit(1);
        }, |_| exit(0));
    } else {
        let code = (|| -> Result<(), String> {
            connect(&mut device)?;
            return match args.command {
                Commands::Read { read_command } => {
                    return (|| -> Result<String, String> {
                        let result: Result<Box<dyn Display>, String> = match read_command {
                            ReadCommand::Firmware =>
                                read_firmware_version(&device).map(|v| -> Box<dyn Display> { Box::new(v) }),
                            ReadCommand::Checksum =>
                                read_checksum(&device).map(|v| -> Box<dyn Display> { Box::new(v) }),
                            other => {
                                let config = read_all(&device)?;
                                let result: Box<dyn Display> = match other {
                                    ReadCommand::All => Box::new(config),
                                    ReadCommand::Config => Box::<ControlsConfig>::new(config.into()),
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
                        return result.map(|v| v.to_string());
                    })().map(|v| {
                        println!("{}", v);
                        return ();
                    });
                }
                Commands::Write {
                    write_command,
                    force
                } => {
                    let mut config = read_config(&device)?;
                    let validator = RangeValidator {
                        force
                    };
                    match write_command {
                        WriteCommand::Config { args } => {
                            let mut str: String = Default::default();
                            args.json.inspect(|v| {
                                str = v.to_owned();
                            });
                            args.file.map(|mut v| -> Result<(), String> {
                                v.read_to_string(&mut str).map_err(|e| e.to_string())?;
                                Ok(())
                            }).unwrap_or(Ok(()))?;
                            debug!("read: {}", str);
                            config = serde_json::from_str(&str).map_err(|e| -> String { e.to_string() })?;
                            debug!("deserialized: {}", config);
                            validator.validate_dead_zones(config.dead_zones.left.border, "Left::Border")?;
                            validator.validate_dead_zones(config.dead_zones.left.center, "Left::Border")?;
                            validator.validate_dead_zones(config.dead_zones.right.border, "Right::Border")?;
                            validator.validate_dead_zones(config.dead_zones.right.center, "Right::Border")?;

                            validator.validate_delay(config.back_button.left.first_delay, "Left::First")?;
                            validator.validate_delay(config.back_button.left.second_delay, "Left::Second")?;
                            validator.validate_delay(config.back_button.left.third_delay, "Left::Third")?;
                            validator.validate_delay(config.back_button.right.first_delay, "Right::First")?;
                            validator.validate_delay(config.back_button.right.second_delay, "Right::Second")?;
                            validator.validate_delay(config.back_button.right.third_delay, "Right::Third")?;
                        }
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
                                DeadZone::LeftCenter => config.dead_zones.left.center = validator.validate_dead_zones(args.value, "Left::Center")?,
                                DeadZone::LeftBorder => config.dead_zones.left.border = validator.validate_dead_zones(args.value, "Left::Border")?,
                                DeadZone::RightCenter => config.dead_zones.right.center = validator.validate_dead_zones(args.value, "Right::Center")?,
                                DeadZone::RightBorder => config.dead_zones.right.border = validator.validate_dead_zones(args.value, "Right::Border")?,
                            }
                        }
                        WriteCommand::BackButtonDelay(args) => {
                            match args.key {
                                gpd_linuxcontrols::enums::BackButtonDelay::Left1 => validator.validate_delay(args.value, "Left::First")?,
                                gpd_linuxcontrols::enums::BackButtonDelay::Left2 => validator.validate_delay(args.value, "Left::Second")?,
                                gpd_linuxcontrols::enums::BackButtonDelay::Left3 => validator.validate_delay(args.value, "Left::Third")?,
                                gpd_linuxcontrols::enums::BackButtonDelay::Right1 => validator.validate_delay(args.value, "Right::First")?,
                                gpd_linuxcontrols::enums::BackButtonDelay::Right2 => validator.validate_delay(args.value, "Right::Second")?,
                                gpd_linuxcontrols::enums::BackButtonDelay::Right3 => validator.validate_delay(args.value, "Right::Third")?,
                            };
                        }
                    }
                    write_config(&device, config)?;
                    save(&device)?;
                    Ok(())
                }
                Commands::Reset { reset_command } => {
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
                }
                Commands::Raw { raw_command } => {
                    return match raw_command {
                        RawCommand::SetReport { data } => {
                            let mut load: [u8; 33] = [0; 33];
                            hex::decode_to_slice(data.trim_start_matches("0x"), &mut load as &mut [u8]).map_err(|e| {
                                return e.to_string();
                            })?;
                            set_report(&device, load).map_err(|e| {
                                return e.to_string();
                            })?;
                            Ok(())
                        }
                        RawCommand::GetReport => {
                            get_report(&device).map(|v| -> () {
                                println!("{:#X}", <[u8; 65] as Into<LoadArray<65>>>::into(v));
                                return ();
                            })?;
                            Ok(())
                        }
                    };
                }
                _ => panic!("should not reach!")
            };
        })().map_or_else(|e| -> i32 {
            error!("{}", e);
            1
        }, |_| 0);

        disconnect(device).err().inspect(|e| error!("{}", e));
        exit(code);
    }
}
