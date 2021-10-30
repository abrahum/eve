use super::*;

pub trait TlvWriteE {
    fn t511(&mut self, domains: Vec<&'static str>);
    // len 8
    fn t516(&mut self);
    // len 10
    fn t521(&mut self, i: u32);
    // len t536.len() + 6
    fn t525(&mut self, t536: Bytes);
    // len data.len() + 4
    fn t536(&mut self, data: Bytes);
}

impl TlvWriteE for BytesMut {
    fn t511(&mut self, domains: Vec<&'static str>) {
        self.tlv_bytes(0x511, {
            let mut bytes_mut = BytesMut::with_capacity(1024);
            bytes_mut.put_u16(domains.len() as u16);
            for domain in domains {
                bytes_mut.put_u8(0x01);
                bytes_mut.str_lv(domain);
            }
            bytes_mut.freeze()
        });
    }

    fn t516(&mut self) {
        self.tlv(0x516, Box::new(|bytes_mut| bytes_mut.put_u32(0)), 4)
    }

    fn t521(&mut self, i: u32) {
        self.tlv(
            0x521,
            Box::new(move |b| {
                b.put_u32(i);
                b.put_u16(0)
            }),
            6,
        )
    }

    fn t525(&mut self, t536: Bytes) {
        let len = t536.len() + 2;
        self.tlv(
            0x525,
            Box::new(move |b| {
                b.put_u16(1);
                b.extend(t536);
            }),
            len,
        )
    }

    fn t536(&mut self, data: Bytes) {
        self.tlv_bytes(0x536, data)
    }
}
