impl crate::Client {
    pub(crate) async fn handle_heartbeat(&self) {
        let b = self.build_heartbeat_pkt().await.bytes();
        self.out_pkt_sender.send(b).unwrap();
    }
}
