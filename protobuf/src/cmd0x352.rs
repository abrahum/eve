// Automatically generated rust module for 'cmd0x352.proto' file

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
pub struct ReqBody<'a> {
    pub subcmd: Option<u32>,
    pub tryupImgReq: Vec<D352TryUpImgReq<'a>>,
    pub netType: Option<u32>,
}

impl<'a> MessageRead<'a> for ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subcmd = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.tryupImgReq.push(r.read_message::<D352TryUpImgReq>(bytes)?),
                Ok(80) => msg.netType = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.subcmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tryupImgReq.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.netType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subcmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.tryupImgReq { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.netType { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspBody<'a> {
    pub subcmd: Option<u32>,
    pub tryupImgRsp: Vec<TryUpImgRsp<'a>>,
    pub newBigchan: Option<bool>,
    pub failMsg: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subcmd = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.tryupImgRsp.push(r.read_message::<TryUpImgRsp>(bytes)?),
                Ok(32) => msg.newBigchan = Some(r.read_bool(bytes)?),
                Ok(82) => msg.failMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.subcmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tryupImgRsp.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.newBigchan.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.failMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subcmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.tryupImgRsp { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.newBigchan { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.failMsg { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D352TryUpImgReq<'a> {
    pub srcUin: Option<u64>,
    pub dstUin: Option<u64>,
    pub fileId: Option<u64>,
    pub fileMd5: Option<Cow<'a, [u8]>>,
    pub fileSize: Option<u64>,
    pub fileName: Option<Cow<'a, [u8]>>,
    pub srcTerm: Option<u32>,
    pub platformType: Option<u32>,
    pub innerIp: Option<u32>,
    pub addressBook: Option<bool>,
    pub retry: Option<u32>,
    pub buType: Option<u32>,
    pub picOriginal: Option<bool>,
    pub picWidth: Option<u32>,
    pub picHeight: Option<u32>,
    pub picType: Option<u32>,
    pub buildVer: Option<Cow<'a, [u8]>>,
    pub fileIndex: Option<Cow<'a, [u8]>>,
    pub storeDays: Option<u32>,
    pub tryupStepflag: Option<u32>,
    pub rejectTryfast: Option<bool>,
    pub srvUpload: Option<u32>,
    pub transferUrl: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D352TryUpImgReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.srcUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.dstUin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.fileId = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.fileMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.fileSize = Some(r.read_uint64(bytes)?),
                Ok(50) => msg.fileName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.srcTerm = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.platformType = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.innerIp = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.addressBook = Some(r.read_bool(bytes)?),
                Ok(88) => msg.retry = Some(r.read_uint32(bytes)?),
                Ok(96) => msg.buType = Some(r.read_uint32(bytes)?),
                Ok(104) => msg.picOriginal = Some(r.read_bool(bytes)?),
                Ok(112) => msg.picWidth = Some(r.read_uint32(bytes)?),
                Ok(120) => msg.picHeight = Some(r.read_uint32(bytes)?),
                Ok(128) => msg.picType = Some(r.read_uint32(bytes)?),
                Ok(138) => msg.buildVer = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(146) => msg.fileIndex = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(152) => msg.storeDays = Some(r.read_uint32(bytes)?),
                Ok(160) => msg.tryupStepflag = Some(r.read_uint32(bytes)?),
                Ok(168) => msg.rejectTryfast = Some(r.read_bool(bytes)?),
                Ok(176) => msg.srvUpload = Some(r.read_uint32(bytes)?),
                Ok(186) => msg.transferUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D352TryUpImgReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.srcUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.dstUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.srcTerm.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.platformType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.innerIp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.addressBook.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retry.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.buType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.picOriginal.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.picWidth.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.picHeight.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.picType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.buildVer.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.fileIndex.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.storeDays.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.tryupStepflag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.rejectTryfast.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.srvUpload.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.transferUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.srcUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.dstUin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fileMd5 { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileSize { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fileName { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.srcTerm { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.platformType { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.innerIp { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.addressBook { w.write_with_tag(80, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.retry { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.buType { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.picOriginal { w.write_with_tag(104, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.picWidth { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.picHeight { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.picType { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.buildVer { w.write_with_tag(138, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileIndex { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.storeDays { w.write_with_tag(152, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.tryupStepflag { w.write_with_tag(160, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.rejectTryfast { w.write_with_tag(168, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.srvUpload { w.write_with_tag(176, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.transferUrl { w.write_with_tag(186, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TryUpImgRsp<'a> {
    pub fileId: Option<u64>,
    pub clientIp: Option<u32>,
    pub result: Option<u32>,
    pub failMsg: Option<Cow<'a, [u8]>>,
    pub fileExit: Option<bool>,
    pub upIp: Vec<u32>,
    pub upPort: Vec<u32>,
    pub upUkey: Option<Cow<'a, [u8]>>,
    pub upResid: Option<Cow<'a, [u8]>>,
    pub upUuid: Option<Cow<'a, [u8]>>,
    pub upOffset: Option<u64>,
    pub blockSize: Option<u64>,
    pub encryptDstip: Option<Cow<'a, [u8]>>,
    pub roamdays: Option<u32>,
    pub clientIp6: Option<Cow<'a, [u8]>>,
    pub thumbDownPara: Option<Cow<'a, [u8]>>,
    pub originalDownPara: Option<Cow<'a, [u8]>>,
    pub downDomain: Option<Cow<'a, [u8]>>,
    pub bigDownPara: Option<Cow<'a, [u8]>>,
    pub bigThumbDownPara: Option<Cow<'a, [u8]>>,
    pub httpsUrlFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for TryUpImgRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fileId = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.clientIp = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.failMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.fileExit = Some(r.read_bool(bytes)?),
                Ok(56) => msg.upIp.push(r.read_uint32(bytes)?),
                Ok(64) => msg.upPort.push(r.read_uint32(bytes)?),
                Ok(74) => msg.upUkey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.upResid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.upUuid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.upOffset = Some(r.read_uint64(bytes)?),
                Ok(104) => msg.blockSize = Some(r.read_uint64(bytes)?),
                Ok(114) => msg.encryptDstip = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.roamdays = Some(r.read_uint32(bytes)?),
                Ok(218) => msg.clientIp6 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(482) => msg.thumbDownPara = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(490) => msg.originalDownPara = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(498) => msg.downDomain = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(514) => msg.bigDownPara = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(522) => msg.bigThumbDownPara = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(528) => msg.httpsUrlFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TryUpImgRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientIp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.failMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileExit.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.upIp.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.upPort.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.upUkey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.upResid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.upUuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.upOffset.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.blockSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.encryptDstip.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.roamdays.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientIp6.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.thumbDownPara.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.originalDownPara.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.downDomain.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.bigDownPara.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.bigThumbDownPara.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.httpsUrlFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileId { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.clientIp { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.failMsg { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileExit { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        for s in &self.upIp { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        for s in &self.upPort { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.upUkey { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.upResid { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.upUuid { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.upOffset { w.write_with_tag(96, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.blockSize { w.write_with_tag(104, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.encryptDstip { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.roamdays { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.clientIp6 { w.write_with_tag(218, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.thumbDownPara { w.write_with_tag(482, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.originalDownPara { w.write_with_tag(490, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.downDomain { w.write_with_tag(498, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.bigDownPara { w.write_with_tag(514, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.bigThumbDownPara { w.write_with_tag(522, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.httpsUrlFlag { w.write_with_tag(528, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

