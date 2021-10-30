use bytes::{BufMut, Bytes, BytesMut};
use crypto::Tea;

pub trait PacketBuild {
    /// value.len() + 4
    fn put_bytes_lvpkt(&mut self, offset: u32, value: &[u8]);
    /// value.len() + 4
    fn put_str_lvpkt(&mut self, value: &str);
}

impl PacketBuild for BytesMut {
    fn put_bytes_lvpkt(&mut self, offset: u32, value: &[u8]) {
        self.put_u32(value.len() as u32 + offset);
        self.extend(value);
    }

    fn put_str_lvpkt(&mut self, value: &str) {
        self.put_bytes_lvpkt(4, value.as_bytes())
    }
}

pub fn build_oicq_request_pkt(
    uin: u32,
    command_id: u16,
    encrypt: &crypto::ECDH,
    key: &[u8],
    data: Bytes,
) -> Bytes {
    let body = encrypt.encrypt(data, key);
    let mut bytes_mut = BytesMut::with_capacity(29 + body.len());
    bytes_mut.put_u8(0x02); // 1
    bytes_mut.put_u16(29 + body.len() as u16); // 2
    bytes_mut.put_u16(8001); // 2
    bytes_mut.put_u16(command_id); // 2
    bytes_mut.put_u16(1); // 2
    bytes_mut.put_u32(uin); // 4
    bytes_mut.put_u8(3); // 1
    bytes_mut.put_u8(encrypt.id()); // 1
    bytes_mut.put_u8(0); // 1
    bytes_mut.put_u32(2); // 4
    bytes_mut.put_u32(0); // 4
    bytes_mut.put_u32(0); // 4
    bytes_mut.extend(body);
    bytes_mut.put_u8(0x03); // 1

    bytes_mut.freeze()
}

pub fn build_sso_pkt(
    seq: u16,
    app_id: u32,
    sub_app_id: u32,
    command_name: &str,
    imei: &str,
    ext_data: &[u8],
    outgoing_pkt_session_id: &[u8],
    body: Bytes,
    ksid: &[u8],
) -> Bytes {
    let len_without_body = command_name.len() + ext_data.len();
    let len_all = len_without_body + body.len();
    let mut bytes_mut = BytesMut::with_capacity(len_all + 58);

    bytes_mut.put_bytes_lvpkt(4, &{
        let mut b = BytesMut::with_capacity(len_without_body + 50);
        b.put_u32(seq as u32); // 4
        b.put_u32(app_id); // 4
        b.put_u32(sub_app_id); // 4
        b.extend([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
        ]); // 12
        let ext_data_len = ext_data.len();
        if ext_data_len == 0 || ext_data_len == 4 {
            b.put_u32(0x04)
        } else {
            b.put_u32(ext_data_len as u32 + 4);
            b.extend(ext_data);
        } // 4
        b.put_str_lvpkt(command_name); // 4
        b.put_bytes_lvpkt(4, outgoing_pkt_session_id); // 4
        b.put_str_lvpkt(imei); // 4
        b.put_u32(0x04); // 4
        b.put_u16(ksid.len() as u16 + 2); // 2
        b.extend(ksid);
        b.put_u32(0x04); // 4
        b.freeze()
    }); // 4

    bytes_mut.put_bytes_lvpkt(4, &body); // 4

    bytes_mut.freeze()
}

pub fn build_lgi_pkt(uin: i64, body_type: u8, key: &[u8], body: Bytes, extra_data: &[u8]) -> Bytes {
    let mut bytes_mut = BytesMut::with_capacity(35 + body.len() + extra_data.len());
    bytes_mut.put_bytes_lvpkt(4, &{
        let mut b = BytesMut::with_capacity(31 + body.len() + extra_data.len());
        b.put_u32(0x00_00_00_0a); // +4
        b.put_u8(body_type); // +1
        b.put_bytes_lvpkt(4, extra_data); // +4
        b.put_u8(0x00); // +1
        b.put_str_lvpkt(&format!("{}", uin)); // +4
        if key.len() == 0 {
            b.extend(body);
        } else {
            let tea = Tea::new_with_ref(key);
            b.extend(tea.encrypt(body));
        } // +17
        b.freeze()
    }); // +4
    bytes_mut.freeze()
}

#[test]
fn lgi_pkt_test() {
    // pass?
    let key = Bytes::from_static(b"0123456789ABCDEF");
    let extra_data = key.clone();
    let lgi = build_lgi_pkt(000, 1, &[], key, &extra_data);
    println!("{:x}-{}", lgi, lgi.len())
    // 000000330000000a01000000143031323334353637383941424344454600000000053030313233343536373839414243444546-51
    // 000000330000000a01000000143031323334353637383941424344454600000000053030313233343536373839414243444546
}

