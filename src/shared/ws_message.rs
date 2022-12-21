use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum WSMessage {
    #[deku(id = "0x01")]
    Connect,
    #[deku(id = "0x02")]
    Accept,
    #[deku(id = "0x03")]
    Frame,
}
