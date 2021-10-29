// Automatically generated rust module for 'protocol.proto' file

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
pub struct QWebReq<'a> {
    pub seq: i64,
    pub qua: Cow<'a, str>,
    pub deviceInfo: Cow<'a, str>,
    pub busiBuff: Cow<'a, [u8]>,
    pub traceId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for QWebReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.seq = r.read_int64(bytes)?,
                Ok(18) => msg.qua = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.deviceInfo = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.busiBuff = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.traceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QWebReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.seq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.qua == "" { 0 } else { 1 + sizeof_len((&self.qua).len()) }
        + if self.deviceInfo == "" { 0 } else { 1 + sizeof_len((&self.deviceInfo).len()) }
        + if self.busiBuff == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.busiBuff).len()) }
        + if self.traceId == "" { 0 } else { 1 + sizeof_len((&self.traceId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.seq != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.seq))?; }
        if self.qua != "" { w.write_with_tag(18, |w| w.write_string(&**&self.qua))?; }
        if self.deviceInfo != "" { w.write_with_tag(26, |w| w.write_string(&**&self.deviceInfo))?; }
        if self.busiBuff != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.busiBuff))?; }
        if self.traceId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.traceId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QWebRsp<'a> {
    pub seq: i64,
    pub retCode: i64,
    pub errMsg: Cow<'a, str>,
    pub busiBuff: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for QWebRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.seq = r.read_int64(bytes)?,
                Ok(16) => msg.retCode = r.read_int64(bytes)?,
                Ok(26) => msg.errMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.busiBuff = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QWebRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.seq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.retCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.errMsg == "" { 0 } else { 1 + sizeof_len((&self.errMsg).len()) }
        + if self.busiBuff == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.busiBuff).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.seq != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.seq))?; }
        if self.retCode != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.retCode))?; }
        if self.errMsg != "" { w.write_with_tag(26, |w| w.write_string(&**&self.errMsg))?; }
        if self.busiBuff != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.busiBuff))?; }
        Ok(())
    }
}

