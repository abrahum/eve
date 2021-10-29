use bytes::{BufMut, Bytes, BytesMut};
use protocol::tlv::*;

impl crate::Client {
    pub(crate) async fn build_change_sig_pkt(&self) -> super::OutComePacket {
        let device_info = &self.device_info;
        let seq = self.next_seq();
        let cache_info = &self.cache_info.read().await;
        let sig_info = &cache_info.sig_info;
        let uin = self.uin.load(std::sync::atomic::Ordering::SeqCst) as u32;

        let mut bytes_mut = BytesMut::with_capacity(10240);
        bytes_mut.put_u16(11);
        bytes_mut.put_u16(17);
        bytes_mut.t100(
            self.version_info.sso_version,
            100,
            self.version_info.main_sigmap,
        );
        bytes_mut.t10a(sig_info.tgt.clone());
        bytes_mut.t116(self.version_info.misc_bitmap, self.version_info.sub_sigmap);
        bytes_mut.t108(&device_info.imei);
        let h = md5::compute(&sig_info.d2key).0;
        bytes_mut.t144(
            &device_info.android_id,
            device_info.gen_protobuf(),
            &device_info.os_type,
            &device_info.version.release,
            &device_info.sim_info,
            &device_info.apn,
            false,
            true,
            false,
            protocol::tlv::guid_flag(),
            &device_info.model,
            &device_info.guid,
            &device_info.brand,
            &h,
        );
        bytes_mut.t143(sig_info.d2.clone());
        bytes_mut.t142(&self.version_info.apk_id);
        bytes_mut.t154(seq);
        bytes_mut.t18(16, uin);
        bytes_mut.t141(&device_info.sim_info, &device_info.apn);
        bytes_mut.t8(2052);
        bytes_mut.t147(
            16,
            &self.version_info.sort_version_name,
            &self.version_info.apk_sign,
        );
        bytes_mut.t177(self.version_info.build_time, &self.version_info.sdk_version);
        bytes_mut.t187(&device_info.mac_address);
        bytes_mut.t188(&device_info.android_id);
        bytes_mut.t194(&device_info.imsi_md5);
        bytes_mut.t511(vec![
            "tenpay.com",
            "openmobile.qq.com",
            "docs.qq.com",
            "connect.qq.com",
            "qzone.qq.com",
            "vip.qq.com",
            "qun.qq.com",
            "game.qq.com",
            "qqweb.qq.com",
            "office.qq.com",
            "ti.qq.com",
            "mail.qq.com",
            "qzone.com",
            "mma.qq.com",
        ]);
        let data = protocol::build_oicq_request_pkt(
            uin,
            0x0810,
            &self.ecdh,
            &self.rand_key,
            bytes_mut.freeze(),
        );
        let sso = protocol::build_sso_pkt(
            seq,
            self.version_info.app_id,
            self.version_info.sub_app_id,
            "wtlogin.exchange_emp",
            &self.device_info.imei,
            &sig_info.tgt,
            &self.outgoing_pkt_session_id.read().await,
            data,
            &cache_info.ksid,
        );
        let pkt = protocol::build_lgi_pkt(
            self.uin.load(std::sync::atomic::Ordering::SeqCst),
            2,
            &[0u8; 16],
            sso,
            &Bytes::default(),
        );

        super::OutComePacket { bytes: pkt, seq }
    }
}
