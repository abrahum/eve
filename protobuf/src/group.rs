// Automatically generated rust module for 'group.proto' file

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
pub struct GroupFileUploadExt<'a> {
    pub unknown1: Option<i32>,
    pub unknown2: Option<i32>,
    pub entry: Option<GroupFileUploadEntry<'a>>,
    pub unknown3: Option<i32>,
}

impl<'a> MessageRead<'a> for GroupFileUploadExt<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unknown1 = Some(r.read_int32(bytes)?),
                Ok(16) => msg.unknown2 = Some(r.read_int32(bytes)?),
                Ok(802) => msg.entry = Some(r.read_message::<GroupFileUploadEntry>(bytes)?),
                Ok(24) => msg.unknown3 = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupFileUploadExt<'a> {
    fn get_size(&self) -> usize {
        0
        + self.unknown1.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unknown2.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.entry.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.unknown3.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unknown1 { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.unknown2 { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.entry { w.write_with_tag(802, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unknown3 { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupFileUploadEntry<'a> {
    pub busiBuff: Option<ExcitingBusiInfo>,
    pub fileEntry: Option<ExcitingFileEntry<'a>>,
    pub clientInfo: Option<ExcitingClientInfo<'a>>,
    pub fileNameInfo: Option<ExcitingFileNameInfo<'a>>,
    pub host: Option<ExcitingHostConfig<'a>>,
}

impl<'a> MessageRead<'a> for GroupFileUploadEntry<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(802) => msg.busiBuff = Some(r.read_message::<ExcitingBusiInfo>(bytes)?),
                Ok(1602) => msg.fileEntry = Some(r.read_message::<ExcitingFileEntry>(bytes)?),
                Ok(2402) => msg.clientInfo = Some(r.read_message::<ExcitingClientInfo>(bytes)?),
                Ok(3202) => msg.fileNameInfo = Some(r.read_message::<ExcitingFileNameInfo>(bytes)?),
                Ok(4002) => msg.host = Some(r.read_message::<ExcitingHostConfig>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupFileUploadEntry<'a> {
    fn get_size(&self) -> usize {
        0
        + self.busiBuff.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.fileEntry.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.clientInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.fileNameInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.host.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.busiBuff { w.write_with_tag(802, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileEntry { w.write_with_tag(1602, |w| w.write_message(s))?; }
        if let Some(ref s) = self.clientInfo { w.write_with_tag(2402, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fileNameInfo { w.write_with_tag(3202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.host { w.write_with_tag(4002, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExcitingBusiInfo {
    pub busId: Option<i32>,
    pub senderUin: Option<i64>,
    pub receiverUin: Option<i64>,
    pub groupCode: Option<i64>,
}

impl<'a> MessageRead<'a> for ExcitingBusiInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.busId = Some(r.read_int32(bytes)?),
                Ok(800) => msg.senderUin = Some(r.read_int64(bytes)?),
                Ok(1600) => msg.receiverUin = Some(r.read_int64(bytes)?),
                Ok(3200) => msg.groupCode = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ExcitingBusiInfo {
    fn get_size(&self) -> usize {
        0
        + self.busId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.senderUin.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.receiverUin.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupCode.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.busId { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.senderUin { w.write_with_tag(800, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.receiverUin { w.write_with_tag(1600, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(3200, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExcitingFileEntry<'a> {
    pub fileSize: Option<i64>,
    pub md5: Option<Cow<'a, [u8]>>,
    pub sha1: Option<Cow<'a, [u8]>>,
    pub fileId: Option<Cow<'a, [u8]>>,
    pub uploadKey: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ExcitingFileEntry<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(800) => msg.fileSize = Some(r.read_int64(bytes)?),
                Ok(1602) => msg.md5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(2402) => msg.sha1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(4802) => msg.fileId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(5602) => msg.uploadKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExcitingFileEntry<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileSize.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.md5.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.sha1.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.fileId.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.uploadKey.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileSize { w.write_with_tag(800, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.md5 { w.write_with_tag(1602, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sha1 { w.write_with_tag(2402, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(4802, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.uploadKey { w.write_with_tag(5602, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExcitingClientInfo<'a> {
    pub clientType: Option<i32>,
    pub appId: Option<Cow<'a, str>>,
    pub terminalType: Option<i32>,
    pub clientVer: Option<Cow<'a, str>>,
    pub unknown: Option<i32>,
}

impl<'a> MessageRead<'a> for ExcitingClientInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(800) => msg.clientType = Some(r.read_int32(bytes)?),
                Ok(1602) => msg.appId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(2400) => msg.terminalType = Some(r.read_int32(bytes)?),
                Ok(3202) => msg.clientVer = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(4800) => msg.unknown = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExcitingClientInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.clientType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.appId.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.terminalType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.clientVer.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.unknown.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.clientType { w.write_with_tag(800, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(1602, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.terminalType { w.write_with_tag(2400, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.clientVer { w.write_with_tag(3202, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.unknown { w.write_with_tag(4800, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExcitingFileNameInfo<'a> {
    pub fileName: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ExcitingFileNameInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(802) => msg.fileName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExcitingFileNameInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileName { w.write_with_tag(802, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExcitingHostConfig<'a> {
    pub hosts: Vec<ExcitingHostInfo<'a>>,
}

impl<'a> MessageRead<'a> for ExcitingHostConfig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(1602) => msg.hosts.push(r.read_message::<ExcitingHostInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExcitingHostConfig<'a> {
    fn get_size(&self) -> usize {
        0
        + self.hosts.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.hosts { w.write_with_tag(1602, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExcitingHostInfo<'a> {
    pub url: Option<ExcitingUrlInfo<'a>>,
    pub port: Option<i32>,
}

impl<'a> MessageRead<'a> for ExcitingHostInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = Some(r.read_message::<ExcitingUrlInfo>(bytes)?),
                Ok(16) => msg.port = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExcitingHostInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.port.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.url { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.port { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExcitingUrlInfo<'a> {
    pub unknown: Option<i32>,
    pub host: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ExcitingUrlInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unknown = Some(r.read_int32(bytes)?),
                Ok(18) => msg.host = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExcitingUrlInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.unknown.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.host.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unknown { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.host { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

