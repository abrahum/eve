// Automatically generated rust module for 'oidb0x8a7.proto' file

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
pub struct D8A7ReqBody {
    pub subCmd: Option<u32>,
    pub limitIntervalTypeForUin: Option<u32>,
    pub limitIntervalTypeForGroup: Option<u32>,
    pub uin: Option<u64>,
    pub groupCode: Option<u64>,
}

impl<'a> MessageRead<'a> for D8A7ReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subCmd = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.limitIntervalTypeForUin = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.limitIntervalTypeForGroup = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D8A7ReqBody {
    fn get_size(&self) -> usize {
        0
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.limitIntervalTypeForUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.limitIntervalTypeForGroup.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subCmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.limitIntervalTypeForUin { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.limitIntervalTypeForGroup { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8A7RspBody<'a> {
    pub canAtAll: Option<bool>,
    pub remainAtAllCountForUin: Option<u32>,
    pub remainAtAllCountForGroup: Option<u32>,
    pub promptMsg1: Option<Cow<'a, [u8]>>,
    pub promptMsg2: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D8A7RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.canAtAll = Some(r.read_bool(bytes)?),
                Ok(16) => msg.remainAtAllCountForUin = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.remainAtAllCountForGroup = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.promptMsg1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.promptMsg2 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8A7RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.canAtAll.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.remainAtAllCountForUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.remainAtAllCountForGroup.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.promptMsg1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.promptMsg2.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.canAtAll { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.remainAtAllCountForUin { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.remainAtAllCountForGroup { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.promptMsg1 { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.promptMsg2 { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

