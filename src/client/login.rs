use super::income_packet::login_resp::LoginResponse;
use async_recursion::async_recursion;
use bytes::Bytes;
use std::sync::atomic::Ordering;
use tracing::{error, trace, warn};

impl super::Client {
    async fn before_login(&self) {
        if self.account_info.read().await.online {
            panic!("already login");
        }
        if !self.connectted.load(Ordering::SeqCst) {
            panic!("login before tcp connected");
        }
    }

    pub async fn login(&self) {
        self.before_login().await;
        let pkt = self.build_login_pkt().await;
        if let Some(ipkt) = self.send_and_wait(pkt).await {
            if let Ok(resp) = self.decode_login_response(ipkt).await {
                trace!("LoginResponse:{:?}", resp);
                self.handle_login_resp(resp).await;
            }
        }
        error!("no resp from login pkt");
        self.shutdown.swap(true, Ordering::SeqCst);
    }

    //todo
    // pub async fn token_login(&self, token: Bytes) {
    //     self.before_login().await;
    //     self.load_from_token(token).await;
    //     let pkt = self.build_change_sig_pkt().await;
    //     self.send_and_wait(pkt).await;
    // }

    #[async_recursion]
    async fn submit_captcha(&self, cap: String, sig: Bytes) {
        let cap_pkt = self.build_captcha_pkt(cap, sig).await;
        if let Some(ipkt) = self.send_and_wait(cap_pkt).await {
            if let Ok(resp) = self.decode_login_response(ipkt).await {
                self.handle_login_resp(resp).await;
            }
        }
    }

    async fn submit_sms(code: String) {
        todo!()
    }

    pub(crate) async fn handle_login_resp(&self, resp: LoginResponse) {
        match resp {
            LoginResponse::Success => {}
            LoginResponse::SliderNeed(s) => {
                warn!("需要滑条验证码");
                todo!()
            }
            LoginResponse::NeedCaptcha(img_bytes, sig_bytes) => {
                warn!("登录需要验证码");
                std::fs::write("captcha.jpg", img_bytes).unwrap();
                println!("请输入验证码 (captcha)：");
                let mut cap = String::default();
                std::io::stdin().read_line(&mut cap).unwrap();
                trace!("get user input cap:{}", cap);
                std::fs::remove_file("captch.jpg").unwrap();
                self.submit_captcha(cap, sig_bytes).await;
            }
            LoginResponse::SMSNeed(phone, msg) => {
                warn!("设备锁已开启，即将向 {} 发送短信验证码", phone);
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }
}
