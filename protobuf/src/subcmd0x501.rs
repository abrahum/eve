// Automatically generated rust module for 'subcmd0x501.proto' file

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
pub struct C501ReqBody<'a> {
    pub ReqBody: Option<SubCmd0x501ReqBody<'a>>,
}

impl<'a> MessageRead<'a> for C501ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10250) => msg.ReqBody = Some(r.read_message::<SubCmd0x501ReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C501ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ReqBody.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ReqBody { w.write_with_tag(10250, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C501RspBody<'a> {
    pub RspBody: Option<SubCmd0x501RspBody<'a>>,
}

impl<'a> MessageRead<'a> for C501RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10250) => msg.RspBody = Some(r.read_message::<SubCmd0x501RspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C501RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.RspBody.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.RspBody { w.write_with_tag(10250, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SubCmd0x501ReqBody<'a> {
    pub uin: Option<u64>,
    pub idcId: Option<u32>,
    pub appid: Option<u32>,
    pub loginSigType: Option<u32>,
    pub loginSigTicket: Option<Cow<'a, [u8]>>,
    pub requestFlag: Option<u32>,
    pub serviceTypes: Vec<u32>,
    pub bid: Option<u32>,
}

impl<'a> MessageRead<'a> for SubCmd0x501ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.idcId = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.appid = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.loginSigType = Some(r.read_uint32(bytes)?),
                Ok(42) => msg.loginSigTicket = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.requestFlag = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.serviceTypes.push(r.read_uint32(bytes)?),
                Ok(64) => msg.bid = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SubCmd0x501ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.idcId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.loginSigType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.loginSigTicket.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.requestFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serviceTypes.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.bid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.idcId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.appid { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.loginSigType { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.loginSigTicket { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.requestFlag { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        for s in &self.serviceTypes { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.bid { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SubCmd0x501RspBody<'a> {
    pub sigSession: Option<Cow<'a, [u8]>>,
    pub sessionKey: Option<Cow<'a, [u8]>>,
    pub addrs: Vec<SrvAddrs>,
}

impl<'a> MessageRead<'a> for SubCmd0x501RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.sigSession = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.sessionKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.addrs.push(r.read_message::<SrvAddrs>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SubCmd0x501RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.sigSession.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sessionKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.addrs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.sigSession { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sessionKey { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        for s in &self.addrs { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SrvAddrs {
    pub serviceType: Option<u32>,
    pub addrs: Vec<IpAddr>,
}

impl<'a> MessageRead<'a> for SrvAddrs {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.serviceType = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.addrs.push(r.read_message::<IpAddr>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SrvAddrs {
    fn get_size(&self) -> usize {
        0
        + self.serviceType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.addrs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.serviceType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.addrs { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IpAddr {
    pub type_pb: Option<u32>,
    pub ip: Option<u32>,
    pub port: Option<u32>,
    pub area: Option<u32>,
}

impl<'a> MessageRead<'a> for IpAddr {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(21) => msg.ip = Some(r.read_fixed32(bytes)?),
                Ok(24) => msg.port = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.area = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for IpAddr {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ip.as_ref().map_or(0, |_| 1 + 4)
        + self.port.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.area.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.ip { w.write_with_tag(21, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.port { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.area { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

