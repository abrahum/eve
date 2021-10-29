use bytes::{BufMut, Bytes, BytesMut};
use protocol::tlv::*;

impl crate::Client {
    pub(crate) async fn build_captcha_pkt(&self, cap: String, sig: Bytes) -> super::OutComePacket {
        let seq = self.next_seq();
        let mut bytes_mut = BytesMut::with_capacity(10240);
        bytes_mut.put_u16(2);
        bytes_mut.put_u16(4);
        bytes_mut.t2(&cap, &sig);
        bytes_mut.t8(2052);
        bytes_mut.t104(self.cache_info.read().await.t104.clone());
        bytes_mut.t116(self.version_info.misc_bitmap, self.version_info.sub_sigmap);
        let sso = protocol::build_sso_pkt(
            seq,
            self.version_info.app_id,
            self.version_info.sub_app_id,
            "wtlogin.login",
            &self.device_info.imei,
            &Bytes::default(),
            &self.outgoing_pkt_session_id.read().await,
            bytes_mut.freeze(),
            &self.cache_info.read().await.ksid,
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
