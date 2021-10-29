// Automatically generated rust module for 'data.proto' file

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
pub struct DeviceInfo<'a> {
    pub bootloader: Cow<'a, str>,
    pub procVersion: Cow<'a, str>,
    pub codename: Cow<'a, str>,
    pub incremental: Cow<'a, str>,
    pub fingerprint: Cow<'a, str>,
    pub bootId: Cow<'a, str>,
    pub androidId: Cow<'a, str>,
    pub baseBand: Cow<'a, str>,
    pub innerVersion: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DeviceInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.bootloader = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.procVersion = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.codename = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.incremental = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.fingerprint = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.bootId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.androidId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.baseBand = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.innerVersion = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.bootloader == "" { 0 } else { 1 + sizeof_len((&self.bootloader).len()) }
        + if self.procVersion == "" { 0 } else { 1 + sizeof_len((&self.procVersion).len()) }
        + if self.codename == "" { 0 } else { 1 + sizeof_len((&self.codename).len()) }
        + if self.incremental == "" { 0 } else { 1 + sizeof_len((&self.incremental).len()) }
        + if self.fingerprint == "" { 0 } else { 1 + sizeof_len((&self.fingerprint).len()) }
        + if self.bootId == "" { 0 } else { 1 + sizeof_len((&self.bootId).len()) }
        + if self.androidId == "" { 0 } else { 1 + sizeof_len((&self.androidId).len()) }
        + if self.baseBand == "" { 0 } else { 1 + sizeof_len((&self.baseBand).len()) }
        + if self.innerVersion == "" { 0 } else { 1 + sizeof_len((&self.innerVersion).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.bootloader != "" { w.write_with_tag(10, |w| w.write_string(&**&self.bootloader))?; }
        if self.procVersion != "" { w.write_with_tag(18, |w| w.write_string(&**&self.procVersion))?; }
        if self.codename != "" { w.write_with_tag(26, |w| w.write_string(&**&self.codename))?; }
        if self.incremental != "" { w.write_with_tag(34, |w| w.write_string(&**&self.incremental))?; }
        if self.fingerprint != "" { w.write_with_tag(42, |w| w.write_string(&**&self.fingerprint))?; }
        if self.bootId != "" { w.write_with_tag(50, |w| w.write_string(&**&self.bootId))?; }
        if self.androidId != "" { w.write_with_tag(58, |w| w.write_string(&**&self.androidId))?; }
        if self.baseBand != "" { w.write_with_tag(66, |w| w.write_string(&**&self.baseBand))?; }
        if self.innerVersion != "" { w.write_with_tag(74, |w| w.write_string(&**&self.innerVersion))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestBody {
    pub rpt_config_list: Vec<ConfigSeq>,
}

impl<'a> MessageRead<'a> for RequestBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.rpt_config_list.push(r.read_message::<ConfigSeq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestBody {
    fn get_size(&self) -> usize {
        0
        + self.rpt_config_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.rpt_config_list { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConfigSeq {
    pub type_pb: i32,
    pub version: i32,
}

impl<'a> MessageRead<'a> for ConfigSeq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_int32(bytes)?,
                Ok(16) => msg.version = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ConfigSeq {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.type_pb))?; }
        if self.version != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.version))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D50ReqBody {
    pub appid: i64,
    pub maxPkgSize: i32,
    pub startTime: i32,
    pub startIndex: i32,
    pub reqNum: i32,
    pub uinList: Vec<i64>,
    pub reqMusicSwitch: i32,
    pub reqMutualmarkAlienation: i32,
    pub reqMutualmarkScore: i32,
    pub reqKsingSwitch: i32,
    pub reqMutualmarkLbsshare: i32,
}

impl<'a> MessageRead<'a> for D50ReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.appid = r.read_int64(bytes)?,
                Ok(16) => msg.maxPkgSize = r.read_int32(bytes)?,
                Ok(24) => msg.startTime = r.read_int32(bytes)?,
                Ok(32) => msg.startIndex = r.read_int32(bytes)?,
                Ok(40) => msg.reqNum = r.read_int32(bytes)?,
                Ok(50) => msg.uinList = r.read_packed(bytes, |r, bytes| Ok(r.read_int64(bytes)?))?,
                Ok(728008) => msg.reqMusicSwitch = r.read_int32(bytes)?,
                Ok(808008) => msg.reqMutualmarkAlienation = r.read_int32(bytes)?,
                Ok(1128008) => msg.reqMutualmarkScore = r.read_int32(bytes)?,
                Ok(1208008) => msg.reqKsingSwitch = r.read_int32(bytes)?,
                Ok(1448008) => msg.reqMutualmarkLbsshare = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D50ReqBody {
    fn get_size(&self) -> usize {
        0
        + if self.appid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.appid) as u64) }
        + if self.maxPkgSize == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.maxPkgSize) as u64) }
        + if self.startTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.startTime) as u64) }
        + if self.startIndex == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.startIndex) as u64) }
        + if self.reqNum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqNum) as u64) }
        + if self.uinList.is_empty() { 0 } else { 1 + sizeof_len(self.uinList.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.reqMusicSwitch == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.reqMusicSwitch) as u64) }
        + if self.reqMutualmarkAlienation == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.reqMutualmarkAlienation) as u64) }
        + if self.reqMutualmarkScore == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.reqMutualmarkScore) as u64) }
        + if self.reqKsingSwitch == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.reqKsingSwitch) as u64) }
        + if self.reqMutualmarkLbsshare == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.reqMutualmarkLbsshare) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.appid != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.appid))?; }
        if self.maxPkgSize != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.maxPkgSize))?; }
        if self.startTime != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.startTime))?; }
        if self.startIndex != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.startIndex))?; }
        if self.reqNum != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.reqNum))?; }
        w.write_packed_with_tag(50, &self.uinList, |w, m| w.write_int64(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.reqMusicSwitch != 0i32 { w.write_with_tag(728008, |w| w.write_int32(*&self.reqMusicSwitch))?; }
        if self.reqMutualmarkAlienation != 0i32 { w.write_with_tag(808008, |w| w.write_int32(*&self.reqMutualmarkAlienation))?; }
        if self.reqMutualmarkScore != 0i32 { w.write_with_tag(1128008, |w| w.write_int32(*&self.reqMutualmarkScore))?; }
        if self.reqKsingSwitch != 0i32 { w.write_with_tag(1208008, |w| w.write_int32(*&self.reqKsingSwitch))?; }
        if self.reqMutualmarkLbsshare != 0i32 { w.write_with_tag(1448008, |w| w.write_int32(*&self.reqMutualmarkLbsshare))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D388ReqBody<'a> {
    pub netType: i32,
    pub subcmd: i32,
    pub msgTryUpImgReq: Vec<TryUpImgReq<'a>>,
    pub msgTryUpPttReq: Vec<TryUpPttReq<'a>>,
    pub msgGetPttReq: Vec<GetPttUrlReq<'a>>,
    pub commandId: i32,
    pub extension: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for D388ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.netType = r.read_int32(bytes)?,
                Ok(16) => msg.subcmd = r.read_int32(bytes)?,
                Ok(26) => msg.msgTryUpImgReq.push(r.read_message::<TryUpImgReq>(bytes)?),
                Ok(42) => msg.msgTryUpPttReq.push(r.read_message::<TryUpPttReq>(bytes)?),
                Ok(50) => msg.msgGetPttReq.push(r.read_message::<GetPttUrlReq>(bytes)?),
                Ok(56) => msg.commandId = r.read_int32(bytes)?,
                Ok(8010) => msg.extension = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D388ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.netType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.netType) as u64) }
        + if self.subcmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subcmd) as u64) }
        + self.msgTryUpImgReq.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgTryUpPttReq.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgGetPttReq.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.commandId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.commandId) as u64) }
        + if self.extension == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.extension).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.netType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.netType))?; }
        if self.subcmd != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.subcmd))?; }
        for s in &self.msgTryUpImgReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.msgTryUpPttReq { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.msgGetPttReq { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.commandId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.commandId))?; }
        if self.extension != Cow::Borrowed(b"") { w.write_with_tag(8010, |w| w.write_bytes(&**&self.extension))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D388RespBody<'a> {
    pub clientIp: i32,
    pub subCmd: i32,
    pub msgTryUpImgRsp: Vec<TryUpImgResp<'a>>,
    pub msgTryUpPttRsp: Vec<TryUpPttResp<'a>>,
    pub msgGetPttUrlRsp: Vec<GetPttUrlRsp<'a>>,
}

