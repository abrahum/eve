// Automatically generated rust module for 'oidb0xD79.proto' file

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
pub struct D79ReqBody<'a> {
    pub seq: u64,
    pub uin: u64,
    pub compress_flag: u32,
    pub content: Cow<'a, [u8]>,
    pub sender_uin: u64,
    pub qua: Cow<'a, [u8]>,
    pub word_ext: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for D79ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.seq = r.read_uint64(bytes)?,
                Ok(16) => msg.uin = r.read_uint64(bytes)?,
                Ok(24) => msg.compress_flag = r.read_uint32(bytes)?,
                Ok(34) => msg.content = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.sender_uin = r.read_uint64(bytes)?,
                Ok(50) => msg.qua = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.word_ext = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D79ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.seq == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.uin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.compress_flag == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.compress_flag) as u64) }
        + if self.content == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.content).len()) }
        + if self.sender_uin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.sender_uin) as u64) }
        + if self.qua == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.qua).len()) }
        + if self.word_ext == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.word_ext).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.seq != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.seq))?; }
        if self.uin != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.uin))?; }
        if self.compress_flag != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.compress_flag))?; }
        if self.content != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.content))?; }
        if self.sender_uin != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.sender_uin))?; }
        if self.qua != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.qua))?; }
        if self.word_ext != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.word_ext))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D79RspBody<'a> {
    pub ret: u32,
    pub seq: u64,
    pub uin: u64,
    pub compress_flag: u32,
    pub content: Option<D79Content<'a>>,
}

impl<'a> MessageRead<'a> for D79RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ret = r.read_uint32(bytes)?,
                Ok(16) => msg.seq = r.read_uint64(bytes)?,
                Ok(24) => msg.uin = r.read_uint64(bytes)?,
                Ok(32) => msg.compress_flag = r.read_uint32(bytes)?,
                Ok(42) => msg.content = Some(r.read_message::<D79Content>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D79RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.ret == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.ret) as u64) }
        + if self.seq == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.uin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.compress_flag == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.compress_flag) as u64) }
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ret != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.ret))?; }
        if self.seq != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.seq))?; }
        if self.uin != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.uin))?; }
        if self.compress_flag != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.compress_flag))?; }
        if let Some(ref s) = self.content { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D79Content<'a> {
    pub slice_content: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D79Content<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.slice_content.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D79Content<'a> {
    fn get_size(&self) -> usize {
        0
        + self.slice_content.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.slice_content { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

