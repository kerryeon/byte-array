/* ------------------------------------------------------------
    ByteArray
    Project.Github: "https://github.com/kerryeon/bytearray"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "3/9/2019"
------------------------------------------------------------ */

use crate::{
    BinaryBuilder,
    ByteArray,
};

impl BinaryBuilder for String {
    fn new() -> Self {
        String::new()
    }
    fn from_raw(ba: &mut ByteArray) -> Self {
        let raw: Vec<u8> = ba.read();
        String::from_utf8(raw).unwrap()
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        ba <<= &self.as_bytes().to_vec();
    }
}
