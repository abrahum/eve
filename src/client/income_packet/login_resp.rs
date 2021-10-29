use super::IncomePacket;
use crate::client::tlv_decode::*;
use crate::utils::MapExt;
use bytes::{Buf, Bytes};

use std::io::Result as IoResult;

#[derive(Debug)]
pub enum LoginResponse {
    Success,
    UnknownLoginError,
    SliderNeed(String),
    /// img data, sign
    NeedCaptcha(Bytes, Bytes),
    AccountFrozenError,
    /// verify url, phone, ErrorMsg
    SMSorVerify(String, String, String),
    /// phone, ErrorMsg
    SMSNeed(String, String),
    SecondCheckNeed,
    /// verify url
    UnsafeDevice(String),
    TooManySMSRequestError,
    /// ErrorMsg
    OtherLoginError(String),
}

impl crate::Client {
    pub(crate) async fn decode_login_response(
        &self,
        mut pkt: IncomePacket,
    ) -> IoResult<LoginResponse> {
        use protocol::tlv::TlvRead;

        let mut cache_info = self.cache_info.write().await;
        let mut account_info = self.account_info.write().await;
        let device_info = &self.device_info;

        let bytes = &mut pkt.payload;
        let _sub_command = bytes.get_u16();
        let t = bytes.get_u8();
        println!("Login t:{}", t);
        bytes.get_u16();
        let mut map = bytes.get_tlv_map(2);
        println!("{:?}", map);
        map.remove(&0x402).then_do(|b| {
            cache_info.dpwd = crate::client::device::gen_rand_string(16);
            cache_info.t402 = b;
            let mut t = Bytes::copy_from_slice(&device_info.guid)
                .chain(cache_info.dpwd.as_bytes())
                .chain(cache_info.t402.clone());
            cache_info.g = Bytes::copy_from_slice(&md5::compute(t.copy_to_bytes(t.remaining())).0);
        });
        match t {
            0 => {
                // login success
                map.remove(&0x150).then_do(|b| cache_info.t150 = b);
                map.remove(&0x161)
                    .then_do(|b| decode_t161(b, &mut cache_info));
                map.remove(&0x403).then_do(|b| cache_info.rand_seed = b);
                decode_t199(
                    map.remove(&0x119).unwrap(),
                    Bytes::copy_from_slice(&device_info.tgtgt_key),
                    &mut cache_info,
                    &mut account_info,
                )
                .await;
                Ok(LoginResponse::Success)
            }

            2 => {
                // need captcha
                cache_info.t104 = map.remove(&0x104).unwrap();
                if let Some(r) = map.remove(&0x192).and_then(|b| {
                    Some(LoginResponse::SliderNeed(
                        String::from_utf8(b.to_vec()).unwrap(),
                    ))
                }) {
                    return Ok(r);
                }
                if let Some(r) = map.remove(&0x165).and_then(|mut b| {
                    let img_data = map.remove(&0x105).unwrap();
                    let sign_len = b.get_u16();
                    b.get_u16();
                    let sign = b.split_off(sign_len as usize);
                    Some(LoginResponse::NeedCaptcha(img_data, sign))
                }) {
                    return Ok(r);
                }
                return Ok(LoginResponse::UnknownLoginError);
            }

            40 => Ok(LoginResponse::AccountFrozenError),

            160 | 239 => {
                // SMS
                if let Some(b) = map.remove(&0x174) {
                    cache_info.t104 = map.remove(&0x104).unwrap();
                    cache_info.t174 = b;
                    cache_info.rand_seed = map.remove(&0x403).unwrap();
                    let phone = {
                        let mut b = map.remove(&0x178).unwrap();
                        let len = b.get_i32() as usize;
                        String::from_utf8(b.split_off(len).to_vec()).unwrap()
                    };
                    let err_msg = String::from_utf8(map.remove(&0x204).unwrap().to_vec()).unwrap();
                    if let Some(b) = map.remove(&0x204) {
                        let verify = String::from_utf8(b.to_vec()).unwrap();
                        Ok(LoginResponse::SMSorVerify(verify, phone, err_msg))
                    } else {
                        Ok(LoginResponse::SMSNeed(phone, err_msg))
                    }
                } else if let Some(_) = map.remove(&0x17b) {
                    cache_info.t104 = map.remove(&0x104).unwrap();
                    Ok(LoginResponse::SecondCheckNeed)
                } else if let Some(b) = map.remove(&0x204) {
                    Ok(LoginResponse::UnsafeDevice(
                        String::from_utf8(b.to_vec()).unwrap(),
                    ))
                } else {
                    Ok(LoginResponse::UnknownLoginError)
                }
            }

            162 => Ok(LoginResponse::TooManySMSRequestError),

            // 204 => {
            //     // drive lock
            // }
            _ => Ok(LoginResponse::UnknownLoginError),
        }
    }
}
