// Automatically generated rust module for 'cmd0x346.proto' file

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
pub struct ApplyCleanTrafficRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ApplyCleanTrafficRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyCleanTrafficRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyCopyFromReq<'a> {
    pub srcUin: i64,
    pub srcGroup: i64,
    pub srcSvcid: i32,
    pub srcParentfolder: Cow<'a, [u8]>,
    pub srcUuid: Cow<'a, [u8]>,
    pub fileMd5: Cow<'a, [u8]>,
    pub dstUin: i64,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyCopyFromReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.srcUin = r.read_int64(bytes)?,
                Ok(160) => msg.srcGroup = r.read_int64(bytes)?,
                Ok(240) => msg.srcSvcid = r.read_int32(bytes)?,
                Ok(322) => msg.srcParentfolder = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(402) => msg.srcUuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.fileMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(560) => msg.dstUin = r.read_int64(bytes)?,
                Ok(640) => msg.fileSize = r.read_int64(bytes)?,
                Ok(722) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(800) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(880) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyCopyFromReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.srcUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.srcUin) as u64) }
        + if self.srcGroup == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.srcGroup) as u64) }
        + if self.srcSvcid == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.srcSvcid) as u64) }
        + if self.srcParentfolder == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.srcParentfolder).len()) }
        + if self.srcUuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.srcUuid).len()) }
        + if self.fileMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.fileMd5).len()) }
        + if self.dstUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.dstUin) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.srcUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.srcUin))?; }
        if self.srcGroup != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.srcGroup))?; }
        if self.srcSvcid != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.srcSvcid))?; }
        if self.srcParentfolder != Cow::Borrowed(b"") { w.write_with_tag(322, |w| w.write_bytes(&**&self.srcParentfolder))?; }
        if self.srcUuid != Cow::Borrowed(b"") { w.write_with_tag(402, |w| w.write_bytes(&**&self.srcUuid))?; }
        if self.fileMd5 != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.fileMd5))?; }
        if self.dstUin != 0i64 { w.write_with_tag(560, |w| w.write_int64(*&self.dstUin))?; }
        if self.fileSize != 0i64 { w.write_with_tag(640, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(722, |w| w.write_string(&**&self.fileName))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(800, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(880, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyCopyFromRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub uuid: Cow<'a, [u8]>,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyCopyFromRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(320) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyCopyFromRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(242, |w| w.write_bytes(&**&self.uuid))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyCopyToReq<'a> {
    pub dstId: i64,
    pub dstUin: i64,
    pub dstSvcid: i32,
    pub srcUin: i64,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub localFilepath: Cow<'a, str>,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ApplyCopyToReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.dstId = r.read_int64(bytes)?,
                Ok(160) => msg.dstUin = r.read_int64(bytes)?,
                Ok(240) => msg.dstSvcid = r.read_int32(bytes)?,
                Ok(320) => msg.srcUin = r.read_int64(bytes)?,
                Ok(400) => msg.fileSize = r.read_int64(bytes)?,
                Ok(482) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.localFilepath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(642) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyCopyToReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.dstId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.dstId) as u64) }
        + if self.dstUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.dstUin) as u64) }
        + if self.dstSvcid == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dstSvcid) as u64) }
        + if self.srcUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.srcUin) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.localFilepath == "" { 0 } else { 2 + sizeof_len((&self.localFilepath).len()) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.dstId != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.dstId))?; }
        if self.dstUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.dstUin))?; }
        if self.dstSvcid != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.dstSvcid))?; }
        if self.srcUin != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.srcUin))?; }
        if self.fileSize != 0i64 { w.write_with_tag(400, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(482, |w| w.write_string(&**&self.fileName))?; }
        if self.localFilepath != "" { w.write_with_tag(562, |w| w.write_string(&**&self.localFilepath))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(642, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyCopyToRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub fileKey: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ApplyCopyToRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.fileKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyCopyToRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.fileKey == "" { 0 } else { 2 + sizeof_len((&self.fileKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.fileKey != "" { w.write_with_tag(242, |w| w.write_string(&**&self.fileKey))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyDownloadAbsReq<'a> {
    pub uin: i64,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ApplyDownloadAbsReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.uin = r.read_int64(bytes)?,
                Ok(162) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyDownloadAbsReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.uin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(162, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyDownloadAbsRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub downloadInfo: Option<DownloadInfo<'a>>,
}

impl<'a> MessageRead<'a> for ApplyDownloadAbsRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.downloadInfo = Some(r.read_message::<DownloadInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyDownloadAbsRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + self.downloadInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if let Some(ref s) = self.downloadInfo { w.write_with_tag(242, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyDownloadReq<'a> {
    pub uin: i64,
    pub uuid: Cow<'a, [u8]>,
    pub ownerType: i32,
    pub extIntype: i32,
}

impl<'a> MessageRead<'a> for ApplyDownloadReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.uin = r.read_int64(bytes)?,
                Ok(162) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.ownerType = r.read_int32(bytes)?,
                Ok(4000) => msg.extIntype = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyDownloadReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.ownerType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.ownerType) as u64) }
        + if self.extIntype == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.extIntype) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.uin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(162, |w| w.write_bytes(&**&self.uuid))?; }
        if self.ownerType != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.ownerType))?; }
        if self.extIntype != 0i32 { w.write_with_tag(4000, |w| w.write_int32(*&self.extIntype))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyDownloadRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub downloadInfo: Option<DownloadInfo<'a>>,
    pub fileInfo: Option<FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for ApplyDownloadRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.downloadInfo = Some(r.read_message::<DownloadInfo>(bytes)?),
                Ok(322) => msg.fileInfo = Some(r.read_message::<FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyDownloadRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + self.downloadInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.fileInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if let Some(ref s) = self.downloadInfo { w.write_with_tag(242, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileInfo { w.write_with_tag(322, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyForwardFileReq<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub uuid: Cow<'a, [u8]>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyForwardFileReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(242) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(320) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(400) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyForwardFileReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(242, |w| w.write_bytes(&**&self.uuid))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(400, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyForwardFileRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub totalSpace: i64,
    pub usedSpace: i64,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ApplyForwardFileRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(320) => msg.usedSpace = r.read_int64(bytes)?,
                Ok(402) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyForwardFileRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
        + if self.usedSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.usedSpace) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.totalSpace))?; }
        if self.usedSpace != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.usedSpace))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(402, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyGetTrafficReq { }

impl<'a> MessageRead<'a> for ApplyGetTrafficReq {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ApplyGetTrafficReq { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyGetTrafficRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub useFileSize: i64,
    pub useFileNum: i32,
    pub allFileSize: i64,
    pub allFileNum: i32,
}

impl<'a> MessageRead<'a> for ApplyGetTrafficRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.useFileSize = r.read_int64(bytes)?,
                Ok(320) => msg.useFileNum = r.read_int32(bytes)?,
                Ok(400) => msg.allFileSize = r.read_int64(bytes)?,
                Ok(480) => msg.allFileNum = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyGetTrafficRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.useFileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.useFileSize) as u64) }
        + if self.useFileNum == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.useFileNum) as u64) }
        + if self.allFileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.allFileSize) as u64) }
        + if self.allFileNum == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.allFileNum) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.useFileSize != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.useFileSize))?; }
        if self.useFileNum != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.useFileNum))?; }
        if self.allFileSize != 0i64 { w.write_with_tag(400, |w| w.write_int64(*&self.allFileSize))?; }
        if self.allFileNum != 0i32 { w.write_with_tag(480, |w| w.write_int32(*&self.allFileNum))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyListDownloadReq {
    pub uin: i64,
    pub beginIndex: i32,
    pub reqCount: i32,
}

impl<'a> MessageRead<'a> for ApplyListDownloadReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.uin = r.read_int64(bytes)?,
                Ok(160) => msg.beginIndex = r.read_int32(bytes)?,
                Ok(240) => msg.reqCount = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ApplyListDownloadReq {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.beginIndex == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.beginIndex) as u64) }
        + if self.reqCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.reqCount) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.uin))?; }
        if self.beginIndex != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.beginIndex))?; }
        if self.reqCount != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.reqCount))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyListDownloadRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub totalCount: i32,
    pub beginIndex: i32,
    pub rspCount: i32,
    pub isEnd: i32,
    pub fileList: Vec<FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for ApplyListDownloadRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.totalCount = r.read_int32(bytes)?,
                Ok(320) => msg.beginIndex = r.read_int32(bytes)?,
                Ok(400) => msg.rspCount = r.read_int32(bytes)?,
                Ok(480) => msg.isEnd = r.read_int32(bytes)?,
                Ok(562) => msg.fileList.push(r.read_message::<FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyListDownloadRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.totalCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.totalCount) as u64) }
        + if self.beginIndex == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.beginIndex) as u64) }
        + if self.rspCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.rspCount) as u64) }
        + if self.isEnd == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.isEnd) as u64) }
        + self.fileList.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.totalCount != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.totalCount))?; }
        if self.beginIndex != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.beginIndex))?; }
        if self.rspCount != 0i32 { w.write_with_tag(400, |w| w.write_int32(*&self.rspCount))?; }
        if self.isEnd != 0i32 { w.write_with_tag(480, |w| w.write_int32(*&self.isEnd))?; }
        for s in &self.fileList { w.write_with_tag(562, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadHitReq<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub bytes_10mMd5: Cow<'a, [u8]>,
    pub localFilepath: Cow<'a, str>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadHitReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(240) => msg.fileSize = r.read_int64(bytes)?,
                Ok(322) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(402) => msg.bytes_10mMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.localFilepath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(560) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(640) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadHitReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.bytes_10mMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_10mMd5).len()) }
        + if self.localFilepath == "" { 0 } else { 2 + sizeof_len((&self.localFilepath).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.fileSize != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(322, |w| w.write_string(&**&self.fileName))?; }
        if self.bytes_10mMd5 != Cow::Borrowed(b"") { w.write_with_tag(402, |w| w.write_bytes(&**&self.bytes_10mMd5))?; }
        if self.localFilepath != "" { w.write_with_tag(482, |w| w.write_string(&**&self.localFilepath))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(560, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(640, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadHitReqV2<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub bytes_10mMd5: Cow<'a, [u8]>,
    pub bytes_3sha: Cow<'a, [u8]>,
    pub sha: Cow<'a, [u8]>,
    pub localFilepath: Cow<'a, str>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadHitReqV2<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(240) => msg.fileSize = r.read_int64(bytes)?,
                Ok(322) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(402) => msg.bytes_10mMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.bytes_3sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(642) => msg.localFilepath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(720) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(800) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadHitReqV2<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.bytes_10mMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_10mMd5).len()) }
        + if self.bytes_3sha == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_3sha).len()) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.sha).len()) }
        + if self.localFilepath == "" { 0 } else { 2 + sizeof_len((&self.localFilepath).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.fileSize != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(322, |w| w.write_string(&**&self.fileName))?; }
        if self.bytes_10mMd5 != Cow::Borrowed(b"") { w.write_with_tag(402, |w| w.write_bytes(&**&self.bytes_10mMd5))?; }
        if self.bytes_3sha != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.bytes_3sha))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(562, |w| w.write_bytes(&**&self.sha))?; }
        if self.localFilepath != "" { w.write_with_tag(642, |w| w.write_string(&**&self.localFilepath))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(720, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(800, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadHitReqV3<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub bytes_10mMd5: Cow<'a, [u8]>,
    pub sha: Cow<'a, [u8]>,
    pub localFilepath: Cow<'a, str>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadHitReqV3<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(240) => msg.fileSize = r.read_int64(bytes)?,
                Ok(322) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(402) => msg.bytes_10mMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.localFilepath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(720) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadHitReqV3<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.bytes_10mMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_10mMd5).len()) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.sha).len()) }
        + if self.localFilepath == "" { 0 } else { 2 + sizeof_len((&self.localFilepath).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.fileSize != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(322, |w| w.write_string(&**&self.fileName))?; }
        if self.bytes_10mMd5 != Cow::Borrowed(b"") { w.write_with_tag(402, |w| w.write_bytes(&**&self.bytes_10mMd5))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.sha))?; }
        if self.localFilepath != "" { w.write_with_tag(562, |w| w.write_string(&**&self.localFilepath))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(640, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(720, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadHitRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub uploadIp: Cow<'a, str>,
    pub uploadPort: i32,
    pub uploadDomain: Cow<'a, str>,
    pub uuid: Cow<'a, [u8]>,
    pub uploadKey: Cow<'a, [u8]>,
    pub totalSpace: i64,
    pub usedSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadHitRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(320) => msg.uploadPort = r.read_int32(bytes)?,
                Ok(402) => msg.uploadDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.uploadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(720) => msg.usedSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadHitRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.uploadIp == "" { 0 } else { 2 + sizeof_len((&self.uploadIp).len()) }
        + if self.uploadPort == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.uploadPort) as u64) }
        + if self.uploadDomain == "" { 0 } else { 2 + sizeof_len((&self.uploadDomain).len()) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.uploadKey == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uploadKey).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
        + if self.usedSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.usedSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.uploadIp != "" { w.write_with_tag(242, |w| w.write_string(&**&self.uploadIp))?; }
        if self.uploadPort != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.uploadPort))?; }
        if self.uploadDomain != "" { w.write_with_tag(402, |w| w.write_string(&**&self.uploadDomain))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.uuid))?; }
        if self.uploadKey != Cow::Borrowed(b"") { w.write_with_tag(562, |w| w.write_bytes(&**&self.uploadKey))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(640, |w| w.write_int64(*&self.totalSpace))?; }
        if self.usedSpace != 0i64 { w.write_with_tag(720, |w| w.write_int64(*&self.usedSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadHitRspV2<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub uploadIp: Cow<'a, str>,
    pub uploadPort: i32,
    pub uploadDomain: Cow<'a, str>,
    pub uuid: Cow<'a, [u8]>,
    pub uploadKey: Cow<'a, [u8]>,
    pub totalSpace: i64,
    pub usedSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadHitRspV2<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(320) => msg.uploadPort = r.read_int32(bytes)?,
                Ok(402) => msg.uploadDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.uploadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(720) => msg.usedSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadHitRspV2<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.uploadIp == "" { 0 } else { 2 + sizeof_len((&self.uploadIp).len()) }
        + if self.uploadPort == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.uploadPort) as u64) }
        + if self.uploadDomain == "" { 0 } else { 2 + sizeof_len((&self.uploadDomain).len()) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.uploadKey == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uploadKey).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
        + if self.usedSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.usedSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.uploadIp != "" { w.write_with_tag(242, |w| w.write_string(&**&self.uploadIp))?; }
        if self.uploadPort != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.uploadPort))?; }
        if self.uploadDomain != "" { w.write_with_tag(402, |w| w.write_string(&**&self.uploadDomain))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.uuid))?; }
        if self.uploadKey != Cow::Borrowed(b"") { w.write_with_tag(562, |w| w.write_bytes(&**&self.uploadKey))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(640, |w| w.write_int64(*&self.totalSpace))?; }
        if self.usedSpace != 0i64 { w.write_with_tag(720, |w| w.write_int64(*&self.usedSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadHitRspV3<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub uploadIp: Cow<'a, str>,
    pub uploadPort: i32,
    pub uploadDomain: Cow<'a, str>,
    pub uuid: Cow<'a, [u8]>,
    pub uploadKey: Cow<'a, [u8]>,
    pub totalSpace: i64,
    pub usedSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadHitRspV3<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(320) => msg.uploadPort = r.read_int32(bytes)?,
                Ok(402) => msg.uploadDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.uploadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(720) => msg.usedSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadHitRspV3<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.uploadIp == "" { 0 } else { 2 + sizeof_len((&self.uploadIp).len()) }
        + if self.uploadPort == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.uploadPort) as u64) }
        + if self.uploadDomain == "" { 0 } else { 2 + sizeof_len((&self.uploadDomain).len()) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.uploadKey == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uploadKey).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
        + if self.usedSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.usedSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.uploadIp != "" { w.write_with_tag(242, |w| w.write_string(&**&self.uploadIp))?; }
        if self.uploadPort != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.uploadPort))?; }
        if self.uploadDomain != "" { w.write_with_tag(402, |w| w.write_string(&**&self.uploadDomain))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.uuid))?; }
        if self.uploadKey != Cow::Borrowed(b"") { w.write_with_tag(562, |w| w.write_bytes(&**&self.uploadKey))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(640, |w| w.write_int64(*&self.totalSpace))?; }
        if self.usedSpace != 0i64 { w.write_with_tag(720, |w| w.write_int64(*&self.usedSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadReq<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub fileType: i32,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub bytes_10mMd5: Cow<'a, [u8]>,
    pub localFilepath: Cow<'a, str>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(240) => msg.fileType = r.read_int32(bytes)?,
                Ok(320) => msg.fileSize = r.read_int64(bytes)?,
                Ok(402) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.bytes_10mMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.localFilepath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(720) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.fileType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.fileType) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.bytes_10mMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_10mMd5).len()) }
        + if self.localFilepath == "" { 0 } else { 2 + sizeof_len((&self.localFilepath).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.fileType != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.fileType))?; }
        if self.fileSize != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(402, |w| w.write_string(&**&self.fileName))?; }
        if self.bytes_10mMd5 != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.bytes_10mMd5))?; }
        if self.localFilepath != "" { w.write_with_tag(562, |w| w.write_string(&**&self.localFilepath))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(640, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(720, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadReqV2<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub bytes_10mMd5: Cow<'a, [u8]>,
    pub bytes_3sha: Cow<'a, [u8]>,
    pub localFilepath: Cow<'a, str>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadReqV2<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(240) => msg.fileSize = r.read_int64(bytes)?,
                Ok(322) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(402) => msg.bytes_10mMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.bytes_3sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.localFilepath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(720) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadReqV2<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.bytes_10mMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_10mMd5).len()) }
        + if self.bytes_3sha == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_3sha).len()) }
        + if self.localFilepath == "" { 0 } else { 2 + sizeof_len((&self.localFilepath).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.fileSize != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(322, |w| w.write_string(&**&self.fileName))?; }
        if self.bytes_10mMd5 != Cow::Borrowed(b"") { w.write_with_tag(402, |w| w.write_bytes(&**&self.bytes_10mMd5))?; }
        if self.bytes_3sha != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.bytes_3sha))?; }
        if self.localFilepath != "" { w.write_with_tag(562, |w| w.write_string(&**&self.localFilepath))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(640, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(720, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadReqV3<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub bytes_10mMd5: Cow<'a, [u8]>,
    pub sha: Cow<'a, [u8]>,
    pub localFilepath: Cow<'a, str>,
    pub dangerLevel: i32,
    pub totalSpace: i64,
}

impl<'a> MessageRead<'a> for ApplyUploadReqV3<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(240) => msg.fileSize = r.read_int64(bytes)?,
                Ok(322) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(402) => msg.bytes_10mMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.localFilepath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.dangerLevel = r.read_int32(bytes)?,
                Ok(720) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadReqV3<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 2 + sizeof_len((&self.fileName).len()) }
        + if self.bytes_10mMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_10mMd5).len()) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.sha).len()) }
        + if self.localFilepath == "" { 0 } else { 2 + sizeof_len((&self.localFilepath).len()) }
        + if self.dangerLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.dangerLevel) as u64) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.fileSize != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(322, |w| w.write_string(&**&self.fileName))?; }
        if self.bytes_10mMd5 != Cow::Borrowed(b"") { w.write_with_tag(402, |w| w.write_bytes(&**&self.bytes_10mMd5))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(482, |w| w.write_bytes(&**&self.sha))?; }
        if self.localFilepath != "" { w.write_with_tag(562, |w| w.write_string(&**&self.localFilepath))?; }
        if self.dangerLevel != 0i32 { w.write_with_tag(640, |w| w.write_int32(*&self.dangerLevel))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(720, |w| w.write_int64(*&self.totalSpace))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub totalSpace: i64,
    pub usedSpace: i64,
    pub uploadedSize: i64,
    pub uploadIp: Cow<'a, str>,
    pub uploadDomain: Cow<'a, str>,
    pub uploadPort: i32,
    pub uuid: Cow<'a, [u8]>,
    pub uploadKey: Cow<'a, [u8]>,
    pub boolFileExist: bool,
    pub packSize: i32,
    pub uploadipList: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ApplyUploadRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(320) => msg.usedSpace = r.read_int64(bytes)?,
                Ok(400) => msg.uploadedSize = r.read_int64(bytes)?,
                Ok(482) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.uploadDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.uploadPort = r.read_int32(bytes)?,
                Ok(722) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(802) => msg.uploadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(880) => msg.boolFileExist = r.read_bool(bytes)?,
                Ok(960) => msg.packSize = r.read_int32(bytes)?,
                Ok(1042) => msg.uploadipList.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
        + if self.usedSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.usedSpace) as u64) }
        + if self.uploadedSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.uploadedSize) as u64) }
        + if self.uploadIp == "" { 0 } else { 2 + sizeof_len((&self.uploadIp).len()) }
        + if self.uploadDomain == "" { 0 } else { 2 + sizeof_len((&self.uploadDomain).len()) }
        + if self.uploadPort == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.uploadPort) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.uploadKey == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uploadKey).len()) }
        + if self.boolFileExist == false { 0 } else { 2 + sizeof_varint(*(&self.boolFileExist) as u64) }
        + if self.packSize == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.packSize) as u64) }
        + self.uploadipList.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.totalSpace))?; }
        if self.usedSpace != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.usedSpace))?; }
        if self.uploadedSize != 0i64 { w.write_with_tag(400, |w| w.write_int64(*&self.uploadedSize))?; }
        if self.uploadIp != "" { w.write_with_tag(482, |w| w.write_string(&**&self.uploadIp))?; }
        if self.uploadDomain != "" { w.write_with_tag(562, |w| w.write_string(&**&self.uploadDomain))?; }
        if self.uploadPort != 0i32 { w.write_with_tag(640, |w| w.write_int32(*&self.uploadPort))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(722, |w| w.write_bytes(&**&self.uuid))?; }
        if self.uploadKey != Cow::Borrowed(b"") { w.write_with_tag(802, |w| w.write_bytes(&**&self.uploadKey))?; }
        if self.boolFileExist != false { w.write_with_tag(880, |w| w.write_bool(*&self.boolFileExist))?; }
        if self.packSize != 0i32 { w.write_with_tag(960, |w| w.write_int32(*&self.packSize))?; }
        for s in &self.uploadipList { w.write_with_tag(1042, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadRspV2<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub totalSpace: i64,
    pub usedSpace: i64,
    pub uploadedSize: i64,
    pub uploadIp: Cow<'a, str>,
    pub uploadDomain: Cow<'a, str>,
    pub uploadPort: i32,
    pub uuid: Cow<'a, [u8]>,
    pub uploadKey: Cow<'a, [u8]>,
    pub boolFileExist: bool,
    pub packSize: i32,
    pub uploadipList: Vec<Cow<'a, str>>,
    pub httpsvrApiVer: i32,
    pub sha: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ApplyUploadRspV2<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(320) => msg.usedSpace = r.read_int64(bytes)?,
                Ok(400) => msg.uploadedSize = r.read_int64(bytes)?,
                Ok(482) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.uploadDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.uploadPort = r.read_int32(bytes)?,
                Ok(722) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(802) => msg.uploadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(880) => msg.boolFileExist = r.read_bool(bytes)?,
                Ok(960) => msg.packSize = r.read_int32(bytes)?,
                Ok(1042) => msg.uploadipList.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(1120) => msg.httpsvrApiVer = r.read_int32(bytes)?,
                Ok(1130) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadRspV2<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
        + if self.usedSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.usedSpace) as u64) }
        + if self.uploadedSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.uploadedSize) as u64) }
        + if self.uploadIp == "" { 0 } else { 2 + sizeof_len((&self.uploadIp).len()) }
        + if self.uploadDomain == "" { 0 } else { 2 + sizeof_len((&self.uploadDomain).len()) }
        + if self.uploadPort == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.uploadPort) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.uploadKey == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uploadKey).len()) }
        + if self.boolFileExist == false { 0 } else { 2 + sizeof_varint(*(&self.boolFileExist) as u64) }
        + if self.packSize == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.packSize) as u64) }
        + self.uploadipList.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + if self.httpsvrApiVer == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.httpsvrApiVer) as u64) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.sha).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.totalSpace))?; }
        if self.usedSpace != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.usedSpace))?; }
        if self.uploadedSize != 0i64 { w.write_with_tag(400, |w| w.write_int64(*&self.uploadedSize))?; }
        if self.uploadIp != "" { w.write_with_tag(482, |w| w.write_string(&**&self.uploadIp))?; }
        if self.uploadDomain != "" { w.write_with_tag(562, |w| w.write_string(&**&self.uploadDomain))?; }
        if self.uploadPort != 0i32 { w.write_with_tag(640, |w| w.write_int32(*&self.uploadPort))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(722, |w| w.write_bytes(&**&self.uuid))?; }
        if self.uploadKey != Cow::Borrowed(b"") { w.write_with_tag(802, |w| w.write_bytes(&**&self.uploadKey))?; }
        if self.boolFileExist != false { w.write_with_tag(880, |w| w.write_bool(*&self.boolFileExist))?; }
        if self.packSize != 0i32 { w.write_with_tag(960, |w| w.write_int32(*&self.packSize))?; }
        for s in &self.uploadipList { w.write_with_tag(1042, |w| w.write_string(&**s))?; }
        if self.httpsvrApiVer != 0i32 { w.write_with_tag(1120, |w| w.write_int32(*&self.httpsvrApiVer))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(1130, |w| w.write_bytes(&**&self.sha))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplyUploadRspV3<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub totalSpace: i64,
    pub usedSpace: i64,
    pub uploadedSize: i64,
    pub uploadIp: Cow<'a, str>,
    pub uploadDomain: Cow<'a, str>,
    pub uploadPort: i32,
    pub uuid: Cow<'a, [u8]>,
    pub uploadKey: Cow<'a, [u8]>,
    pub boolFileExist: bool,
    pub packSize: i32,
    pub uploadIpList: Vec<Cow<'a, str>>,
    pub uploadHttpsPort: i32,
    pub uploadHttpsDomain: Cow<'a, str>,
    pub uploadDns: Cow<'a, str>,
    pub uploadLanip: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ApplyUploadRspV3<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.totalSpace = r.read_int64(bytes)?,
                Ok(320) => msg.usedSpace = r.read_int64(bytes)?,
                Ok(400) => msg.uploadedSize = r.read_int64(bytes)?,
                Ok(482) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(562) => msg.uploadDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(640) => msg.uploadPort = r.read_int32(bytes)?,
                Ok(722) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(802) => msg.uploadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(880) => msg.boolFileExist = r.read_bool(bytes)?,
                Ok(960) => msg.packSize = r.read_int32(bytes)?,
                Ok(1042) => msg.uploadIpList.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(1120) => msg.uploadHttpsPort = r.read_int32(bytes)?,
                Ok(1202) => msg.uploadHttpsDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(1282) => msg.uploadDns = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(1362) => msg.uploadLanip = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplyUploadRspV3<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.totalSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.totalSpace) as u64) }
        + if self.usedSpace == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.usedSpace) as u64) }
        + if self.uploadedSize == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.uploadedSize) as u64) }
        + if self.uploadIp == "" { 0 } else { 2 + sizeof_len((&self.uploadIp).len()) }
        + if self.uploadDomain == "" { 0 } else { 2 + sizeof_len((&self.uploadDomain).len()) }
        + if self.uploadPort == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.uploadPort) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
        + if self.uploadKey == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uploadKey).len()) }
        + if self.boolFileExist == false { 0 } else { 2 + sizeof_varint(*(&self.boolFileExist) as u64) }
        + if self.packSize == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.packSize) as u64) }
        + self.uploadIpList.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + if self.uploadHttpsPort == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.uploadHttpsPort) as u64) }
        + if self.uploadHttpsDomain == "" { 0 } else { 2 + sizeof_len((&self.uploadHttpsDomain).len()) }
        + if self.uploadDns == "" { 0 } else { 2 + sizeof_len((&self.uploadDns).len()) }
        + if self.uploadLanip == "" { 0 } else { 2 + sizeof_len((&self.uploadLanip).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.totalSpace != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.totalSpace))?; }
        if self.usedSpace != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.usedSpace))?; }
        if self.uploadedSize != 0i64 { w.write_with_tag(400, |w| w.write_int64(*&self.uploadedSize))?; }
        if self.uploadIp != "" { w.write_with_tag(482, |w| w.write_string(&**&self.uploadIp))?; }
        if self.uploadDomain != "" { w.write_with_tag(562, |w| w.write_string(&**&self.uploadDomain))?; }
        if self.uploadPort != 0i32 { w.write_with_tag(640, |w| w.write_int32(*&self.uploadPort))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(722, |w| w.write_bytes(&**&self.uuid))?; }
        if self.uploadKey != Cow::Borrowed(b"") { w.write_with_tag(802, |w| w.write_bytes(&**&self.uploadKey))?; }
        if self.boolFileExist != false { w.write_with_tag(880, |w| w.write_bool(*&self.boolFileExist))?; }
        if self.packSize != 0i32 { w.write_with_tag(960, |w| w.write_int32(*&self.packSize))?; }
        for s in &self.uploadIpList { w.write_with_tag(1042, |w| w.write_string(&**s))?; }
        if self.uploadHttpsPort != 0i32 { w.write_with_tag(1120, |w| w.write_int32(*&self.uploadHttpsPort))?; }
        if self.uploadHttpsDomain != "" { w.write_with_tag(1202, |w| w.write_string(&**&self.uploadHttpsDomain))?; }
        if self.uploadDns != "" { w.write_with_tag(1282, |w| w.write_string(&**&self.uploadDns))?; }
        if self.uploadLanip != "" { w.write_with_tag(1362, |w| w.write_string(&**&self.uploadLanip))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DelMessageReq {
    pub uinSender: i64,
    pub uinReceiver: i64,
    pub time: i32,
    pub random: i32,
    pub seqNo: i32,
}

impl<'a> MessageRead<'a> for DelMessageReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uinSender = r.read_int64(bytes)?,
                Ok(16) => msg.uinReceiver = r.read_int64(bytes)?,
                Ok(80) => msg.time = r.read_int32(bytes)?,
                Ok(160) => msg.random = r.read_int32(bytes)?,
                Ok(240) => msg.seqNo = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DelMessageReq {
    fn get_size(&self) -> usize {
        0
        + if self.uinSender == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uinSender) as u64) }
        + if self.uinReceiver == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uinReceiver) as u64) }
        + if self.time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.time) as u64) }
        + if self.random == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.random) as u64) }
        + if self.seqNo == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.seqNo) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uinSender != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uinSender))?; }
        if self.uinReceiver != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.uinReceiver))?; }
        if self.time != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.time))?; }
        if self.random != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.random))?; }
        if self.seqNo != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.seqNo))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteFileReq<'a> {
    pub uin: i64,
    pub peerUin: i64,
    pub deleteType: i32,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for DeleteFileReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.uin = r.read_int64(bytes)?,
                Ok(160) => msg.peerUin = r.read_int64(bytes)?,
                Ok(240) => msg.deleteType = r.read_int32(bytes)?,
                Ok(322) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteFileReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.peerUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.peerUin) as u64) }
        + if self.deleteType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.deleteType) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.uin))?; }
        if self.peerUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.peerUin))?; }
        if self.deleteType != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.deleteType))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(322, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteFileRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DeleteFileRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteFileRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DownloadInfo<'a> {
    pub downloadKey: Cow<'a, [u8]>,
    pub downloadIp: Cow<'a, str>,
    pub downloadDomain: Cow<'a, str>,
    pub port: i32,
    pub downloadUrl: Cow<'a, str>,
    pub downloadipList: Vec<Cow<'a, str>>,
    pub cookie: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DownloadInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(82) => msg.downloadKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(162) => msg.downloadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.downloadDomain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(320) => msg.port = r.read_int32(bytes)?,
                Ok(402) => msg.downloadUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.downloadipList.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(562) => msg.cookie = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DownloadInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.downloadKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.downloadKey).len()) }
        + if self.downloadIp == "" { 0 } else { 2 + sizeof_len((&self.downloadIp).len()) }
        + if self.downloadDomain == "" { 0 } else { 2 + sizeof_len((&self.downloadDomain).len()) }
        + if self.port == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.port) as u64) }
        + if self.downloadUrl == "" { 0 } else { 2 + sizeof_len((&self.downloadUrl).len()) }
        + self.downloadipList.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + if self.cookie == "" { 0 } else { 2 + sizeof_len((&self.cookie).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.downloadKey != Cow::Borrowed(b"") { w.write_with_tag(82, |w| w.write_bytes(&**&self.downloadKey))?; }
        if self.downloadIp != "" { w.write_with_tag(162, |w| w.write_string(&**&self.downloadIp))?; }
        if self.downloadDomain != "" { w.write_with_tag(242, |w| w.write_string(&**&self.downloadDomain))?; }
        if self.port != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.port))?; }
        if self.downloadUrl != "" { w.write_with_tag(402, |w| w.write_string(&**&self.downloadUrl))?; }
        for s in &self.downloadipList { w.write_with_tag(482, |w| w.write_string(&**s))?; }
        if self.cookie != "" { w.write_with_tag(562, |w| w.write_string(&**&self.cookie))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DownloadSuccReq<'a> {
    pub uin: i64,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for DownloadSuccReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.uin = r.read_int64(bytes)?,
                Ok(162) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DownloadSuccReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.uin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(162, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DownloadSuccRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub downStat: i32,
}

impl<'a> MessageRead<'a> for DownloadSuccRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.downStat = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DownloadSuccRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + if self.downStat == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.downStat) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if self.downStat != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.downStat))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExtensionReq<'a> {
    pub id: i64,
    pub type_pb: i64,
    pub dstPhonenum: Cow<'a, str>,
    pub phoneConvertType: i32,
    pub sig: Cow<'a, [u8]>,
    pub routeId: i64,
    pub delMessageReq: Option<DelMessageReq>,
    pub downloadUrlType: i32,
    pub pttFormat: i32,
    pub isNeedInnerIp: i32,
    pub netType: i32,
    pub voiceType: i32,
    pub fileType: i32,
    pub pttTime: i32,
}

