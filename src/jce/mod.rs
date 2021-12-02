use bytes::Bytes;
use jcers::{JceGet, JcePut};
use std::collections::HashMap;

mod test;

#[derive(Debug, Clone, Default, JceGet, JcePut)]
pub(crate) struct RequestPacket {
    #[jce(1)]
    pub(crate) i_version:      i16,
    #[jce(2)]
    pub(crate) c_packet_type:  i16,
    #[jce(3)]
    pub(crate) i_message_type: i32,
    #[jce(4)]
    pub(crate) i_request_id:   i32,
    #[jce(5)]
    pub(crate) s_servant_name: String,
    #[jce(6)]
    pub(crate) s_func_name:    String,
    #[jce(7)]
    pub(crate) s_buffer:       Bytes,
    #[jce(8)]
    pub(crate) i_timeout:      i32,
    #[jce(9)]
    pub(crate) context:        HashMap<String, String>,
    #[jce(10)]
    pub(crate) status:         HashMap<String, String>,
}

#[derive(Debug, Clone, Default, JceGet, JcePut)]
pub(crate) struct RequestDataVersion3 {
    #[jce(0)]
    pub(crate) map: HashMap<String, Bytes>,
}

#[derive(Debug, Clone, Default, JceGet, JcePut)]
pub(crate) struct RequestDataVersion2 {
    #[jce(0)]
    pub(crate) map: HashMap<String, Bytes>,
}

#[derive(Debug, Clone, Default, JceGet, JcePut)]
pub(crate) struct HttpServerListRes {
    #[jce(2)]
    pub(crate) sso_server_infos: Vec<SsoServerInfo>,
}

#[derive(Debug, Clone, Default, JceGet, JcePut)]
pub(crate) struct SsoServerInfo {
    #[jce(1)]
    pub(crate) server:   String,
    #[jce(2)]
    pub(crate) port:     i32,
    #[jce(8)]
    pub(crate) location: String,
}

macro_rules! JceStruct {
    ($struct_name: ident {$($tag: expr => $field: ident: $type_: ty,)*}) => {
        #[derive(Debug, Clone, Default, JceGet, JcePut)]
        pub(crate) struct $struct_name {
            $(
                #[jce($tag)]
                pub(crate) $field: $type_,
            )*
        }
    };
    ($struct_name: ident {$tag: expr => $field: ident: $type_: ty,}) => {
        #[derive(Debug, Clone, Default, JceGet, JcePut)]
        pub(crate) struct $struct_name {
                #[jce($tag)]
                pub(crate) $field: $type_,
        }
    };
}

JceStruct!(FileStoragePushFSSvcList {
    0  => upload_list: Vec<FileStorageServerInfo>,
    1  => pic_download_list: Vec<FileStorageServerInfo>,
    2  => g_pic_download_list: Vec<FileStorageServerInfo>,
    3  => q_zone_proxy_service_list: Vec<FileStorageServerInfo>,
    4  => url_encode_service_list: Vec<FileStorageServerInfo>,
    5  => big_data_channel: BigDataChannel,
    6  => vip_emotion_list: Vec<FileStorageServerInfo>,
    7  => c2c_pic_down_list: Vec<FileStorageServerInfo>,
    // 8  => fmt_ip_info: FmtIpInfo,
    // 9  => domain_ip_channel : DomainIpChannel,
    10 => ptt_list: Bytes,
});

JceStruct!(FileStorageServerInfo {
    1 => server: String,
    2 => port: i32,
});

JceStruct!(BigDataChannel {
    0 => ip_list: Vec<BigDataIp>, // gocq BigDataIPList
    1 => sig_session: Bytes,
    2 => key_session: Bytes,
    3 => sig_uin: i64,
    4 => connect_flag: i32,
    5 => pb_buf: Bytes,
});

JceStruct!(BigDataIp {
    0 => service_type: i64,
    1 => ip_list: BigDataIpInfo,
    2 => fragment_size: i64,
});

JceStruct!(BigDataIpInfo {
    0 => type_: i64,
    1 => server: String,
    2 => port: i64,
});

JceStruct!(SvcReqRegister {
        0  => uin: i64,
        1  => bid: i64,
        2  => conn_type: u8,
        3  => other: String,
        4  => status: i32,
        5  => online_push: u8,
        6  => is_online: u8,
        7  => is_show_online: u8,
        8  => kick_pc: u8,
        9  => kick_weak: u8,
        10 => timestamp : i64,
        11 => ios_version: i64,
        12 => net_type: u8,
        13 => build_ver: String,
        14 => reg_type: u8,
        15 => dev_param: Bytes ,
        16 => guid: Bytes,
        17 => locale_id: i32,
        18 => silent_push: u8,
        19 => dev_name: String,
        20 => dev_type: String,
        21 => os_ver: String,
        22 => open_push : u8,
        23 => large_seq: i64,
        24 => last_watch_start_time: i64,
        26 => old_sso_ip: i64,
        27 => new_sso_ip: i64,
        28 => channel_no: String,
        29 => cpid: i64,
        30 => vendor_name: String,
        31 => vendor_os_name: String,
        32 => ios_idfa: String,
        33 => b769: Bytes,
        34 => is_set_status: u8,
        35 => server_buf: Bytes,
        36 => set_mute: u8,
        38 => ext_online_status: i64,
        39 => battery_status: i32,
});

JceStruct!(SvcRespRegister {
    0  => uin: i64,
    1  => bid: i64,
    2  => reply_code: u8,
    3  => result: String,
    4  => server_time: i64,
    5  => log_qq: u8,
    6  => need_kik: u8,
    7  => update_flag: u8,
    8  => timestamp: i64,
    9  => crash_flag: u8,
    10 => client_ip: String,
    11 => client_port: i32,
    12 => hello_interval: i32,
    13 => large_seq: i32,
    14 => large_seq_update: u8,
    15 => d769_rsp_body: Bytes,
    16 => status: i32,
    17 => ext_online_status: i64,
    18 => client_battery_get_interval: i64,
    19 => client_auto_status_interval: i64,
});

// JceStruct!(SvcReqRegisterNew {
//     0  => request_optional: i64,
//     1  => c2c_msg:
// });

// todo
// 抄不动了，毁灭吧
