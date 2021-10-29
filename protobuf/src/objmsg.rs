// Automatically generated rust module for 'objmsg.proto' file

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
pub struct MsgPic<'a> {
    pub smallPicUrl: Cow<'a, [u8]>,
    pub originalPicUrl: Cow<'a, [u8]>,
    pub localPicId: i32,
}

impl<'a> MessageRead<'a> for MsgPic<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.smallPicUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.originalPicUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.localPicId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MsgPic<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.smallPicUrl == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.smallPicUrl).len()) }
        + if self.originalPicUrl == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.originalPicUrl).len()) }
        + if self.localPicId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.localPicId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.smallPicUrl != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.smallPicUrl))?; }
        if self.originalPicUrl != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.originalPicUrl))?; }
        if self.localPicId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.localPicId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjMsg<'a> {
    pub msgType: i32,
    pub title: Cow<'a, [u8]>,
    pub bytesAbstact: Cow<'a, [u8]>,
    pub titleExt: Cow<'a, [u8]>,
    pub msgPic: Vec<MsgPic<'a>>,
    pub msgContentInfo: Vec<MsgContentInfo<'a>>,
    pub reportIdShow: i32,
}

impl<'a> MessageRead<'a> for ObjMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgType = r.read_int32(bytes)?,
                Ok(18) => msg.title = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.bytesAbstact = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.titleExt = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.msgPic.push(r.read_message::<MsgPic>(bytes)?),
                Ok(58) => msg.msgContentInfo.push(r.read_message::<MsgContentInfo>(bytes)?),
                Ok(64) => msg.reportIdShow = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObjMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.title == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.bytesAbstact == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bytesAbstact).len()) }
        + if self.titleExt == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.titleExt).len()) }
        + self.msgPic.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgContentInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.reportIdShow == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reportIdShow) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.msgType))?; }
        if self.title != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.title))?; }
        if self.bytesAbstact != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.bytesAbstact))?; }
        if self.titleExt != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.titleExt))?; }
        for s in &self.msgPic { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.msgContentInfo { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.reportIdShow != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.reportIdShow))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgContentInfo<'a> {
    pub contentInfoId: Cow<'a, [u8]>,
    pub msgFile: Option<MsgFile<'a>>,
}

impl<'a> MessageRead<'a> for MsgContentInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.contentInfoId = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.msgFile = Some(r.read_message::<MsgFile>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MsgContentInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.contentInfoId == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.contentInfoId).len()) }
        + self.msgFile.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.contentInfoId != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.contentInfoId))?; }
        if let Some(ref s) = self.msgFile { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgFile<'a> {
    pub busId: i32,
    pub filePath: Cow<'a, [u8]>,
    pub fileSize: i64,
    pub fileName: Cow<'a, str>,
    pub int64DeadTime: i64,
    pub fileSha1: Cow<'a, [u8]>,
    pub ext: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for MsgFile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.busId = r.read_int32(bytes)?,
                Ok(18) => msg.filePath = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.fileSize = r.read_int64(bytes)?,
                Ok(34) => msg.fileName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.int64DeadTime = r.read_int64(bytes)?,
                Ok(50) => msg.fileSha1 = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.ext = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MsgFile<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.busId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.busId) as u64) }
        + if self.filePath == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.filePath).len()) }
        + if self.fileSize == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
        + if self.fileName == "" { 0 } else { 1 + sizeof_len((&self.fileName).len()) }
        + if self.int64DeadTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.int64DeadTime) as u64) }
        + if self.fileSha1 == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fileSha1).len()) }
        + if self.ext == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ext).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.busId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.busId))?; }
        if self.filePath != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.filePath))?; }
        if self.fileSize != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.fileSize))?; }
        if self.fileName != "" { w.write_with_tag(34, |w| w.write_string(&**&self.fileName))?; }
        if self.int64DeadTime != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.int64DeadTime))?; }
        if self.fileSha1 != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.fileSha1))?; }
        if self.ext != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.ext))?; }
        Ok(())
    }
}

