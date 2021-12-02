use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpSocket, TcpStream},
    sync::mpsc,
};
use tracing::{debug, info, trace};

use std::io::Result as IoResult;
use std::net::SocketAddr;
use std::sync::{atomic::Ordering, Arc};

use super::income_packet::IncomePacket;

pub type OutPktSender = mpsc::UnboundedSender<Bytes>;
pub type OutPktReceiver = mpsc::UnboundedReceiver<Bytes>;

pub struct ClientNet {
    client:   Arc<super::Client>,
    receiver: OutPktReceiver,
}

impl ClientNet {
    pub fn new(client: Arc<super::Client>, receiver: OutPktReceiver) -> Self {
        Self { client, receiver }
    }

    pub async fn connect_tcp(&self) -> TcpStream {
        let addr = {
            let s = self.client.servers.read().await;
            let index = self.client.connecting_server_index.load(Ordering::SeqCst);
            (*s)[index].clone()
        };
        info!("connecting TcpAddress {}", addr);
        match connect(addr).await {
            Ok(stream) => {
                self.client.connectted.swap(true, Ordering::SeqCst);
                stream
            }
            Err(_) => {
                panic!("Tcp connect error") // todo
            }
        }
    }

    pub async fn send_bytes_to_tcp_stream(
        &mut self,
        stream: &mut TcpStream,
        bytes: Bytes,
    ) -> IoResult<()> {
        stream.writable().await?;
        stream.write(&bytes).await?;
        Ok(())
    }

    pub async fn read_from_tcp_stream(&self, stream: &mut TcpStream) -> IoResult<Bytes> {
        stream.readable().await?;
        let len = stream.read_i32().await?;
        let mut data = BytesMut::with_capacity(len as usize - 4);
        stream.read_buf(&mut data).await?;
        Ok(data.freeze())
    }

    pub async fn read_and_parase(&self, stream: &mut TcpStream) -> IoResult<IncomePacket> {
        let in_bytes = self.read_from_tcp_stream(stream).await?;
        trace!("Get income TCP bytes data:{:x}", in_bytes);
        match IncomePacket::parase_income_packet(
            in_bytes,
            self.client.cache_info.read().await.sig_info.d2key.clone(),
        ) {
            Ok(mut pkt) => {
                if pkt.flag2 == 2 {
                    pkt.decrypt_payload(
                        self.client.rand_key.clone(),
                        self.client
                            .cache_info
                            .read()
                            .await
                            .sig_info
                            .wt_session_ticket_key
                            .clone(),
                        self.client.ecdh.initial_share_key.clone(),
                    )
                    .unwrap();
                }
                Ok(pkt)
            }
            Err(e) => panic!("handle parase income packet err here {:?}", e),
        }
    }

    /// loop for handle all
    pub async fn net_loop(mut self, mut stream: TcpStream) -> IoResult<()> {
        loop {
            tokio::select! {
                _ = stream.readable() => {
                    let pkt = self.read_and_parase(&mut stream).await?;
                    debug!("Get IncomePkt {:?}", pkt);
                    self.client.handle_income_packet(pkt).await;
                }
                bytes_option = self.receiver.recv() => {
                    if let Some(bytes) = bytes_option {
                        trace!("Get outcome bytes:{:x}", bytes);
                        self.send_bytes_to_tcp_stream(&mut stream, bytes).await?;
                    }
                }
            }
        }
    }
}

async fn connect(addr: SocketAddr) -> std::io::Result<TcpStream> {
    let tcp_connect = TcpSocket::new_v4()?;
    let stream: TcpStream = tcp_connect.connect(addr).await?;
    Ok(stream)
}

pub async fn loopup_ip() -> Vec<SocketAddr> {
    info!("{}", "Looking for IP...");
    static URL: &'static str = "msfwifi.3g.qq.com:8080";
    let mut servers = vec![];
    let resp = tokio::net::lookup_host(URL).await.expect("IpLookup error");
    for address in resp {
        servers.push(std::net::SocketAddr::new(address.ip(), 8080));
    }
    servers_quality_check(&mut servers).await;
    servers
}

pub async fn servers_quality_check(_servers: &mut Vec<SocketAddr>) {
    todo!()
}

impl super::Client {
    pub(crate) async fn send_and_wait(
        &self,
        pkt: super::outcome_packet::OutComePacket,
    ) -> Option<IncomePacket> {
        struct TempHandler {
            seq:    u16,
            sender: tokio::sync::mpsc::Sender<IncomePacket>,
        }

        #[async_trait::async_trait]
        impl super::handlers::PktHandler for TempHandler {
            fn match_(&self, pkt: &IncomePacket) -> bool {
                if self.seq == pkt.sequence_id {
                    true
                } else {
                    false
                }
            }

            async fn handle(&self, pkt: IncomePacket) {
                self.sender.send(pkt).await.unwrap();
            }
        }

        let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
        self.add_handler(
            format!("{}", pkt.seq()),
            Box::new(TempHandler {
                seq: pkt.seq(),
                sender,
            }),
        )
        .await;
        self.out_pkt_sender.send(pkt.bytes()).unwrap();
        if let Ok(p) =
            tokio::time::timeout(std::time::Duration::from_secs(15), receiver.recv()).await
        {
            p
        } else {
            None
        }
    }
}
