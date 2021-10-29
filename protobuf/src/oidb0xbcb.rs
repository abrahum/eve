// Automatically generated rust module for 'oidb0xbcb.proto' file

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
pub struct CheckUrlReq<'a> {
    pub url: Vec<Cow<'a, str>>,
    pub refer: Option<Cow<'a, str>>,
    pub plateform: Option<Cow<'a, str>>,
    pub qqPfTo: Option<Cow<'a, str>>,
    pub type_pb: Option<u32>,
    pub from: Option<u32>,
    pub chatid: Option<u64>,
    pub serviceType: Option<u64>,
    pub sendUin: Option<u64>,
    pub reqType: Option<Cow<'a, str>>,
    pub originalUrl: Option<Cow<'a, str>>,
    pub isArk: Option<bool>,
    pub arkName: Option<Cow<'a, str>>,
    pub isFinish: Option<bool>,
    pub srcUrls: Vec<Cow<'a, str>>,
    pub srcPlatform: Option<u32>,
    pub qua: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for CheckUrlReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.refer = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.plateform = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.qqPfTo = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.from = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.chatid = Some(r.read_uint64(bytes)?),
                Ok(64) => msg.serviceType = Some(r.read_uint64(bytes)?),
                Ok(72) => msg.sendUin = Some(r.read_uint64(bytes)?),
                Ok(82) => msg.reqType = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.originalUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.isArk = Some(r.read_bool(bytes)?),
                Ok(106) => msg.arkName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.isFinish = Some(r.read_bool(bytes)?),
                Ok(122) => msg.srcUrls.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.srcPlatform = Some(r.read_uint32(bytes)?),
                Ok(138) => msg.qua = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckUrlReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.url.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.refer.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.plateform.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.qqPfTo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.from.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chatid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serviceType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sendUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reqType.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.originalUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.isArk.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.arkName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.isFinish.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.srcUrls.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.srcPlatform.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.qua.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.url { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.refer { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.plateform { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.qqPfTo { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.from { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.chatid { w.write_with_tag(56, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.serviceType { w.write_with_tag(64, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sendUin { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.reqType { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.originalUrl { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.isArk { w.write_with_tag(96, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.arkName { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.isFinish { w.write_with_tag(112, |w| w.write_bool(*s))?; }
        for s in &self.srcUrls { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.srcPlatform { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.qua { w.write_with_tag(138, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckUrlReqItem<'a> {
    pub url: Option<Cow<'a, str>>,
    pub refer: Option<Cow<'a, str>>,
    pub plateform: Option<Cow<'a, str>>,
    pub qqPfTo: Option<Cow<'a, str>>,
    pub type_pb: Option<u32>,
    pub from: Option<u32>,
    pub chatid: Option<u64>,
    pub serviceType: Option<u64>,
    pub sendUin: Option<u64>,
    pub reqType: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for CheckUrlReqItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.refer = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.plateform = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.qqPfTo = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.from = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.chatid = Some(r.read_uint64(bytes)?),
                Ok(64) => msg.serviceType = Some(r.read_uint64(bytes)?),
                Ok(72) => msg.sendUin = Some(r.read_uint64(bytes)?),
                Ok(82) => msg.reqType = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckUrlReqItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.refer.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.plateform.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.qqPfTo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.from.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chatid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serviceType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sendUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reqType.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.url { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.refer { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.plateform { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.qqPfTo { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.from { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.chatid { w.write_with_tag(56, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.serviceType { w.write_with_tag(64, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sendUin { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.reqType { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckUrlRsp<'a> {
    pub results: Vec<UrlCheckResult<'a>>,
    pub nextReqDuration: Option<u32>,
}

impl<'a> MessageRead<'a> for CheckUrlRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.results.push(r.read_message::<UrlCheckResult>(bytes)?),
                Ok(16) => msg.nextReqDuration = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckUrlRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.nextReqDuration.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.results { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.nextReqDuration { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DBCBReqBody<'a> {
    pub notUseCache: Option<i32>,
    pub checkUrlReq: Option<CheckUrlReq<'a>>,
}

impl<'a> MessageRead<'a> for DBCBReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(72) => msg.notUseCache = Some(r.read_int32(bytes)?),
                Ok(82) => msg.checkUrlReq = Some(r.read_message::<CheckUrlReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DBCBReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.notUseCache.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.checkUrlReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.notUseCache { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.checkUrlReq { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DBCBRspBody<'a> {
    pub wording: Option<Cow<'a, str>>,
    pub checkUrlRsp: Option<CheckUrlRsp<'a>>,
}

impl<'a> MessageRead<'a> for DBCBRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.wording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.checkUrlRsp = Some(r.read_message::<CheckUrlRsp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DBCBRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.wording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.checkUrlRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.wording { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.checkUrlRsp { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UrlCheckResult<'a> {
    pub url: Option<Cow<'a, str>>,
    pub result: Option<u32>,
    pub jumpResult: Option<u32>,
    pub jumpUrl: Option<Cow<'a, str>>,
    pub level: Option<u32>,
    pub subLevel: Option<u32>,
    pub umrtype: Option<u32>,
    pub retFrom: Option<u32>,
    pub operationBit: Option<u64>,
}

impl<'a> MessageRead<'a> for UrlCheckResult<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.jumpResult = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.jumpUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.level = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.subLevel = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.umrtype = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.retFrom = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.operationBit = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UrlCheckResult<'a> {
    fn get_size(&self) -> usize {
        0
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.jumpResult.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.jumpUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.subLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.umrtype.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retFrom.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.operationBit.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.url { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.jumpResult { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.jumpUrl { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.level { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.subLevel { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.umrtype { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.retFrom { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.operationBit { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

