use super::*;

pub trait TlvWriteB {
    /// t100 capacity 26
    fn t100(&mut self, sso_version: u32, protocol: u32, main_sigmap: u32);
    /// t104 data.len() + 6
    fn t104(&mut self, data: Bytes);
    fn t106(
        &mut self,
        uin: u32,
        salt: u32,
        app_id: u32,
        sso_ver: u32,
        password_md5: &[u8],
        is_guid_available: bool,
        guid: &[u8],
        tgtgt_key: &[u8],
        wtf: u32,
    );
    /// len 10
    fn t107(&mut self, pic_type: u16);
    /// t108 len imei.len() + 6
    fn t108(&mut self, imei: &str);
    /// t109 len 22
    fn t109(&mut self, android_id: &str);
    /// t116 len 18
    fn t116(&mut self, misc_bitmap: u32, sub_sigmap: u32);
    /// t124
    fn t124(&mut self, os_type: &str, os_version: &str, sim_info: &str, apn: &str);
    /// t128
    fn t128(
        &mut self,
        is_guid_from_file_null: bool,
        is_guid_available: bool,
        is_guid_changed: bool,
        guid_flag: u32,
        build_model: &str,
        guid: &[u8],
        build_brand: &str,
    );
    /// t141 len sim_info.len() + apn.len() + 12
    fn t141(&mut self, sim_info: &str, apn: &str);
    /// t142
    fn t142(&mut self, apk_id: &str);
    /// t143 arr.len() + 4
    fn t143(&mut self, arr: Bytes);
    /// t144
    fn t144(
        &mut self,
        imei: &str,
        dev_info: Bytes,
        os_type: &str,
        os_version: &str,
        sim_info: &str,
        apn: &str,
        is_guid_from_file_null: bool,
        is_guid_available: bool,
        is_guid_changed: bool,
        guid_flag: u32,
        build_model: &str,
        guid: &[u8],
        build_brand: &str,
        tgtgt_key: &[u8],
    );
    /// guid len + 4
    fn t145(&mut self, guid: &[u8]);
    /// t147
    fn t147(&mut self, app_id: u32, apk_version_name: &str, apk_signature_md5: &[u8]);
    /// t154 len 8
    fn t154(&mut self, seq: u16);
    /// t177
    fn t177(&mut self, build_time: u32, sdk_version: &str);
    /// t187 len 22
    fn t187(&mut self, mac_address: &str);
    /// t188 len 22
    fn t188(&mut self, android_id: &str);
    /// k.len() + 4
    fn t191(&mut self, k: Bytes);
    /// t194 len 22
    fn t194(&mut self, imsi_md5: &[u8]);
}

