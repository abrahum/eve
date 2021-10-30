use std::collections::HashMap;

use bytes::Buf;
pub(crate) use bytes::{BufMut, Bytes, BytesMut};

mod a;
mod b;
mod c;
mod e;

pub use a::*;
pub use b::*;
pub use c::*;
pub use e::*;

pub trait TlvWrite {
    fn put_bool(&mut self, b: bool);
    fn tlv(&mut self, tag: u16, put_fn: PutFn, len: usize);
    /// value len + 4
    fn tlv_bytes(&mut self, tag: u16, value: Bytes);
    /// str len + 2
    fn str_lv(&mut self, value: &str);
    /// bytes len + 2
    fn bytes_lv(&mut self, value: Bytes);
    fn str_lv_limit_size(&mut self, value: &str, limit: usize);
}

pub type PutFn = Box<dyn FnOnce(&mut BytesMut)>;

impl TlvWrite for BytesMut {
    fn put_bool(&mut self, b: bool) {
        if b {
            self.put_u8(0x01)
        } else {
            self.put_u8(0x00)
        }
    }

    fn tlv(&mut self, tag: u16, put_fn: PutFn, len: usize) {
        self.put_u16(tag);
        self.put_u16(len as u16);
        put_fn(self);
    }

    /// value len + 4
    fn tlv_bytes(&mut self, tag: u16, value: Bytes) {
        let len = value.len();
        self.put_u16(tag);
        self.put_u16(len as u16);
        self.extend(value);
    }

    /// str len + 2
    fn str_lv(&mut self, value: &str) {
        self.put_u16(value.len() as u16);
        self.put(value.as_bytes());
    }

    /// bytes len + 2
    fn bytes_lv(&mut self, value: Bytes) {
        self.put_u16(value.len() as u16);
        self.extend(value);
    }

    fn str_lv_limit_size(&mut self, value: &str, limit: usize) {
        if value.len() <= limit {
            self.str_lv(value);
        } else {
            self.str_lv(&value[..limit]);
        }
    }
}

pub trait TlvRead {
    /// get bytes len i32 - 4
    fn get_bytes(&mut self) -> Bytes;
    /// get bytes len u16
    fn get_bytes_short(&mut self) -> Bytes;
    /// get string len i32 - 4
    fn get_string(&mut self) -> String;
    /// get string len u16
    fn get_string_short(&mut self) -> String;
    /// get tlv map
    fn get_tlv_map(&mut self, tag_size: usize) -> HashMap<u16, Bytes>;
}

impl TlvRead for Bytes {
    fn get_bytes(&mut self) -> Bytes {
        let len = self.get_i32() as usize - 4;
        self.split_to(len)
    }

    fn get_bytes_short(&mut self) -> Bytes {
        let len = self.get_u16() as usize;
        self.split_to(len)
    }

    fn get_string(&mut self) -> String {
        let len = self.get_i32() as usize - 4;
        let strb = self.split_to(len);
        String::from_utf8(strb.to_vec()).unwrap()
    }

    fn get_string_short(&mut self) -> String {
        let len = self.get_u16() as usize;
        let strb = self.split_to(len);
        String::from_utf8(strb.to_vec()).unwrap()
    }

    fn get_tlv_map(&mut self, tag_size: usize) -> HashMap<u16, Bytes> {
        let mut map = HashMap::new();
        loop {
            if self.remaining() < tag_size {
                return map;
            }
            let k: u16;
            match tag_size {
                1 => k = self.get_u8() as u16,
                2 => k = self.get_u16(),
                4 => k = self.get_i32() as u16,
                _ => panic!("tag_size out of size"),
            }
            if k == 255 {
                return map;
            }
            let len = self.get_u16();
            map.insert(k, self.split_to(len as usize));
        }
    }
}

pub fn guid_flag() -> u32 {
    let mut flag = 0u32;
    flag |= 1 << 24 & 0xff000000;
    flag |= 0 << 8 & 0xff00;
    flag
}

#[test]
fn test_guid_flag() {
    println!("{}", guid_flag())
}

pub fn timestamp() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32
}

#[test]
fn test_time() {
    println!("{}", timestamp())
}