impl<'a> MessageRead<'a> for D388RespBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.clientIp = r.read_int32(bytes)?,
                Ok(16) => msg.subCmd = r.read_int32(bytes)?,
                Ok(26) => msg.msgTryUpImgRsp.push(r.read_message::<TryUpImgResp>(bytes)?),
                Ok(42) => msg.msgTryUpPttRsp.push(r.read_message::<TryUpPttResp>(bytes)?),
                Ok(50) => msg.msgGetPttUrlRsp.push(r.read_message::<GetPttUrlRsp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D388RespBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.clientIp == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.clientIp) as u64) }
        + if self.subCmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subCmd) as u64) }
        + self.msgTryUpImgRsp.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgTryUpPttRsp.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgGetPttUrlRsp.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.clientIp != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.clientIp))?; }
        if self.subCmd != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.subCmd))?; }
        for s in &self.msgTryUpImgRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.msgTryUpPttRsp { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.msgGetPttUrlRsp { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetPttUrlReq<'a> {
    pub groupCode: i64,
    pub dstUin: i64,
    pub fileId: i64,
    pub fileMd5: Cow<'a, [u8]>,
    pub reqTerm: i32,
    pub reqPlatformType: i32,
    pub innerIp: i32,
    pub buType: i32,
    pub buildVer: Cow<'a, [u8]>,
    pub fileKey: Cow<'a, [u8]>,
    pub codec: i32,
    pub buId: i32,
    pub reqTransferType: i32,
    pub isAuto: i32,
}

impl<'a> MessageRead<'a> for GetPttUrlReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.dstUin = r.read_int64(bytes)?,
                Ok(24) => msg.fileId = r.read_int64(bytes)?,
                Ok(34) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.reqTerm = r.read_int32(bytes)?,
                Ok(48) => msg.reqPlatformType = r.read_int32(bytes)?,
                Ok(56) => msg.innerIp = r.read_int32(bytes)?,
                Ok(64) => msg.buType = r.read_int32(bytes)?,
                Ok(74) => msg.buildVer = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.fileKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(96) => msg.codec = r.read_int32(bytes)?,
                Ok(104) => msg.buId = r.read_int32(bytes)?,
                Ok(112) => msg.reqTransferType = r.read_int32(bytes)?,
                Ok(120) => msg.isAuto = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetPttUrlReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.dstUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.dstUin) as u64) }
        + if self.fileId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileId) as u64) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.reqTerm == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqTerm) as u64) }
        + if self.reqPlatformType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqPlatformType) as u64) }
        + if self.innerIp == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.innerIp) as u64) }
        + if self.buType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.buType) as u64) }
        + if self.buildVer == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.buildVer).len()) }
        + if self.fileKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileKey).len()) }
        + if self.codec == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.codec) as u64) }
        + if self.buId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.buId) as u64) }
        + if self.reqTransferType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqTransferType) as u64) }
        + if self.isAuto == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.isAuto) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.dstUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.dstUin))?; }
        if self.fileId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.fileId))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.reqTerm != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.reqTerm))?; }
        if self.reqPlatformType != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.reqPlatformType))?; }
        if self.innerIp != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.innerIp))?; }
        if self.buType != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.buType))?; }
        if self.buildVer != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.buildVer))?; }
        if self.fileKey != Cow::Borrowed(b"") { w.write_with_tag(90, |w| w.write_bytes(&**&self.fileKey))?; }
        if self.codec != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.codec))?; }
        if self.buId != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.buId))?; }
        if self.reqTransferType != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.reqTransferType))?; }
        if self.isAuto != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.isAuto))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetPttUrlRsp<'a> {
    pub fileId: i64,
    pub fileMd5: Cow<'a, [u8]>,
    pub result: i32,
    pub failMsg: Cow<'a, [u8]>,
    pub bytesDownUrl: Cow<'a, [u8]>,
    pub uint32DownIp: Vec<i32>,
    pub uint32DownPort: Vec<i32>,
    pub downDomain: Cow<'a, [u8]>,
    pub downPara: Cow<'a, [u8]>,
    pub transferType: i32,
    pub allowRetry: i32,
    pub clientIp6: Cow<'a, [u8]>,
    pub strDomain: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for GetPttUrlRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fileId = r.read_int64(bytes)?,
                Ok(18) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.result = r.read_int32(bytes)?,
                Ok(34) => msg.failMsg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.bytesDownUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.uint32DownIp = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(58) => msg.uint32DownPort = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(66) => msg.downDomain = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.downPara = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(88) => msg.transferType = r.read_int32(bytes)?,
                Ok(96) => msg.allowRetry = r.read_int32(bytes)?,
                Ok(218) => msg.clientIp6 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(226) => msg.strDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetPttUrlRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fileId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileId) as u64) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.failMsg == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.failMsg).len()) }
        + if self.bytesDownUrl == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bytesDownUrl).len()) }
        + if self.uint32DownIp.is_empty() { 0 } else { 1 + sizeof_len(self.uint32DownIp.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.uint32DownPort.is_empty() { 0 } else { 1 + sizeof_len(self.uint32DownPort.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.downDomain == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.downDomain).len()) }
        + if self.downPara == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.downPara).len()) }
        + if self.transferType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.transferType) as u64) }
        + if self.allowRetry == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.allowRetry) as u64) }
        + if self.clientIp6 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.clientIp6).len()) }
        + if self.strDomain == "" { 0 } else { 2 + sizeof_len((&self.strDomain).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fileId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.fileId))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.result != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.result))?; }
        if self.failMsg != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.failMsg))?; }
        if self.bytesDownUrl != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.bytesDownUrl))?; }
        w.write_packed_with_tag(50, &self.uint32DownIp, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(58, &self.uint32DownPort, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.downDomain != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.downDomain))?; }
        if self.downPara != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.downPara))?; }
        if self.transferType != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.transferType))?; }
        if self.allowRetry != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.allowRetry))?; }
        if self.clientIp6 != Cow::Borrowed(b"") { w.write_with_tag(218, |w| w.write_bytes(&**&self.clientIp6))?; }
        if self.strDomain != "" { w.write_with_tag(226, |w| w.write_string(&**&self.strDomain))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqDataHighwayHead<'a> {
    pub msgBasehead: Option<DataHighwayHead<'a>>,
    pub msgSeghead: Option<SegHead<'a>>,
    pub reqExtendinfo: Cow<'a, [u8]>,
    pub timestamp: i64,
}

