pub mod raw;
pub mod command;
pub mod function;

use colored::Colorize;
use log::warn;
use rusb::{DeviceHandle, GlobalContext};

const VENDOR_ID: u16 = 0x2F24;
const PRODUCT_ID: u16 = 0x0135;

pub fn connect() -> Result<DeviceHandle<GlobalContext>, String> {
    return rusb::open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID).map_or_else(|| {
        Err(format!("Could not find device with idVendor {} and idProduct {}\n\n{}\n\nOr apply udev rule {}", VENDOR_ID, PRODUCT_ID,
                    "Do you run as root or Administrator?".red().bold(),
                    r#"SUBSYSTEM=="usb", ATTRS{idVendor}=="2f24", ATTRS{idProduct}=="0135", MODE="0666", GROUP="plugdev""#.italic()))
    }, |mut device| {
        return (|| -> Result<DeviceHandle<GlobalContext>, rusb::Error> {
            device.reset()?;
            if rusb::supports_detach_kernel_driver() {
                (|| -> Result<(), rusb::Error> {
                    let num = device.device().device_descriptor()?.num_configurations();
                    (0..num).for_each(|i| {
                       device.device().config_descriptor(0).map_or_else(|e| {
                            warn!("{}", e)
                        }, |config| {
                            config.interfaces().for_each(|interface| {
                                device.kernel_driver_active(interface.number()).map_or_else(|e| {
                                    warn!("{}", e)
                                }, |v| {
                                    if v {
                                        device.detach_kernel_driver(interface.number()).err().inspect(|e| {
                                            warn!("{}", e);
                                        });
                                    }
                                });
                            });
                        });
                    });
                    return Ok(());
                })().err().inspect(|e| {
                    warn!("{}", e)
                });
            }
            let config = device.device().config_descriptor(0)?;
            device.set_active_configuration(config.number())?;
            return Ok(device);
        })().map_err(|e: rusb::Error| e.to_string());
    });
}