impl TlvWriteB for BytesMut {
    fn t100(&mut self, sso_version: u32, protocol: u32, main_sigmap: u32) {
        self.tlv(
            0x100,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u16(1);
                bytes_mut.put_u32(sso_version);
                bytes_mut.put_u32(16);
                bytes_mut.put_u32(protocol);
                bytes_mut.put_u32(0);
                bytes_mut.put_u32(main_sigmap);
            }),
            22,
        )
    }

    fn t104(&mut self, data: Bytes) {
        self.tlv_bytes(0x104, {
            let mut bytes_mut = BytesMut::with_capacity(data.len() + 2);
            bytes_mut.bytes_lv(data);
            bytes_mut.freeze()
        })
    }

    fn t106(
        &mut self,
        uin: u32,
        salt: u32,
        app_id: u32,
        sso_ver: u32,
        password_md5: &[u8],
        is_guid_available: bool,
        guid: &[u8],
        tgtgt_key: &[u8],
        wtf: u32,
    ) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let body = {
            let mut bytes_mut = BytesMut::with_capacity(10240); // todo
            bytes_mut.put_u16(4);
            bytes_mut.put_u32(rng.gen());
            bytes_mut.put_u32(sso_ver);
            bytes_mut.put_u32(16); // app_id
            bytes_mut.put_u32(0); // app client version
            if uin == 0 {
                bytes_mut.put_u64(salt as u64);
            } else {
                bytes_mut.put_u64(uin as u64);
            }
            bytes_mut.put_u32(super::timestamp());
            bytes_mut.put_bytes(0x00, 4); // fake ip
            bytes_mut.put_u8(0x01);
            bytes_mut.extend(password_md5);
            bytes_mut.extend(tgtgt_key);
            bytes_mut.put_u32(wtf);
            bytes_mut.put_bool(is_guid_available);
            if guid.len() == 0 {
                for _ in 0..4 {
                    bytes_mut.put_u32(rng.gen());
                }
            } else {
                bytes_mut.extend(guid);
            }
            bytes_mut.put_u32(app_id);
            bytes_mut.put_u32(1); // password login
            bytes_mut.str_lv(&format!("{}", uin));
            bytes_mut.put_u16(0);

            bytes_mut.freeze()
        };

        let b = {
            if salt != 0 {
                salt.to_be_bytes()
            } else {
                uin.to_be_bytes()
            }
        };
        let key = md5::compute(password_md5.chain(&[0u8; 4][..]).chain(&b[..]).chunk()).0;
        let tea = crypto::tea::Tea::new_with_ref(&key);
        let value = tea.encrypt(body);
        self.tlv_bytes(0x106, value)
    }

    fn t107(&mut self, pic_type: u16) {
        self.tlv(
            0x107,
            Box::new(move |bytes_mut| {
                bytes_mut.put_u16(pic_type);
                bytes_mut.put_bytes(0x00, 4);
            }),
            6,
        )
    }

    fn t108(&mut self, imei: &str) {
        self.tlv_bytes(0x108, {
            let mut bytes_mut = BytesMut::with_capacity(imei.len() + 2);
            bytes_mut.str_lv(imei);
            bytes_mut.freeze()
        })
    }

    fn t109(&mut self, android_id: &str) {
        let h = md5::compute(android_id).0;
        self.tlv(
            0x109,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.bytes_lv(Bytes::copy_from_slice(&h[..]));
            }),
            18,
        )
    }

    fn t116(&mut self, misc_bitmap: u32, sub_sigmap: u32) {
        self.tlv(
            0x116,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u8(0x00);
                bytes_mut.put_u32(misc_bitmap);
                bytes_mut.put_u32(sub_sigmap);
                bytes_mut.put_u8(0x01);
                bytes_mut.put_u32(1600000226);
            }),
            14,
        )
    }

    fn t124(&mut self, os_type: &str, os_version: &str, sim_info: &str, apn: &str) {
        self.tlv_bytes(0x124, {
            let mut bytes_mut = BytesMut::with_capacity(92); // max
            bytes_mut.str_lv_limit_size(os_type, 16);
            bytes_mut.str_lv_limit_size(os_version, 16);
            bytes_mut.put_u16(2);
            bytes_mut.str_lv_limit_size(sim_info, 16);
            bytes_mut.str_lv_limit_size("", 16);
            bytes_mut.str_lv_limit_size(apn, 16);
            bytes_mut.freeze()
        })
    }

    fn t128(
        &mut self,
        is_guid_from_file_null: bool,
        is_guid_available: bool,
        is_guid_changed: bool,
        guid_flag: u32,
        build_model: &str,
        guid: &[u8],
        build_brand: &str,
    ) {
        let guid = Bytes::copy_from_slice(guid);
        fn put_bool(bytes_mut: &mut BytesMut, b: bool) {
            bytes_mut.put_u8(if b { 1 } else { 0 })
        }

        self.tlv_bytes(0x128, {
            let mut bytes_mut = BytesMut::with_capacity(77); // max
            bytes_mut.put_u16(0);
            put_bool(&mut bytes_mut, is_guid_from_file_null);
            put_bool(&mut bytes_mut, is_guid_available);
            put_bool(&mut bytes_mut, is_guid_changed);
            bytes_mut.put_u32(guid_flag);
            bytes_mut.str_lv_limit_size(build_model, 32);
            bytes_mut.bytes_lv(guid);
            bytes_mut.str_lv_limit_size(build_brand, 32);
            bytes_mut.freeze()
        })
    }

    fn t141(&mut self, sim_info: &str, apn: &str) {
        let len = sim_info.len() + apn.len() + 8;
        self.tlv_bytes(0x141, {
            let mut bytes_mut = BytesMut::with_capacity(len);
            bytes_mut.put_u16(1);
            bytes_mut.str_lv(sim_info);
            bytes_mut.put_u16(2);
            bytes_mut.str_lv(apn);
            bytes_mut.freeze()
        });
    }

    fn t142(&mut self, apk_id: &str) {
        self.tlv_bytes(0x142, {
            let mut bytes_mut = BytesMut::with_capacity(34);
            bytes_mut.put_u16(0);
            bytes_mut.str_lv_limit_size(apk_id, 32);
            bytes_mut.freeze()
        })
    }

    fn t143(&mut self, arr: Bytes) {
        self.tlv_bytes(0x143, arr)
    }

    fn t144(
        &mut self,
        imei: &str,
        dev_info: Bytes,
        os_type: &str,
        os_version: &str,
        sim_info: &str,
        apn: &str,
        is_guid_from_file_null: bool,
        is_guid_available: bool,
        is_guid_changed: bool,
        guid_flag: u32,
        build_model: &str,
        guid: &[u8],
        build_brand: &str,
        tgtgt_key: &[u8],
    ) {
        self.tlv_bytes(0x144, {
            let mut bytes_mut = BytesMut::with_capacity(1024); //todo
            let tea = crypto::Tea::new_with_ref(tgtgt_key);
            let mut bytes_mut_ = BytesMut::with_capacity(1024); //todo
            bytes_mut_.put_u16(5);
            bytes_mut_.t109(imei);
            bytes_mut_.t52d(dev_info);
            bytes_mut_.t124(os_type, os_version, sim_info, apn);
            bytes_mut_.t128(
                is_guid_from_file_null,
                is_guid_available,
                is_guid_changed,
                guid_flag,
                build_model,
                guid,
                build_brand,
            );
            bytes_mut_.t16e(build_model);

            bytes_mut.bytes_lv(tea.encrypt(bytes_mut_.freeze()));
            bytes_mut.freeze()
        })
    }

    fn t145(&mut self, guid: &[u8]) {
        self.tlv_bytes(0x145, Bytes::copy_from_slice(guid))
    }

    fn t147(&mut self, app_id: u32, apk_version_name: &str, apk_signature_md5: &[u8]) {
        self.tlv_bytes(0x147, {
            let mut bytes_mut = BytesMut::with_capacity(70);
            bytes_mut.put_u32(app_id);
            bytes_mut.str_lv_limit_size(apk_version_name, 32);
            bytes_mut.bytes_lv(Bytes::copy_from_slice(apk_signature_md5));
            bytes_mut.freeze()
        })
    }

    fn t154(&mut self, seq: u16) {
        self.tlv(
            0x154,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u32(seq as u32);
            }),
            4,
        )
    }

    fn t177(&mut self, build_time: u32, sdk_version: &str) {
        let sdk_version = Bytes::copy_from_slice(sdk_version.as_bytes());
        let len = 5 + 2 + sdk_version.len();

        self.tlv(
            0x177,
            Box::new(move |bytes_mut: &mut BytesMut| {
                bytes_mut.put_u8(0x01);
                bytes_mut.put_u32(build_time);
                bytes_mut.bytes_lv(sdk_version);
            }),
            len,
        )
    }

    fn t187(&mut self, mac_address: &str) {
        let h = Bytes::copy_from_slice(&md5::compute(mac_address).0);
        self.tlv(
            0x187,
            Box::new(move |bytes_mut: &mut BytesMut| bytes_mut.bytes_lv(h)),
            18,
        );
    }

    fn t188(&mut self, android_id: &str) {
        let h = Bytes::copy_from_slice(&md5::compute(android_id).0);
        self.tlv(
            0x187,
            Box::new(move |bytes_mut: &mut BytesMut| bytes_mut.bytes_lv(h)),
            18,
        );
    }

    fn t191(&mut self, k: Bytes) {
        self.tlv_bytes(0x191, k)
    }

    fn t194(&mut self, imsi_md5: &[u8]) {
        let value = Bytes::copy_from_slice(imsi_md5);
        self.tlv(
            0x188,
            Box::new(move |bytes_mut: &mut BytesMut| bytes_mut.bytes_lv(value)),
            18,
        );
    }
}
