use deku::prelude::*;
use serde::{Serialize, Deserialize};
#[derive(Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(type = "u8")]
pub enum WSMessage {
    #[deku(id = "0x01")]
    Connect(ConnectData),
    #[deku(id = "0x02")]
    Accept,
    #[deku(id = "0x03")]
    Deny,
    #[deku(id = "0x04")]
    Frame,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct ConnectData {
    pub destination_port: u16,
    pub destination_address_count: u8,
    #[deku(count = "destination_address_count")]
    pub destination_address: Vec<u8>, 
}

impl ConnectData {
    pub fn new(destination_port: u16, destination_address: &str) -> ConnectData {
        let bytes = destination_address.as_bytes();
        ConnectData {
            destination_port,
            destination_address_count: bytes.len() as u8,
            destination_address: bytes.to_vec(),
        }
    }
    pub fn destination_address(&self) -> String {
        return String::from_utf8(self.destination_address.clone()).unwrap();
    }
}
