use crate::error::*;
use bytes::{Buf, Bytes, BytesMut};
use protocol::tlv::TlvRead;
use std::io::prelude::*;

pub(crate) mod login_resp;

#[derive(Clone, Debug)]
pub struct IncomePacket {
    pub sequence_id:  u16,
    pub flag2:        u8,
    pub command_name: CommandName,
    pub session_id:   Bytes,
    pub payload:      Bytes,
}

#[derive(Clone, Debug)]
pub enum CommandName {
    Other(String),
    Heartbeat,
    Login,
}

impl CommandName {
    pub fn from_str(str: String) -> Self {
        match str.as_str() {
            "Heartbeat.Alive" => Self::Heartbeat,
            "wtlogin.login" => Self::Login,
            _ => Self::Other(str),
        }
    }
}

impl IncomePacket {
    pub fn parase_income_packet(mut data: Bytes, d2key: Bytes) -> EveResult<Self> {
        if data.len() < 6 {
            return EveError::DecodeIncomeInvalid(data).err();
        }

        let flag1 = data.get_i32();
        let flag2 = data.get_u8();
        let _flag3 = data.get_u8();
        data.get_string(); // get uin string
        let de = match flag2 {
            0 => data,
            1 => {
                let d2 = crypto::Tea::new(d2key);
                d2.decrypt(data).unwrap()
            }
            2 => {
                let z16 = crypto::Tea::new_with_u32([0; 4]);
                z16.decrypt(data).unwrap()
            }
            _ => Bytes::default(),
        };
        if de.is_empty() {
            return EveError::DecodeIncomeUnkownFlag(flag2 as i32).err();
        }
        if flag1 != 0x0a && flag1 != 0x0b {
            return EveError::DecodeIncomeUnkownFlag(flag1).err();
        }
        Self::parse_sso_frame(de, flag2)
    }

    pub fn parse_sso_frame(mut data: Bytes, flag2: u8) -> EveResult<Self> {
        if data.get_i32() - 4 > data.len() as i32 {
            return EveError::SsoPacketDropped.err();
        }
        let seq = data.get_i32();
        let ret_code = data.get_i32();
        if ret_code != 0 {
            if ret_code == -10008 {
                return EveError::SsoSessionExpired.err();
            } else {
                return EveError::SsoUnsuccessRetCode(ret_code).err();
            }
        }
        data.get_bytes(); // extra data
        let command_name = CommandName::from_str(data.get_string());
        let session_id = data.get_bytes();
        if let CommandName::Heartbeat = command_name {
            return Ok(IncomePacket {
                sequence_id: seq as u16,
                flag2,
                command_name,
                session_id,
                payload: Bytes::default(),
            });
        }
        let compressed_flag = data.get_i32();
        let payload = match compressed_flag {
            0 => {
                let _len = data.get_i32() as i64 & 0xffffffff; // pkt_size
                data
            }
            1 => {
                // uncheck zlib
                data.get_u32();
                let mut de = flate2::read::ZlibDecoder::new(data.reader());
                let mut bytes = BytesMut::with_capacity(de.total_out() as usize);
                de.read(&mut bytes).unwrap();
                bytes.freeze()
            }
            8 => data,
            _ => Bytes::default(),
        };
        Ok(IncomePacket {
            sequence_id: seq as u16,
            flag2,
            command_name,
            session_id,
            payload,
        })
    }

    pub fn decrypt_payload(
        &mut self,
        random: Bytes,
        session_key: Bytes,
        share_key: Bytes,
    ) -> Result<(), &str> {
        let old = &mut self.payload;
        if old.get_u8() != 2 {
            return Err("UnkownFlag");
        }
        let _ = old.split_to(12);
        let encrypt_type = old.get_u16();
        old.get_u8();
        match encrypt_type {
            0 => {
                let d = old.split_to(old.len() - 1);
                let tea = crypto::Tea::new(share_key);
                let decrypto = match tea.decrypt(d.clone()) {
                    Ok(de) => de,
                    Err(_) => {
                        let tea = crypto::Tea::new(random);
                        tea.decrypt(d)?
                    }
                };
                self.payload = decrypto;
            }
            3 => {
                let d = old.split_to(old.len() - 1);
                let tea = crypto::Tea::new(session_key);
                let decrypto = tea.decrypt(d)?;
                self.payload = decrypto;
            }
            4 => todo!(),
            _ => return Err("UnkownFlag"),
        };
        Ok(())
    }
}
