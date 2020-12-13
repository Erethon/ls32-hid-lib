mod charset;
use hidapi::{HidApi, HidDevice, HidResult};
use serde::Serialize;
use std::convert::TryInto;

const VENDORID: u16 = 0x0416;
const PRODUCTID: u16 = 0x5020;

#[derive(Serialize, Default)]
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

pub fn open_badge() -> HidResult<HidDevice> {
    let hid = HidApi::new().unwrap();
    Ok(hid
        .open(VENDORID, PRODUCTID)
        .expect("Failed to open device with VendorID and ProductID"))
}

pub fn send_msg(badge: HidDevice, msg: &str, blink: bool, brightness: u8) -> HidResult<usize> {
    let msg_len: u8 = msg.len().try_into().expect("Message longer than then maximum length of 255 characters");

    let header = Header {
        preamble: [0x77, 0x61, 0x6e, 0x67, 0x00],
        brightness,
        blink: if blink { 0xff } else { 0x00 },
        mode: [0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
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
    badge.write(&data)
}
