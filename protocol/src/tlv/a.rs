use super::*;
use rand::prelude::*;

pub trait TlvWriteA {
    /// t1 capacity 24
    fn t1(&mut self, uin: u32, ip: std::net::Ipv4Addr);
    /// t1b capacity 34
    fn t1b(
        &mut self,
        micro: u32,
        version: u32,
        size: u32,
        margin: u32,
        dpi: u32,
        ec_level: u32,
        hint: u32,
    );
    /// t1d capacity 18
    fn t1d(&mut self, misc_bitmap: u32);
    /// t1f capacity unknow (test unpassed)
    fn t1f(
        &mut self,
        is_root: bool,
        os_name: &str,
        os_version: &str,
        sim_operator_name: &str,
        apn: &str,
        network_type: u16,
    );
    fn t2(&mut self, result: &str, sign: &[u8]);
    /// t8 capacity 12
    fn t8(&mut self, local_id: u32);
    /// t10a len arr.len() + 4
    fn t10a(&mut self, arr: Bytes);
    /// t16e len build_model.len() + 4
    fn t16e(&mut self, build_model: &str);
    /// t18 len 22
    fn t18(&mut self, app_id: u32, uin: u32);
    /// t52d len dev_info.len() + 4
    fn t52d(&mut self, dev_info: Bytes);
}

impl TlvWriteA for BytesMut {
    fn t1(&mut self, uin: u32, ip: std::net::Ipv4Addr) {
        let mut rng = rand::thread_rng();
        let value = move |bytes_mut: &mut BytesMut| {
            bytes_mut.put_u16(1);
            bytes_mut.put_u32(rng.gen::<u32>());
            bytes_mut.put_u32(uin);
            bytes_mut.put_u32(timestamp()); //todo unpass
            bytes_mut.extend(ip.octets());
            bytes_mut.put_u16(0);
        };
        self.tlv(1, Box::new(value), 20);
    }

    fn t1b(
        &mut self,
        micro: u32,
        version: u32,
        size: u32,
        margin: u32,
        dpi: u32,
        ec_level: u32,
        hint: u32,
    ) {
        self.tlv(
            0x1b,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u32(micro);
                bytes_mut.put_u32(version);
                bytes_mut.put_u32(size);
                bytes_mut.put_u32(margin);
                bytes_mut.put_u32(dpi);
                bytes_mut.put_u32(ec_level);
                bytes_mut.put_u32(hint);
                bytes_mut.put_u16(0);
            }),
            30,
        )
    }

    fn t1d(&mut self, misc_bitmap: u32) {
        self.tlv(
            0x1d,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u8(1);
                bytes_mut.put_u32(misc_bitmap);
                bytes_mut.put_u32(0);
                bytes_mut.put_u8(0);
                bytes_mut.put_u32(0);
            }),
            14,
        )
    }

    fn t1f(
        &mut self,
        is_root: bool,
        os_name: &str,
        os_version: &str,
        sim_operator_name: &str,
        apn: &str,
        network_type: u16,
    ) {
        self.tlv_bytes(0x1f, {
            let mut bytes_mut = BytesMut::with_capacity(
                17 + os_name.len() + os_version.len() + sim_operator_name.len() + apn.len(),
            );
            bytes_mut.put_u8(if is_root { 1 } else { 0 });
            bytes_mut.str_lv(os_name);
            bytes_mut.str_lv(os_version);
            bytes_mut.put_u16(network_type);
            bytes_mut.str_lv(sim_operator_name);
            bytes_mut.put_u16(0);
            bytes_mut.str_lv(apn);
            bytes_mut.freeze()
        });
    }

    fn t2(&mut self, result: &str, sign: &[u8]) {
        self.tlv_bytes(0x02, {
            let mut bytes_mut = BytesMut::with_capacity(6 + result.len() + sign.len());
            bytes_mut.put_u16(0);
            bytes_mut.str_lv(result);
            bytes_mut.bytes_lv(Bytes::copy_from_slice(sign));
            bytes_mut.freeze()
        });
    }

    fn t8(&mut self, local_id: u32) {
        self.tlv(
            0x8,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u16(0);
                bytes_mut.put_u32(local_id);
                bytes_mut.put_u16(0);
            }),
            8,
        )
    }

    fn t10a(&mut self, arr: Bytes) {
        self.tlv_bytes(0x10a, arr)
    }

    fn t16e(&mut self, build_model: &str) {
        self.tlv_bytes(0x16e, Bytes::copy_from_slice(build_model.as_bytes()))
    }

    fn t18(&mut self, app_id: u32, uin: u32) {
        self.tlv(
            0x18,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u16(1);
                bytes_mut.put_u32(1536);
                bytes_mut.put_u32(app_id);
                bytes_mut.put_u32(0);
                bytes_mut.put_u32(uin);
                bytes_mut.put_u16(0);
                bytes_mut.put_u16(0);
            }),
            22,
        )
    }

    fn t52d(&mut self, dev_info: Bytes) {
        self.tlv_bytes(0x52d, dev_info)
    }
}

#[test]
fn t1_test() {
    let mut bytes_mut = BytesMut::with_capacity(24);
    bytes_mut.t1(444888, std::net::Ipv4Addr::new(127, 0, 0, 1));
    println!("{:x}-{}", bytes_mut, bytes_mut.len());
    // 000100140001c52f48410006c9d86159ab3b7f0000010000-24
}

#[test]
fn t1b_test() {
    let mut bytes_mut = BytesMut::with_capacity(34);
    bytes_mut.t1b(0, 0, 0, 0, 0, 0, 0);
    print!("{:x}-{}", bytes_mut, bytes_mut.len());
    // 001b001e000000000000000000000000000000000000000000000000000000000000-34
    // 001b001e000000000000000000000000000000000000000000000000000000000000
}

#[test]
fn t1d_test() {
    let mut bytes_mut = BytesMut::with_capacity(18);
    bytes_mut.t1d(0);
    print!("{:x}-{}", bytes_mut, bytes_mut.len());
    // 001d000e0100000000000000000000000000-18
    // 001d000e0100000000000000000000000000
}

#[test]
fn t1f_test() {
    let mut bytes_mut = BytesMut::with_capacity(1024);
    bytes_mut.t1f(true, "A", "10", "T-Mobile", "wifi", 0);
    print!("{:x}-{}", bytes_mut, bytes_mut.len());
    // 001f001c 010001410002313000000008542d4d6f62696c65000  0000477696669-32
    // 001f001d 010001410002313000000008542d4d6f62696c65000100000477696669
}
