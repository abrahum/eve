use std::collections::HashMap;

use bytes::Bytes;

#[derive(Debug, Default)]
pub struct FriendInfo {
    pub uin: i64,
    pub nickname: String,
    pub remark: String,
    pub face_id: i16,
}

#[derive(Debug, Default)]
pub struct GroupInfo {
    pub uin: i64,
    pub code: i64,
    pub name: String,
    pub memo: String,
    pub owner_uin: i64,
    pub group_create_time: u32,
    pub group_level: u32,
    pub member_count: u16,
    pub max_member_count: u16,
    pub members: Vec<GroupMemberInfo>,
    pub last_msg_seq: i64,
    pub client: i64, // cq_client uin
}

#[derive(Debug)]
pub struct GroupMemberInfo {
    pub group: i64, // group uin
    pub uid: i64,
    pub gender: u8,
    pub nickname: String,
    pub card_name: String,
    pub level: u16,
    pub join_time: i64,
    pub last_speak_time: i64,
    pub special_title: String,
    pub special_title_expire_time: i64,
    pub shut_up_timestap: i64,
    pub permission: MemberPermission,
}

#[derive(Debug)]
pub enum MemberPermission {
    Owner,         // 0 gocq 1
    Administrator, // 1 gocq 2
    Member,        // 2 gocq 3
}

#[derive(Debug, Default)]
pub struct OtherDeviceInfo {
    pub app_id: i64,
    pub device_name: String,
    pub device_kind: String,
}

#[derive(Debug, Default)]
pub struct QiDianAccountInfo {
    pub master_uin: i64,
    pub ext_name: String,
    pub create_time: i64,
    /* big_data_req_addrs:   Vec<String>,
     * big_data_req_session: BigDataSessionInfo, */
}

#[derive(Default)]
pub struct BigDataSessionInfo {
    pub sig_session: Bytes,
    pub session_key: Bytes,
}

#[derive(Default)]
pub struct LoginSigInfo {
    pub login_bit_map: u64,
    pub tgt: Bytes,
    pub tgt_key: Bytes,

    pub srm_token: Bytes,
    pub t133: Bytes,
    pub encrypted_a1: Bytes,
    pub user_st_key: Bytes,
    pub user_st_web_sig: Bytes,
    pub s_key: Bytes,
    pub s_key_expired_time: i64,
    pub d2: Bytes,
    pub d2key: Bytes,
    pub wt_session_ticket_key: Bytes,
    pub device_token: Bytes,

    pub ps_key_map: HashMap<String, Bytes>,
    pub pt4_token_map: HashMap<String, Bytes>,
}
