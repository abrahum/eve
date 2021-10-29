use super::OutComePacket;

impl crate::Client {
    pub(crate) async fn build_sms_code_pkt(&self, code: String) -> OutComePacket {
        let seq = self.next_seq();
        todo!()
    }
}