impl<'a> MessageRead<'a> for ExtensionReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(16) => msg.type_pb = r.read_int64(bytes)?,
                Ok(26) => msg.dstPhonenum = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.phoneConvertType = r.read_int32(bytes)?,
                Ok(162) => msg.sig = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(800) => msg.routeId = r.read_int64(bytes)?,
                Ok(720802) => msg.delMessageReq = Some(r.read_message::<DelMessageReq>(bytes)?),
                Ok(721600) => msg.downloadUrlType = r.read_int32(bytes)?,
                Ok(722400) => msg.pttFormat = r.read_int32(bytes)?,
                Ok(723200) => msg.isNeedInnerIp = r.read_int32(bytes)?,
                Ok(724000) => msg.netType = r.read_int32(bytes)?,
                Ok(724800) => msg.voiceType = r.read_int32(bytes)?,
                Ok(725600) => msg.fileType = r.read_int32(bytes)?,
                Ok(726400) => msg.pttTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExtensionReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.type_pb == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.dstPhonenum == "" { 0 } else { 1 + sizeof_len((&self.dstPhonenum).len()) }
        + if self.phoneConvertType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.phoneConvertType) as u64) }
        + if self.sig == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.sig).len()) }
        + if self.routeId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.routeId) as u64) }
        + self.delMessageReq.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
        + if self.downloadUrlType == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.downloadUrlType) as u64) }
        + if self.pttFormat == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.pttFormat) as u64) }
        + if self.isNeedInnerIp == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.isNeedInnerIp) as u64) }
        + if self.netType == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.netType) as u64) }
        + if self.voiceType == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.voiceType) as u64) }
        + if self.fileType == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.fileType) as u64) }
        + if self.pttTime == 0i32 { 0 } else { 3 + sizeof_varint(*(&self.pttTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.id))?; }
        if self.type_pb != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.type_pb))?; }
        if self.dstPhonenum != "" { w.write_with_tag(26, |w| w.write_string(&**&self.dstPhonenum))?; }
        if self.phoneConvertType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.phoneConvertType))?; }
        if self.sig != Cow::Borrowed(b"") { w.write_with_tag(162, |w| w.write_bytes(&**&self.sig))?; }
        if self.routeId != 0i64 { w.write_with_tag(800, |w| w.write_int64(*&self.routeId))?; }
        if let Some(ref s) = self.delMessageReq { w.write_with_tag(720802, |w| w.write_message(s))?; }
        if self.downloadUrlType != 0i32 { w.write_with_tag(721600, |w| w.write_int32(*&self.downloadUrlType))?; }
        if self.pttFormat != 0i32 { w.write_with_tag(722400, |w| w.write_int32(*&self.pttFormat))?; }
        if self.isNeedInnerIp != 0i32 { w.write_with_tag(723200, |w| w.write_int32(*&self.isNeedInnerIp))?; }
        if self.netType != 0i32 { w.write_with_tag(724000, |w| w.write_int32(*&self.netType))?; }
        if self.voiceType != 0i32 { w.write_with_tag(724800, |w| w.write_int32(*&self.voiceType))?; }
        if self.fileType != 0i32 { w.write_with_tag(725600, |w| w.write_int32(*&self.fileType))?; }
        if self.pttTime != 0i32 { w.write_with_tag(726400, |w| w.write_int32(*&self.pttTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExtensionRsp { }

impl<'a> MessageRead<'a> for ExtensionRsp {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ExtensionRsp { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileInfo<'a> {
    pub uin: i64,
    pub dangerEvel: i32,
    pub fileSize: i64,
    pub lifeTime: i32,
    pub uploadTime: i32,
    pub uuid: Cow<'a, [u8]>,
    pub fileName: Cow<'a, str>,
    pub absFileType: i32,
    pub bytes_10mMd5: Cow<'a, [u8]>,
    pub sha: Cow<'a, [u8]>,
    pub clientType: i32,
    pub ownerUin: i64,
    pub peerUin: i64,
    pub expireTime: i32,
}

impl<'a> MessageRead<'a> for FileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(16) => msg.dangerEvel = r.read_int32(bytes)?,
                Ok(24) => msg.fileSize = r.read_int64(bytes)?,
                Ok(32) => msg.lifeTime = r.read_int32(bytes)?,
                Ok(40) => msg.uploadTime = r.read_int32(bytes)?,
                Ok(50) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(720) => msg.absFileType = r.read_int32(bytes)?,
                Ok(802) => msg.bytes_10mMd5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(810) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(880) => msg.clientType = r.read_int32(bytes)?,
                Ok(960) => msg.ownerUin = r.read_int64(bytes)?,
                Ok(968) => msg.peerUin = r.read_int64(bytes)?,
                Ok(1040) => msg.expireTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.dangerEvel == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.dangerEvel) as u64) }
        + if self.fileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.lifeTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.lifeTime) as u64) }
        + if self.uploadTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.uploadTime) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.uuid).len()) }
        + if self.fileName == "" { 0 } else { 1 + sizeof_len((&self.fileName).len()) }
        + if self.absFileType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.absFileType) as u64) }
        + if self.bytes_10mMd5 == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.bytes_10mMd5).len()) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.sha).len()) }
        + if self.clientType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.clientType) as u64) }
        + if self.ownerUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.ownerUin) as u64) }
        + if self.peerUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.peerUin) as u64) }
        + if self.expireTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.expireTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.dangerEvel != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.dangerEvel))?; }
        if self.fileSize != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.fileSize))?; }
        if self.lifeTime != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.lifeTime))?; }
        if self.uploadTime != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.uploadTime))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.uuid))?; }
        if self.fileName != "" { w.write_with_tag(58, |w| w.write_string(&**&self.fileName))?; }
        if self.absFileType != 0i32 { w.write_with_tag(720, |w| w.write_int32(*&self.absFileType))?; }
        if self.bytes_10mMd5 != Cow::Borrowed(b"") { w.write_with_tag(802, |w| w.write_bytes(&**&self.bytes_10mMd5))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(810, |w| w.write_bytes(&**&self.sha))?; }
        if self.clientType != 0i32 { w.write_with_tag(880, |w| w.write_int32(*&self.clientType))?; }
        if self.ownerUin != 0i64 { w.write_with_tag(960, |w| w.write_int64(*&self.ownerUin))?; }
        if self.peerUin != 0i64 { w.write_with_tag(968, |w| w.write_int64(*&self.peerUin))?; }
        if self.expireTime != 0i32 { w.write_with_tag(1040, |w| w.write_int32(*&self.expireTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileQueryReq<'a> {
    pub uin: i64,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for FileQueryReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.uin = r.read_int64(bytes)?,
                Ok(162) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileQueryReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.uin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(162, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileQueryRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub fileInfo: Option<FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for FileQueryRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.fileInfo = Some(r.read_message::<FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileQueryRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + self.fileInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if let Some(ref s) = self.fileInfo { w.write_with_tag(242, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RecallFileReq<'a> {
    pub uin: i64,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for RecallFileReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(18) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RecallFileReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RecallFileRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RecallFileRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RecallFileRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RecvListQueryReq {
    pub uin: i64,
    pub beginIndex: i32,
    pub reqCount: i32,
}

impl<'a> MessageRead<'a> for RecvListQueryReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(16) => msg.beginIndex = r.read_int32(bytes)?,
                Ok(24) => msg.reqCount = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RecvListQueryReq {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.beginIndex == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.beginIndex) as u64) }
        + if self.reqCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqCount) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.beginIndex != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.beginIndex))?; }
        if self.reqCount != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.reqCount))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RecvListQueryRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub fileTotCount: i32,
    pub beginIndex: i32,
    pub rspFileCount: i32,
    pub isEnd: i32,
    pub fileList: Vec<FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for RecvListQueryRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.fileTotCount = r.read_int32(bytes)?,
                Ok(32) => msg.beginIndex = r.read_int32(bytes)?,
                Ok(40) => msg.rspFileCount = r.read_int32(bytes)?,
                Ok(48) => msg.isEnd = r.read_int32(bytes)?,
                Ok(58) => msg.fileList.push(r.read_message::<FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RecvListQueryRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.fileTotCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileTotCount) as u64) }
        + if self.beginIndex == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.beginIndex) as u64) }
        + if self.rspFileCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.rspFileCount) as u64) }
        + if self.isEnd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.isEnd) as u64) }
        + self.fileList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.fileTotCount != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.fileTotCount))?; }
        if self.beginIndex != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.beginIndex))?; }
        if self.rspFileCount != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.rspFileCount))?; }
        if self.isEnd != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.isEnd))?; }
        for s in &self.fileList { w.write_with_tag(58, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RenewFileReq<'a> {
    pub uin: i64,
    pub uuid: Cow<'a, [u8]>,
    pub addTtl: i32,
}

impl<'a> MessageRead<'a> for RenewFileReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(18) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.addTtl = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RenewFileReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.uuid).len()) }
        + if self.addTtl == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.addTtl) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.uuid))?; }
        if self.addTtl != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.addTtl))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RenewFileRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RenewFileRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RenewFileRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C346ReqBody<'a> {
    pub cmd: i32,
    pub seq: i32,
    pub recvListQueryReq: Option<RecvListQueryReq>,
    pub sendListQueryReq: Option<SendListQueryReq>,
    pub renewFileReq: Option<RenewFileReq<'a>>,
    pub recallFileReq: Option<RecallFileReq<'a>>,
    pub applyUploadReq: Option<ApplyUploadReq<'a>>,
    pub applyUploadHitReq: Option<ApplyUploadHitReq<'a>>,
    pub applyForwardFileReq: Option<ApplyForwardFileReq<'a>>,
    pub uploadSuccReq: Option<UploadSuccReq<'a>>,
    pub deleteFileReq: Option<DeleteFileReq<'a>>,
    pub downloadSuccReq: Option<DownloadSuccReq<'a>>,
    pub applyDownloadAbsReq: Option<ApplyDownloadAbsReq<'a>>,
    pub applyDownloadReq: Option<ApplyDownloadReq<'a>>,
    pub applyListDownloadReq: Option<ApplyListDownloadReq>,
    pub fileQueryReq: Option<FileQueryReq<'a>>,
    pub applyCopyFromReq: Option<ApplyCopyFromReq<'a>>,
    pub applyUploadReqV2: Option<ApplyUploadReqV2<'a>>,
    pub applyUploadReqV3: Option<ApplyUploadReqV3<'a>>,
    pub applyUploadHitReqV2: Option<ApplyUploadHitReqV2<'a>>,
    pub applyUploadHitReqV3: Option<ApplyUploadHitReqV3<'a>>,
    pub businessId: i32,
    pub clientType: i32,
    pub applyCopyToReq: Option<ApplyCopyToReq<'a>>,
    pub applyGetTrafficReq: Option<ApplyGetTrafficReq>,
    pub extensionReq: Option<ExtensionReq<'a>>,
}

