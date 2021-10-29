// Automatically generated rust module for 'shortvideo.proto' file

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
pub struct ShortVideoReqBody<'a> {
    pub cmd: i32,
    pub seq: i32,
    pub pttShortVideoUploadReq: Option<ShortVideoUploadReq<'a>>,
    pub pttShortVideoDownloadReq: Option<ShortVideoDownloadReq<'a>>,
    pub extensionReq: Vec<ShortVideoExtensionReq>,
}

impl<'a> MessageRead<'a> for ShortVideoReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cmd = r.read_int32(bytes)?,
                Ok(16) => msg.seq = r.read_int32(bytes)?,
                Ok(26) => msg.pttShortVideoUploadReq = Some(r.read_message::<ShortVideoUploadReq>(bytes)?),
                Ok(34) => msg.pttShortVideoDownloadReq = Some(r.read_message::<ShortVideoDownloadReq>(bytes)?),
                Ok(802) => msg.extensionReq.push(r.read_message::<ShortVideoExtensionReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.cmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.cmd) as u64) }
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + self.pttShortVideoUploadReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.pttShortVideoDownloadReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.extensionReq.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cmd != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.cmd))?; }
        if self.seq != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.seq))?; }
        if let Some(ref s) = self.pttShortVideoUploadReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pttShortVideoDownloadReq { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.extensionReq { w.write_with_tag(802, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoRspBody<'a> {
    pub cmd: i32,
    pub seq: i32,
    pub pttShortVideoUploadRsp: Option<ShortVideoUploadRsp<'a>>,
    pub pttShortVideoDownloadRsp: Option<ShortVideoDownloadRsp<'a>>,
}

impl<'a> MessageRead<'a> for ShortVideoRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cmd = r.read_int32(bytes)?,
                Ok(16) => msg.seq = r.read_int32(bytes)?,
                Ok(26) => msg.pttShortVideoUploadRsp = Some(r.read_message::<ShortVideoUploadRsp>(bytes)?),
                Ok(34) => msg.pttShortVideoDownloadRsp = Some(r.read_message::<ShortVideoDownloadRsp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.cmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.cmd) as u64) }
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + self.pttShortVideoUploadRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.pttShortVideoDownloadRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cmd != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.cmd))?; }
        if self.seq != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.seq))?; }
        if let Some(ref s) = self.pttShortVideoUploadRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pttShortVideoDownloadRsp { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoUploadReq<'a> {
    pub fromUin: i64,
    pub toUin: i64,
    pub chatType: i32,
    pub clientType: i32,
    pub info: Option<ShortVideoFileInfo<'a>>,
    pub groupCode: i64,
    pub agentType: i32,
    pub businessType: i32,
    pub supportLargeSize: i32,
}

impl<'a> MessageRead<'a> for ShortVideoUploadReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = r.read_int64(bytes)?,
                Ok(16) => msg.toUin = r.read_int64(bytes)?,
                Ok(24) => msg.chatType = r.read_int32(bytes)?,
                Ok(32) => msg.clientType = r.read_int32(bytes)?,
                Ok(42) => msg.info = Some(r.read_message::<ShortVideoFileInfo>(bytes)?),
                Ok(48) => msg.groupCode = r.read_int64(bytes)?,
                Ok(56) => msg.agentType = r.read_int32(bytes)?,
                Ok(64) => msg.businessType = r.read_int32(bytes)?,
                Ok(160) => msg.supportLargeSize = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoUploadReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fromUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fromUin) as u64) }
        + if self.toUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.toUin) as u64) }
        + if self.chatType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.chatType) as u64) }
        + if self.clientType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.clientType) as u64) }
        + self.info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.agentType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.agentType) as u64) }
        + if self.businessType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.businessType) as u64) }
        + if self.supportLargeSize == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.supportLargeSize) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fromUin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.fromUin))?; }
        if self.toUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.toUin))?; }
        if self.chatType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.chatType))?; }
        if self.clientType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.clientType))?; }
        if let Some(ref s) = self.info { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.groupCode != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.groupCode))?; }
        if self.agentType != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.agentType))?; }
        if self.businessType != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.businessType))?; }
        if self.supportLargeSize != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.supportLargeSize))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoDownloadReq<'a> {
    pub fromUin: i64,
    pub toUin: i64,
    pub chatType: i32,
    pub clientType: i32,
    pub fileId: Cow<'a, str>,
    pub groupCode: i64,
    pub agentType: i32,
    pub fileMd5: Cow<'a, [u8]>,
    pub businessType: i32,
    pub fileType: i32,
    pub downType: i32,
    pub sceneType: i32,
}

