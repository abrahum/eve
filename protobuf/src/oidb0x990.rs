// Automatically generated rust module for 'oidb0x990.proto' file

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
pub struct TranslateReqBody<'a> {
    pub batch_translate_req: Option<BatchTranslateReq<'a>>,
}

impl<'a> MessageRead<'a> for TranslateReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.batch_translate_req = Some(r.read_message::<BatchTranslateReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TranslateReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.batch_translate_req.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.batch_translate_req { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TranslateRspBody<'a> {
    pub batch_translate_rsp: Option<BatchTranslateRsp<'a>>,
}

impl<'a> MessageRead<'a> for TranslateRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.batch_translate_rsp = Some(r.read_message::<BatchTranslateRsp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TranslateRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.batch_translate_rsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.batch_translate_rsp { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BatchTranslateReq<'a> {
    pub src_language: Cow<'a, str>,
    pub dst_language: Cow<'a, str>,
    pub src_text_list: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BatchTranslateReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.src_language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.dst_language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.src_text_list.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BatchTranslateReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.src_language == "" { 0 } else { 1 + sizeof_len((&self.src_language).len()) }
        + if self.dst_language == "" { 0 } else { 1 + sizeof_len((&self.dst_language).len()) }
        + self.src_text_list.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.src_language != "" { w.write_with_tag(10, |w| w.write_string(&**&self.src_language))?; }
        if self.dst_language != "" { w.write_with_tag(18, |w| w.write_string(&**&self.dst_language))?; }
        for s in &self.src_text_list { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BatchTranslateRsp<'a> {
    pub error_code: i32,
    pub error_msg: Cow<'a, [u8]>,
    pub src_language: Cow<'a, str>,
    pub dst_language: Cow<'a, str>,
    pub src_text_list: Vec<Cow<'a, str>>,
    pub dst_text_list: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BatchTranslateRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error_code = r.read_int32(bytes)?,
                Ok(18) => msg.error_msg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.src_language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.dst_language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.src_text_list.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.dst_text_list.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BatchTranslateRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.error_code == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.error_code) as u64) }
        + if self.error_msg == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.error_msg).len()) }
        + if self.src_language == "" { 0 } else { 1 + sizeof_len((&self.src_language).len()) }
        + if self.dst_language == "" { 0 } else { 1 + sizeof_len((&self.dst_language).len()) }
        + self.src_text_list.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.dst_text_list.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.error_code != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.error_code))?; }
        if self.error_msg != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.error_msg))?; }
        if self.src_language != "" { w.write_with_tag(26, |w| w.write_string(&**&self.src_language))?; }
        if self.dst_language != "" { w.write_with_tag(34, |w| w.write_string(&**&self.dst_language))?; }
        for s in &self.src_text_list { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        for s in &self.dst_text_list { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

