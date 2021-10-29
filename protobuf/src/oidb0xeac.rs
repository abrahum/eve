// Automatically generated rust module for 'oidb0xeac.proto' file

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
pub struct EACReqBody {
    pub groupCode: Option<u64>,
    pub seq: Option<u32>,
    pub random: Option<u32>,
}

impl<'a> MessageRead<'a> for EACReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.seq = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.random = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for EACReqBody {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.seq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.random.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.seq { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.random { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EACRspBody<'a> {
    pub wording: Option<Cow<'a, str>>,
    pub digestUin: Option<u64>,
    pub digestTime: Option<u32>,
    pub errorCode: Option<u32>,
}

impl<'a> MessageRead<'a> for EACRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.wording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.digestUin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.digestTime = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.errorCode = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for EACRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.wording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.digestUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.digestTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errorCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.wording { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.digestUin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.digestTime { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errorCode { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

