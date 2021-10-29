// Automatically generated rust module for 'oidb0x6d7.proto' file

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
pub struct CreateFolderReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub parentFolderId: Option<Cow<'a, str>>,
    pub folderName: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for CreateFolderReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.parentFolderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.folderName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreateFolderReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.parentFolderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.folderName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.parentFolderId { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.folderName { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateFolderRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for CreateFolderRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreateFolderRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteFolderReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub folderId: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for DeleteFolderReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.folderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteFolderReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.folderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.folderId { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteFolderRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for DeleteFolderRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteFolderRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MoveFolderReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub folderId: Option<Cow<'a, str>>,
    pub parentFolderId: Option<Cow<'a, str>>,
    pub destFolderId: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for MoveFolderReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.folderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.parentFolderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.destFolderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MoveFolderReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.folderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.parentFolderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.destFolderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.folderId { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.parentFolderId { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.destFolderId { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MoveFolderRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for MoveFolderRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MoveFolderRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RenameFolderReqBody<'a> {
    pub groupCode: Option<u64>,
    pub appId: Option<u32>,
    pub folderId: Option<Cow<'a, str>>,
    pub newFolderName: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for RenameFolderReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.folderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.newFolderName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RenameFolderReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.folderId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.newFolderName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.folderId { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.newFolderName { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RenameFolderRspBody<'a> {
    pub retCode: Option<i32>,
    pub retMsg: Option<Cow<'a, str>>,
    pub clientWording: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for RenameFolderRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.retMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RenameFolderRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.retMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientWording { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D6D7ReqBody<'a> {
    pub createFolderReq: Option<CreateFolderReqBody<'a>>,
    pub deleteFolderReq: Option<DeleteFolderReqBody<'a>>,
    pub renameFolderReq: Option<RenameFolderReqBody<'a>>,
    pub moveFolderReq: Option<MoveFolderReqBody<'a>>,
}

impl<'a> MessageRead<'a> for D6D7ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.createFolderReq = Some(r.read_message::<CreateFolderReqBody>(bytes)?),
                Ok(18) => msg.deleteFolderReq = Some(r.read_message::<DeleteFolderReqBody>(bytes)?),
                Ok(26) => msg.renameFolderReq = Some(r.read_message::<RenameFolderReqBody>(bytes)?),
                Ok(34) => msg.moveFolderReq = Some(r.read_message::<MoveFolderReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D7ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.createFolderReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deleteFolderReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.renameFolderReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.moveFolderReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.createFolderReq { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deleteFolderReq { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.renameFolderReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.moveFolderReq { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D6D7RspBody<'a> {
    pub createFolderRsp: Option<CreateFolderRspBody<'a>>,
    pub deleteFolderRsp: Option<DeleteFolderRspBody<'a>>,
    pub renameFolderRsp: Option<RenameFolderRspBody<'a>>,
    pub moveFolderRsp: Option<MoveFolderRspBody<'a>>,
}

impl<'a> MessageRead<'a> for D6D7RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.createFolderRsp = Some(r.read_message::<CreateFolderRspBody>(bytes)?),
                Ok(18) => msg.deleteFolderRsp = Some(r.read_message::<DeleteFolderRspBody>(bytes)?),
                Ok(26) => msg.renameFolderRsp = Some(r.read_message::<RenameFolderRspBody>(bytes)?),
                Ok(34) => msg.moveFolderRsp = Some(r.read_message::<MoveFolderRspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D6D7RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.createFolderRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deleteFolderRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.renameFolderRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.moveFolderRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.createFolderRsp { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deleteFolderRsp { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.renameFolderRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.moveFolderRsp { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

