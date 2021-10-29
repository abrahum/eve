// Automatically generated rust module for 'oidb0x6d8.proto' file

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
pub struct D6D8ReqBody<'a> {
    pub fileInfoReq: Option<GetFileInfoReqBody<'a>>,
    pub fileListInfoReq: Option<GetFileListReqBody<'a>>,
    pub groupFileCountReq: Option<GetFileCountReqBody>,
    pub groupSpaceReq: Option<GetSpaceReqBody>,
}

impl<'a> MessageRead<'a> for D6D8ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileInfoReq = Some(r.read_message::<GetFileInfoReqBody>(bytes)?),
                Ok(18) => msg.fileListInfoReq = Some(r.read_message::<GetFileListReqBody>(bytes)?),
                Ok(26) => msg.groupFileCountReq = Some(r.read_message::<GetFileCountReqBody>(bytes)?),
                Ok(34) => msg.groupSpaceReq = Some(r.read_message::<GetSpaceReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D8ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileInfoReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileListInfoReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupFileCountReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupSpaceReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileInfoReq { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileListInfoReq { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupFileCountReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupSpaceReq { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D6D8RspBody<'a> {
    pub fileInfoRsp: Option<GetFileInfoRspBody<'a>>,
    pub fileListInfoRsp: Option<GetFileListRspBody<'a>>,
    pub fileCountRsp: Option<GetFileCountRspBody<'a>>,
    pub groupSpaceRsp: Option<GetSpaceRspBody<'a>>,
}

impl<'a> MessageRead<'a> for D6D8RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileInfoRsp = Some(r.read_message::<GetFileInfoRspBody>(bytes)?),
                Ok(18) => msg.fileListInfoRsp = Some(r.read_message::<GetFileListRspBody>(bytes)?),
                Ok(26) => msg.fileCountRsp = Some(r.read_message::<GetFileCountRspBody>(bytes)?),
                Ok(34) => msg.groupSpaceRsp = Some(r.read_message::<GetSpaceRspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D8RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileInfoRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileListInfoRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileCountRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupSpaceRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileInfoRsp { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileListInfoRsp { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileCountRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupSpaceRsp { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFileInfoReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub busId: Option<u32>,
    pub fileId: Option<Cow<'a, str>>,
    pub fieldFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for GetFileInfoReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.busId = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.fileId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.fieldFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetFileInfoReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fieldFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.busId { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fieldFlag { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFileInfoRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub fileInfo: Option<GroupFileInfo<'a>>,
}

impl<'a> MessageRead<'a> for GetFileInfoRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.fileInfo = Some(r.read_message::<GroupFileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetFileInfoRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fileInfo { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFileListRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub isEnd: Option<bool>,
    pub itemList: Vec<mod_GetFileListRspBody::Item<'a>>,
    pub maxTimestamp: Option<FileTimeStamp<'a>>,
    pub allFileCount: Option<u32>,
    pub filterCode: Option<u32>,
    pub safeCheckFlag: Option<bool>,
    pub safeCheckRes: Option<u32>,
    pub nextIndex: Option<u32>,
    pub context: Option<Cow<'a, [u8]>>,
    pub role: Option<u32>,
    pub openFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for GetFileListRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.isEnd = Some(r.read_bool(bytes)?),
                Ok(42) => msg.itemList.push(r.read_message::<mod_GetFileListRspBody::Item>(bytes)?),
                Ok(50) => msg.maxTimestamp = Some(r.read_message::<FileTimeStamp>(bytes)?),
                Ok(56) => msg.allFileCount = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.filterCode = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.safeCheckFlag = Some(r.read_bool(bytes)?),
                Ok(96) => msg.safeCheckRes = Some(r.read_uint32(bytes)?),
                Ok(104) => msg.nextIndex = Some(r.read_uint32(bytes)?),
                Ok(114) => msg.context = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.role = Some(r.read_uint32(bytes)?),
                Ok(128) => msg.openFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetFileListRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.isEnd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.itemList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.maxTimestamp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.allFileCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.filterCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.safeCheckFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.safeCheckRes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.nextIndex.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.context.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.role.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.openFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.isEnd { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        for s in &self.itemList { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.maxTimestamp { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.allFileCount { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.filterCode { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.safeCheckFlag { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.safeCheckRes { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.nextIndex { w.write_with_tag(104, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.context { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.role { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.openFlag { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

pub mod mod_GetFileListRspBody {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Item<'a> {
    pub type_pb: Option<u32>,
    pub folderInfo: Option<GroupFolderInfo<'a>>,
    pub fileInfo: Option<GroupFileInfo<'a>>,
}

impl<'a> MessageRead<'a> for Item<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.folderInfo = Some(r.read_message::<GroupFolderInfo>(bytes)?),
                Ok(26) => msg.fileInfo = Some(r.read_message::<GroupFileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Item<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.folderInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.folderInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileInfo { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupFileInfo<'a> {
    pub fileId: Option<Cow<'a, str>>,
    pub fileName: Option<Cow<'a, str>>,
    pub fileSize: Option<u64>,
    pub busId: Option<u32>,
    pub uploadedSize: Option<u64>,
    pub uploadTime: Option<u32>,
    pub deadTime: Option<u32>,
    pub modifyTime: Option<u32>,
    pub downloadTimes: Option<u32>,
    pub sha: Option<Cow<'a, [u8]>>,
    pub sha3: Option<Cow<'a, [u8]>>,
    pub md5: Option<Cow<'a, [u8]>>,
    pub localPath: Option<Cow<'a, str>>,
    pub uploaderName: Option<Cow<'a, str>>,
    pub uploaderUin: Option<u64>,
    pub parentFolderId: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for GroupFileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.fileName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.fileSize = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.busId = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.uploadedSize = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.uploadTime = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.deadTime = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.modifyTime = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.downloadTimes = Some(r.read_uint32(bytes)?),
                Ok(82) => msg.sha = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.sha3 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.md5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.localPath = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(114) => msg.uploaderName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.uploaderUin = Some(r.read_uint64(bytes)?),
                Ok(130) => msg.parentFolderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupFileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uploadedSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uploadTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.deadTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.modifyTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.downloadTimes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sha.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sha3.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.md5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.localPath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.uploaderName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.uploaderUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.parentFolderId.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileId { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fileName { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fileSize { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.busId { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uploadedSize { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.uploadTime { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.deadTime { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.modifyTime { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.downloadTimes { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sha { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sha3 { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.md5 { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.localPath { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.uploaderName { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.uploaderUin { w.write_with_tag(120, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.parentFolderId { w.write_with_tag(130, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupFolderInfo<'a> {
    pub folderId: Option<Cow<'a, str>>,
    pub parentFolderId: Option<Cow<'a, str>>,
    pub folderName: Option<Cow<'a, str>>,
    pub createTime: Option<u32>,
    pub modifyTime: Option<u32>,
    pub createUin: Option<u64>,
    pub creatorName: Option<Cow<'a, str>>,
    pub totalFileCount: Option<u32>,
}

impl<'a> MessageRead<'a> for GroupFolderInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.folderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.parentFolderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.folderName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.createTime = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.modifyTime = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.createUin = Some(r.read_uint64(bytes)?),
                Ok(58) => msg.creatorName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.totalFileCount = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupFolderInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.folderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.parentFolderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.folderName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.createTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.modifyTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.createUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.creatorName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.totalFileCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.folderId { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.parentFolderId { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.folderName { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.createTime { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.modifyTime { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.createUin { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.creatorName { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.totalFileCount { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFileListReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub folderId: Option<Cow<'a, str>>,
    pub startTimestamp: Option<FileTimeStamp<'a>>,
    pub fileCount: Option<u32>,
    pub maxTimestamp: Option<FileTimeStamp<'a>>,
    pub allFileCount: Option<u32>,
    pub reqFrom: Option<u32>,
    pub sortBy: Option<u32>,
    pub filterCode: Option<u32>,
    pub uin: Option<u64>,
    pub fieldFlag: Option<u32>,
    pub startIndex: Option<u32>,
    pub context: Option<Cow<'a, [u8]>>,
    pub clientVersion: Option<u32>,
}

impl<'a> MessageRead<'a> for GetFileListReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.folderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.startTimestamp = Some(r.read_message::<FileTimeStamp>(bytes)?),
                Ok(40) => msg.fileCount = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.maxTimestamp = Some(r.read_message::<FileTimeStamp>(bytes)?),
                Ok(56) => msg.allFileCount = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.reqFrom = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.sortBy = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.filterCode = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(96) => msg.fieldFlag = Some(r.read_uint32(bytes)?),
                Ok(104) => msg.startIndex = Some(r.read_uint32(bytes)?),
                Ok(114) => msg.context = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.clientVersion = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetFileListReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.folderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.startTimestamp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.maxTimestamp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.allFileCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reqFrom.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sortBy.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.filterCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fieldFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.startIndex.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.context.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientVersion.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.folderId { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.startTimestamp { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileCount { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.maxTimestamp { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.allFileCount { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.reqFrom { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sortBy { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.filterCode { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fieldFlag { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.startIndex { w.write_with_tag(104, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.context { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.clientVersion { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFileCountReqBody {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub busId: Option<u32>,
}

impl<'a> MessageRead<'a> for GetFileCountReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.busId = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetFileCountReqBody {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.busId { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSpaceReqBody {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
}

impl<'a> MessageRead<'a> for GetSpaceReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetSpaceReqBody {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFileCountRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub allFileCount: Option<u32>,
    pub fileTooMany: Option<bool>,
    pub limitCount: Option<u32>,
    pub isFull: Option<bool>,
}

impl<'a> MessageRead<'a> for GetFileCountRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.allFileCount = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.fileTooMany = Some(r.read_bool(bytes)?),
                Ok(48) => msg.limitCount = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.isFull = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetFileCountRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.allFileCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileTooMany.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.limitCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.isFull.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.allFileCount { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fileTooMany { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.limitCount { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.isFull { w.write_with_tag(56, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSpaceRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
    pub totalSpace: Option<u64>,
    pub usedSpace: Option<u64>,
}

impl<'a> MessageRead<'a> for GetSpaceRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.totalSpace = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.usedSpace = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetSpaceRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.totalSpace.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.usedSpace.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.totalSpace { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.usedSpace { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileTimeStamp<'a> {
    pub uploadTime: Option<u32>,
    pub fileId: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for FileTimeStamp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uploadTime = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.fileId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileTimeStamp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uploadTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uploadTime { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

