// Automatically generated rust module for 'oidb0x6d6.proto' file

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
pub struct DeleteFileReqBody<'a> {
    pub groupCode: i64,
    pub appId: i32,
    pub busId: i32,
    pub parentFolderId: Cow<'a, str>,
    pub fileId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DeleteFileReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.appId = r.read_int32(bytes)?,
                Ok(24) => msg.busId = r.read_int32(bytes)?,
                Ok(34) => msg.parentFolderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteFileReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.appId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.parentFolderId == "" { 0 } else { 1 + sizeof_len((&self.parentFolderId).len()) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.appId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.appId))?; }
        if self.busId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.busId))?; }
        if self.parentFolderId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.parentFolderId))?; }
        if self.fileId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.fileId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteFileRspBody<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub clientWording: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DeleteFileRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.clientWording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteFileRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.clientWording == "" { 0 } else { 1 + sizeof_len((&self.clientWording).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.clientWording != "" { w.write_with_tag(26, |w| w.write_string(&**&self.clientWording))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DownloadFileReqBody<'a> {
    pub groupCode: i64,
    pub appId: i32,
    pub busId: i32,
    pub fileId: Cow<'a, str>,
    pub boolThumbnailReq: bool,
    pub urlType: i32,
    pub boolPreviewReq: bool,
}

impl<'a> MessageRead<'a> for DownloadFileReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.appId = r.read_int32(bytes)?,
                Ok(24) => msg.busId = r.read_int32(bytes)?,
                Ok(34) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.boolThumbnailReq = r.read_bool(bytes)?,
                Ok(48) => msg.urlType = r.read_int32(bytes)?,
                Ok(56) => msg.boolPreviewReq = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DownloadFileReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.appId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + if self.boolThumbnailReq == false { 0 } else { 1 + sizeof_varint(*(&self.boolThumbnailReq) as u64) }
        + if self.urlType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.urlType) as u64) }
        + if self.boolPreviewReq == false { 0 } else { 1 + sizeof_varint(*(&self.boolPreviewReq) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.appId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.appId))?; }
        if self.busId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.busId))?; }
        if self.fileId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.fileId))?; }
        if self.boolThumbnailReq != false { w.write_with_tag(40, |w| w.write_bool(*&self.boolThumbnailReq))?; }
        if self.urlType != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.urlType))?; }
        if self.boolPreviewReq != false { w.write_with_tag(56, |w| w.write_bool(*&self.boolPreviewReq))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DownloadFileRspBody<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub clientWording: Cow<'a, str>,
    pub downloadIp: Cow<'a, str>,
    pub downloadDns: Cow<'a, [u8]>,
    pub downloadUrl: Cow<'a, [u8]>,
    pub sha: Cow<'a, [u8]>,
    pub sha3: Cow<'a, [u8]>,
    pub md5: Cow<'a, [u8]>,
    pub cookieVal: Cow<'a, [u8]>,
    pub saveFileName: Cow<'a, str>,
    pub previewPort: i32,
}