impl<'a> MessageRead<'a> for ShortVideoDownloadReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = r.read_int64(bytes)?,
                Ok(16) => msg.toUin = r.read_int64(bytes)?,
                Ok(24) => msg.chatType = r.read_int32(bytes)?,
                Ok(32) => msg.clientType = r.read_int32(bytes)?,
                Ok(42) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.groupCode = r.read_int64(bytes)?,
                Ok(56) => msg.agentType = r.read_int32(bytes)?,
                Ok(66) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.businessType = r.read_int32(bytes)?,
                Ok(80) => msg.fileType = r.read_int32(bytes)?,
                Ok(88) => msg.downType = r.read_int32(bytes)?,
                Ok(96) => msg.sceneType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoDownloadReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fromUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fromUin) as u64) }
        + if self.toUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.toUin) as u64) }
        + if self.chatType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.chatType) as u64) }
        + if self.clientType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.clientType) as u64) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.agentType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.agentType) as u64) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.businessType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.businessType) as u64) }
        + if self.fileType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileType) as u64) }
        + if self.downType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.downType) as u64) }
        + if self.sceneType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.sceneType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fromUin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.fromUin))?; }
        if self.toUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.toUin))?; }
        if self.chatType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.chatType))?; }
        if self.clientType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.clientType))?; }
        if self.fileId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.fileId))?; }
        if self.groupCode != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.groupCode))?; }
        if self.agentType != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.agentType))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.businessType != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.businessType))?; }
        if self.fileType != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.fileType))?; }
        if self.downType != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.downType))?; }
        if self.sceneType != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.sceneType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoDownloadRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub sameAreaOutAddr: Vec<ShortVideoIpList>,
    pub diffAreaOutAddr: Vec<ShortVideoIpList>,
    pub downloadKey: Cow<'a, [u8]>,
    pub fileMd5: Cow<'a, [u8]>,
    pub sameAreaInnerAddr: Vec<ShortVideoIpList>,
    pub diffAreaInnerAddr: Vec<ShortVideoIpList>,
    pub downloadAddr: Option<ShortVideoAddr<'a>>,
    pub encryptKey: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ShortVideoDownloadRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.sameAreaOutAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(34) => msg.diffAreaOutAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(42) => msg.downloadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.sameAreaInnerAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(66) => msg.diffAreaInnerAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(74) => msg.downloadAddr = Some(r.read_message::<ShortVideoAddr>(bytes)?),
                Ok(82) => msg.encryptKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoDownloadRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + self.sameAreaOutAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.diffAreaOutAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.downloadKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.downloadKey).len()) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + self.sameAreaInnerAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.diffAreaInnerAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.downloadAddr.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.encryptKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.encryptKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        for s in &self.sameAreaOutAddr { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.diffAreaOutAddr { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.downloadKey != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.downloadKey))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.fileMd5))?; }
        for s in &self.sameAreaInnerAddr { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.diffAreaInnerAddr { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.downloadAddr { w.write_with_tag(74, |w| w.write_message(s))?; }
        if self.encryptKey != Cow::Borrowed(b"") { w.write_with_tag(82, |w| w.write_bytes(&**&self.encryptKey))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoUploadRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub sameAreaOutAddr: Vec<ShortVideoIpList>,
    pub diffAreaOutAddr: Vec<ShortVideoIpList>,
    pub fileId: Cow<'a, str>,
    pub uKey: Cow<'a, [u8]>,
    pub fileExists: i32,
    pub sameAreaInnerAddr: Vec<ShortVideoIpList>,
    pub diffAreaInnerAddr: Vec<ShortVideoIpList>,
    pub dataHole: Vec<DataHole>,
}

impl<'a> MessageRead<'a> for ShortVideoUploadRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.sameAreaOutAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(34) => msg.diffAreaOutAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(42) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.uKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.fileExists = r.read_int32(bytes)?,
                Ok(66) => msg.sameAreaInnerAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(74) => msg.diffAreaInnerAddr.push(r.read_message::<ShortVideoIpList>(bytes)?),
                Ok(82) => msg.dataHole.push(r.read_message::<DataHole>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoUploadRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + self.sameAreaOutAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.diffAreaOutAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + if self.uKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.uKey).len()) }
        + if self.fileExists == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileExists) as u64) }
        + self.sameAreaInnerAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.diffAreaInnerAddr.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.dataHole.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        for s in &self.sameAreaOutAddr { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.diffAreaOutAddr { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.fileId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.fileId))?; }
        if self.uKey != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.uKey))?; }
        if self.fileExists != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.fileExists))?; }
        for s in &self.sameAreaInnerAddr { w.write_with_tag(66, |w| w.write_message(s))?; }
        for s in &self.diffAreaInnerAddr { w.write_with_tag(74, |w| w.write_message(s))?; }
        for s in &self.dataHole { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoFileInfo<'a> {
    pub fileName: Cow<'a, str>,
    pub fileMd5: Cow<'a, [u8]>,
    pub thumbFileMd5: Cow<'a, [u8]>,
    pub fileSize: i64,
    pub fileResLength: i32,
    pub fileResWidth: i32,
    pub fileFormat: i32,
    pub fileTime: i32,
    pub thumbFileSize: i64,
}

