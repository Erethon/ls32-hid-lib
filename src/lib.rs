extern crate hidapi;

mod charset;
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

pub fn send_msg(msg: &str, blink: bool, brightness: u8) {
    let message: u8 = msg.len().try_into().unwrap();

    let header = Header {
        preamble: [0x77, 0x61, 0x6e, 0x67, 0x00],
        brightness,
        blink: if blink { 0xff } else { 0x00 },
        mode: [0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        length: [
            0x00, message, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        ..Default::default()
    };
    let hid = hidapi::HidApi::new().unwrap();
    let badge = hid
                .open(VENDORID, PRODUCTID)
                .expect("Failed to open device with VendorID and ProductID");
    let mut data = bincode::serialize(&header).unwrap();
    for character in msg.chars() {
        for byte in charset::CHARSET.get::<str>(&character.to_string()).unwrap() {
            data.push(*byte);
        }
    }
    let _led = badge.write(&data);
}
