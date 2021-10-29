mod captcha;
mod sms_code;
mod change_sig;
mod heartbeat;
mod login;
mod utils;

use bytes::Bytes;

#[derive(Debug)]
pub(crate) struct OutComePacket {
    bytes: Bytes,
    seq:   u16,
}

impl OutComePacket {
    pub(crate) fn bytes(self) -> Bytes {
        self.bytes
    }
    pub(crate) fn seq(&self) -> u16 {
        self.seq
    }
}
