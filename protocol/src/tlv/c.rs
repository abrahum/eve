use super::*;

pub trait TlvBuildC {
    fn t202(&mut self, wifi_bssid: &str, wifi_ssid: &str);
}

impl TlvBuildC for BytesMut {
    fn t202(&mut self, wifi_bssid: &str, wifi_ssid: &str) {
        self.tlv_bytes(0x202, {
            let mut bytes_mut = BytesMut::with_capacity(48);
            bytes_mut.str_lv_limit_size(wifi_bssid, 16);
            bytes_mut.str_lv_limit_size(wifi_ssid, 32);
            bytes_mut.freeze()
        })
    }
}