impl<'a> MessageRead<'a> for ReqDataHighwayHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msgBasehead = Some(r.read_message::<DataHighwayHead>(bytes)?),
                Ok(18) => msg.msgSeghead = Some(r.read_message::<SegHead>(bytes)?),
                Ok(26) => msg.reqExtendinfo = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.timestamp = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReqDataHighwayHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msgBasehead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgSeghead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.reqExtendinfo == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.reqExtendinfo).len()) }
        + if self.timestamp == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msgBasehead { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgSeghead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.reqExtendinfo != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.reqExtendinfo))?; }
        if self.timestamp != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.timestamp))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspDataHighwayHead<'a> {
    pub msgBasehead: Option<DataHighwayHead<'a>>,
    pub msgSeghead: Option<SegHead<'a>>,
    pub errorCode: i32,
    pub allowRetry: i32,
    pub cachecost: i32,
    pub htcost: i32,
    pub rspExtendinfo: Cow<'a, [u8]>,
    pub timestamp: i64,
    pub range: i64,
    pub isReset: i32,
}

impl<'a> MessageRead<'a> for RspDataHighwayHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msgBasehead = Some(r.read_message::<DataHighwayHead>(bytes)?),
                Ok(18) => msg.msgSeghead = Some(r.read_message::<SegHead>(bytes)?),
                Ok(24) => msg.errorCode = r.read_int32(bytes)?,
                Ok(32) => msg.allowRetry = r.read_int32(bytes)?,
                Ok(40) => msg.cachecost = r.read_int32(bytes)?,
                Ok(48) => msg.htcost = r.read_int32(bytes)?,
                Ok(58) => msg.rspExtendinfo = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.timestamp = r.read_int64(bytes)?,
                Ok(72) => msg.range = r.read_int64(bytes)?,
                Ok(80) => msg.isReset = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspDataHighwayHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msgBasehead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgSeghead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.errorCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.errorCode) as u64) }
        + if self.allowRetry == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.allowRetry) as u64) }
        + if self.cachecost == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.cachecost) as u64) }
        + if self.htcost == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.htcost) as u64) }
        + if self.rspExtendinfo == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.rspExtendinfo).len()) }
        + if self.timestamp == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.range == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.range) as u64) }
        + if self.isReset == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.isReset) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msgBasehead { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgSeghead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.errorCode != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.errorCode))?; }
        if self.allowRetry != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.allowRetry))?; }
        if self.cachecost != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.cachecost))?; }
        if self.htcost != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.htcost))?; }
        if self.rspExtendinfo != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.rspExtendinfo))?; }
        if self.timestamp != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.timestamp))?; }
        if self.range != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.range))?; }
        if self.isReset != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.isReset))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DataHighwayHead<'a> {
    pub version: i32,
    pub uin: Cow<'a, str>,
    pub command: Cow<'a, str>,
    pub seq: i32,
    pub retryTimes: i32,
    pub appid: i32,
    pub dataflag: i32,
    pub commandId: i32,
    pub buildVer: Cow<'a, str>,
    pub localeId: i32,
}

impl<'a> MessageRead<'a> for DataHighwayHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_int32(bytes)?,
                Ok(18) => msg.uin = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.command = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.seq = r.read_int32(bytes)?,
                Ok(40) => msg.retryTimes = r.read_int32(bytes)?,
                Ok(48) => msg.appid = r.read_int32(bytes)?,
                Ok(56) => msg.dataflag = r.read_int32(bytes)?,
                Ok(64) => msg.commandId = r.read_int32(bytes)?,
                Ok(74) => msg.buildVer = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.localeId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DataHighwayHead<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.uin == "" { 0 } else { 1 + sizeof_len((&self.uin).len()) }
        + if self.command == "" { 0 } else { 1 + sizeof_len((&self.command).len()) }
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.retryTimes == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retryTimes) as u64) }
        + if self.appid == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appid) as u64) }
        + if self.dataflag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.dataflag) as u64) }
        + if self.commandId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.commandId) as u64) }
        + if self.buildVer == "" { 0 } else { 1 + sizeof_len((&self.buildVer).len()) }
        + if self.localeId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.localeId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.version))?; }
        if self.uin != "" { w.write_with_tag(18, |w| w.write_string(&**&self.uin))?; }
        if self.command != "" { w.write_with_tag(26, |w| w.write_string(&**&self.command))?; }
        if self.seq != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.seq))?; }
        if self.retryTimes != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.retryTimes))?; }
        if self.appid != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.appid))?; }
        if self.dataflag != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.dataflag))?; }
        if self.commandId != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.commandId))?; }
        if self.buildVer != "" { w.write_with_tag(74, |w| w.write_string(&**&self.buildVer))?; }
        if self.localeId != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.localeId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SegHead<'a> {
    pub serviceid: i32,
    pub filesize: i64,
    pub dataoffset: i64,
    pub datalength: i32,
    pub rtcode: i32,
    pub serviceticket: Cow<'a, [u8]>,
    pub flag: i32,
    pub md5: Cow<'a, [u8]>,
    pub fileMd5: Cow<'a, [u8]>,
    pub cacheAddr: i32,
    pub queryTimes: i32,
    pub updateCacheip: i32,
}

