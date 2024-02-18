use log::warn;
use rusb::{DeviceHandle, UsbContext};

use crate::controls_field::{ControlsConfig, ControlsField, FirmwareVersion, Version};
use crate::controls_field::back_button::{BackButtonConfig, SpecificBackButtonConfig};
use crate::controls_field::Checksum;
use crate::controls_field::dead_zones::{DeadZonesConfig, SpecificDeadZone};
use crate::controls_field::keyboard_mouse::{DirectionalPadConfig, KeyboardMouseConfig, LeftStickConfig};
use crate::enums::Vibrate;
use crate::enums::Vibrate::Disable;
use crate::protocol::command::{read_command, ReadCommandMajor1MinorSerial, ReadCommandMajorSerial, write_command, WriteCommandMajor1MinorSerial, WriteCommandMajorSerial};

pub fn read_firmware_version<T: UsbContext>(device: &DeviceHandle<T>) -> Result<FirmwareVersion, String> {
    let load = read_command(device, ReadCommandMajorSerial::Major0)?;
    return Ok(FirmwareVersion {
        gamepad_firmware: Version {
            major_version: load[9],
            minor_version: load[10],
        },
        keyboard_mouse_firmware: Version {
            major_version: load[11],
            minor_version: load[12],
        },
    });
}

pub fn read_config<T: UsbContext>(device: &DeviceHandle<T>) -> Result<ControlsConfig, String> {
    let load0 = read_command(device, ReadCommandMajorSerial::Major1(ReadCommandMajor1MinorSerial::Minor0))?;
    let load1 = read_command(device, ReadCommandMajorSerial::Major1(ReadCommandMajor1MinorSerial::Minor1))?;
    return Ok(ControlsConfig {
        keyboard_mouse: KeyboardMouseConfig {
            directional_pad: DirectionalPadConfig {
                up: load0[0].into(),
                down: load0[2].into(),
                left: load0[4].into(),
                right: load0[6].into(),
            },
            a: load0[8].into(),
            b: load0[10].into(),
            x: load0[12].into(),
            y: load0[14].into(),
            left_stick: LeftStickConfig {
                up: load0[16].into(),
                down: load0[18].into(),
                left: load0[20].into(),
                right: load0[22].into(),
                push: load0[24].into(),
            },
            right_stick_push: load0[26].into(),
        },
        back_button: BackButtonConfig {
            left: SpecificBackButtonConfig {
                first: load0[50].into(),
                second: load0[52].into(),
                third: load0[54].into(),
                forth: load0[56].into(),
                first_delay: load1[16],
                second_delay: load1[18],
                third_delay: load1[20],
            },
            right: SpecificBackButtonConfig {
                first: load0[58].into(),
                second: load0[60].into(),
                third: load0[62].into(),
                forth: load1[0].into(),
                first_delay: load1[24],
                second_delay: load1[26],
                third_delay: load1[28],
            },
        },
        vibrate: Vibrate::try_from(load1[2]).unwrap_or_else(|e| {
            warn!("{}", e);
            warn!("Vibrate field is set to Vibrate::Disable");
            return Disable;
        }),
        dead_zones: DeadZonesConfig {
            left: SpecificDeadZone {
                center: load1[8] as i8,
                border: load1[9] as i8,
            },
            right: SpecificDeadZone {
                center: load1[10] as i8,
                border: load1[11] as i8,
            },
        },
    });
}

pub fn read_checksum<T: UsbContext>(device: &DeviceHandle<T>) -> Result<Checksum, String> {
    let load = read_command(device, ReadCommandMajorSerial::Major2)?;
    return Ok(u64::from_be_bytes(<[u8; 8]>::try_from(&load[24..32]).map_err(|e| e.to_string())?).into());
}

pub fn read_all<T: UsbContext>(device: &DeviceHandle<T>) -> Result<ControlsField, String> {
    let config = read_config(device)?;
    return Ok(ControlsField {
        firmware_version: read_firmware_version(device)?,
        keyboard_mouse: config.keyboard_mouse,
        back_button: config.back_button,
        vibrate: config.vibrate,
        dead_zones: config.dead_zones,
        checksum: read_checksum(device)?.into(),
    });
}

pub fn write_config<T: UsbContext>(device: &DeviceHandle<T>, config: ControlsConfig) -> Result<(), String> {
    let mut load0 = [0u8; 25];
    let mut load1 = [0u8; 25];
    let mut load3 = [0u8; 25];
    let mut load4 = [0u8; 25];
    let mut load5 = [0u8; 25];

    load0[0] = config.keyboard_mouse.directional_pad.up.into();
    load0[2] = config.keyboard_mouse.directional_pad.down.into();
    load0[4] = config.keyboard_mouse.directional_pad.left.into();
    load0[6] = config.keyboard_mouse.directional_pad.right.into();
    load0[8] = config.keyboard_mouse.a.into();
    load0[10] = config.keyboard_mouse.b.into();
    load0[12] = config.keyboard_mouse.x.into();
    load0[14] = config.keyboard_mouse.y.into();

    load1[0] = config.keyboard_mouse.left_stick.up.into();
    load1[2] = config.keyboard_mouse.left_stick.down.into();
    load1[4] = config.keyboard_mouse.left_stick.left.into();
    load1[6] = config.keyboard_mouse.left_stick.right.into();
    load1[8] = config.keyboard_mouse.left_stick.push.into();
    load1[10] = config.keyboard_mouse.right_stick_push.into();

    load3[2] = config.back_button.left.first.into();
    load3[4] = config.back_button.left.second.into();
    load3[6] = config.back_button.left.third.into();
    load3[8] = config.back_button.left.forth.into();
    load3[10] = config.back_button.right.first.into();
    load3[12] = config.back_button.right.second.into();
    load3[14] = config.back_button.right.third.into();

    load4[0] = config.back_button.right.forth.into();
    load4[2] = config.vibrate.into();
    load4[8] = config.dead_zones.left.center as u8;
    load4[9] = config.dead_zones.left.border as u8;
    load4[10] = config.dead_zones.right.center as u8;
    load4[11] = config.dead_zones.right.border as u8;

    load5[0] = config.back_button.left.first_delay.into();
    load5[2] = config.back_button.left.second_delay.into();
    load5[4] = config.back_button.left.third_delay.into();
    load5[8] = config.back_button.right.first_delay.into();
    load5[10] = config.back_button.right.second_delay.into();
    load5[12] = config.back_button.right.third_delay.into();

    write_command(device, WriteCommandMajorSerial::Major1(WriteCommandMajor1MinorSerial::Minor0), load0)?;
    write_command(device, WriteCommandMajorSerial::Major1(WriteCommandMajor1MinorSerial::Minor1), load1)?;
    write_command(device, WriteCommandMajorSerial::Major1(WriteCommandMajor1MinorSerial::Minor3), load3)?;
    write_command(device, WriteCommandMajorSerial::Major1(WriteCommandMajor1MinorSerial::Minor4), load4)?;
    write_command(device, WriteCommandMajorSerial::Major1(WriteCommandMajor1MinorSerial::Minor5), load4)?;
    return Ok(());
}

pub fn save<T: UsbContext>(device: &DeviceHandle<T>) -> Result<(), String> {
    return write_command(device, WriteCommandMajorSerial::Major3, [0u8; 25]);
}