use std::{
    collections::HashMap,
    convert::TryInto,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::atomic::{AtomicBool, AtomicI64, AtomicU16, AtomicUsize},
};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use jce_struct::*;
use tokio::sync::{mpsc::UnboundedReceiver, RwLock};
use tracing::{info, trace};

use crate::client::AccountInfo;
use crate::jce::*;
use crate::utils::log_trace_bytes;

use super::device::{DeviceInfo, VersionInfo};
use super::Password;

impl super::Client {
    /// new Client
    pub async fn new(
        uin: i64,
        password: Password,
        allow_slide: bool,
        mut device_info: DeviceInfo,
    ) -> (Self, UnboundedReceiver<Bytes>) {
        let password_md5 = password.md5();
        device_info.gen();

        let (out_pkt_sender, out_pkt_receiver) = tokio::sync::mpsc::unbounded_channel();

        let cli = Self {
            uin: AtomicI64::new(uin),
            password_md5,
            allow_slide,
            temp_handlers: RwLock::default(),
            connectted: AtomicBool::default(),
            shutdown: AtomicBool::default(),
            out_pkt_sender,
            account_info: RwLock::new(AccountInfo::default()),
            version_info: VersionInfo::new(&device_info.protocol),
            cache_info: RwLock::new(super::CacheInfo::new_with_imei(&device_info.imei)),
            device_info,
            servers: RwLock::new(vec![]),
            connecting_server_index: AtomicUsize::new(0),
            sequence_id: AtomicU16::new(0x3635),
            outgoing_pkt_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            ecdh: crypto::ECDH::new(uin).await,
            rand_key: gen_rand_key(),
        };
        *cli.servers.write().await = cli.get_sso_address().await;
        //todo? loopup ip
        (cli, out_pkt_receiver)
    }

    pub async fn update_servers(&self) {
        *self.servers.write().await = super::net::loopup_ip().await;
    }

    pub async fn get_sso_address(&self) -> Vec<SocketAddr> {
        let protocol_info = VersionInfo::new(&self.device_info.protocol);
        let key = hex::decode("F0441F5FF42DA58FDCF7949ABA62D411").unwrap();
        let payload = {
            let mut jce = jce_struct::JceMut::with_capacity(1024);
            jce.put_i64(0, 1)
                .put_i64(0, 2)
                .put_i64(1, 3)
                .put_string("00000".to_owned(), 4)
                .put_i32(100, 5)
                .put_i32(protocol_info.app_id.try_into().unwrap(), 6)
                .put_string(self.device_info.imei.clone(), 7)
                .put_i64(0, 8)
                .put_i64(0, 9)
                .put_i64(0, 10)
                .put_i64(0, 11)
                .put_i64(0, 12)
                .put_i64(0, 13)
                .put_i64(1, 14);
            jce.freeze()
        };
        let mut map = HashMap::new();
        map.insert(
            "HttpServerListReq".to_owned(),
            super::utils::pack_jce_struct_data(payload),
        );
        let buf = RequestDataVersion3 { map };
        let pkt = RequestPacket {
            i_version:      3,
            s_servant_name: "ConfigHttp".to_owned(),
            s_func_name:    "HttpServerListReq".to_owned(),
            s_buffer:       buf.build(),
            c_packet_type:  0,
            i_message_type: 0,
            i_request_id:   0,
            i_timeout:      0,
            context:        HashMap::default(),
            status:         HashMap::default(),
        };

        let tea = crypto::tea::Tea::new(Bytes::from(key));
        trace!("Getting SSOAdress");
        let rsp = super::utils::http_post(
            "https://configsvr.msf.3g.qq.com/configsvr/serverlist.jsp",
            tea.encrypt({
                let data = pkt.build();
                let intlv: u32 = data.len() as u32 + 0;
                let mut bytes = BytesMut::with_capacity(data.len() + 4);
                bytes.put_u32(intlv);
                bytes.extend(data);
                bytes.freeze()
            }),
        )
        .await
        .expect("Get SSOAddress error");
        log_trace_bytes("Get resp", &rsp);

        let mut de_rsp = tea.decrypt(rsp).unwrap();
        let _ = de_rsp.split_to(4);
        let mut request_packet: RequestPacket = Jce::read_from_bytes(&mut de_rsp);

        let mut request_data_version3: RequestDataVersion3 =
            Jce::read_from_bytes(&mut request_packet.s_buffer);

        let sso_server_infos: HttpServerListRes = Jce::read_from_bytes(
            request_data_version3
                .map
                .get_mut("HttpServerListRes")
                .unwrap(),
        );
        let mut servers = vec![];
        for s in sso_server_infos.sso_server_infos {
            match std::net::Ipv4Addr::from_str(&s.server) {
                Ok(addr) => {
                    servers.push(SocketAddr::new(IpAddr::V4(addr), s.port as u16));
                }
                Err(_) => {}
            }
        }
        info!("Get Address {:?}", servers);
        servers
    }

    pub fn next_seq(&self) -> u16 {
        self.sequence_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            + 1
    }

    pub async fn load_from_token(&mut self, mut token: Bytes) {
        use protocol::tlv::TlvRead;

        let uin = token.get_i64();
        self.uin.store(uin, std::sync::atomic::Ordering::SeqCst);
        {
            let mut cache_info = self.cache_info.write().await;
            cache_info.sig_info.d2 = token.get_bytes_short();
            cache_info.sig_info.d2key = token.get_bytes_short();
            cache_info.sig_info.tgt = token.get_bytes_short();
            cache_info.sig_info.srm_token = token.get_bytes_short();
            cache_info.sig_info.t133 = token.get_bytes_short();
            cache_info.sig_info.encrypted_a1 = token.get_bytes_short();
            cache_info.sig_info.wt_session_ticket_key = token.get_bytes_short();
        }
        *self.outgoing_pkt_session_id.write().await = token.get_bytes_short();
        self.device_info.tgtgt_key = {
            let mut data = token.get_bytes_short();
            let mut r = BytesMut::with_capacity(16);
            for b in r.as_mut() {
                *b = data.get_u8();
            }
            r.freeze()
        };
        // let pkt = self.build_change_sig_pkt().await.pack();
        // // todo
        // self.action_sender
        //     .send(Action::SendPackage(pkt))
        //     .await
        //     .expect("send token login pkt fail");
    }

    pub async fn gen_token(&self) -> Bytes {
        use protocol::tlv::TlvWrite;

        let cache_info = self.cache_info.read().await;
        let mut token = BytesMut::with_capacity(2048); //todo
        token.put_u64(self.uin.load(std::sync::atomic::Ordering::SeqCst) as u64);
        token.bytes_lv(cache_info.sig_info.d2.clone());
        token.bytes_lv(cache_info.sig_info.d2key.clone());
        token.bytes_lv(cache_info.sig_info.tgt.clone());
        token.bytes_lv(cache_info.sig_info.srm_token.clone());
        token.bytes_lv(cache_info.sig_info.t133.clone());
        token.bytes_lv(cache_info.sig_info.encrypted_a1.clone());
        token.bytes_lv(cache_info.sig_info.wt_session_ticket_key.clone());
        token.bytes_lv(self.outgoing_pkt_session_id.read().await.clone());
        token.bytes_lv(Bytes::copy_from_slice(&self.device_info.tgtgt_key));
        token.freeze()
    }
}

fn gen_rand_key() -> Bytes {
    use rand::{thread_rng, RngCore};
    let mut key = [0u8; 16];
    thread_rng().fill_bytes(&mut key);
    Bytes::copy_from_slice(&key)
}
