use bytes::Bytes;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::atomic::{AtomicBool, AtomicI64, AtomicU16, AtomicUsize},
};

/// Client
mod client;
/// DeviceInfos
mod device;
/// IncomePacket and Action Handlers
mod handlers;
/// IncomePacket
mod income_packet;
mod login;
/// NetWork
pub mod net;
/// OutComePacket
mod outcome_packet;
/// other structs for Client
mod structs;
mod tlv_decode;
/// utils
pub mod utils;

pub use device::DeviceInfo;
pub use net::ClientNet;
pub use structs::*;
use tokio::sync::RwLock;

/// CqClient
pub struct Client {
    /// Client uin
    pub uin:          AtomicI64,
    /// password encode by md5
    pub password_md5: Bytes, // [u8; 16]
    allow_slide:      bool,

    temp_handlers: RwLock<HashMap<String, handlers::PktHandlerStruct>>,

    // signals
    connectted: AtomicBool,
    shutdown:   AtomicBool,

    // channels
    pub out_pkt_sender: net::OutPktSender,

    // account info
    account_info: RwLock<AccountInfo>,

    // internal state
    pub version_info:        device::VersionInfo,
    pub device_info:         device::DeviceInfo,
    servers:                 RwLock<Vec<SocketAddr>>,
    connecting_server_index: AtomicUsize,
    sequence_id:             AtomicU16,
    outgoing_pkt_session_id: RwLock<Bytes>,

    // crypto
    ecdh:     crypto::ECDH,
    /// [u8; 16]
    rand_key: Bytes,

    cache_info: RwLock<CacheInfo>,
}

#[derive(Debug, Default)]
pub struct AccountInfo {
    pub nickname:       String,
    pub age:            u16,
    pub gender:         u16,
    pub friend_list:    Vec<FriendInfo>,
    pub group_list:     Vec<GroupInfo>,
    pub online_devices: Vec<OtherDeviceInfo>,
    pub online:         bool,
    pub qi_dian:        QiDianAccountInfo,
}

#[derive(Default)]
pub struct CacheInfo {
    // tlv cache
    pub t104:         Bytes,
    pub t174:         Bytes,
    pub g:            Bytes,
    pub t402:         Bytes,
    pub t150:         Bytes,
    pub t149:         Bytes,
    pub t528:         Bytes,
    pub t530:         Bytes,
    pub rand_seed:    Bytes, // t403
    pub rollback_sig: Bytes, // t161-t172
    // sync info
    pub ksid:         Bytes, // t108

    // session info
    pub sig_info:  LoginSigInfo,
    pub dpwd:      String,
    pub time_diff: i64,
    pub pwd_flag:  bool,
}

impl CacheInfo {
    pub fn new_with_imei(imei: &str) -> Self {
        let mut info = Self::default();
        let ksid = Bytes::copy_from_slice(format!("|{}|A8.2.7.27f6ea96", imei).as_bytes());
        info.ksid = ksid;
        info
    }
}

/// Password enum
pub enum Password {
    String(String),
    /// [u8; 16]
    Md5(Bytes),
}

impl Password {
    /// compute password md5(do nothing if already md5)
    pub fn md5(&self) -> Bytes {
        match self {
            Self::String(s) => Bytes::copy_from_slice(&md5::compute(s).0),
            Self::Md5(m) => m.clone(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}