impl<'a> MessageRead<'a> for SegHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.serviceid = r.read_int32(bytes)?,
                Ok(16) => msg.filesize = r.read_int64(bytes)?,
                Ok(24) => msg.dataoffset = r.read_int64(bytes)?,
                Ok(32) => msg.datalength = r.read_int32(bytes)?,
                Ok(40) => msg.rtcode = r.read_int32(bytes)?,
                Ok(50) => msg.serviceticket = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.flag = r.read_int32(bytes)?,
                Ok(66) => msg.md5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.cacheAddr = r.read_int32(bytes)?,
                Ok(88) => msg.queryTimes = r.read_int32(bytes)?,
                Ok(96) => msg.updateCacheip = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SegHead<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.serviceid == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.serviceid) as u64) }
        + if self.filesize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.filesize) as u64) }
        + if self.dataoffset == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.dataoffset) as u64) }
        + if self.datalength == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.datalength) as u64) }
        + if self.rtcode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.rtcode) as u64) }
        + if self.serviceticket == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.serviceticket).len()) }
        + if self.flag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.flag) as u64) }
        + if self.md5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.md5).len()) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.cacheAddr == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.cacheAddr) as u64) }
        + if self.queryTimes == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.queryTimes) as u64) }
        + if self.updateCacheip == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.updateCacheip) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.serviceid != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.serviceid))?; }
        if self.filesize != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.filesize))?; }
        if self.dataoffset != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.dataoffset))?; }
        if self.datalength != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.datalength))?; }
        if self.rtcode != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.rtcode))?; }
        if self.serviceticket != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.serviceticket))?; }
        if self.flag != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.flag))?; }
        if self.md5 != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.md5))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.cacheAddr != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.cacheAddr))?; }
        if self.queryTimes != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.queryTimes))?; }
        if self.updateCacheip != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.updateCacheip))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TryUpImgReq<'a> {
    pub groupCode: i64,
    pub srcUin: i64,
    pub fileId: i64,
    pub fileMd5: Cow<'a, [u8]>,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub srcTerm: i32,
    pub platformType: i32,
    pub buType: i32,
    pub picWidth: i32,
    pub picHeight: i32,
    pub picType: i32,
    pub buildVer: Cow<'a, str>,
    pub innerIp: i32,
    pub appPicType: i32,
    pub originalPic: i32,
    pub fileIndex: Cow<'a, [u8]>,
    pub dstUin: i64,
    pub srvUpload: i32,
    pub transferUrl: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for TryUpImgReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.srcUin = r.read_int64(bytes)?,
                Ok(24) => msg.fileId = r.read_int64(bytes)?,
                Ok(34) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.fileSize = r.read_int64(bytes)?,
                Ok(50) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.srcTerm = r.read_int32(bytes)?,
                Ok(64) => msg.platformType = r.read_int32(bytes)?,
                Ok(72) => msg.buType = r.read_int32(bytes)?,
                Ok(80) => msg.picWidth = r.read_int32(bytes)?,
                Ok(88) => msg.picHeight = r.read_int32(bytes)?,
                Ok(96) => msg.picType = r.read_int32(bytes)?,
                Ok(106) => msg.buildVer = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(112) => msg.innerIp = r.read_int32(bytes)?,
                Ok(120) => msg.appPicType = r.read_int32(bytes)?,
                Ok(128) => msg.originalPic = r.read_int32(bytes)?,
                Ok(138) => msg.fileIndex = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(144) => msg.dstUin = r.read_int64(bytes)?,
                Ok(152) => msg.srvUpload = r.read_int32(bytes)?,
                Ok(162) => msg.transferUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TryUpImgReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.srcUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.srcUin) as u64) }
        + if self.fileId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileId) as u64) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.fileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 1 + sizeof_len((&self.fileName).len()) }
        + if self.srcTerm == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.srcTerm) as u64) }
        + if self.platformType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.platformType) as u64) }
        + if self.buType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.buType) as u64) }
        + if self.picWidth == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.picWidth) as u64) }
        + if self.picHeight == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.picHeight) as u64) }
        + if self.picType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.picType) as u64) }
        + if self.buildVer == "" { 0 } else { 1 + sizeof_len((&self.buildVer).len()) }
        + if self.innerIp == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.innerIp) as u64) }
        + if self.appPicType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appPicType) as u64) }
        + if self.originalPic == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.originalPic) as u64) }
        + if self.fileIndex == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.fileIndex).len()) }
        + if self.dstUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.dstUin) as u64) }
        + if self.srvUpload == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.srvUpload) as u64) }
        + if self.transferUrl == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.transferUrl).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.srcUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.srcUin))?; }
        if self.fileId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.fileId))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.fileSize != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(50, |w| w.write_string(&**&self.fileName))?; }
        if self.srcTerm != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.srcTerm))?; }
        if self.platformType != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.platformType))?; }
        if self.buType != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.buType))?; }
        if self.picWidth != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.picWidth))?; }
        if self.picHeight != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.picHeight))?; }
        if self.picType != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.picType))?; }
        if self.buildVer != "" { w.write_with_tag(106, |w| w.write_string(&**&self.buildVer))?; }
        if self.innerIp != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.innerIp))?; }
        if self.appPicType != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.appPicType))?; }
        if self.originalPic != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.originalPic))?; }
        if self.fileIndex != Cow::Borrowed(b"") { w.write_with_tag(138, |w| w.write_bytes(&**&self.fileIndex))?; }
        if self.dstUin != 0i64 { w.write_with_tag(144, |w| w.write_int64(*&self.dstUin))?; }
        if self.srvUpload != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.srvUpload))?; }
        if self.transferUrl != Cow::Borrowed(b"") { w.write_with_tag(162, |w| w.write_bytes(&**&self.transferUrl))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TryUpImgResp<'a> {
    pub fileId: i64,
    pub result: i32,
    pub failMsg: Cow<'a, str>,
    pub boolFileExit: bool,
    pub msgImgInfo: Option<ImgInfo<'a>>,
    pub uint32UpIp: Vec<u32>,
    pub uint32UpPort: Vec<u32>,
    pub upUkey: Cow<'a, [u8]>,
    pub fid: i64,
}

impl<'a> MessageRead<'a> for TryUpImgResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fileId = r.read_int64(bytes)?,
                Ok(16) => msg.result = r.read_int32(bytes)?,
                Ok(26) => msg.failMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.boolFileExit = r.read_bool(bytes)?,
                Ok(42) => msg.msgImgInfo = Some(r.read_message::<ImgInfo>(bytes)?),
                Ok(50) => msg.uint32UpIp = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(58) => msg.uint32UpPort = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(66) => msg.upUkey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.fid = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TryUpImgResp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fileId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileId) as u64) }
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.failMsg == "" { 0 } else { 1 + sizeof_len((&self.failMsg).len()) }
        + if self.boolFileExit == false { 0 } else { 1 + sizeof_varint(*(&self.boolFileExit) as u64) }
        + self.msgImgInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.uint32UpIp.is_empty() { 0 } else { 1 + sizeof_len(self.uint32UpIp.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.uint32UpPort.is_empty() { 0 } else { 1 + sizeof_len(self.uint32UpPort.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.upUkey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.upUkey).len()) }
        + if self.fid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fid) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fileId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.fileId))?; }
        if self.result != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.result))?; }
        if self.failMsg != "" { w.write_with_tag(26, |w| w.write_string(&**&self.failMsg))?; }
        if self.boolFileExit != false { w.write_with_tag(32, |w| w.write_bool(*&self.boolFileExit))?; }
        if let Some(ref s) = self.msgImgInfo { w.write_with_tag(42, |w| w.write_message(s))?; }
        w.write_packed_with_tag(50, &self.uint32UpIp, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(58, &self.uint32UpPort, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.upUkey != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.upUkey))?; }
        if self.fid != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.fid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TryUpPttReq<'a> {
    pub groupCode: i64,
    pub srcUin: i64,
    pub fileId: i64,
    pub fileMd5: Cow<'a, [u8]>,
    pub fileSize: i64,
    pub fileName: Cow<'a, [u8]>,
    pub srcTerm: i32,
    pub platformType: i32,
    pub buType: i32,
    pub buildVer: Cow<'a, str>,
    pub innerIp: i32,
    pub voiceLength: i32,
    pub boolNewUpChan: bool,
    pub codec: i32,
    pub voiceType: i32,
    pub buId: i32,
}