impl<'a> MessageRead<'a> for DownloadFileRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.clientWording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.downloadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.downloadDns = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.downloadUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.sha3 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.md5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.cookieVal = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.saveFileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(96) => msg.previewPort = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DownloadFileRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.clientWording == "" { 0 } else { 1 + sizeof_len((&self.clientWording).len()) }
        + if self.downloadIp == "" { 0 } else { 1 + sizeof_len((&self.downloadIp).len()) }
        + if self.downloadDns == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.downloadDns).len()) }
        + if self.downloadUrl == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.downloadUrl).len()) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sha).len()) }
        + if self.sha3 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sha3).len()) }
        + if self.md5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.md5).len()) }
        + if self.cookieVal == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.cookieVal).len()) }
        + if self.saveFileName == "" { 0 } else { 1 + sizeof_len((&self.saveFileName).len()) }
        + if self.previewPort == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.previewPort) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.clientWording != "" { w.write_with_tag(26, |w| w.write_string(&**&self.clientWording))?; }
        if self.downloadIp != "" { w.write_with_tag(34, |w| w.write_string(&**&self.downloadIp))?; }
        if self.downloadDns != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.downloadDns))?; }
        if self.downloadUrl != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.downloadUrl))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.sha))?; }
        if self.sha3 != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.sha3))?; }
        if self.md5 != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.md5))?; }
        if self.cookieVal != Cow::Borrowed(b"") { w.write_with_tag(82, |w| w.write_bytes(&**&self.cookieVal))?; }
        if self.saveFileName != "" { w.write_with_tag(90, |w| w.write_string(&**&self.saveFileName))?; }
        if self.previewPort != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.previewPort))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MoveFileReqBody<'a> {
    pub groupCode: i64,
    pub appId: i32,
    pub busId: i32,
    pub fileId: Cow<'a, str>,
    pub parentFolderId: Cow<'a, str>,
    pub destFolderId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for MoveFileReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.appId = r.read_int32(bytes)?,
                Ok(24) => msg.busId = r.read_int32(bytes)?,
                Ok(34) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.parentFolderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.destFolderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MoveFileReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.appId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + if self.parentFolderId == "" { 0 } else { 1 + sizeof_len((&self.parentFolderId).len()) }
        + if self.destFolderId == "" { 0 } else { 1 + sizeof_len((&self.destFolderId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.appId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.appId))?; }
        if self.busId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.busId))?; }
        if self.fileId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.fileId))?; }
        if self.parentFolderId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.parentFolderId))?; }
        if self.destFolderId != "" { w.write_with_tag(50, |w| w.write_string(&**&self.destFolderId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MoveFileRspBody<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub clientWording: Cow<'a, str>,
    pub parentFolderId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for MoveFileRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.clientWording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.parentFolderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MoveFileRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.clientWording == "" { 0 } else { 1 + sizeof_len((&self.clientWording).len()) }
        + if self.parentFolderId == "" { 0 } else { 1 + sizeof_len((&self.parentFolderId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.clientWording != "" { w.write_with_tag(26, |w| w.write_string(&**&self.clientWording))?; }
        if self.parentFolderId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.parentFolderId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RenameFileReqBody<'a> {
    pub groupCode: i64,
    pub appId: i32,
    pub busId: i32,
    pub fileId: Cow<'a, str>,
    pub parentFolderId: Cow<'a, str>,
    pub newFileName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RenameFileReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.appId = r.read_int32(bytes)?,
                Ok(24) => msg.busId = r.read_int32(bytes)?,
                Ok(34) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.parentFolderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.newFileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RenameFileReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.appId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + if self.parentFolderId == "" { 0 } else { 1 + sizeof_len((&self.parentFolderId).len()) }
        + if self.newFileName == "" { 0 } else { 1 + sizeof_len((&self.newFileName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.appId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.appId))?; }
        if self.busId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.busId))?; }
        if self.fileId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.fileId))?; }
        if self.parentFolderId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.parentFolderId))?; }
        if self.newFileName != "" { w.write_with_tag(50, |w| w.write_string(&**&self.newFileName))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RenameFileRspBody<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub clientWording: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RenameFileRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.clientWording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RenameFileRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.clientWording == "" { 0 } else { 1 + sizeof_len((&self.clientWording).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.clientWording != "" { w.write_with_tag(26, |w| w.write_string(&**&self.clientWording))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D6D6ReqBody<'a> {
    pub uploadFileReq: Option<UploadFileReqBody<'a>>,
    pub resendFileReq: Option<ResendReqBody<'a>>,
    pub downloadFileReq: Option<DownloadFileReqBody<'a>>,
    pub deleteFileReq: Option<DeleteFileReqBody<'a>>,
    pub renameFileReq: Option<RenameFileReqBody<'a>>,
    pub moveFileReq: Option<MoveFileReqBody<'a>>,
}

impl<'a> MessageRead<'a> for D6D6ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uploadFileReq = Some(r.read_message::<UploadFileReqBody>(bytes)?),
                Ok(18) => msg.resendFileReq = Some(r.read_message::<ResendReqBody>(bytes)?),
                Ok(26) => msg.downloadFileReq = Some(r.read_message::<DownloadFileReqBody>(bytes)?),
                Ok(34) => msg.deleteFileReq = Some(r.read_message::<DeleteFileReqBody>(bytes)?),
                Ok(42) => msg.renameFileReq = Some(r.read_message::<RenameFileReqBody>(bytes)?),
                Ok(50) => msg.moveFileReq = Some(r.read_message::<MoveFileReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D6ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uploadFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.resendFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.downloadFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deleteFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.renameFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.moveFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uploadFileReq { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.resendFileReq { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.downloadFileReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deleteFileReq { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.renameFileReq { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.moveFileReq { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResendReqBody<'a> {
    pub groupCode: i64,
    pub appId: i32,
    pub busId: i32,
    pub fileId: Cow<'a, str>,
    pub sha: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ResendReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.appId = r.read_int32(bytes)?,
                Ok(24) => msg.busId = r.read_int32(bytes)?,
                Ok(34) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResendReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.appId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sha).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.appId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.appId))?; }
        if self.busId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.busId))?; }
        if self.fileId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.fileId))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.sha))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResendRspBody<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub clientWording: Cow<'a, str>,
    pub uploadIp: Cow<'a, str>,
    pub fileKey: Cow<'a, [u8]>,
    pub checkKey: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ResendRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.clientWording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.fileKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.checkKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResendRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.clientWording == "" { 0 } else { 1 + sizeof_len((&self.clientWording).len()) }
        + if self.uploadIp == "" { 0 } else { 1 + sizeof_len((&self.uploadIp).len()) }
        + if self.fileKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileKey).len()) }
        + if self.checkKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.checkKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.clientWording != "" { w.write_with_tag(26, |w| w.write_string(&**&self.clientWording))?; }
        if self.uploadIp != "" { w.write_with_tag(34, |w| w.write_string(&**&self.uploadIp))?; }
        if self.fileKey != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.fileKey))?; }
        if self.checkKey != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.checkKey))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D6D6RspBody<'a> {
    pub uploadFileRsp: Option<UploadFileRspBody<'a>>,
    pub resendFileRsp: Option<ResendRspBody<'a>>,
    pub downloadFileRsp: Option<DownloadFileRspBody<'a>>,
    pub deleteFileRsp: Option<DeleteFileRspBody<'a>>,
    pub renameFileRsp: Option<RenameFileRspBody<'a>>,
    pub moveFileRsp: Option<MoveFileRspBody<'a>>,
}

