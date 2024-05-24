use num_enum::IntoPrimitive;
use rusb::{DeviceHandle, UsbContext};

use crate::protocol::raw::{get_report, set_report};

pub fn read_command<T: UsbContext>(
    device: &DeviceHandle<T>,
    serial: ReadCommandMajorSerial,
) -> Result<[u8; 65], String> {
    let mut data: [u8; 33] = [0; 33];
    data[0] = 0x01;
    data[1] = 0xA5;
    data[3] = 0x5A;
    match serial {
        ReadCommandMajorSerial::Major0 => {
            data[2] = 0x10;
            data[4] = 0xEF;
        }
        ReadCommandMajorSerial::Major1(minor) => {
            data[2] = 0x11;
            data[4] = 0xEE;
            data[6] = minor.into()
        }
        ReadCommandMajorSerial::Major2 => {
            data[2] = 0x12;
            data[4] = 0xED;
        }
    }
    set_report(device, data)?;
    get_report(device)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ReadCommandMajorSerial {
    Major0 = 0,
    Major1(ReadCommandMajor1MinorSerial) = 1,
    Major2 = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, IntoPrimitive)]
#[repr(u8)]
pub enum ReadCommandMajor1MinorSerial {
    Minor0 = 0,
    Minor1 = 1,
    Minor2 = 2,
    Minor3 = 3,
}

pub fn write_command<T: UsbContext>(
    device: &DeviceHandle<T>,
    serial: WriteCommandMajorSerial,
    load: [u8; 25],
) -> Result<(), String> {
    let mut data: [u8; 33] = [0; 33];
    data[0] = 0x01;
    data[1] = 0xA5;
    data[3] = 0x5A;
    match serial {
        WriteCommandMajorSerial::Major1(minor) => {
            data[2] = 0x21;
            data[4] = 0xDE;
            data[6] = minor.into();
        }
        WriteCommandMajorSerial::Major3 => {
            data[2] = 0x23;
            data[4] = 0xDC;
        }
    }
    data[8..].copy_from_slice(&load);
    set_report(device, data)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum WriteCommandMajorSerial {
    Major1(WriteCommandMajor1MinorSerial) = 1,
    Major3 = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, IntoPrimitive)]
#[repr(u8)]
pub enum WriteCommandMajor1MinorSerial {
    Minor0 = 0,
    Minor1 = 1,
    Minor2 = 2,
    Minor3 = 3,
    Minor4 = 4,
    Minor5 = 5,
    Minor6 = 6,
    Minor7 = 7,
}
