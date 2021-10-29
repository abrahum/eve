use bytes::Bytes;

impl crate::Client {
    pub(crate) async fn build_heartbeat_pkt(&self) -> super::OutComePacket {
        let seq = self.next_seq();
        let sso = protocol::build_sso_pkt(
            seq,
            self.version_info.app_id,
            self.version_info.sub_app_id,
            "Heartbeat.Alive",
            &self.device_info.imei,
            &Bytes::default(),
            &self.outgoing_pkt_session_id.read().await,
            Bytes::default(),
            &self.cache_info.read().await.ksid,
        );
        let pkt = protocol::build_lgi_pkt(
            self.uin.load(std::sync::atomic::Ordering::SeqCst),
            0,
            &Bytes::default(),
            sso,
            &Bytes::default(),
        );
        super::OutComePacket { bytes: pkt, seq }
    }
}
