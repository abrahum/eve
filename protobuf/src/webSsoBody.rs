// Automatically generated rust module for 'webSsoBody.proto' file

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
pub struct STServiceMonitItem<'a> {
    pub cmd: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub errcode: Option<i32>,
    pub cost: Option<u32>,
    pub src: Option<u32>,
}

impl<'a> MessageRead<'a> for STServiceMonitItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.cmd = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.errcode = Some(r.read_int32(bytes)?),
                Ok(32) => msg.cost = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.src = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for STServiceMonitItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.cmd.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.errcode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cost.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.src.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.cmd { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.url { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.errcode { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.cost { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.src { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct STServiceMonitReq<'a> {
    pub list: Vec<STServiceMonitItem<'a>>,
}

impl<'a> MessageRead<'a> for STServiceMonitReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.list.push(r.read_message::<STServiceMonitItem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for STServiceMonitReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.list { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WebSsoControlData {
    pub frequency: Option<u32>,
    pub packageSize: Option<u32>,
}

impl<'a> MessageRead<'a> for WebSsoControlData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.frequency = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.packageSize = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WebSsoControlData {
    fn get_size(&self) -> usize {
        0
        + self.frequency.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.packageSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.frequency { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.packageSize { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WebSsoRequestBody<'a> {
    pub version: Option<u32>,
    pub type_pb: Option<u32>,
    pub data: Option<Cow<'a, str>>,
    pub webData: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for WebSsoRequestBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.data = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.webData = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for WebSsoRequestBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.webData.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.version { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.webData { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WebSsoResponseBody<'a> {
    pub version: Option<u32>,
    pub type_pb: Option<u32>,
    pub ret: Option<u32>,
    pub data: Option<Cow<'a, str>>,
    pub controlData: Option<WebSsoControlData>,
}

impl<'a> MessageRead<'a> for WebSsoResponseBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.ret = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.data = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.controlData = Some(r.read_message::<WebSsoControlData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for WebSsoResponseBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ret.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.controlData.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.version { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.ret { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.controlData { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

