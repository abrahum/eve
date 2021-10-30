use bytes::{BufMut, Bytes, BytesMut};
use protocol::tlv::*;

impl crate::Client {
    pub(crate) async fn build_login_pkt(&self) -> super::OutComePacket {
        let device_info = &self.device_info;
        let uin = self.uin.load(std::sync::atomic::Ordering::SeqCst) as u32;
        let seq = self.next_seq();
        let mut bytes_mut = BytesMut::with_capacity(10240); // todo ckeck size
        bytes_mut.put_u16(9);
        // todo allowslider
        bytes_mut.put_u16(if self.allow_slide { 0x17 } else { 0x16 });
        bytes_mut.t18(16, uin);
        bytes_mut.t1(uin, device_info.ip_address);
        bytes_mut.t106(
            uin,
            0,
            self.version_info.app_id,
            self.version_info.sso_version,
            &self.password_md5,
            true,
            &device_info.guid,
            &device_info.tgtgt_key,
            0,
        );
        bytes_mut.t116(self.version_info.misc_bitmap, self.version_info.sub_sigmap);
        bytes_mut.t100(
            self.version_info.sso_version,
            self.version_info.sub_app_id,
            self.version_info.main_sigmap,
        );
        bytes_mut.t107(0);
        bytes_mut.t142(&self.version_info.apk_id);
        bytes_mut.t144(
            &device_info.imei,
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
            &device_info.tgtgt_key,
        );
        bytes_mut.t145(&device_info.guid);
        bytes_mut.t147(
            16,
            &self.version_info.sort_version_name,
            &self.version_info.apk_sign,
        );
        bytes_mut.t154(seq);
        bytes_mut.t141(&device_info.sim_info, &device_info.apn);
        bytes_mut.t8(2052);
        bytes_mut.t511(vec![
            "tenpay.com",
            "openmobile.qq.com",
            "docs.qq.com",
            "connect.qq.com",
            "qzone.qq.com",
            "vip.qq.com",
            "gamecenter.qq.com",
            "qun.qq.com",
            "game.qq.com",
            "qqweb.qq.com",
            "office.qq.com",
            "ti.qq.com",
            "mail.qq.com",
            "mma.qq.com",
        ]);
        bytes_mut.t187(&device_info.mac_address);
        bytes_mut.t188(&device_info.android_id);
        bytes_mut.t194(&device_info.imsi_md5);
        if self.allow_slide {
            bytes_mut.t191(Bytes::from(vec![0x82u8]));
        }
        bytes_mut.t202(&device_info.wifi_bssid, &device_info.wifi_ssid);
        bytes_mut.t177(self.version_info.build_time, &self.version_info.sdk_version);
        bytes_mut.t516();
        bytes_mut.t521(0);
        bytes_mut.t525({
            let mut b = BytesMut::with_capacity(6);
            b.t536(Bytes::copy_from_slice(&[0x01, 0x00][..]));
            b.freeze()
        });
        let b = bytes_mut.freeze();
        println!("b:{:x}", b);

        let datapkt = protocol::build_oicq_request_pkt(uin, 0x0810, &self.ecdh, &self.rand_key, b);
        let sso = protocol::build_sso_pkt(
            seq,
            self.version_info.app_id,
            self.version_info.sub_app_id,
            "wtlogin.login",
            &device_info.imei,
            &Bytes::default(),
            &self.outgoing_pkt_session_id.read().await,
            datapkt,
            &self.cache_info.read().await.ksid,
        );
        let pkt = protocol::build_lgi_pkt(uin as i64, 2, &[0; 16], sso, &Bytes::default());

        super::OutComePacket { bytes: pkt, seq }
    }
}

#[tokio::test]
async fn test() {
    use crate::{Client, Config, DeviceInfo, Password};
    let config = Config::load_or_new();
    let (mut cli, _) = Client::new(
        config.clients[0].uid,
        Password::from_str(&config.clients[0].password),
        false,
        DeviceInfo::load_or_new(),
    )
    .await;
    cli.allow_slide = true;
    let pkt = cli.build_login_pkt().await;
}

#[test]
fn just_teso() {
    let data = crate::DeviceInfo::default();
    let b = data.gen_protobuf();
    println!("{:x}-{}", b, b.len())
}
