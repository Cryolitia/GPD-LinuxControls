use std::time::Duration;

use log::debug;
use rusb::{DeviceHandle, UsbContext};

use crate::LoadArray;

#[derive(Debug)]
struct ReportHeader {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
}

const SET_REPORT_HEADER: ReportHeader = ReportHeader {
    request_type: 0x21,
    request: 0x09,
    value: 0x0201,
    index: 2,
};

const GET_REPORT_HEADER: ReportHeader = ReportHeader {
    request_type: 0xA1,
    request: 0x01,
    value: 0x0101,
    index: 2,
};

pub fn set_report<T: UsbContext>(device: &DeviceHandle<T>, data: [u8; 33]) -> Result<(), String> {
    debug!("SET_REPORT: {:#X}", <[u8; 33] as Into<LoadArray<33>>>::into(data));
    return device.write_control(SET_REPORT_HEADER.request_type, SET_REPORT_HEADER.request, SET_REPORT_HEADER.value, SET_REPORT_HEADER.index,
                                &data, Duration::from_secs(1)).map_or_else(|e| {
        Err(e.to_string())
    }, |_| {
        Ok(())
    });
}

pub fn get_report<T: UsbContext>(device: &DeviceHandle<T>) -> Result<[u8; 65], String> {
    let mut data: [u8; 65] = [0; 65];
    return device.read_control(GET_REPORT_HEADER.request_type, GET_REPORT_HEADER.request, GET_REPORT_HEADER.value, GET_REPORT_HEADER.index,
                               &mut data, Duration::from_secs(1)).map_or_else(|e| {
        Err(e.to_string())
    }, |_| {
        debug!("GET_REPORT: {:X}", <[u8; 65] as Into<LoadArray<65>>>::into(data));
        Ok(data)
    });
}