impl<'a> MessageRead<'a> for TryUpPttReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.srcUin = r.read_int64(bytes)?,
                Ok(24) => msg.fileId = r.read_int64(bytes)?,
                Ok(34) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.fileSize = r.read_int64(bytes)?,
                Ok(50) => msg.fileName = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.srcTerm = r.read_int32(bytes)?,
                Ok(64) => msg.platformType = r.read_int32(bytes)?,
                Ok(72) => msg.buType = r.read_int32(bytes)?,
                Ok(82) => msg.buildVer = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(88) => msg.innerIp = r.read_int32(bytes)?,
                Ok(96) => msg.voiceLength = r.read_int32(bytes)?,
                Ok(104) => msg.boolNewUpChan = r.read_bool(bytes)?,
                Ok(112) => msg.codec = r.read_int32(bytes)?,
                Ok(120) => msg.voiceType = r.read_int32(bytes)?,
                Ok(128) => msg.buId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TryUpPttReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.srcUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.srcUin) as u64) }
        + if self.fileId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileId) as u64) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.fileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileName).len()) }
        + if self.srcTerm == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.srcTerm) as u64) }
        + if self.platformType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.platformType) as u64) }
        + if self.buType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.buType) as u64) }
        + if self.buildVer == "" { 0 } else { 1 + sizeof_len((&self.buildVer).len()) }
        + if self.innerIp == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.innerIp) as u64) }
        + if self.voiceLength == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.voiceLength) as u64) }
        + if self.boolNewUpChan == false { 0 } else { 1 + sizeof_varint(*(&self.boolNewUpChan) as u64) }
        + if self.codec == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.codec) as u64) }
        + if self.voiceType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.voiceType) as u64) }
        + if self.buId == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.buId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.srcUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.srcUin))?; }
        if self.fileId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.fileId))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.fileSize != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.fileName))?; }
        if self.srcTerm != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.srcTerm))?; }
        if self.platformType != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.platformType))?; }
        if self.buType != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.buType))?; }
        if self.buildVer != "" { w.write_with_tag(82, |w| w.write_string(&**&self.buildVer))?; }
        if self.innerIp != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.innerIp))?; }
        if self.voiceLength != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.voiceLength))?; }
        if self.boolNewUpChan != false { w.write_with_tag(104, |w| w.write_bool(*&self.boolNewUpChan))?; }
        if self.codec != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.codec))?; }
        if self.voiceType != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.voiceType))?; }
        if self.buId != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.buId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TryUpPttResp<'a> {
    pub fileId: i64,
    pub result: i32,
    pub failMsg: Cow<'a, str>,
    pub boolFileExit: bool,
    pub uint32UpIp: Vec<i32>,
    pub uint32UpPort: Vec<i32>,
    pub upUkey: Cow<'a, [u8]>,
    pub fileId2: i64,
    pub upOffset: i64,
    pub blockSize: i64,
    pub fileKey: Cow<'a, [u8]>,
    pub channelType: i32,
}

impl<'a> MessageRead<'a> for TryUpPttResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fileId = r.read_int64(bytes)?,
                Ok(16) => msg.result = r.read_int32(bytes)?,
                Ok(26) => msg.failMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.boolFileExit = r.read_bool(bytes)?,
                Ok(42) => msg.uint32UpIp = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(50) => msg.uint32UpPort = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(58) => msg.upUkey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.fileId2 = r.read_int64(bytes)?,
                Ok(72) => msg.upOffset = r.read_int64(bytes)?,
                Ok(80) => msg.blockSize = r.read_int64(bytes)?,
                Ok(90) => msg.fileKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(96) => msg.channelType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TryUpPttResp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fileId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileId) as u64) }
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.failMsg == "" { 0 } else { 1 + sizeof_len((&self.failMsg).len()) }
        + if self.boolFileExit == false { 0 } else { 1 + sizeof_varint(*(&self.boolFileExit) as u64) }
        + if self.uint32UpIp.is_empty() { 0 } else { 1 + sizeof_len(self.uint32UpIp.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.uint32UpPort.is_empty() { 0 } else { 1 + sizeof_len(self.uint32UpPort.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.upUkey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.upUkey).len()) }
        + if self.fileId2 == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileId2) as u64) }
        + if self.upOffset == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.upOffset) as u64) }
        + if self.blockSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.blockSize) as u64) }
        + if self.fileKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileKey).len()) }
        + if self.channelType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.channelType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fileId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.fileId))?; }
        if self.result != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.result))?; }
        if self.failMsg != "" { w.write_with_tag(26, |w| w.write_string(&**&self.failMsg))?; }
        if self.boolFileExit != false { w.write_with_tag(32, |w| w.write_bool(*&self.boolFileExit))?; }
        w.write_packed_with_tag(42, &self.uint32UpIp, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(50, &self.uint32UpPort, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.upUkey != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.upUkey))?; }
        if self.fileId2 != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.fileId2))?; }
        if self.upOffset != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.upOffset))?; }
        if self.blockSize != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.blockSize))?; }
        if self.fileKey != Cow::Borrowed(b"") { w.write_with_tag(90, |w| w.write_bytes(&**&self.fileKey))?; }
        if self.channelType != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.channelType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImgInfo<'a> {
    pub fileMd5: Cow<'a, [u8]>,
    pub fileType: i32,
    pub fileSize: i64,
    pub fileWidth: i32,
    pub fileHeight: i32,
}

impl<'a> MessageRead<'a> for ImgInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.fileType = r.read_int32(bytes)?,
                Ok(24) => msg.fileSize = r.read_int64(bytes)?,
                Ok(32) => msg.fileWidth = r.read_int32(bytes)?,
                Ok(40) => msg.fileHeight = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ImgInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.fileType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileType) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileWidth == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileWidth) as u64) }
        + if self.fileHeight == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileHeight) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.fileType != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.fileType))?; }
        if self.fileSize != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileWidth != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.fileWidth))?; }
        if self.fileHeight != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.fileHeight))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteMessageRequest<'a> {
    pub items: Vec<MessageItem<'a>>,
}

impl<'a> MessageRead<'a> for DeleteMessageRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.items.push(r.read_message::<MessageItem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteMessageRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.items.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.items { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageItem<'a> {
    pub fromUin: i64,
    pub toUin: i64,
    pub msgType: i32,
    pub msgSeq: i32,
    pub msgUid: i64,
    pub sig: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for MessageItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = r.read_int64(bytes)?,
                Ok(16) => msg.toUin = r.read_int64(bytes)?,
                Ok(24) => msg.msgType = r.read_int32(bytes)?,
                Ok(32) => msg.msgSeq = r.read_int32(bytes)?,
                Ok(40) => msg.msgUid = r.read_int64(bytes)?,
                Ok(58) => msg.sig = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MessageItem<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fromUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fromUin) as u64) }
        + if self.toUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.toUin) as u64) }
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.msgSeq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgSeq) as u64) }
        + if self.msgUid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgUid) as u64) }
        + if self.sig == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sig).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fromUin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.fromUin))?; }
        if self.toUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.toUin))?; }
        if self.msgType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.msgType))?; }
        if self.msgSeq != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.msgSeq))?; }
        if self.msgUid != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.msgUid))?; }
        if self.sig != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.sig))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SubD4 {
    pub uin: i64,
}

