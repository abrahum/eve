use std::collections::HashMap;

use crate::utils::MapExt;
use bytes::{Buf, Bytes};
use protocol::tlv::TlvRead;

use super::{AccountInfo, CacheInfo, LoginSigInfo};

pub fn decode_t161(mut data: Bytes, cache_info: &mut CacheInfo) {
    data.get_i16();
    let mut map = data.get_tlv_map(2);
    map.remove(&0x172).then_do(|b| cache_info.rollback_sig = b);
}

pub async fn decode_t199(
    data: Bytes,
    tea_key: Bytes,
    cache_info: &mut CacheInfo,
    account_info: &mut AccountInfo,
) {
    let tea = crypto::Tea::new(tea_key);
    let mut data = tea.decrypt(data).unwrap();
    data.get_i16();
    let mut map = data.get_tlv_map(2);
    map.remove(&0x130).then_do(|b| decode_t130(b, cache_info));
    map.remove(&0x113).then_do(|b| decode_t113(b));
    map.remove(&0x528).then_do(|b| cache_info.t528 = b);
    map.remove(&0x530).then_do(|b| cache_info.t530 = b);
    map.remove(&0x108).then_do(|b| cache_info.ksid = b);
    map.remove(&0x186)
        .then_do(|b| cache_info.pwd_flag = b[1] == 1);
    map.remove(&0x11a).then_do(|b| {
        let t = decode_t11a(b); // issue #71126 la ji rust
        account_info.nickname = t.0;
        account_info.age = t.1;
        account_info.gender = t.2;
    });
    let mut ps_key_map = HashMap::new();
    let mut pt4_token_map = HashMap::new();
    map.remove(&0x512).then_do(|b| {
        let t = decode_t512(b);
        ps_key_map = t.0;
        pt4_token_map = t.1;
    });

    cache_info.sig_info = LoginSigInfo {
        login_bit_map: 0,
        srm_token: map
            .remove(&0x16a)
            .unwrap_or(cache_info.sig_info.srm_token.clone()),
        t133: map
            .remove(&0x133)
            .unwrap_or(cache_info.sig_info.t133.clone()),
        encrypted_a1: map
            .remove(&0x106)
            .unwrap_or(cache_info.sig_info.encrypted_a1.clone()),
        tgt: map.remove(&0x10a).unwrap(),
        tgt_key: map.remove(&0x10d).unwrap(),
        user_st_key: map.remove(&0x10e).unwrap(),
        user_st_web_sig: map.remove(&0x103).unwrap(),
        s_key: map.remove(&0x120).unwrap(),
        s_key_expired_time: chrono::Local::now().timestamp() + 21600,
        d2: map.remove(&0x143).unwrap(),
        d2key: map.remove(&0x305).unwrap(),
        wt_session_ticket_key: map
            .remove(&0x305)
            .unwrap_or(cache_info.sig_info.wt_session_ticket_key.clone()),
        device_token: map.remove(&0x322).unwrap(),

        ps_key_map,
        pt4_token_map,
    };
}

pub fn decode_t130(mut data: Bytes, cache_info: &mut CacheInfo) {
    data.get_i16();
    cache_info.time_diff = data.get_i32() as i64 - chrono::Local::now().timestamp();
    cache_info.t149 = data.split_off(4);
}

pub fn decode_t113(mut data: Bytes) {
    use tracing::trace;
    let uin = data.get_i32();
    trace!("got t113 uin {}", uin);
}

pub fn decode_t11a(mut data: Bytes) -> (String, u16, u16) {
    data.get_i16();
    let age = data.get_u8() as u16;
    let gender = data.get_u8() as u16;
    let len = data.get_u8() as usize;
    let nick = String::from_utf8(data.copy_to_bytes(len).to_vec()).unwrap();
    (nick, age, gender)
}

pub fn decode_t512(mut data: Bytes) -> (HashMap<String, Bytes>, HashMap<String, Bytes>) {
    let len = data.get_u16() as usize;

    let (mut ps_key_map, mut pt4_token_map) = (HashMap::new(), HashMap::new());
    for _ in 0..len {
        let domain = data.get_string_short();
        let ps_key = data.get_bytes_short();
        let pt4_token = data.get_bytes_short();
        ps_key_map.insert(domain.clone(), ps_key);
        pt4_token_map.insert(domain, pt4_token);
    }

    (ps_key_map, pt4_token_map)
}
