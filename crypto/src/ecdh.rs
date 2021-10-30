use reqwest::get;
use serde::Deserialize;

use bytes::{BufMut, Bytes, BytesMut};
use p256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};

static BASE_URL: &str = "https://keyrotate.qq.com/rotate_key?cipher_suite_ver=305&uin=";

#[derive(Debug)]
pub struct ECDH {
    /// [u8; 16]
    pub initial_share_key:  Bytes,
    /// len 65
    pub public_key:         Bytes,
    pub public_key_version: u16,
}

#[derive(Deserialize)]
pub struct PubKeyResp {
    // #[serde(rename = "QuerySpan")]
    // pub query_span:   i64,
    #[serde(rename = "PubKeyMeta")]
    pub pub_key_meta: PubKeyMeta,
}

#[derive(Deserialize)]
pub struct PubKeyMeta {
    #[serde(rename = "KeyVer")]
    pub key_ver:      u16,
    #[serde(rename = "PubKey")]
    pub pub_key:      String,
    #[serde(rename = "PubKeySign")]
    pub pub_key_sign: String,
}

impl ECDH {
    pub async fn new(uin: i64) -> Self {
        let mut e = ECDH {
            initial_share_key:  Bytes::default(),
            public_key:         Bytes::default(), // len 65
            public_key_version: 1,
        };
        e.fetch_pub_key(uin).await.unwrap();
        e
    }

    pub fn test_new() -> Self {
        static SERVER_PUB_KEY: &str = "04EBCA94D733E399B2DB96EACDD3F69A8BB0F74224E2B44E3357812211D2E62EFBC91BB553098E25E33A799ADC7F76FEB208DA7C6522CDB0719A305180CC54A82E";
        let mut e = ECDH {
            initial_share_key:  Bytes::default(),
            public_key:         Bytes::default(), // len 65
            public_key_version: 1,
        };
        e.generate_key(SERVER_PUB_KEY);
        e
    }

    pub fn gen2(&mut self, s_pub_key: &str) {
        let key = EphemeralSecret::random(rand::thread_rng());

        let s_pub_key = hex::decode(s_pub_key).unwrap();
        let pub_key = PublicKey::from_sec1_bytes(&s_pub_key).unwrap();

        let share = key.diffie_hellman(&pub_key);
    }

    pub fn generate_key(&mut self, s_pub_key: &str) {
        // error
        let s_pub_key = hex::decode(s_pub_key).unwrap(); // decode pub key
        let secret = EphemeralSecret::random(rand::thread_rng()); // gen private key
        let pub_key = PublicKey::from_sec1_bytes(&s_pub_key).unwrap(); // gen public key

        let share = secret.diffie_hellman(&pub_key); // count public share
        let share_x = &share.as_bytes()[0..16];
        self.initial_share_key = Bytes::copy_from_slice(&md5::compute(share_x).0);

        let self_public_key = secret.public_key();
        let point = EncodedPoint::from(self_public_key);
        self.public_key = Bytes::copy_from_slice(point.as_bytes());
    }

    pub fn encrypt(&self, data: Bytes, key: &[u8]) -> Bytes {
        let mut bytes_mut = BytesMut::with_capacity(1024);
        bytes_mut.extend([0x02u8, 0x01u8]);
        bytes_mut.extend(key);
        bytes_mut.put_u16(0x01_31);
        bytes_mut.put_u16(self.public_key_version);
        bytes_mut.put_u16(self.public_key.len() as u16);
        bytes_mut.extend(&self.public_key);
        let tea = super::Tea::new_with_ref(&self.initial_share_key);
        let tea_data = tea.encrypt(data);
        bytes_mut.extend(tea_data);
        bytes_mut.freeze()
    }

    pub fn id(&self) -> u8 {
        0x87
    }

    pub async fn fetch_pub_key(&mut self, uin: i64) -> reqwest::Result<()> {
        let url = format!("{}{}", BASE_URL, uin);
        let resp = get(url).await?;
        let data = resp.text().await?;
        let pub_key_resp: PubKeyResp = serde_json::from_str(&data).unwrap();
        self.public_key_version = pub_key_resp.pub_key_meta.key_ver;
        self.generate_key(&pub_key_resp.pub_key_meta.pub_key);
        Ok(())
    }
}
