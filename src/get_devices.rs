use crate::modal::*;
use hidapi::HidApi;
use anyhow::{Result,anyhow};

pub fn get_gamepad_list(log_lvl: u8) -> anyhow::Result<Vec<Device>> {
    
    let mut gamepad_list: Vec<Device> = Vec::new();
    
    let api = HidApi::new()?;
    
    if (log_lvl > 0) {
        println!("Checking HID devices...");
    }
    let mut gamepad_found: bool = false;
    for device_info in api.device_list() {
        // Filter for gamepads: usage page = 0x01 and usage = 0x04 (joystick) or 0x05 (gamepad)
        let is_gamepad = match (device_info.usage_page(), device_info.usage()) {
            (0x01, 0x05) | (0x01, 0x04) => true,
            _ => false,
        };

        if log_lvl > 1 {
            println!("Found device: {} (VID: {:04x}, PID: {:04x}, Usage Page: {:04x}, Usage: {:04x})", 
                device_info.product_string().unwrap_or("Unknown Product"), 
                device_info.vendor_id(), 
                device_info.product_id(),
                device_info.usage_page(),
                device_info.usage());
        }

        if !is_gamepad {
            continue;
        };

        if log_lvl > 0 {
            println!("Found gamepad: {} (VID: {:04x}, PID: {:04x})", 
                device_info.product_string().unwrap_or("Unknown Gamepad"), 
                device_info.vendor_id(), 
                device_info.product_id());
        }

        gamepad_found = true;
        let device = Device::new(
            device_info.vendor_id(), 
            device_info.product_id(),
            device_info.manufacturer_string().map(|s| s.to_string()), 
            device_info.product_string().map(|s| s.to_string()), 
            device_info.serial_number().map(|s| s.to_string()), 
            device_info.path().to_owned(), 
            device_info.usage_page(), 
            device_info.usage(), 
            device_info.interface_number());

        gamepad_list.push(device);
    }

    if !gamepad_found {
        if log_lvl > 0 {
            println!("No gamepad devices found.");
        }
        return Err(anyhow!("No gamepad devices found."));
    }

    Ok(gamepad_list)
}