impl<'a> MessageRead<'a> for C346ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cmd = r.read_int32(bytes)?,
                Ok(16) => msg.seq = r.read_int32(bytes)?,
                Ok(26) => msg.recvListQueryReq = Some(r.read_message::<RecvListQueryReq>(bytes)?),
                Ok(34) => msg.sendListQueryReq = Some(r.read_message::<SendListQueryReq>(bytes)?),
                Ok(42) => msg.renewFileReq = Some(r.read_message::<RenewFileReq>(bytes)?),
                Ok(50) => msg.recallFileReq = Some(r.read_message::<RecallFileReq>(bytes)?),
                Ok(58) => msg.applyUploadReq = Some(r.read_message::<ApplyUploadReq>(bytes)?),
                Ok(66) => msg.applyUploadHitReq = Some(r.read_message::<ApplyUploadHitReq>(bytes)?),
                Ok(74) => msg.applyForwardFileReq = Some(r.read_message::<ApplyForwardFileReq>(bytes)?),
                Ok(82) => msg.uploadSuccReq = Some(r.read_message::<UploadSuccReq>(bytes)?),
                Ok(90) => msg.deleteFileReq = Some(r.read_message::<DeleteFileReq>(bytes)?),
                Ok(98) => msg.downloadSuccReq = Some(r.read_message::<DownloadSuccReq>(bytes)?),
                Ok(106) => msg.applyDownloadAbsReq = Some(r.read_message::<ApplyDownloadAbsReq>(bytes)?),
                Ok(114) => msg.applyDownloadReq = Some(r.read_message::<ApplyDownloadReq>(bytes)?),
                Ok(122) => msg.applyListDownloadReq = Some(r.read_message::<ApplyListDownloadReq>(bytes)?),
                Ok(130) => msg.fileQueryReq = Some(r.read_message::<FileQueryReq>(bytes)?),
                Ok(138) => msg.applyCopyFromReq = Some(r.read_message::<ApplyCopyFromReq>(bytes)?),
                Ok(146) => msg.applyUploadReqV2 = Some(r.read_message::<ApplyUploadReqV2>(bytes)?),
                Ok(154) => msg.applyUploadReqV3 = Some(r.read_message::<ApplyUploadReqV3>(bytes)?),
                Ok(162) => msg.applyUploadHitReqV2 = Some(r.read_message::<ApplyUploadHitReqV2>(bytes)?),
                Ok(170) => msg.applyUploadHitReqV3 = Some(r.read_message::<ApplyUploadHitReqV3>(bytes)?),
                Ok(808) => msg.businessId = r.read_int32(bytes)?,
                Ok(816) => msg.clientType = r.read_int32(bytes)?,
                Ok(720002) => msg.applyCopyToReq = Some(r.read_message::<ApplyCopyToReq>(bytes)?),
                Ok(720018) => msg.applyGetTrafficReq = Some(r.read_message::<ApplyGetTrafficReq>(bytes)?),
                Ok(799994) => msg.extensionReq = Some(r.read_message::<ExtensionReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C346ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.cmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.cmd) as u64) }
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + self.recvListQueryReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.sendListQueryReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.renewFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.recallFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyUploadReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyUploadHitReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyForwardFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.uploadSuccReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deleteFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.downloadSuccReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyDownloadAbsReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyDownloadReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyListDownloadReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileQueryReq.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyCopyFromReq.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadReqV2.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadReqV3.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadHitReqV2.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadHitReqV3.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.businessId == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.businessId) as u64) }
        + if self.clientType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.clientType) as u64) }
        + self.applyCopyToReq.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
        + self.applyGetTrafficReq.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
        + self.extensionReq.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cmd != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.cmd))?; }
        if self.seq != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.seq))?; }
        if let Some(ref s) = self.recvListQueryReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.sendListQueryReq { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.renewFileReq { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.recallFileReq { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadReq { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadHitReq { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyForwardFileReq { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.uploadSuccReq { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deleteFileReq { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.downloadSuccReq { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyDownloadAbsReq { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyDownloadReq { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyListDownloadReq { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileQueryReq { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyCopyFromReq { w.write_with_tag(138, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadReqV2 { w.write_with_tag(146, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadReqV3 { w.write_with_tag(154, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadHitReqV2 { w.write_with_tag(162, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadHitReqV3 { w.write_with_tag(170, |w| w.write_message(s))?; }
        if self.businessId != 0i32 { w.write_with_tag(808, |w| w.write_int32(*&self.businessId))?; }
        if self.clientType != 0i32 { w.write_with_tag(816, |w| w.write_int32(*&self.clientType))?; }
        if let Some(ref s) = self.applyCopyToReq { w.write_with_tag(720002, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyGetTrafficReq { w.write_with_tag(720018, |w| w.write_message(s))?; }
        if let Some(ref s) = self.extensionReq { w.write_with_tag(799994, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C346RspBody<'a> {
    pub cmd: i32,
    pub seq: i32,
    pub recvListQueryRsp: Option<RecvListQueryRsp<'a>>,
    pub sendListQueryRsp: Option<SendListQueryRsp<'a>>,
    pub renewFileRsp: Option<RenewFileRsp<'a>>,
    pub recallFileRsp: Option<RecallFileRsp<'a>>,
    pub applyUploadRsp: Option<ApplyUploadRsp<'a>>,
    pub applyUploadHitRsp: Option<ApplyUploadHitRsp<'a>>,
    pub applyForwardFileRsp: Option<ApplyForwardFileRsp<'a>>,
    pub uploadSuccRsp: Option<UploadSuccRsp<'a>>,
    pub deleteFileRsp: Option<DeleteFileRsp<'a>>,
    pub downloadSuccRsp: Option<DownloadSuccRsp<'a>>,
    pub applyDownloadAbsRsp: Option<ApplyDownloadAbsRsp<'a>>,
    pub applyDownloadRsp: Option<ApplyDownloadRsp<'a>>,
    pub applyListDownloadRsp: Option<ApplyListDownloadRsp<'a>>,
    pub fileQueryRsp: Option<FileQueryRsp<'a>>,
    pub applyCopyFromRsp: Option<ApplyCopyFromRsp<'a>>,
    pub applyUploadRspV2: Option<ApplyUploadRspV2<'a>>,
    pub applyUploadRspV3: Option<ApplyUploadRspV3<'a>>,
    pub applyUploadHitRspV2: Option<ApplyUploadHitRspV2<'a>>,
    pub applyUploadHitRspV3: Option<ApplyUploadHitRspV3<'a>>,
    pub businessId: i32,
    pub clientType: i32,
    pub applyCopyToRsp: Option<ApplyCopyToRsp<'a>>,
    pub applyCleanTrafficRsp: Option<ApplyCleanTrafficRsp<'a>>,
    pub applyGetTrafficRsp: Option<ApplyGetTrafficRsp<'a>>,
    pub extensionRsp: Option<ExtensionRsp>,
}

impl<'a> MessageRead<'a> for C346RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cmd = r.read_int32(bytes)?,
                Ok(16) => msg.seq = r.read_int32(bytes)?,
                Ok(26) => msg.recvListQueryRsp = Some(r.read_message::<RecvListQueryRsp>(bytes)?),
                Ok(34) => msg.sendListQueryRsp = Some(r.read_message::<SendListQueryRsp>(bytes)?),
                Ok(42) => msg.renewFileRsp = Some(r.read_message::<RenewFileRsp>(bytes)?),
                Ok(50) => msg.recallFileRsp = Some(r.read_message::<RecallFileRsp>(bytes)?),
                Ok(58) => msg.applyUploadRsp = Some(r.read_message::<ApplyUploadRsp>(bytes)?),
                Ok(66) => msg.applyUploadHitRsp = Some(r.read_message::<ApplyUploadHitRsp>(bytes)?),
                Ok(74) => msg.applyForwardFileRsp = Some(r.read_message::<ApplyForwardFileRsp>(bytes)?),
                Ok(82) => msg.uploadSuccRsp = Some(r.read_message::<UploadSuccRsp>(bytes)?),
                Ok(90) => msg.deleteFileRsp = Some(r.read_message::<DeleteFileRsp>(bytes)?),
                Ok(98) => msg.downloadSuccRsp = Some(r.read_message::<DownloadSuccRsp>(bytes)?),
                Ok(106) => msg.applyDownloadAbsRsp = Some(r.read_message::<ApplyDownloadAbsRsp>(bytes)?),
                Ok(114) => msg.applyDownloadRsp = Some(r.read_message::<ApplyDownloadRsp>(bytes)?),
                Ok(122) => msg.applyListDownloadRsp = Some(r.read_message::<ApplyListDownloadRsp>(bytes)?),
                Ok(130) => msg.fileQueryRsp = Some(r.read_message::<FileQueryRsp>(bytes)?),
                Ok(138) => msg.applyCopyFromRsp = Some(r.read_message::<ApplyCopyFromRsp>(bytes)?),
                Ok(146) => msg.applyUploadRspV2 = Some(r.read_message::<ApplyUploadRspV2>(bytes)?),
                Ok(154) => msg.applyUploadRspV3 = Some(r.read_message::<ApplyUploadRspV3>(bytes)?),
                Ok(162) => msg.applyUploadHitRspV2 = Some(r.read_message::<ApplyUploadHitRspV2>(bytes)?),
                Ok(170) => msg.applyUploadHitRspV3 = Some(r.read_message::<ApplyUploadHitRspV3>(bytes)?),
                Ok(808) => msg.businessId = r.read_int32(bytes)?,
                Ok(816) => msg.clientType = r.read_int32(bytes)?,
                Ok(720002) => msg.applyCopyToRsp = Some(r.read_message::<ApplyCopyToRsp>(bytes)?),
                Ok(720010) => msg.applyCleanTrafficRsp = Some(r.read_message::<ApplyCleanTrafficRsp>(bytes)?),
                Ok(720018) => msg.applyGetTrafficRsp = Some(r.read_message::<ApplyGetTrafficRsp>(bytes)?),
                Ok(799994) => msg.extensionRsp = Some(r.read_message::<ExtensionRsp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C346RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.cmd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.cmd) as u64) }
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + self.recvListQueryRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.sendListQueryRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.renewFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.recallFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyUploadRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyUploadHitRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyForwardFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.uploadSuccRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deleteFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.downloadSuccRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyDownloadAbsRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyDownloadRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applyListDownloadRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileQueryRsp.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyCopyFromRsp.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadRspV2.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadRspV3.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadHitRspV2.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.applyUploadHitRspV3.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.businessId == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.businessId) as u64) }
        + if self.clientType == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.clientType) as u64) }
        + self.applyCopyToRsp.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
        + self.applyCleanTrafficRsp.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
        + self.applyGetTrafficRsp.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
        + self.extensionRsp.as_ref().map_or(0, |m| 3 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cmd != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.cmd))?; }
        if self.seq != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.seq))?; }
        if let Some(ref s) = self.recvListQueryRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.sendListQueryRsp { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.renewFileRsp { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.recallFileRsp { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadRsp { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadHitRsp { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyForwardFileRsp { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.uploadSuccRsp { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deleteFileRsp { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.downloadSuccRsp { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyDownloadAbsRsp { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyDownloadRsp { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyListDownloadRsp { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileQueryRsp { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyCopyFromRsp { w.write_with_tag(138, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadRspV2 { w.write_with_tag(146, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadRspV3 { w.write_with_tag(154, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadHitRspV2 { w.write_with_tag(162, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyUploadHitRspV3 { w.write_with_tag(170, |w| w.write_message(s))?; }
        if self.businessId != 0i32 { w.write_with_tag(808, |w| w.write_int32(*&self.businessId))?; }
        if self.clientType != 0i32 { w.write_with_tag(816, |w| w.write_int32(*&self.clientType))?; }
        if let Some(ref s) = self.applyCopyToRsp { w.write_with_tag(720002, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyCleanTrafficRsp { w.write_with_tag(720010, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applyGetTrafficRsp { w.write_with_tag(720018, |w| w.write_message(s))?; }
        if let Some(ref s) = self.extensionRsp { w.write_with_tag(799994, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SendListQueryReq {
    pub uin: i64,
    pub beginIndex: i32,
    pub reqCount: i32,
}

impl<'a> MessageRead<'a> for SendListQueryReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(16) => msg.beginIndex = r.read_int32(bytes)?,
                Ok(24) => msg.reqCount = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SendListQueryReq {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.beginIndex == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.beginIndex) as u64) }
        + if self.reqCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqCount) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.beginIndex != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.beginIndex))?; }
        if self.reqCount != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.reqCount))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SendListQueryRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub fileTotCount: i32,
    pub beginIndex: i32,
    pub rspFileCount: i32,
    pub isEnd: i32,
    pub totLimit: i64,
    pub usedLimit: i64,
    pub fileList: Vec<FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for SendListQueryRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.fileTotCount = r.read_int32(bytes)?,
                Ok(32) => msg.beginIndex = r.read_int32(bytes)?,
                Ok(40) => msg.rspFileCount = r.read_int32(bytes)?,
                Ok(48) => msg.isEnd = r.read_int32(bytes)?,
                Ok(56) => msg.totLimit = r.read_int64(bytes)?,
                Ok(64) => msg.usedLimit = r.read_int64(bytes)?,
                Ok(74) => msg.fileList.push(r.read_message::<FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SendListQueryRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.fileTotCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileTotCount) as u64) }
        + if self.beginIndex == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.beginIndex) as u64) }
        + if self.rspFileCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.rspFileCount) as u64) }
        + if self.isEnd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.isEnd) as u64) }
        + if self.totLimit == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.totLimit) as u64) }
        + if self.usedLimit == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.usedLimit) as u64) }
        + self.fileList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.fileTotCount != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.fileTotCount))?; }
        if self.beginIndex != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.beginIndex))?; }
        if self.rspFileCount != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.rspFileCount))?; }
        if self.isEnd != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.isEnd))?; }
        if self.totLimit != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.totLimit))?; }
        if self.usedLimit != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.usedLimit))?; }
        for s in &self.fileList { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UploadSuccReq<'a> {
    pub senderUin: i64,
    pub recverUin: i64,
    pub uuid: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for UploadSuccReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.senderUin = r.read_int64(bytes)?,
                Ok(160) => msg.recverUin = r.read_int64(bytes)?,
                Ok(242) => msg.uuid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UploadSuccReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.senderUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.recverUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.recverUin) as u64) }
        + if self.uuid == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.senderUin != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.senderUin))?; }
        if self.recverUin != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.recverUin))?; }
        if self.uuid != Cow::Borrowed(b"") { w.write_with_tag(242, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UploadSuccRsp<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub fileInfo: Option<FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for UploadSuccRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.retCode = r.read_int32(bytes)?,
                Ok(162) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(242) => msg.fileInfo = Some(r.read_message::<FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UploadSuccRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 2 + sizeof_len((&self.retMsg).len()) }
        + self.fileInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(162, |w| w.write_string(&**&self.retMsg))?; }
        if let Some(ref s) = self.fileInfo { w.write_with_tag(242, |w| w.write_message(s))?; }
        Ok(())
    }
}
