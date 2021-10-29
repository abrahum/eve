// Automatically generated rust module for 'oidb0xdad.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DADReqBody<'a> {
    pub client: i64,
    pub product_id: u64,
    pub amount: i64,
    pub to_uin: u64,
    pub gc: u64,
    pub ip: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub sig: Option<DADLoginSig<'a>>,
}

impl<'a> MessageRead<'a> for DADReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.client = r.read_int64(bytes)?,
                Ok(16) => msg.product_id = r.read_uint64(bytes)?,
                Ok(24) => msg.amount = r.read_int64(bytes)?,
                Ok(32) => msg.to_uin = r.read_uint64(bytes)?,
                Ok(40) => msg.gc = r.read_uint64(bytes)?,
                Ok(50) => msg.ip = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.sig = Some(r.read_message::<DADLoginSig>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DADReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.client == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.client) as u64) }
        + if self.product_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.product_id) as u64) }
        + if self.amount == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.amount) as u64) }
        + if self.to_uin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.to_uin) as u64) }
        + if self.gc == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.gc) as u64) }
        + if self.ip == "" { 0 } else { 1 + sizeof_len((&self.ip).len()) }
        + if self.version == "" { 0 } else { 1 + sizeof_len((&self.version).len()) }
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.client != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.client))?; }
        if self.product_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.product_id))?; }
        if self.amount != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.amount))?; }
        if self.to_uin != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.to_uin))?; }
        if self.gc != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.gc))?; }
        if self.ip != "" { w.write_with_tag(50, |w| w.write_string(&**&self.ip))?; }
        if self.version != "" { w.write_with_tag(58, |w| w.write_string(&**&self.version))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DADLoginSig<'a> {
    pub type_pb: u32,
    pub sig: Cow<'a, [u8]>,
    pub appid: u32,
}

impl<'a> MessageRead<'a> for DADLoginSig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_uint32(bytes)?,
                Ok(18) => msg.sig = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.appid = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DADLoginSig<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.sig == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sig).len()) }
        + if self.appid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.appid) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.type_pb))?; }
        if self.sig != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.sig))?; }
        if self.appid != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.appid))?; }
        Ok(())
    }
}

