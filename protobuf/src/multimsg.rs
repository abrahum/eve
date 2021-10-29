// Automatically generated rust module for 'multimsg.proto' file

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
pub struct ExternMsg {
    pub channelType: i32,
}

impl<'a> MessageRead<'a> for ExternMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channelType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ExternMsg {
    fn get_size(&self) -> usize {
        0
        + if self.channelType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.channelType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channelType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.channelType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MultiMsgApplyDownReq<'a> {
    pub msgResid: Cow<'a, [u8]>,
    pub msgType: i32,
    pub srcUin: i64,
}

impl<'a> MessageRead<'a> for MultiMsgApplyDownReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msgResid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.msgType = r.read_int32(bytes)?,
                Ok(24) => msg.srcUin = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MultiMsgApplyDownReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.msgResid == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgResid).len()) }
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.srcUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.srcUin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgResid != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.msgResid))?; }
        if self.msgType != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.msgType))?; }
        if self.srcUin != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.srcUin))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MultiMsgApplyDownRsp<'a> {
    pub result: i32,
    pub thumbDownPara: Cow<'a, [u8]>,
    pub msgKey: Cow<'a, [u8]>,
    pub uint32DownIp: Vec<i32>,
    pub uint32DownPort: Vec<i32>,
    pub msgResid: Cow<'a, [u8]>,
    pub msgExternInfo: Option<ExternMsg>,
    pub bytesDownIpV6: Vec<Cow<'a, [u8]>>,
    pub uint32DownV6Port: Vec<i32>,
}

impl<'a> MessageRead<'a> for MultiMsgApplyDownRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = r.read_int32(bytes)?,
                Ok(18) => msg.thumbDownPara = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.msgKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.uint32DownIp = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(42) => msg.uint32DownPort = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(50) => msg.msgResid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.msgExternInfo = Some(r.read_message::<ExternMsg>(bytes)?),
                Ok(66) => msg.bytesDownIpV6.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.uint32DownV6Port = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MultiMsgApplyDownRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.thumbDownPara == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.thumbDownPara).len()) }
        + if self.msgKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgKey).len()) }
        + if self.uint32DownIp.is_empty() { 0 } else { 1 + sizeof_len(self.uint32DownIp.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.uint32DownPort.is_empty() { 0 } else { 1 + sizeof_len(self.uint32DownPort.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.msgResid == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgResid).len()) }
        + self.msgExternInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.bytesDownIpV6.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.uint32DownV6Port.is_empty() { 0 } else { 1 + sizeof_len(self.uint32DownV6Port.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.result != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.result))?; }
        if self.thumbDownPara != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.thumbDownPara))?; }
        if self.msgKey != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.msgKey))?; }
        w.write_packed_with_tag(34, &self.uint32DownIp, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(42, &self.uint32DownPort, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.msgResid != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.msgResid))?; }
        if let Some(ref s) = self.msgExternInfo { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.bytesDownIpV6 { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        w.write_packed_with_tag(74, &self.uint32DownV6Port, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MultiMsgApplyUpReq<'a> {
    pub dstUin: i64,
    pub msgSize: i64,
    pub msgMd5: Cow<'a, [u8]>,
    pub msgType: i32,
    pub applyId: i32,
}

impl<'a> MessageRead<'a> for MultiMsgApplyUpReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.dstUin = r.read_int64(bytes)?,
                Ok(16) => msg.msgSize = r.read_int64(bytes)?,
                Ok(26) => msg.msgMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.msgType = r.read_int32(bytes)?,
                Ok(40) => msg.applyId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MultiMsgApplyUpReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.dstUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.dstUin) as u64) }
        + if self.msgSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgSize) as u64) }
        + if self.msgMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgMd5).len()) }
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.applyId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.applyId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.dstUin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.dstUin))?; }
        if self.msgSize != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.msgSize))?; }
        if self.msgMd5 != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.msgMd5))?; }
        if self.msgType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.msgType))?; }
        if self.applyId != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.applyId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MultiMsgApplyUpRsp<'a> {
    pub result: i32,
    pub msgResid: Cow<'a, str>,
    pub msgUkey: Cow<'a, [u8]>,
    pub uint32UpIp: Vec<i32>,
    pub uint32UpPort: Vec<i32>,
    pub blockSize: i64,
    pub upOffset: i64,
    pub applyId: i32,
    pub msgKey: Cow<'a, [u8]>,
    pub msgSig: Cow<'a, [u8]>,
    pub msgExternInfo: Option<ExternMsg>,
    pub bytesUpIpV6: Vec<Cow<'a, [u8]>>,
    pub uint32UpV6Port: Vec<i32>,
}

