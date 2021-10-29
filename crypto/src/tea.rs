use bytes::{Buf, BufMut, Bytes, BytesMut};
use rand::Rng;

static SUM_TABLE: [u32; 0x10] = [
    0x9e3779b9, 0x3c6ef372, 0xdaa66d2b, 0x78dde6e4, 0x1715609d, 0xb54cda56, 0x5384540f, 0xf1bbcdc8,
    0x8ff34781, 0x2e2ac13a, 0xcc623af3, 0x6a99b4ac, 0x08d12e65, 0xa708a81e, 0x454021d7, 0xe3779b90,
];

pub struct Tea([u32; 4]);

impl Tea {
    pub fn new(mut bytes: Bytes) -> Self {
        // test passed
        Tea([
            bytes.get_u32(),
            bytes.get_u32(),
            bytes.get_u32(),
            bytes.get_u32(),
        ])
    }

    pub fn new_with_u32(key: [u32; 4]) -> Self {
        Tea([key[0], key[1], key[2], key[3]])
    }

    pub fn new_with_ref(key: &[u8]) -> Self {
        let bytes = Bytes::copy_from_slice(key);
        Self::new(bytes)
    }

    // max len + 17
    pub fn encrypt(&self, src: Bytes) -> Bytes {
        let mut rng = rand::thread_rng();

        let mut len = src.len();
        let fill = 10 - (len + 1) % 8; // 3-10
        let mut temp1 = [0u8; 8];
        let mut temp2 = [0u8; 8];
        let mut dst = BytesMut::with_capacity(fill + len + 7);

        let pad_len: u8 = ((fill - 3) as u8) | 0xF8;
        dst.put_u8(pad_len);
        for _ in 0..fill - 1 {
            dst.put_u8(rng.gen());
        }
        let mut in_ = 0usize;

        if fill < 8 {
            in_ = 8 - fill;
            dst.extend(&src[0..in_]);
        }

        (&dst[0..8]).copy_to_slice(&mut temp2);
        self.encode(&temp2, &mut dst[0..8]);
        let mut out = 8usize;

        if fill > 8 {
            dst.extend(&src[0..16 - fill]);
            xor_slice(
                &dst[8..16].to_owned(),
                &dst[0..8].to_owned(),
                &mut dst[8..16],
            );
            (&dst[8..16]).copy_to_slice(&mut temp1);
            self.encode(&temp1, &mut dst[8..16]);
            xor_slice(&dst[8..16].to_owned(), &temp2, &mut dst[8..16]);
            temp2 = temp1;
            in_ = 16 - fill;
            out = 16;
        }

        len = if len > 7 { len - 8 } else { 0 };
        while in_ < len {
            dst.extend([0u8; 8]);
            xor_slice(
                &src[in_..in_ + 8].to_owned(),
                &dst[out - 8..out].to_owned(),
                &mut dst[out..out + 8],
            );
            (&dst[out..out + 8]).copy_to_slice(&mut temp1);
            self.encode(&temp1, &mut dst[out..out + 8]);
            xor_slice(
                &dst[out..out + 8].to_owned(),
                &temp2,
                &mut dst[out..out + 8],
            );
            temp2 = temp1;
            in_ += 8;
            out += 8;
        }

        let mut temp3 = [0u8; 8];
        for i in in_..src.len() {
            temp3[i - in_] = src[i];
        }
        dst.extend(temp3);
        xor_slice(
            &temp3,
            &dst[out - 8..out].to_owned(),
            &mut dst[out..out + 8],
        );
        self.encode(&dst[out..out + 8].to_owned(), &mut dst[out..out + 8]);
        xor_slice(
            &dst[out..out + 8].to_owned(),
            &temp2,
            &mut dst[out..out + 8],
        );

        dst.freeze()
    }

    pub fn decrypt(&self, data: Bytes) -> Result<Bytes, &'static str> {
        let len = data.len();
        if len < 16 || len % 8 != 0 {
            return Err("Decrypt fail data len illegal");
        }

        let mut dst = BytesMut::with_capacity(len);
        dst.extend(data.clone());
        self.decode(&dst[0..8].to_owned(), &mut dst[0..8]);
        let mut temp = [0u8; 8];
        (&dst[0..8]).copy_to_slice(&mut temp);
        for i in 1..(len / 8) {
            let index = i * 8;
            xor_slice(
                &dst[index..index + 8].to_owned(),
                &temp,
                &mut dst[index..index + 8],
            );
            self.decode(
                &dst[index..index + 8].to_owned(),
                &mut dst[index..index + 8],
            );
            xor_slice(
                &dst[index..index + 8].to_owned(),
                &data[index - 8..index].to_owned(),
                &mut dst[index..index + 8],
            );
            xor_slice(
                &dst[index..index + 8].to_owned(),
                &data[index - 8..index].to_owned(),
                &mut temp,
            );
        }

