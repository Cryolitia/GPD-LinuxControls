use colored::Colorize;
use log::{LevelFilter, warn};
use rusb::{DeviceHandle, GlobalContext, LogLevel, UsbContext};

pub mod raw;
pub mod command;
pub mod function;

const VENDOR_ID: u16 = 0x2F24;
const PRODUCT_ID: u16 = 0x0135;

pub fn connect(log_level: LevelFilter) -> Result<DeviceHandle<GlobalContext>, String> {
    rusb::set_log_level(match log_level {
        LevelFilter::Off => LogLevel::None,
        LevelFilter::Error => LogLevel::Error,
        LevelFilter::Warn => LogLevel::Warning,
        LevelFilter::Info => LogLevel::Info,
        LevelFilter::Debug => LogLevel::Debug,
        LevelFilter::Trace => LogLevel::Debug,
    });
    return rusb::open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID).map_or_else(|| {
        Err(format!("Could not find device with idVendor {} and idProduct {}\n\n{}\n\nOr apply udev rule {}", VENDOR_ID, PRODUCT_ID,
                    "Do you run as root or Administrator?".red().bold(),
                    r#"SUBSYSTEM=="usb", ATTRS{idVendor}=="2f24", ATTRS{idProduct}=="0135", MODE="0666", GROUP="plugdev""#.italic()))
    }, |mut device| {
        return (|| -> Result<DeviceHandle<GlobalContext>, String> {
            device.reset().map_err(|e| e.to_string())?;
            return Ok(device);
        })();
    });
}

pub fn process_kernel_driver<T: UsbContext>(device: &mut DeviceHandle<T>, attach_or_detach: bool) -> Result<(), String> {
    return if rusb::supports_detach_kernel_driver() {
        (|| -> Result<(), rusb::Error> {
            device.set_auto_detach_kernel_driver(true)?;
            let num = device.device().device_descriptor()?.num_configurations();
            (0..num).for_each(|i| {
                device.device().config_descriptor(i).map_or_else(|e| {
                    warn!("{}", e)
                }, |config| {
                    config.interfaces().for_each(|interface| {
                        if attach_or_detach {
                            device.claim_interface(interface.number()).err().inspect(|e| warn!("{}", e));
                        } else {
                            device.release_interface(interface.number()).err().inspect(|e| warn!("{}", e));
                        }
                    });
                });
            });
            return Ok(());
        })().map_err(|e| {
            return e.to_string();
        })
    } else {
        Err("rusb: Not support detaching the kernel driver".to_string())
    };
}

pub unsafe fn unsafe_detach_kernel_driver<T: UsbContext>(mut device: DeviceHandle<T>) -> Result<(), String> {
    return if rusb::supports_detach_kernel_driver() {
        (|| -> Result<(), rusb::Error> {
            let num = device.device().device_descriptor()?.num_configurations();
            (0..num).for_each(|i| {
                device.device().config_descriptor(i).map_or_else(|e| {
                    warn!("{}", e)
                }, |config| {
                    config.interfaces().for_each(|interface| {
                        device.detach_kernel_driver(interface.number()).err().inspect(|e| { warn!("{}", e); });
                    });
                });
            });
            return Ok(());
        })().map_err(|e| {
            return e.to_string();
        })
    } else {
        Err("rusb: Not support detaching the kernel driver".to_string())
    };
}

pub fn disconnect<T: UsbContext>(mut device: DeviceHandle<T>) -> Result<(), String> {
    process_kernel_driver(&mut device, false)
}