impl<'a> MessageRead<'a> for D6D6RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uploadFileRsp = Some(r.read_message::<UploadFileRspBody>(bytes)?),
                Ok(18) => msg.resendFileRsp = Some(r.read_message::<ResendRspBody>(bytes)?),
                Ok(26) => msg.downloadFileRsp = Some(r.read_message::<DownloadFileRspBody>(bytes)?),
                Ok(34) => msg.deleteFileRsp = Some(r.read_message::<DeleteFileRspBody>(bytes)?),
                Ok(42) => msg.renameFileRsp = Some(r.read_message::<RenameFileRspBody>(bytes)?),
                Ok(50) => msg.moveFileRsp = Some(r.read_message::<MoveFileRspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D6RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uploadFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.resendFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.downloadFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deleteFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.renameFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.moveFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uploadFileRsp { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.resendFileRsp { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.downloadFileRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deleteFileRsp { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.renameFileRsp { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.moveFileRsp { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UploadFileReqBody<'a> {
    pub groupCode: i64,
    pub appId: i32,
    pub busId: i32,
    pub entrance: i32,
    pub parentFolderId: Cow<'a, str>,
    pub fileName: Cow<'a, str>,
    pub localPath: Cow<'a, str>,
    pub int64FileSize: i64,
    pub sha: Cow<'a, [u8]>,
    pub sha3: Cow<'a, [u8]>,
    pub md5: Cow<'a, [u8]>,
    pub supportMultiUpload: bool,
}

impl<'a> MessageRead<'a> for UploadFileReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(16) => msg.appId = r.read_int32(bytes)?,
                Ok(24) => msg.busId = r.read_int32(bytes)?,
                Ok(32) => msg.entrance = r.read_int32(bytes)?,
                Ok(42) => msg.parentFolderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.localPath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.int64FileSize = r.read_int64(bytes)?,
                Ok(74) => msg.sha = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.sha3 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.md5 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(120) => msg.supportMultiUpload = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UploadFileReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.appId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.entrance == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.entrance) as u64) }
        + if self.parentFolderId == "" { 0 } else { 1 + sizeof_len((&self.parentFolderId).len()) }
        + if self.fileName == "" { 0 } else { 1 + sizeof_len((&self.fileName).len()) }
        + if self.localPath == "" { 0 } else { 1 + sizeof_len((&self.localPath).len()) }
        + if self.int64FileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.int64FileSize) as u64) }
        + if self.sha == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sha).len()) }
        + if self.sha3 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sha3).len()) }
        + if self.md5 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.md5).len()) }
        + if self.supportMultiUpload == false { 0 } else { 1 + sizeof_varint(*(&self.supportMultiUpload) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if self.appId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.appId))?; }
        if self.busId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.busId))?; }
        if self.entrance != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.entrance))?; }
        if self.parentFolderId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.parentFolderId))?; }
        if self.fileName != "" { w.write_with_tag(50, |w| w.write_string(&**&self.fileName))?; }
        if self.localPath != "" { w.write_with_tag(58, |w| w.write_string(&**&self.localPath))?; }
        if self.int64FileSize != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.int64FileSize))?; }
        if self.sha != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.sha))?; }
        if self.sha3 != Cow::Borrowed(b"") { w.write_with_tag(82, |w| w.write_bytes(&**&self.sha3))?; }
        if self.md5 != Cow::Borrowed(b"") { w.write_with_tag(90, |w| w.write_bytes(&**&self.md5))?; }
        if self.supportMultiUpload != false { w.write_with_tag(120, |w| w.write_bool(*&self.supportMultiUpload))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UploadFileRspBody<'a> {
    pub retCode: i32,
    pub retMsg: Cow<'a, str>,
    pub clientWording: Cow<'a, str>,
    pub uploadIp: Cow<'a, str>,
    pub serverDns: Cow<'a, str>,
    pub busId: i32,
    pub fileId: Cow<'a, str>,
    pub fileKey: Cow<'a, [u8]>,
    pub checkKey: Cow<'a, [u8]>,
    pub boolFileExist: bool,
    pub uploadIpLanV4: Vec<Cow<'a, str>>,
    pub uploadIpLanV6: Vec<Cow<'a, str>>,
    pub uploadPort: i32,
}