impl<'a> MessageRead<'a> for ShortVideoFileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.thumbFileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.fileSize = r.read_int64(bytes)?,
                Ok(40) => msg.fileResLength = r.read_int32(bytes)?,
                Ok(48) => msg.fileResWidth = r.read_int32(bytes)?,
                Ok(56) => msg.fileFormat = r.read_int32(bytes)?,
                Ok(64) => msg.fileTime = r.read_int32(bytes)?,
                Ok(72) => msg.thumbFileSize = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoFileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fileName == "" { 0 } else { 1 + sizeof_len((&self.fileName).len()) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileMd5).len()) }
        + if self.thumbFileMd5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.thumbFileMd5).len()) }
        + if self.fileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileResLength == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileResLength) as u64) }
        + if self.fileResWidth == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileResWidth) as u64) }
        + if self.fileFormat == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileFormat) as u64) }
        + if self.fileTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileTime) as u64) }
        + if self.thumbFileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.thumbFileSize) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fileName != "" { w.write_with_tag(10, |w| w.write_string(&**&self.fileName))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.thumbFileMd5 != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.thumbFileMd5))?; }
        if self.fileSize != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileResLength != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.fileResLength))?; }
        if self.fileResWidth != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.fileResWidth))?; }
        if self.fileFormat != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.fileFormat))?; }
        if self.fileTime != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.fileTime))?; }
        if self.thumbFileSize != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.thumbFileSize))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DataHole {
    pub begin: i64,
    pub end: i64,
}

impl<'a> MessageRead<'a> for DataHole {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.begin = r.read_int64(bytes)?,
                Ok(16) => msg.end = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DataHole {
    fn get_size(&self) -> usize {
        0
        + if self.begin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.begin) as u64) }
        + if self.end == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.end) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.begin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.begin))?; }
        if self.end != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.end))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoIpList {
    pub ip: i32,
    pub port: i32,
}

impl<'a> MessageRead<'a> for ShortVideoIpList {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ip = r.read_int32(bytes)?,
                Ok(16) => msg.port = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ShortVideoIpList {
    fn get_size(&self) -> usize {
        0
        + if self.ip == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ip) as u64) }
        + if self.port == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.port) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ip != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.ip))?; }
        if self.port != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.port))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoAddr<'a> {
    pub host: Vec<Cow<'a, str>>,
    pub urlArgs: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ShortVideoAddr<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(82) => msg.host.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.urlArgs = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoAddr<'a> {
    fn get_size(&self) -> usize {
        0
        + self.host.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.urlArgs == "" { 0 } else { 1 + sizeof_len((&self.urlArgs).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.host { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if self.urlArgs != "" { w.write_with_tag(90, |w| w.write_string(&**&self.urlArgs))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoExtensionReq {
    pub subBusiType: i32,
    pub userCnt: i32,
}

impl<'a> MessageRead<'a> for ShortVideoExtensionReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subBusiType = r.read_int32(bytes)?,
                Ok(16) => msg.userCnt = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ShortVideoExtensionReq {
    fn get_size(&self) -> usize {
        0
        + if self.subBusiType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subBusiType) as u64) }
        + if self.userCnt == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.userCnt) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subBusiType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.subBusiType))?; }
        if self.userCnt != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.userCnt))?; }
        Ok(())
    }
}