impl<'a> MessageRead<'a> for SubD4 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SubD4 {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sub8A<'a> {
    pub msg_info: Vec<Sub8AMsgInfo>,
    pub appId: i32,
    pub instId: i32,
    pub longMessageFlag: i32,
    pub reserved: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Sub8A<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg_info.push(r.read_message::<Sub8AMsgInfo>(bytes)?),
                Ok(16) => msg.appId = r.read_int32(bytes)?,
                Ok(24) => msg.instId = r.read_int32(bytes)?,
                Ok(32) => msg.longMessageFlag = r.read_int32(bytes)?,
                Ok(42) => msg.reserved = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sub8A<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msg_info.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.appId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.instId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.instId) as u64) }
        + if self.longMessageFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.longMessageFlag) as u64) }
        + if self.reserved == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.reserved).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.msg_info { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.appId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.appId))?; }
        if self.instId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.instId))?; }
        if self.longMessageFlag != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.longMessageFlag))?; }
        if self.reserved != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.reserved))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sub8AMsgInfo {
    pub fromUin: i64,
    pub toUin: i64,
    pub msgSeq: i32,
    pub msgUid: i64,
    pub msgTime: i64,
    pub msgRandom: i32,
    pub pkgNum: i32,
    pub pkgIndex: i32,
    pub devSeq: i32,
}

impl<'a> MessageRead<'a> for Sub8AMsgInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = r.read_int64(bytes)?,
                Ok(16) => msg.toUin = r.read_int64(bytes)?,
                Ok(24) => msg.msgSeq = r.read_int32(bytes)?,
                Ok(32) => msg.msgUid = r.read_int64(bytes)?,
                Ok(40) => msg.msgTime = r.read_int64(bytes)?,
                Ok(48) => msg.msgRandom = r.read_int32(bytes)?,
                Ok(56) => msg.pkgNum = r.read_int32(bytes)?,
                Ok(64) => msg.pkgIndex = r.read_int32(bytes)?,
                Ok(72) => msg.devSeq = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Sub8AMsgInfo {
    fn get_size(&self) -> usize {
        0
        + if self.fromUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fromUin) as u64) }
        + if self.toUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.toUin) as u64) }
        + if self.msgSeq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgSeq) as u64) }
        + if self.msgUid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgUid) as u64) }
        + if self.msgTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgTime) as u64) }
        + if self.msgRandom == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgRandom) as u64) }
        + if self.pkgNum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.pkgNum) as u64) }
        + if self.pkgIndex == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.pkgIndex) as u64) }
        + if self.devSeq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.devSeq) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fromUin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.fromUin))?; }
        if self.toUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.toUin))?; }
        if self.msgSeq != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.msgSeq))?; }
        if self.msgUid != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.msgUid))?; }
        if self.msgTime != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.msgTime))?; }
        if self.msgRandom != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.msgRandom))?; }
        if self.pkgNum != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.pkgNum))?; }
        if self.pkgIndex != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.pkgIndex))?; }
        if self.devSeq != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.devSeq))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SubB3<'a> {
    pub type_pb: i32,
    pub msgAddFrdNotify: Option<SubB3AddFrdNotify<'a>>,
}

impl<'a> MessageRead<'a> for SubB3<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_int32(bytes)?,
                Ok(18) => msg.msgAddFrdNotify = Some(r.read_message::<SubB3AddFrdNotify>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SubB3<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + self.msgAddFrdNotify.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.type_pb))?; }
        if let Some(ref s) = self.msgAddFrdNotify { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SubB3AddFrdNotify<'a> {
    pub uin: i64,
    pub nick: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SubB3AddFrdNotify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(42) => msg.nick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SubB3AddFrdNotify<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.nick == "" { 0 } else { 1 + sizeof_len((&self.nick).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.nick != "" { w.write_with_tag(42, |w| w.write_string(&**&self.nick))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sub44<'a> {
    pub friendSyncMsg: Option<Sub44FriendSyncMsg<'a>>,
    pub groupSyncMsg: Option<Sub44GroupSyncMsg<'a>>,
}

impl<'a> MessageRead<'a> for Sub44<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.friendSyncMsg = Some(r.read_message::<Sub44FriendSyncMsg>(bytes)?),
                Ok(18) => msg.groupSyncMsg = Some(r.read_message::<Sub44GroupSyncMsg>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sub44<'a> {
    fn get_size(&self) -> usize {
        0
        + self.friendSyncMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupSyncMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.friendSyncMsg { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupSyncMsg { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sub44FriendSyncMsg<'a> {
    pub uin: i64,
    pub fUin: i64,
    pub processType: i32,
    pub time: i32,
    pub processFlag: i32,
    pub sourceId: i32,
    pub sourceSubId: i32,
    pub strWording: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Sub44FriendSyncMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(16) => msg.fUin = r.read_int64(bytes)?,
                Ok(24) => msg.processType = r.read_int32(bytes)?,
                Ok(32) => msg.time = r.read_int32(bytes)?,
                Ok(40) => msg.processFlag = r.read_int32(bytes)?,
                Ok(48) => msg.sourceId = r.read_int32(bytes)?,
                Ok(56) => msg.sourceSubId = r.read_int32(bytes)?,
                Ok(66) => msg.strWording.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sub44FriendSyncMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.fUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fUin) as u64) }
        + if self.processType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.processType) as u64) }
        + if self.time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.time) as u64) }
        + if self.processFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.processFlag) as u64) }
        + if self.sourceId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.sourceId) as u64) }
        + if self.sourceSubId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.sourceSubId) as u64) }
        + self.strWording.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.fUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.fUin))?; }
        if self.processType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.processType))?; }
        if self.time != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.time))?; }
        if self.processFlag != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.processFlag))?; }
        if self.sourceId != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.sourceId))?; }
        if self.sourceSubId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.sourceSubId))?; }
        for s in &self.strWording { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sub44GroupSyncMsg<'a> {
    pub msgType: i32,
    pub msgSeq: i64,
    pub grpCode: i64,
    pub gaCode: i64,
    pub optUin1: i64,
    pub optUin2: i64,
    pub msgBuf: Cow<'a, [u8]>,
    pub authKey: Cow<'a, [u8]>,
    pub msgStatus: i32,
    pub actionUin: i64,
    pub actionTime: i64,
    pub curMaxMemCount: i32,
    pub nextMaxMemCount: i32,
    pub curMemCount: i32,
    pub reqSrcId: i32,
    pub reqSrcSubId: i32,
    pub inviterRole: i32,
    pub extAdminNum: i32,
    pub processFlag: i32,
}

