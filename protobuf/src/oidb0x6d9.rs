// Automatically generated rust module for 'oidb0x6d9.proto' file

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
pub struct GroupFileFeedsInfo<'a> {
    pub busId: Option<u32>,
    pub fileId: Option<Cow<'a, str>>,
    pub msgRandom: Option<u32>,
    pub ext: Option<Cow<'a, [u8]>>,
    pub feedFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for GroupFileFeedsInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.busId = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.fileId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.msgRandom = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.ext = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.feedFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupFileFeedsInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.msgRandom.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ext.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.feedFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.busId { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.msgRandom { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.ext { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.feedFlag { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CopyFromReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub srcBusId: Option<u32>,
    pub srcParentFolder: Option<Cow<'a, [u8]>>,
    pub srcFilePath: Option<Cow<'a, [u8]>>,
    pub dstBusId: Option<u32>,
    pub dstFolderId: Option<Cow<'a, [u8]>>,
    pub fileSize: Option<u64>,
    pub localPath: Option<Cow<'a, str>>,
    pub fileName: Option<Cow<'a, str>>,
    pub srcUin: Option<u64>,
    pub md5: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for CopyFromReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.srcBusId = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.srcParentFolder = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.srcFilePath = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.dstBusId = Some(r.read_uint32(bytes)?),
                Ok(58) => msg.dstFolderId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.fileSize = Some(r.read_uint64(bytes)?),
                Ok(74) => msg.localPath = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.fileName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.srcUin = Some(r.read_uint64(bytes)?),
                Ok(98) => msg.md5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CopyFromReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.srcBusId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.srcParentFolder.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.srcFilePath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.dstBusId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.dstFolderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.localPath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.srcUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.md5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.srcBusId { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.srcParentFolder { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.srcFilePath { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.dstBusId { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.dstFolderId { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileSize { w.write_with_tag(64, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.localPath { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fileName { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.srcUin { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.md5 { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CopyFromRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub saveFilePath: Option<Cow<'a, [u8]>>,
    pub busId: Option<u32>,
}

impl<'a> MessageRead<'a> for CopyFromRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.saveFilePath = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.busId = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CopyFromRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.saveFilePath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.saveFilePath { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.busId { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CopyToReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub srcBusId: Option<u32>,
    pub srcFileId: Option<Cow<'a, str>>,
    pub dstBusId: Option<u32>,
    pub dstUin: Option<u64>,
    pub newFileName: Option<Cow<'a, str>>,
    pub timCloudPdirKey: Option<Cow<'a, [u8]>>,
    pub timCloudPpdirKey: Option<Cow<'a, [u8]>>,
    pub timCloudExtensionInfo: Option<Cow<'a, [u8]>>,
    pub timFileExistOption: Option<u32>,
}

impl<'a> MessageRead<'a> for CopyToReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.srcBusId = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.srcFileId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.dstBusId = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.dstUin = Some(r.read_uint64(bytes)?),
                Ok(322) => msg.newFileName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(802) => msg.timCloudPdirKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(810) => msg.timCloudPpdirKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(818) => msg.timCloudExtensionInfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(824) => msg.timFileExistOption = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CopyToReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.srcBusId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.srcFileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.dstBusId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.dstUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.newFileName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.timCloudPdirKey.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.timCloudPpdirKey.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.timCloudExtensionInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.timFileExistOption.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.srcBusId { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.srcFileId { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.dstBusId { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.dstUin { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.newFileName { w.write_with_tag(322, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.timCloudPdirKey { w.write_with_tag(802, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.timCloudPpdirKey { w.write_with_tag(810, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.timCloudExtensionInfo { w.write_with_tag(818, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.timFileExistOption { w.write_with_tag(824, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CopyToRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub saveFilePath: Option<Cow<'a, str>>,
    pub busId: Option<u32>,
    pub fileName: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for CopyToRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.saveFilePath = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.busId = Some(r.read_uint32(bytes)?),
                Ok(322) => msg.fileName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CopyToRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.saveFilePath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.saveFilePath { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.busId { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fileName { w.write_with_tag(322, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeedsReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub feedsInfoList: Vec<GroupFileFeedsInfo<'a>>,
    pub multiSendSeq: Option<u32>,
}

impl<'a> MessageRead<'a> for FeedsReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.feedsInfoList.push(r.read_message::<GroupFileFeedsInfo>(bytes)?),
                Ok(32) => msg.multiSendSeq = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FeedsReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.feedsInfoList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.multiSendSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        for s in &self.feedsInfoList { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.multiSendSeq { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeedsRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub svrbusyWaitTime: Option<u32>,
}

impl<'a> MessageRead<'a> for FeedsRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.svrbusyWaitTime = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FeedsRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.svrbusyWaitTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.svrbusyWaitTime { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D6D9ReqBody<'a> {
    pub transFileReq: Option<TransFileReqBody<'a>>,
    pub copyFromReq: Option<CopyFromReqBody<'a>>,
    pub copyToReq: Option<CopyToReqBody<'a>>,
    pub feedsInfoReq: Option<FeedsReqBody<'a>>,
}

impl<'a> MessageRead<'a> for D6D9ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.transFileReq = Some(r.read_message::<TransFileReqBody>(bytes)?),
                Ok(18) => msg.copyFromReq = Some(r.read_message::<CopyFromReqBody>(bytes)?),
                Ok(26) => msg.copyToReq = Some(r.read_message::<CopyToReqBody>(bytes)?),
                Ok(42) => msg.feedsInfoReq = Some(r.read_message::<FeedsReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D9ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.transFileReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.copyFromReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.copyToReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.feedsInfoReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.transFileReq { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.copyFromReq { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.copyToReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.feedsInfoReq { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D6D9RspBody<'a> {
    pub transFileRsp: Option<TransFileRspBody<'a>>,
    pub copyFromRsp: Option<CopyFromRspBody<'a>>,
    pub copyToRsp: Option<CopyToRspBody<'a>>,
    pub feedsInfoRsp: Option<FeedsRspBody<'a>>,
}

impl<'a> MessageRead<'a> for D6D9RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.transFileRsp = Some(r.read_message::<TransFileRspBody>(bytes)?),
                Ok(18) => msg.copyFromRsp = Some(r.read_message::<CopyFromRspBody>(bytes)?),
                Ok(26) => msg.copyToRsp = Some(r.read_message::<CopyToRspBody>(bytes)?),
                Ok(42) => msg.feedsInfoRsp = Some(r.read_message::<FeedsRspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D9RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.transFileRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.copyFromRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.copyToRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.feedsInfoRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.transFileRsp { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.copyFromRsp { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.copyToRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.feedsInfoRsp { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransFileReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub busId: Option<u32>,
    pub fileId: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for TransFileReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.busId = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.fileId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransFileReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.busId { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransFileRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub saveBusId: Option<u32>,
    pub saveFilePath: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for TransFileRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.saveBusId = Some(r.read_uint32(bytes)?),
                Ok(42) => msg.saveFilePath = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransFileRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.saveBusId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.saveFilePath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.saveBusId { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.saveFilePath { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