#[test]
fn sso_pkt_test() {
    // pass
    let pkt = build_sso_pkt(
        0,
        1,
        2,
        "name",
        "imei",
        &[00],
        &[01],
        Bytes::copy_from_slice(&[02]),
        &[03],
    );
    println!("{:x}", pkt)
    //000000410000000000000001000000020100000000000000000001000000000500000000086e616d65000000050100000008696d656900000004000303000000040000000502
    //000000410000000000000001000000020100000000000000000001000000000500000000086e616d65000000050100000008696d656900000004000303000000040000000502
}

pub fn build_uni_pkt(
    uin: i64,
    seq: u16,
    command_name: &str,
    encrypt_type: u8,
    session_id: Bytes,
    extra_data: Bytes,
    key: Bytes,
    body: Bytes,
) -> Bytes {
    let len_without_body = command_name.len() + session_id.len() + extra_data.len();
    let len_all = len_without_body + body.len();

    let mut uni = BytesMut::with_capacity(20 + len_all);
    uni.put_bytes_lvpkt(4, &{
        let mut bytes_mut = BytesMut::with_capacity(12 + len_without_body);
        bytes_mut.put_str_lvpkt(command_name); // +4
        bytes_mut.put_u32(8); // +4
        bytes_mut.extend(session_id);
        bytes_mut.put_u32(extra_data.len() as u32 + 0x04); // +4
        bytes_mut.extend(extra_data);
        bytes_mut.freeze()
    }); // +4
    uni.put_bytes_lvpkt(4, &body); // +4

    let mut bytes_mut = BytesMut::with_capacity(55 + len_all);
    bytes_mut.put_bytes_lvpkt(4, &{
        let mut b = BytesMut::with_capacity(51 + len_all);
        b.put_u32(0x0b); // +4
        b.put_u8(encrypt_type); // +1
        b.put_u32(seq as u32); // +4
        b.put_u8(0); // +1
        b.put_str_lvpkt(&format!("{}", uin)); // +4
        let tea = Tea::new(key);
        b.extend(tea.encrypt(uni.freeze())); // +17
        b.freeze()
    }); // +4
    bytes_mut.freeze()
}

#[test]
fn oicp_pkt_test() {
    let ecdh = crypto::ECDH::test_new();
    let pkt = build_oicq_request_pkt(
        101,
        4,
        &ecdh,
        &[0u8; 16],
        Bytes::copy_from_slice(&[0, 1, 2, 3]),
    );
    println!("{:x}", pkt)
    // 02007a1f410004000100000065038700000000020000000000000000 0201 00000000000000000000000000000000 0131 0001 004104 130a4e905a8a4d744a79d3201dc4c115258666f00ee472d1ba5edb16d584f3831ab27742f1a70f11f45e93bd7dc96bcc0cbd629be5f725dc6d1b0335a6b87457 0001020303
    // 02007a1f410004000100000065038700000000020000000000000000 0201 00000000000000000000000000000000 0131 0001 004104 0331863a42fffab7cf8ff0fa3946425de9fb676837c556b7d15f83f0a4a51992e05e9382ee7a41d2ae324a0e3d912816464c16c4286973c2a1ac2c4c19cd57ef 0001020303
}

#[test]
fn uni_pkt_test() {
    // pass?
    let key = Bytes::from_static(b"0123456789ABCDEF");
    let body = key.clone();
    let extra_data = key.clone();
    let session_id = key.clone();
    let uni = build_uni_pkt(000, 1, "command_name", 1, session_id, extra_data, key, body);
    println!("{:x}-{}", uni, uni.len())
    // 000000730000000b0100000001000000000530 d271d423444eda8b8af5aaa5fb5f4ccafc23f92e5bc8254ff3bfabe42850a9894d967ded8a2d49b48c631213fca21d18bbd210f1c5997b6e8d69dd87821cc862e76f309f322d0a39cee5b3b725fc482fd5fbf180931ef99bf52f849b489f0682-115
    // 000000730000000b0100000001000000000530 66ffcdd3c07240a12a87439766a450fe8df783beed993f5e52df83da5d9bfd53d2bf782941f124fb99c66b05bdb6ed208fc4d716c2a3ce6523ac81bab6209ad11ed8b8a0d1b8cf5c950e01fa1af8d24a8ac188df0a64f5ad7939094eebb99e8b
}
