mod heartbeat;
mod temps;

pub use heartbeat::*;

use super::income_packet::{CommandName, IncomePacket};
use async_trait::async_trait;

pub type PktHandlerStruct = Box<dyn PktHandler + Send + Sync>;

/// handle income packet
#[async_trait]
pub trait PktHandler {
    /// handler main method
    async fn handle(&self, pkt: IncomePacket);

    /// matcher for temp pkt handler
    #[allow(unused_variables)]
    fn match_(&self, pkt: &IncomePacket) -> bool {
        true
    }
}

impl super::Client {
    pub async fn handle_income_packet(&self, pkt: IncomePacket) {
        match &pkt.command_name {
            &CommandName::Heartbeat => self.handle_heartbeat().await,
            _ => {}
        }

        // temp pkt handlers
        let mut matched = vec![];
        {
            for (k, h) in &*self.temp_handlers.read().await {
                if h.match_(&pkt) {
                    matched.push(k.clone());
                }
            }
        }
        for k in matched {
            let h = self.remove_handler(&k).await;
            let pkt_ = pkt.clone();
            tokio::spawn(async move { h.handle(pkt_).await });
        }
    }

    /// add temp handler
    pub async fn add_handler(&self, key: String, handler: PktHandlerStruct) {
        self.temp_handlers.write().await.insert(key, handler);
    }

    /// remove temp handler and return
    pub async fn remove_handler(&self, key: &str) -> PktHandlerStruct {
        self.temp_handlers.write().await.remove(key).unwrap()
    }
}
