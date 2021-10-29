// Automatically generated rust module for 'faceroam.proto' file

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
pub struct PlatInfo<'a> {
    pub implat: Option<i64>,
    pub osver: Option<Cow<'a, str>>,
    pub mqqver: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for PlatInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.implat = Some(r.read_int64(bytes)?),
                Ok(18) => msg.osver = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.mqqver = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PlatInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.implat.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.osver.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.mqqver.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.implat { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.osver { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.mqqver { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FaceroamReqBody<'a> {
    pub comm: Option<PlatInfo<'a>>,
    pub uin: Option<u64>,
    pub subCmd: Option<u32>,
    pub reqUserInfo: Option<ReqUserInfo>,
    pub reqDeleteItem: Option<ReqDeleteItem<'a>>,
}

impl<'a> MessageRead<'a> for FaceroamReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.comm = Some(r.read_message::<PlatInfo>(bytes)?),
                Ok(16) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.subCmd = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.reqUserInfo = Some(r.read_message::<ReqUserInfo>(bytes)?),
                Ok(42) => msg.reqDeleteItem = Some(r.read_message::<ReqDeleteItem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FaceroamReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.comm.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reqUserInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.reqDeleteItem.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.comm { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.subCmd { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.reqUserInfo { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.reqDeleteItem { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqDeleteItem<'a> {
    pub filename: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ReqDeleteItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.filename.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReqDeleteItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.filename.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.filename { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqUserInfo { }

impl<'a> MessageRead<'a> for ReqUserInfo {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ReqUserInfo { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FaceroamRspBody<'a> {
    pub ret: Option<i64>,
    pub errmsg: Option<Cow<'a, str>>,
    pub subCmd: Option<u32>,
    pub rspUserInfo: Option<RspUserInfo<'a>>,
    pub rspDeleteItem: Option<RspDeleteItem<'a>>,
}

impl<'a> MessageRead<'a> for FaceroamRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ret = Some(r.read_int64(bytes)?),
                Ok(18) => msg.errmsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.subCmd = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.rspUserInfo = Some(r.read_message::<RspUserInfo>(bytes)?),
                Ok(42) => msg.rspDeleteItem = Some(r.read_message::<RspDeleteItem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FaceroamRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ret.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errmsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rspUserInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.rspDeleteItem.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ret { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.errmsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.subCmd { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.rspUserInfo { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.rspDeleteItem { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspDeleteItem<'a> {
    pub filename: Vec<Cow<'a, str>>,
    pub ret: Vec<i64>,
}

impl<'a> MessageRead<'a> for RspDeleteItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.filename.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.ret.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspDeleteItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.filename.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.ret.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.filename { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.ret { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspUserInfo<'a> {
    pub filename: Vec<Cow<'a, str>>,
    pub deleteFile: Vec<Cow<'a, str>>,
    pub bid: Option<Cow<'a, str>>,
    pub maxRoamSize: Option<u32>,
    pub emojiType: Vec<u32>,
}

impl<'a> MessageRead<'a> for RspUserInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.filename.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.deleteFile.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.bid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.maxRoamSize = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.emojiType.push(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspUserInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.filename.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.deleteFile.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.bid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.maxRoamSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.emojiType.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.filename { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.deleteFile { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bid { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.maxRoamSize { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        for s in &self.emojiType { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