impl<'a> MessageRead<'a> for MultiMsgApplyUpRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = r.read_int32(bytes)?,
                Ok(18) => msg.msgResid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.msgUkey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.uint32UpIp = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(42) => msg.uint32UpPort = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(48) => msg.blockSize = r.read_int64(bytes)?,
                Ok(56) => msg.upOffset = r.read_int64(bytes)?,
                Ok(64) => msg.applyId = r.read_int32(bytes)?,
                Ok(74) => msg.msgKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.msgSig = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.msgExternInfo = Some(r.read_message::<ExternMsg>(bytes)?),
                Ok(98) => msg.bytesUpIpV6.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.uint32UpV6Port = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MultiMsgApplyUpRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.msgResid == "" { 0 } else { 1 + sizeof_len((&self.msgResid).len()) }
        + if self.msgUkey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgUkey).len()) }
        + if self.uint32UpIp.is_empty() { 0 } else { 1 + sizeof_len(self.uint32UpIp.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.uint32UpPort.is_empty() { 0 } else { 1 + sizeof_len(self.uint32UpPort.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.blockSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.blockSize) as u64) }
        + if self.upOffset == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.upOffset) as u64) }
        + if self.applyId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.applyId) as u64) }
        + if self.msgKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgKey).len()) }
        + if self.msgSig == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgSig).len()) }
        + self.msgExternInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.bytesUpIpV6.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.uint32UpV6Port.is_empty() { 0 } else { 1 + sizeof_len(self.uint32UpV6Port.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.result != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.result))?; }
        if self.msgResid != "" { w.write_with_tag(18, |w| w.write_string(&**&self.msgResid))?; }
        if self.msgUkey != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.msgUkey))?; }
        w.write_packed_with_tag(34, &self.uint32UpIp, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(42, &self.uint32UpPort, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.blockSize != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.blockSize))?; }
        if self.upOffset != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.upOffset))?; }
        if self.applyId != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.applyId))?; }
        if self.msgKey != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.msgKey))?; }
        if self.msgSig != Cow::Borrowed(b"") { w.write_with_tag(82, |w| w.write_bytes(&**&self.msgSig))?; }
        if let Some(ref s) = self.msgExternInfo { w.write_with_tag(90, |w| w.write_message(s))?; }
        for s in &self.bytesUpIpV6 { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        w.write_packed_with_tag(106, &self.uint32UpV6Port, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MultiReqBody<'a> {
    pub subcmd: i32,
    pub termType: i32,
    pub platformType: i32,
    pub netType: i32,
    pub buildVer: Cow<'a, str>,
    pub multimsgApplyupReq: Vec<MultiMsgApplyUpReq<'a>>,
    pub multimsgApplydownReq: Vec<MultiMsgApplyDownReq<'a>>,
    pub buType: i32,
    pub reqChannelType: i32,
}

impl<'a> MessageRead<'a> for MultiReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subcmd = r.read_int32(bytes)?,
                Ok(16) => msg.termType = r.read_int32(bytes)?,
                Ok(24) => msg.platformType = r.read_int32(bytes)?,
                Ok(32) => msg.netType = r.read_int32(bytes)?,
                Ok(42) => msg.buildVer = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.multimsgApplyupReq.push(r.read_message::<MultiMsgApplyUpReq>(bytes)?),
                Ok(58) => msg.multimsgApplydownReq.push(r.read_message::<MultiMsgApplyDownReq>(bytes)?),
                Ok(64) => msg.buType = r.read_int32(bytes)?,
                Ok(72) => msg.reqChannelType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MultiReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.subcmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subcmd) as u64) }
        + if self.termType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.termType) as u64) }
        + if self.platformType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.platformType) as u64) }
        + if self.netType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.netType) as u64) }
        + if self.buildVer == "" { 0 } else { 1 + sizeof_len((&self.buildVer).len()) }
        + self.multimsgApplyupReq.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.multimsgApplydownReq.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.buType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.buType) as u64) }
        + if self.reqChannelType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqChannelType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subcmd != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.subcmd))?; }
        if self.termType != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.termType))?; }
        if self.platformType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.platformType))?; }
        if self.netType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.netType))?; }
        if self.buildVer != "" { w.write_with_tag(42, |w| w.write_string(&**&self.buildVer))?; }
        for s in &self.multimsgApplyupReq { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.multimsgApplydownReq { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.buType != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.buType))?; }
        if self.reqChannelType != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.reqChannelType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MultiRspBody<'a> {
    pub subcmd: i32,
    pub multimsgApplyupRsp: Vec<MultiMsgApplyUpRsp<'a>>,
    pub multimsgApplydownRsp: Vec<MultiMsgApplyDownRsp<'a>>,
}

impl<'a> MessageRead<'a> for MultiRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subcmd = r.read_int32(bytes)?,
                Ok(18) => msg.multimsgApplyupRsp.push(r.read_message::<MultiMsgApplyUpRsp>(bytes)?),
                Ok(26) => msg.multimsgApplydownRsp.push(r.read_message::<MultiMsgApplyDownRsp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MultiRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.subcmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subcmd) as u64) }
        + self.multimsgApplyupRsp.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.multimsgApplydownRsp.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subcmd != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.subcmd))?; }
        for s in &self.multimsgApplyupRsp { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.multimsgApplydownRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

