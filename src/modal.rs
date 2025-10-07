use std::ffi::CString;

use serde::{Serialize, Deserialize};
use getset::{Getters, Setters};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Device {
    vendor_id: u16,
    product_id: u16,
    manufacturer_string: Option<String>,
    product_string: Option<String>,
    serial_number: Option<String>,
    path: CString,
    usage_page: u16,
    usage: u16,
    interface_number: i32,
}

impl Device {
    pub fn new(vendor_id: u16, 
        product_id: u16,
        manufacturer_string: Option<String>,
        product_string: Option<String>,
        serial_number: Option<String>,
        path: CString,
        usage_page: u16,
        usage: u16,
        interface_number: i32) -> Device {
            Device {
                vendor_id,
                product_id,
                manufacturer_string,
                product_string,
                serial_number,
                path,
                usage_page,
                usage,
                interface_number,
            }
    }
}

pub struct HIDDescriptorReport {
    claimed_logical_min: Option<i64>,
    claimed_logical_max: Option<i64>,
    claimed_bits: Option<u32>,
    usage_page: Option<u16>,
    usage: Option<u16>,
}