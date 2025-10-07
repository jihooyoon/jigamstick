use std::io::Write;
use hidapi::HidApi;

mod modal;
use crate::modal::*;
mod get_devices;
use crate::get_devices::*;
mod collect_data;
use crate::collect_data::*;


fn main() {
    let log_lvl:u8 = 1;
    
    //get gamepad device list
    let mut gamepad_list: Vec<Device> = Vec::new();
    match get_gamepad_list(log_lvl) {
        Ok(v) => {
            gamepad_list = v;
        }

        Err(e) => {
            eprintln!("Error: {}", e);
            return
        }
    }

    //Select gamepad to use
    println!("Total gamepads found: {}", gamepad_list.len());
    for (i, device) in gamepad_list.iter().enumerate() {
        println!("Gamepad {}: {} (VID: {:04x}, PID: {:04x})", 
            i + 1, 
            device.product_string().as_deref().unwrap_or("Unknown Gamepad"), 
            device.vendor_id(), 
            device.product_id());
    }
    print!("Selected gamepad: ");
    let mut input_text = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let selected_index: usize = match input_text.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= gamepad_list.len() => num - 1,
        _ => {
            eprintln!("Invalid selection.");
            return;
        }
    };

    // Try open device
    let hid_selected_device: hidapi::HidDevice = HidApi::new().unwrap().open_path(&gamepad_list[selected_index].path().as_c_str()).unwrap();


}