impl<'a> MessageRead<'a> for Sub44GroupSyncMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgType = r.read_int32(bytes)?,
                Ok(16) => msg.msgSeq = r.read_int64(bytes)?,
                Ok(24) => msg.grpCode = r.read_int64(bytes)?,
                Ok(32) => msg.gaCode = r.read_int64(bytes)?,
                Ok(40) => msg.optUin1 = r.read_int64(bytes)?,
                Ok(48) => msg.optUin2 = r.read_int64(bytes)?,
                Ok(58) => msg.msgBuf = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.authKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.msgStatus = r.read_int32(bytes)?,
                Ok(80) => msg.actionUin = r.read_int64(bytes)?,
                Ok(88) => msg.actionTime = r.read_int64(bytes)?,
                Ok(96) => msg.curMaxMemCount = r.read_int32(bytes)?,
                Ok(104) => msg.nextMaxMemCount = r.read_int32(bytes)?,
                Ok(112) => msg.curMemCount = r.read_int32(bytes)?,
                Ok(120) => msg.reqSrcId = r.read_int32(bytes)?,
                Ok(128) => msg.reqSrcSubId = r.read_int32(bytes)?,
                Ok(136) => msg.inviterRole = r.read_int32(bytes)?,
                Ok(144) => msg.extAdminNum = r.read_int32(bytes)?,
                Ok(152) => msg.processFlag = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sub44GroupSyncMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.msgSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgSeq) as u64) }
        + if self.grpCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.grpCode) as u64) }
        + if self.gaCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.gaCode) as u64) }
        + if self.optUin1 == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.optUin1) as u64) }
        + if self.optUin2 == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.optUin2) as u64) }
        + if self.msgBuf == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msgBuf).len()) }
        + if self.authKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.authKey).len()) }
        + if self.msgStatus == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgStatus) as u64) }
        + if self.actionUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.actionUin) as u64) }
        + if self.actionTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.actionTime) as u64) }
        + if self.curMaxMemCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.curMaxMemCount) as u64) }
        + if self.nextMaxMemCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.nextMaxMemCount) as u64) }
        + if self.curMemCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.curMemCount) as u64) }
        + if self.reqSrcId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqSrcId) as u64) }
        + if self.reqSrcSubId == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.reqSrcSubId) as u64) }
        + if self.inviterRole == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.inviterRole) as u64) }
        + if self.extAdminNum == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.extAdminNum) as u64) }
        + if self.processFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.processFlag) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.msgType))?; }
        if self.msgSeq != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.msgSeq))?; }
        if self.grpCode != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.grpCode))?; }
        if self.gaCode != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.gaCode))?; }
        if self.optUin1 != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.optUin1))?; }
        if self.optUin2 != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.optUin2))?; }
        if self.msgBuf != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.msgBuf))?; }
        if self.authKey != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.authKey))?; }
        if self.msgStatus != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.msgStatus))?; }
        if self.actionUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.actionUin))?; }
        if self.actionTime != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.actionTime))?; }
        if self.curMaxMemCount != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.curMaxMemCount))?; }
        if self.nextMaxMemCount != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.nextMaxMemCount))?; }
        if self.curMemCount != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.curMemCount))?; }
        if self.reqSrcId != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.reqSrcId))?; }
        if self.reqSrcSubId != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.reqSrcSubId))?; }
        if self.inviterRole != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.inviterRole))?; }
        if self.extAdminNum != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.extAdminNum))?; }
        if self.processFlag != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.processFlag))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMemberReqBody {
    pub groupCode: i64,
    pub uin: i64,
    pub newClient: bool,
    pub clientType: i32,
    pub richCardNameVer: i32,
}

impl<'a> MessageRead<'a> for GroupMemberReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.uin = r.read_int64(bytes)?,
                Ok(24) => msg.newClient = r.read_bool(bytes)?,
                Ok(32) => msg.clientType = r.read_int32(bytes)?,
                Ok(40) => msg.richCardNameVer = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupMemberReqBody {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.newClient == false { 0 } else { 1 + sizeof_varint(*(&self.newClient) as u64) }
        + if self.clientType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.clientType) as u64) }
        + if self.richCardNameVer == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.richCardNameVer) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.uin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.uin))?; }
        if self.newClient != false { w.write_with_tag(24, |w| w.write_bool(*&self.newClient))?; }
        if self.clientType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.clientType))?; }
        if self.richCardNameVer != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.richCardNameVer))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMemberRspBody<'a> {
    pub groupCode: i64,
    pub selfRole: i32,
    pub memInfo: Option<GroupMemberInfo<'a>>,
    pub boolSelfLocationShared: bool,
    pub groupType: i32,
}