impl<'a> MessageRead<'a> for UploadFileRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.retMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.clientWording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.uploadIp = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.serverDns = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.busId = r.read_int32(bytes)?,
                Ok(58) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.fileKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.checkKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.boolFileExist = r.read_bool(bytes)?,
                Ok(98) => msg.uploadIpLanV4.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.uploadIpLanV6.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.uploadPort = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UploadFileRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.retMsg == "" { 0 } else { 1 + sizeof_len((&self.retMsg).len()) }
        + if self.clientWording == "" { 0 } else { 1 + sizeof_len((&self.clientWording).len()) }
        + if self.uploadIp == "" { 0 } else { 1 + sizeof_len((&self.uploadIp).len()) }
        + if self.serverDns == "" { 0 } else { 1 + sizeof_len((&self.serverDns).len()) }
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + if self.fileKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileKey).len()) }
        + if self.checkKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.checkKey).len()) }
        + if self.boolFileExist == false { 0 } else { 1 + sizeof_varint(*(&self.boolFileExist) as u64) }
        + self.uploadIpLanV4.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.uploadIpLanV6.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.uploadPort == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.uploadPort) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.retMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.retMsg))?; }
        if self.clientWording != "" { w.write_with_tag(26, |w| w.write_string(&**&self.clientWording))?; }
        if self.uploadIp != "" { w.write_with_tag(34, |w| w.write_string(&**&self.uploadIp))?; }
        if self.serverDns != "" { w.write_with_tag(42, |w| w.write_string(&**&self.serverDns))?; }
        if self.busId != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.busId))?; }
        if self.fileId != "" { w.write_with_tag(58, |w| w.write_string(&**&self.fileId))?; }
        if self.fileKey != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.fileKey))?; }
        if self.checkKey != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.checkKey))?; }
        if self.boolFileExist != false { w.write_with_tag(80, |w| w.write_bool(*&self.boolFileExist))?; }
        for s in &self.uploadIpLanV4 { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        for s in &self.uploadIpLanV6 { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        if self.uploadPort != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.uploadPort))?; }
        Ok(())
    }
}

