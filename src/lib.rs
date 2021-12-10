mod charset;
use hidapi::{HidApi, HidDevice};
use serde::Serialize;
use std::convert::TryInto;

const VENDORID: u16 = 0x0416;
const PRODUCTID: u16 = 0x5020;

#[derive(Serialize)]
struct Header {
    preamble: [u8; 5],
    brightness: u8,
    blink: u8,
    border: u8,
    mode: [u8; 8],
    length: [u8; 16],
    separator: [u8; 6],
    date: [u8; 6],
    boundary: [u8; 20],
}

impl Default for Header {
    fn default() -> Header {
        Header {
            preamble: [0x77, 0x61, 0x6e, 0x67, 0x00],
            brightness: 0x00,
            blink: 0x00,
            border: 0x00,
            mode: [0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            length: [0; 16],
            separator: [0; 6],
            date: [0; 6],
            boundary: [0; 20],
        }
    }
}

pub fn open_badge() -> Result<HidDevice, String> {
    match HidApi::new() {
        Ok(hid) => {
            let devices = hid.device_list();
            for device in devices {
                if device.vendor_id() == VENDORID && device.product_id() == PRODUCTID {
                    match hid.open(VENDORID, PRODUCTID) {
                        Ok(open_dev) => return Ok(open_dev),
                        Err(_) => return Err(String::from("Couldn't open HID device")),
                    }
                }
            }
            Err(format!(
                "No device found with VendorID: {} and ProductID: {}",
                VENDORID, PRODUCTID
            ))
        }
        Err(_) => Err(String::from("Couldn't get HID devices")),
    }
}

pub fn send_msg(badge: HidDevice, msg: &str, blink: bool, brightness: u8) -> Result<usize, String> {
    let msg_len: u8 = msg
        .len()
        .try_into()
        .expect("Message longer than then maximum length of 255 characters");

    let header = Header {
        brightness,
        blink: if blink { 0xff } else { 0x00 },
        length: [
            0x00, msg_len, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        ..Default::default()
    };

    let mut data = bincode::serialize(&header).expect("Failed to serialize header");
    for character in msg.chars() {
        for byte in charset::CHARSET.get::<str>(&character.to_string()).unwrap() {
            data.push(*byte);
        }
    }
    match badge.write(&data) {
        Ok(size) => Ok(size),
        Err(_) => Err(String::from("Failed to write on device")),
    }
}