impl<'a> MessageRead<'a> for GroupMemberRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.selfRole = r.read_int32(bytes)?,
                Ok(26) => msg.memInfo = Some(r.read_message::<GroupMemberInfo>(bytes)?),
                Ok(32) => msg.boolSelfLocationShared = r.read_bool(bytes)?,
                Ok(40) => msg.groupType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupMemberRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.selfRole == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.selfRole) as u64) }
        + self.memInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.boolSelfLocationShared == false { 0 } else { 1 + sizeof_varint(*(&self.boolSelfLocationShared) as u64) }
        + if self.groupType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.selfRole != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.selfRole))?; }
        if let Some(ref s) = self.memInfo { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.boolSelfLocationShared != false { w.write_with_tag(32, |w| w.write_bool(*&self.boolSelfLocationShared))?; }
        if self.groupType != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.groupType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMemberInfo<'a> {
    pub uin: i64,
    pub result: i32,
    pub errmsg: Cow<'a, [u8]>,
    pub IsFriend: bool,
    pub remark: Cow<'a, [u8]>,
    pub IsConcerned: bool,
    pub credit: i32,
    pub card: Cow<'a, [u8]>,
    pub sex: i32,
    pub location: Cow<'a, [u8]>,
    pub nick: Cow<'a, [u8]>,
    pub age: i32,
    pub lev: Cow<'a, [u8]>,
    pub join: i64,
    pub lastSpeak: i64,
    pub gbarTitle: Cow<'a, [u8]>,
    pub gbarUrl: Cow<'a, [u8]>,
    pub gbarCnt: i32,
    pub isAllowModCard: bool,
    pub isVip: bool,
    pub isYearVip: bool,
    pub isSuperVip: bool,
    pub isSuperQq: bool,
    pub vipLev: i32,
    pub role: i32,
    pub locationShared: bool,
    pub int64Distance: i64,
    pub concernType: i32,
    pub specialTitle: Cow<'a, [u8]>,
    pub specialTitleExpireTime: i32,
    pub phoneNum: Cow<'a, [u8]>,
    pub job: Cow<'a, [u8]>,
    pub medalId: i32,
    pub level: i32,
    pub honor: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for GroupMemberInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(16) => msg.result = r.read_int32(bytes)?,
                Ok(26) => msg.errmsg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.IsFriend = r.read_bool(bytes)?,
                Ok(42) => msg.remark = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.IsConcerned = r.read_bool(bytes)?,
                Ok(56) => msg.credit = r.read_int32(bytes)?,
                Ok(66) => msg.card = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.sex = r.read_int32(bytes)?,
                Ok(82) => msg.location = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.nick = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(96) => msg.age = r.read_int32(bytes)?,
                Ok(106) => msg.lev = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(112) => msg.join = r.read_int64(bytes)?,
                Ok(120) => msg.lastSpeak = r.read_int64(bytes)?,
                Ok(146) => msg.gbarTitle = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(154) => msg.gbarUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(160) => msg.gbarCnt = r.read_int32(bytes)?,
                Ok(168) => msg.isAllowModCard = r.read_bool(bytes)?,
                Ok(176) => msg.isVip = r.read_bool(bytes)?,
                Ok(184) => msg.isYearVip = r.read_bool(bytes)?,
                Ok(192) => msg.isSuperVip = r.read_bool(bytes)?,
                Ok(200) => msg.isSuperQq = r.read_bool(bytes)?,
                Ok(208) => msg.vipLev = r.read_int32(bytes)?,
                Ok(216) => msg.role = r.read_int32(bytes)?,
                Ok(224) => msg.locationShared = r.read_bool(bytes)?,
                Ok(232) => msg.int64Distance = r.read_int64(bytes)?,
                Ok(240) => msg.concernType = r.read_int32(bytes)?,
                Ok(250) => msg.specialTitle = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(256) => msg.specialTitleExpireTime = r.read_int32(bytes)?,
                Ok(282) => msg.phoneNum = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(290) => msg.job = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(296) => msg.medalId = r.read_int32(bytes)?,
                Ok(312) => msg.level = r.read_int32(bytes)?,
                Ok(330) => msg.honor = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupMemberInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.errmsg == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.errmsg).len()) }
        + if self.IsFriend == false { 0 } else { 1 + sizeof_varint(*(&self.IsFriend) as u64) }
        + if self.remark == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.remark).len()) }
        + if self.IsConcerned == false { 0 } else { 1 + sizeof_varint(*(&self.IsConcerned) as u64) }
        + if self.credit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.credit) as u64) }
        + if self.card == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.card).len()) }
        + if self.sex == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.sex) as u64) }
        + if self.location == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.location).len()) }
        + if self.nick == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.nick).len()) }
        + if self.age == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.age) as u64) }
        + if self.lev == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.lev).len()) }
        + if self.join == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.join) as u64) }
        + if self.lastSpeak == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.lastSpeak) as u64) }
        + if self.gbarTitle == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.gbarTitle).len()) }
        + if self.gbarUrl == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.gbarUrl).len()) }
        + if self.gbarCnt == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.gbarCnt) as u64) }
        + if self.isAllowModCard == false { 0 } else { 2 + sizeof_varint(*(&self.isAllowModCard) as u64) }
        + if self.isVip == false { 0 } else { 2 + sizeof_varint(*(&self.isVip) as u64) }
        + if self.isYearVip == false { 0 } else { 2 + sizeof_varint(*(&self.isYearVip) as u64) }
        + if self.isSuperVip == false { 0 } else { 2 + sizeof_varint(*(&self.isSuperVip) as u64) }
        + if self.isSuperQq == false { 0 } else { 2 + sizeof_varint(*(&self.isSuperQq) as u64) }
        + if self.vipLev == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.vipLev) as u64) }
        + if self.role == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.role) as u64) }
        + if self.locationShared == false { 0 } else { 2 + sizeof_varint(*(&self.locationShared) as u64) }
        + if self.int64Distance == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.int64Distance) as u64) }
        + if self.concernType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.concernType) as u64) }
        + if self.specialTitle == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.specialTitle).len()) }
        + if self.specialTitleExpireTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.specialTitleExpireTime) as u64) }
        + if self.phoneNum == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.phoneNum).len()) }
        + if self.job == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.job).len()) }
        + if self.medalId == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.medalId) as u64) }
        + if self.level == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.level) as u64) }
        + if self.honor == "" { 0 } else { 2 + sizeof_len((&self.honor).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.result != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.result))?; }
        if self.errmsg != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.errmsg))?; }
        if self.IsFriend != false { w.write_with_tag(32, |w| w.write_bool(*&self.IsFriend))?; }
        if self.remark != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.remark))?; }
        if self.IsConcerned != false { w.write_with_tag(48, |w| w.write_bool(*&self.IsConcerned))?; }
        if self.credit != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.credit))?; }
        if self.card != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.card))?; }
        if self.sex != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.sex))?; }
        if self.location != Cow::Borrowed(b"") { w.write_with_tag(82, |w| w.write_bytes(&**&self.location))?; }
        if self.nick != Cow::Borrowed(b"") { w.write_with_tag(90, |w| w.write_bytes(&**&self.nick))?; }
        if self.age != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.age))?; }
        if self.lev != Cow::Borrowed(b"") { w.write_with_tag(106, |w| w.write_bytes(&**&self.lev))?; }
        if self.join != 0i64 { w.write_with_tag(112, |w| w.write_int64(*&self.join))?; }
        if self.lastSpeak != 0i64 { w.write_with_tag(120, |w| w.write_int64(*&self.lastSpeak))?; }
        if self.gbarTitle != Cow::Borrowed(b"") { w.write_with_tag(146, |w| w.write_bytes(&**&self.gbarTitle))?; }
        if self.gbarUrl != Cow::Borrowed(b"") { w.write_with_tag(154, |w| w.write_bytes(&**&self.gbarUrl))?; }
        if self.gbarCnt != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.gbarCnt))?; }
        if self.isAllowModCard != false { w.write_with_tag(168, |w| w.write_bool(*&self.isAllowModCard))?; }
        if self.isVip != false { w.write_with_tag(176, |w| w.write_bool(*&self.isVip))?; }
        if self.isYearVip != false { w.write_with_tag(184, |w| w.write_bool(*&self.isYearVip))?; }
        if self.isSuperVip != false { w.write_with_tag(192, |w| w.write_bool(*&self.isSuperVip))?; }
        if self.isSuperQq != false { w.write_with_tag(200, |w| w.write_bool(*&self.isSuperQq))?; }
        if self.vipLev != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.vipLev))?; }
        if self.role != 0i32 { w.write_with_tag(216, |w| w.write_int32(*&self.role))?; }
        if self.locationShared != false { w.write_with_tag(224, |w| w.write_bool(*&self.locationShared))?; }
        if self.int64Distance != 0i64 { w.write_with_tag(232, |w| w.write_int64(*&self.int64Distance))?; }
        if self.concernType != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.concernType))?; }
        if self.specialTitle != Cow::Borrowed(b"") { w.write_with_tag(250, |w| w.write_bytes(&**&self.specialTitle))?; }
        if self.specialTitleExpireTime != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.specialTitleExpireTime))?; }
        if self.phoneNum != Cow::Borrowed(b"") { w.write_with_tag(282, |w| w.write_bytes(&**&self.phoneNum))?; }
        if self.job != Cow::Borrowed(b"") { w.write_with_tag(290, |w| w.write_bytes(&**&self.job))?; }
        if self.medalId != 0i32 { w.write_with_tag(296, |w| w.write_int32(*&self.medalId))?; }
        if self.level != 0i32 { w.write_with_tag(312, |w| w.write_int32(*&self.level))?; }
        if self.honor != "" { w.write_with_tag(330, |w| w.write_string(&**&self.honor))?; }
        Ok(())
    }
}