        let s = (dst[0] & 7) + 3;
        Ok(dst.freeze().slice(s as usize..(len - 7)))
    }

    #[inline]
    fn encode(&self, src: &[u8], dst: &mut [u8]) {
        // test passed
        let (mut v0, mut v1) = unpack(src);
        for sum in SUM_TABLE {
            v0 = v0.wrapping_add(
                self.0[0].wrapping_add(v1 << 4)
                    ^ v1.wrapping_add(sum)
                    ^ self.0[1].wrapping_add(v1 >> 5),
            );
            v1 = v1.wrapping_add(
                self.0[2].wrapping_add(v0 << 4)
                    ^ v0.wrapping_add(sum)
                    ^ self.0[3].wrapping_add(v0 >> 5),
            );
        }
        pack(v0, v1, dst);
    }

    fn decode(&self, src: &[u8], dst: &mut [u8]) {
        // test passed
        let (mut v0, mut v1) = unpack(src);
        for i in 0..16 {
            v1 = v1.wrapping_sub(
                self.0[2].wrapping_add(v0 << 4)
                    ^ (v0.wrapping_add(SUM_TABLE[15 - i]))
                    ^ self.0[3].wrapping_add(v0 >> 5),
            );
            v0 = v0.wrapping_sub(
                self.0[0].wrapping_add(v1 << 4)
                    ^ v1.wrapping_add(SUM_TABLE[15 - i])
                    ^ self.0[1].wrapping_add(v1 >> 5),
            );
        }
        pack(v0, v1, dst);
    }
}

fn unpack(i: &[u8]) -> (u32, u32) {
    // test passed
    let mut bytes_mut = BytesMut::with_capacity(8);
    for index in 0..8 {
        bytes_mut.put_u8(i[index])
    }
    (bytes_mut.get_u32(), bytes_mut.get_u32())
}

fn pack(v0: u32, v1: u32, dst: &mut [u8]) {
    // test passed
    let mut bytes_mut = BytesMut::with_capacity(8);
    bytes_mut.put_u32(v0);
    bytes_mut.put_u32(v1);
    bytes_mut.copy_to_slice(dst);
}

fn xor_slice<I, L>(scr0: &I, scr1: &L, dst: &mut [u8])
where
    I: std::ops::Index<usize, Output = u8> + AsRef<[u8]>,
    L: std::ops::Index<usize, Output = u8> + AsRef<[u8]>,
{
    // test passed
    for i in 0..8usize {
        dst[i] = scr0[i] ^ scr1[i]
    }
}

#[test]
fn test() {
    // use std::iter::FromIterator;
    static SAMPLES:[(&'static [u8], &'static str, &'static str);5] = [
        (b"0123456789ABCDEF", "MiraiGO Here", "b7b2e52af7f5b1fbf37fc3d5546ac7569aecd01bbacf09bf"),
        (b"0123456789ABCDEF", "LXY Testing~", "9d0ab85aa14f5434ee83cd2a6b28bf306263cdf88e01264c"),
        (b"0123456789ABCDEF", "s", "528e8b5c48300b548e94262736ebb8b7"),
        (b"0123456789ABCDEF", "long long long long long long long", "95715fab6efbd0fd4b76dbc80bd633ebe805849dbc242053b06557f87e748effd9f613f782749fb9fdfa3f45c0c26161"),
        (b"LXY1226    Mrs4s", "LXY Testing~", "ab20caa63f3a6503a84f3cb28f9e26b6c18c051e995d1721")
    ];

    for sample in &SAMPLES {
        let tea = Tea::new(Bytes::from_static(sample.0));
        let byte_data = Bytes::from(hex::decode(sample.2).unwrap());
        let de = tea.decrypt(byte_data).unwrap();
        let de_string = String::from_utf8(de.to_vec()).unwrap();
        assert_eq!(sample.1, &de_string);

        let str_data = Bytes::from(sample.1.as_bytes());
        let en = tea.encrypt(str_data);
        let de = tea.decrypt(en).unwrap();
        let de_string = String::from_utf8(de.to_vec()).unwrap();
        assert_eq!(sample.1, &de_string);
    }
}

#[allow(dead_code)]
fn gen_tea() -> Tea {
    let mut rng = rand::thread_rng();
    let mut key = [0u32; 4];
    for i in 0..key.len() {
        key[i] = rng.gen();
    }
    Tea::new_with_u32(key)
}

#[test]
fn rand_test() {
    let mut rng = rand::thread_rng();
    for i in 1..0xff {
        let tea = gen_tea();
        let mut data = BytesMut::with_capacity(i);
        for _ in 0..i {
            data.put_u8(rng.gen());
        }
        let data = data.freeze();

        let en = tea.encrypt(data.clone());
        let de = tea.decrypt(en).unwrap();
        assert_eq!(de, data);
    }
}
