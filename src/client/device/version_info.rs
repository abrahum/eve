use std::iter::FromIterator;

use super::ClientProtocol;

pub struct VersionInfo {
    pub apk_sign: bytes::Bytes,
    pub apk_id: String,
    pub sort_version_name: String,
    pub sdk_version: String,
    pub app_id: u32,
    pub sub_app_id: u32,
    pub build_time: u32,
    pub sso_version: u32,
    pub misc_bitmap: u32,
    pub sub_sigmap: u32,
    pub main_sigmap: u32,
    pub protocol: ClientProtocol,
}

impl VersionInfo {
    pub fn new(protocol: &ClientProtocol) -> Self {
        match protocol {
            ClientProtocol::AndroidPhone => VersionInfo {
                apk_id: "com.tencent.mobileqq".to_owned(),
                app_id: 537066738,
                sub_app_id: 537066738,
                sort_version_name: "8.5.0".to_owned(),
                build_time: 1607689988,
                apk_sign: bytes::Bytes::from_iter([
                    0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3,
                    0x6E, 0xB6, 0x8D,
                ]),
                sdk_version: "6.0.0.2454".to_owned(),
                sso_version: 15,
                misc_bitmap: 184024956,
                sub_sigmap: 0x10400,
                main_sigmap: 34869472,
                protocol: protocol.clone(),
            },
            ClientProtocol::AndroidWatch => VersionInfo {
                apk_id: "com.tencent.qqlite".to_owned(),
                app_id: 537064446,
                sub_app_id: 537064446,
                sort_version_name: "2.0.5".to_owned(),
                build_time: 1559564731,
                apk_sign: bytes::Bytes::from_iter([
                    0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3,
                    0x6E, 0xB6, 0x8D,
                ]),
                sdk_version: "6.0.0.236".to_owned(),
                sso_version: 5,
                misc_bitmap: 16252796,
                sub_sigmap: 0x10400,
                main_sigmap: 34869472,
                protocol: protocol.clone(),
            },
            ClientProtocol::IPad => VersionInfo {
                apk_id: "com.tencent.minihd.qq".to_owned(),
                app_id: 537065739,
                sub_app_id: 537065739,
                sort_version_name: "5.8.9".to_owned(),
                build_time: 1595836208,
                apk_sign: bytes::Bytes::from_iter([
                    170, 57, 120, 244, 31, 217, 111, 249, 145, 74, 102, 158, 24, 100, 116, 199,
                ]),
                sdk_version: "6.0.0.2433".to_owned(),
                sso_version: 12,
                misc_bitmap: 150470524,
                sub_sigmap: 66560,
                main_sigmap: 1970400,
                protocol: protocol.clone(),
            },
            ClientProtocol::MacOS => VersionInfo {
                apk_id: "com.tencent.minihd.qq".to_owned(),
                app_id: 537064315,
                sub_app_id: 537064315,
                sort_version_name: "5.8.9".to_owned(),
                build_time: 1595836208,
                apk_sign: bytes::Bytes::from_iter([
                    170, 57, 120, 244, 31, 217, 111, 249, 145, 74, 102, 158, 24, 100, 116, 199,
                ]),
                sdk_version: "6.0.0.2433".to_owned(),
                sso_version: 12,
                misc_bitmap: 150470524,
                sub_sigmap: 66560,
                main_sigmap: 1970400,
                protocol: protocol.clone(),
            },
            ClientProtocol::QiDian => VersionInfo {
                apk_id: "com.tencent.qidian".to_owned(),
                app_id: 537061386,
                sub_app_id: 537036590,
                sort_version_name: "3.8.6".to_owned(),
                build_time: 1556628836,
                apk_sign: bytes::Bytes::from_iter([
                    160, 30, 236, 171, 133, 233, 227, 186, 43, 15, 106, 21, 140, 133, 92, 41,
                ]),
                sdk_version: "6.0.0.2365".to_owned(),
                sso_version: 5,
                misc_bitmap: 49807228,
                sub_sigmap: 66560,
                main_sigmap: 34869472,
                protocol: protocol.clone(),
            },
        }
    }
}
