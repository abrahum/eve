// Automatically generated rust module for 'head.proto' file

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
pub struct C2CHead<'a> {
    pub toUin: Option<u64>,
    pub fromUin: Option<u64>,
    pub ccType: Option<u32>,
    pub ccCmd: Option<u32>,
    pub authPicSig: Option<Cow<'a, [u8]>>,
    pub authSig: Option<Cow<'a, [u8]>>,
    pub authBuf: Option<Cow<'a, [u8]>>,
    pub serverTime: Option<u32>,
    pub clientTime: Option<u32>,
    pub rand: Option<u32>,
    pub phoneNumber: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for C2CHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.toUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.ccType = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.ccCmd = Some(r.read_uint32(bytes)?),
                Ok(42) => msg.authPicSig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.authSig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.authBuf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.serverTime = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.clientTime = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.rand = Some(r.read_uint32(bytes)?),
                Ok(90) => msg.phoneNumber = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C2CHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ccType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ccCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.authPicSig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.authSig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.authBuf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.serverTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rand.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.phoneNumber.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.toUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fromUin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ccType { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.ccCmd { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.authPicSig { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.authSig { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.authBuf { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.serverTime { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.clientTime { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.rand { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.phoneNumber { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CSHead {
    pub uin: Option<u64>,
    pub command: Option<u32>,
    pub seq: Option<u32>,
    pub version: Option<u32>,
    pub retryTimes: Option<u32>,
    pub clientType: Option<u32>,
    pub pubno: Option<u32>,
    pub localid: Option<u32>,
    pub timezone: Option<u32>,
    pub clientIp: Option<u32>,
    pub clientPort: Option<u32>,
    pub connIp: Option<u32>,
    pub connPort: Option<u32>,
    pub interfaceIp: Option<u32>,
    pub interfacePort: Option<u32>,
    pub actualIp: Option<u32>,
    pub flag: Option<u32>,
    pub timestamp: Option<u32>,
    pub subcmd: Option<u32>,
    pub result: Option<u32>,
    pub appId: Option<u32>,
    pub instanceId: Option<u32>,
    pub sessionId: Option<u64>,
    pub idcId: Option<u32>,
}

impl<'a> MessageRead<'a> for CSHead {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.command = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.seq = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.version = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.retryTimes = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.clientType = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.pubno = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.localid = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.timezone = Some(r.read_uint32(bytes)?),
                Ok(85) => msg.clientIp = Some(r.read_fixed32(bytes)?),
                Ok(88) => msg.clientPort = Some(r.read_uint32(bytes)?),
                Ok(101) => msg.connIp = Some(r.read_fixed32(bytes)?),
                Ok(104) => msg.connPort = Some(r.read_uint32(bytes)?),
                Ok(117) => msg.interfaceIp = Some(r.read_fixed32(bytes)?),
                Ok(120) => msg.interfacePort = Some(r.read_uint32(bytes)?),
                Ok(133) => msg.actualIp = Some(r.read_fixed32(bytes)?),
                Ok(136) => msg.flag = Some(r.read_uint32(bytes)?),
                Ok(149) => msg.timestamp = Some(r.read_fixed32(bytes)?),
                Ok(152) => msg.subcmd = Some(r.read_uint32(bytes)?),
                Ok(160) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(168) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(176) => msg.instanceId = Some(r.read_uint32(bytes)?),
                Ok(184) => msg.sessionId = Some(r.read_uint64(bytes)?),
                Ok(192) => msg.idcId = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CSHead {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.seq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retryTimes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pubno.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.localid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.timezone.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientIp.as_ref().map_or(0, |_| 1 + 4)
        + self.clientPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.connIp.as_ref().map_or(0, |_| 1 + 4)
        + self.connPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.interfaceIp.as_ref().map_or(0, |_| 1 + 4)
        + self.interfacePort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.actualIp.as_ref().map_or(0, |_| 2 + 4)
        + self.flag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.timestamp.as_ref().map_or(0, |_| 2 + 4)
        + self.subcmd.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.instanceId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.sessionId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.idcId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.command { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.seq { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.retryTimes { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.clientType { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.pubno { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.localid { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.timezone { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.clientIp { w.write_with_tag(85, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.clientPort { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.connIp { w.write_with_tag(101, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.connPort { w.write_with_tag(104, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.interfaceIp { w.write_with_tag(117, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.interfacePort { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.actualIp { w.write_with_tag(133, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(136, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(149, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.subcmd { w.write_with_tag(152, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(160, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(168, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.instanceId { w.write_with_tag(176, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sessionId { w.write_with_tag(184, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.idcId { w.write_with_tag(192, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeltaHead<'a> {
    pub totalLen: Option<u64>,
    pub offset: Option<u64>,
    pub ackOffset: Option<u64>,
    pub cookie: Option<Cow<'a, [u8]>>,
    pub ackCookie: Option<Cow<'a, [u8]>>,
    pub result: Option<u32>,
    pub flags: Option<u32>,
}

impl<'a> MessageRead<'a> for DeltaHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.totalLen = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.offset = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.ackOffset = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.cookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.ackCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.flags = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeltaHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.totalLen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.offset.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ackOffset.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ackCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flags.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.totalLen { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.offset { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ackOffset { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cookie { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.ackCookie { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.flags { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IMHead<'a> {
    pub headType: Option<u32>,
    pub csHead: Option<CSHead>,
    pub s2CHead: Option<S2CHead>,
    pub httpconnHead: Option<HttpConnHead<'a>>,
    pub paintFlag: Option<u32>,
    pub loginSig: Option<LoginSig<'a>>,
    pub deltaHead: Option<DeltaHead<'a>>,
    pub c2CHead: Option<C2CHead<'a>>,
}

impl<'a> MessageRead<'a> for IMHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.headType = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.csHead = Some(r.read_message::<CSHead>(bytes)?),
                Ok(26) => msg.s2CHead = Some(r.read_message::<S2CHead>(bytes)?),
                Ok(34) => msg.httpconnHead = Some(r.read_message::<HttpConnHead>(bytes)?),
                Ok(40) => msg.paintFlag = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.loginSig = Some(r.read_message::<LoginSig>(bytes)?),
                Ok(58) => msg.deltaHead = Some(r.read_message::<DeltaHead>(bytes)?),
                Ok(66) => msg.c2CHead = Some(r.read_message::<C2CHead>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IMHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.headType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.csHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.s2CHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.httpconnHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.paintFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.loginSig.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deltaHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.c2CHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.headType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.csHead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.s2CHead { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.httpconnHead { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.paintFlag { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.loginSig { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deltaHead { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.c2CHead { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HttpConnHead<'a> {
    pub uin: Option<u64>,
    pub command: Option<u32>,
    pub subCommand: Option<u32>,
    pub seq: Option<u32>,
    pub version: Option<u32>,
    pub retryTimes: Option<u32>,
    pub clientType: Option<u32>,
    pub pubNo: Option<u32>,
    pub localId: Option<u32>,
    pub timeZone: Option<u32>,
    pub clientIp: Option<u32>,
    pub clientPort: Option<u32>,
    pub qzhttpIp: Option<u32>,
    pub qzhttpPort: Option<u32>,
    pub sppIp: Option<u32>,
    pub sppPort: Option<u32>,
    pub flag: Option<u32>,
    pub key: Option<Cow<'a, [u8]>>,
    pub compressType: Option<u32>,
    pub originSize: Option<u32>,
    pub errorCode: Option<u32>,
    pub redirect: Option<RedirectMsg>,
    pub commandId: Option<u32>,
    pub serviceCmdid: Option<u32>,
    pub oidbhead: Option<TransOidbHead<'a>>,
}

impl<'a> MessageRead<'a> for HttpConnHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.command = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.subCommand = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.seq = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.version = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.retryTimes = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.clientType = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.pubNo = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.localId = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.timeZone = Some(r.read_uint32(bytes)?),
                Ok(93) => msg.clientIp = Some(r.read_fixed32(bytes)?),
                Ok(96) => msg.clientPort = Some(r.read_uint32(bytes)?),
                Ok(109) => msg.qzhttpIp = Some(r.read_fixed32(bytes)?),
                Ok(112) => msg.qzhttpPort = Some(r.read_uint32(bytes)?),
                Ok(125) => msg.sppIp = Some(r.read_fixed32(bytes)?),
                Ok(128) => msg.sppPort = Some(r.read_uint32(bytes)?),
                Ok(136) => msg.flag = Some(r.read_uint32(bytes)?),
                Ok(146) => msg.key = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(152) => msg.compressType = Some(r.read_uint32(bytes)?),
                Ok(160) => msg.originSize = Some(r.read_uint32(bytes)?),
                Ok(168) => msg.errorCode = Some(r.read_uint32(bytes)?),
                Ok(178) => msg.redirect = Some(r.read_message::<RedirectMsg>(bytes)?),
                Ok(184) => msg.commandId = Some(r.read_uint32(bytes)?),
                Ok(192) => msg.serviceCmdid = Some(r.read_uint32(bytes)?),
                Ok(202) => msg.oidbhead = Some(r.read_message::<TransOidbHead>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HttpConnHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.subCommand.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.seq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retryTimes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pubNo.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.localId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.timeZone.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientIp.as_ref().map_or(0, |_| 1 + 4)
        + self.clientPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.qzhttpIp.as_ref().map_or(0, |_| 1 + 4)
        + self.qzhttpPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sppIp.as_ref().map_or(0, |_| 1 + 4)
        + self.sppPort.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.flag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.key.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.compressType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.originSize.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.errorCode.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.redirect.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.commandId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.serviceCmdid.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.oidbhead.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.command { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.subCommand { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.seq { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.retryTimes { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.clientType { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.pubNo { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.localId { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.timeZone { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.clientIp { w.write_with_tag(93, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.clientPort { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.qzhttpIp { w.write_with_tag(109, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.qzhttpPort { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sppIp { w.write_with_tag(125, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.sppPort { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(136, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.key { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.compressType { w.write_with_tag(152, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.originSize { w.write_with_tag(160, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errorCode { w.write_with_tag(168, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.redirect { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.commandId { w.write_with_tag(184, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.serviceCmdid { w.write_with_tag(192, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.oidbhead { w.write_with_tag(202, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoginSig<'a> {
    pub type_pb: Option<u32>,
    pub sig: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for LoginSig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LoginSig<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RedirectMsg {
    pub lastRedirectIp: Option<u32>,
    pub lastRedirectPort: Option<u32>,
    pub redirectIp: Option<u32>,
    pub redirectPort: Option<u32>,
    pub redirectCount: Option<u32>,
}

impl<'a> MessageRead<'a> for RedirectMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.lastRedirectIp = Some(r.read_fixed32(bytes)?),
                Ok(16) => msg.lastRedirectPort = Some(r.read_uint32(bytes)?),
                Ok(29) => msg.redirectIp = Some(r.read_fixed32(bytes)?),
                Ok(32) => msg.redirectPort = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.redirectCount = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RedirectMsg {
    fn get_size(&self) -> usize {
        0
        + self.lastRedirectIp.as_ref().map_or(0, |_| 1 + 4)
        + self.lastRedirectPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.redirectIp.as_ref().map_or(0, |_| 1 + 4)
        + self.redirectPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.redirectCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.lastRedirectIp { w.write_with_tag(13, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.lastRedirectPort { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.redirectIp { w.write_with_tag(29, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.redirectPort { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.redirectCount { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct S2CHead {
    pub subMsgtype: Option<u32>,
    pub msgType: Option<u32>,
    pub fromUin: Option<u64>,
    pub msgId: Option<u32>,
    pub relayIp: Option<u32>,
    pub relayPort: Option<u32>,
    pub toUin: Option<u64>,
}

impl<'a> MessageRead<'a> for S2CHead {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subMsgtype = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.msgType = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.msgId = Some(r.read_uint32(bytes)?),
                Ok(45) => msg.relayIp = Some(r.read_fixed32(bytes)?),
                Ok(48) => msg.relayPort = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.toUin = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for S2CHead {
    fn get_size(&self) -> usize {
        0
        + self.subMsgtype.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.relayIp.as_ref().map_or(0, |_| 1 + 4)
        + self.relayPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subMsgtype { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.msgType { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fromUin { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.msgId { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.relayIp { w.write_with_tag(45, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.relayPort { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(56, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransOidbHead<'a> {
    pub command: Option<u32>,
    pub serviceType: Option<u32>,
    pub result: Option<u32>,
    pub errorMsg: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for TransOidbHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.command = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.serviceType = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.errorMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransOidbHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.command.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serviceType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errorMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.command { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.serviceType { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errorMsg { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

