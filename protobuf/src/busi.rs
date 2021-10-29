// Automatically generated rust module for 'busi.proto' file

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
pub struct BusiColor {
    pub r: Option<i32>,
    pub g: Option<i32>,
    pub b: Option<i32>,
}

impl<'a> MessageRead<'a> for BusiColor {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.r = Some(r.read_int32(bytes)?),
                Ok(16) => msg.g = Some(r.read_int32(bytes)?),
                Ok(24) => msg.b = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BusiColor {
    fn get_size(&self) -> usize {
        0
        + self.r.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.g.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.r { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.g { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.b { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiComm<'a> {
    pub ver: Option<i32>,
    pub seq: Option<i32>,
    pub fromuin: Option<i64>,
    pub touin: Option<i64>,
    pub service: Option<i32>,
    pub sessionType: Option<i32>,
    pub sessionKey: Option<Cow<'a, [u8]>>,
    pub clientIp: Option<i32>,
    pub display: Option<BusiUi<'a>>,
    pub result: Option<i32>,
    pub errMsg: Option<Cow<'a, str>>,
    pub platform: Option<i32>,
    pub qqver: Option<Cow<'a, str>>,
    pub build: Option<i32>,
    pub msgLoginSig: Option<BusiLoginSig<'a>>,
    pub version: Option<i32>,
    pub msgUinInfo: Option<BusiUinInfo>,
    pub msgRichDisplay: Option<BusiRichUi<'a>>,
}

impl<'a> MessageRead<'a> for BusiComm<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ver = Some(r.read_int32(bytes)?),
                Ok(16) => msg.seq = Some(r.read_int32(bytes)?),
                Ok(24) => msg.fromuin = Some(r.read_int64(bytes)?),
                Ok(32) => msg.touin = Some(r.read_int64(bytes)?),
                Ok(40) => msg.service = Some(r.read_int32(bytes)?),
                Ok(48) => msg.sessionType = Some(r.read_int32(bytes)?),
                Ok(58) => msg.sessionKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.clientIp = Some(r.read_int32(bytes)?),
                Ok(74) => msg.display = Some(r.read_message::<BusiUi>(bytes)?),
                Ok(80) => msg.result = Some(r.read_int32(bytes)?),
                Ok(90) => msg.errMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.platform = Some(r.read_int32(bytes)?),
                Ok(106) => msg.qqver = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.build = Some(r.read_int32(bytes)?),
                Ok(122) => msg.msgLoginSig = Some(r.read_message::<BusiLoginSig>(bytes)?),
                Ok(136) => msg.version = Some(r.read_int32(bytes)?),
                Ok(146) => msg.msgUinInfo = Some(r.read_message::<BusiUinInfo>(bytes)?),
                Ok(154) => msg.msgRichDisplay = Some(r.read_message::<BusiRichUi>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BusiComm<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ver.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.seq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.touin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.service.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sessionType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sessionKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientIp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.display.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.platform.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.qqver.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.build.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgLoginSig.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.version.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.msgUinInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.msgRichDisplay.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ver { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.seq { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fromuin { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.touin { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.service { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.sessionType { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.sessionKey { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.clientIp { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.display { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.errMsg { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.platform { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.qqver { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.build { w.write_with_tag(112, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgLoginSig { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgUinInfo { w.write_with_tag(146, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgRichDisplay { w.write_with_tag(154, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiCommonReq<'a> {
    pub serviceCmd: Option<Cow<'a, str>>,
    pub vcReq: Option<BusiVisitorCountReq>,
    pub hrReq: Option<BusiHideRecordsReq>,
}

impl<'a> MessageRead<'a> for BusiCommonReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.serviceCmd = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.vcReq = Some(r.read_message::<BusiVisitorCountReq>(bytes)?),
                Ok(26) => msg.hrReq = Some(r.read_message::<BusiHideRecordsReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BusiCommonReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.serviceCmd.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.vcReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.hrReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.serviceCmd { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.vcReq { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.hrReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiDetailRecord {
    pub fuin: Option<i32>,
    pub source: Option<i32>,
    pub vtime: Option<i32>,
    pub mod_pb: Option<i32>,
    pub hideFlag: Option<i32>,
}

impl<'a> MessageRead<'a> for BusiDetailRecord {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fuin = Some(r.read_int32(bytes)?),
                Ok(16) => msg.source = Some(r.read_int32(bytes)?),
                Ok(24) => msg.vtime = Some(r.read_int32(bytes)?),
                Ok(32) => msg.mod_pb = Some(r.read_int32(bytes)?),
                Ok(40) => msg.hideFlag = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BusiDetailRecord {
    fn get_size(&self) -> usize {
        0
        + self.fuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.vtime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mod_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hideFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fuin { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.vtime { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.mod_pb { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.hideFlag { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiHideRecordsReq {
    pub huin: Option<i32>,
    pub fuin: Option<i32>,
    pub records: Vec<BusiDetailRecord>,
}

impl<'a> MessageRead<'a> for BusiHideRecordsReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.huin = Some(r.read_int32(bytes)?),
                Ok(16) => msg.fuin = Some(r.read_int32(bytes)?),
                Ok(26) => msg.records.push(r.read_message::<BusiDetailRecord>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BusiHideRecordsReq {
    fn get_size(&self) -> usize {
        0
        + self.huin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.records.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.huin { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fuin { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        for s in &self.records { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiLabel<'a> {
    pub name: Option<Cow<'a, [u8]>>,
    pub enumType: Option<i32>,
    pub textColor: Option<BusiColor>,
    pub edgingColor: Option<BusiColor>,
    pub labelAttr: Option<i32>,
    pub labelType: Option<i32>,
}

impl<'a> MessageRead<'a> for BusiLabel<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.enumType = Some(r.read_int32(bytes)?),
                Ok(26) => msg.textColor = Some(r.read_message::<BusiColor>(bytes)?),
                Ok(34) => msg.edgingColor = Some(r.read_message::<BusiColor>(bytes)?),
                Ok(40) => msg.labelAttr = Some(r.read_int32(bytes)?),
                Ok(48) => msg.labelType = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BusiLabel<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.enumType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.textColor.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.edgingColor.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.labelAttr.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.labelType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.enumType { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.textColor { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.edgingColor { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.labelAttr { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.labelType { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiLoginSig<'a> {
    pub type_pb: Option<i32>,
    pub sig: Option<Cow<'a, [u8]>>,
    pub appid: Option<i32>,
}

impl<'a> MessageRead<'a> for BusiLoginSig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_int32(bytes)?),
                Ok(18) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.appid = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BusiLoginSig<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.appid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.appid { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiRichUi<'a> {
    pub name: Option<Cow<'a, str>>,
    pub serviceUrl: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BusiRichUi<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.serviceUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BusiRichUi<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.serviceUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.serviceUrl { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiUi<'a> {
    pub url: Option<Cow<'a, str>>,
    pub title: Option<Cow<'a, str>>,
    pub content: Option<Cow<'a, str>>,
    pub jumpUrl: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BusiUi<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.title = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.content = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.jumpUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BusiUi<'a> {
    fn get_size(&self) -> usize {
        0
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.title.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.jumpUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.url { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.title { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.content { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.jumpUrl { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiUinInfo {
    pub int64Longitude: Option<i64>,
    pub int64Latitude: Option<i64>,
}

impl<'a> MessageRead<'a> for BusiUinInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.int64Longitude = Some(r.read_int64(bytes)?),
                Ok(16) => msg.int64Latitude = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BusiUinInfo {
    fn get_size(&self) -> usize {
        0
        + self.int64Longitude.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.int64Latitude.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.int64Longitude { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.int64Latitude { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiVisitorCountReq {
    pub requireuin: Option<i32>,
    pub operuin: Option<i32>,
    pub mod_pb: Option<i32>,
    pub reportFlag: Option<i32>,
}

impl<'a> MessageRead<'a> for BusiVisitorCountReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.requireuin = Some(r.read_int32(bytes)?),
                Ok(16) => msg.operuin = Some(r.read_int32(bytes)?),
                Ok(24) => msg.mod_pb = Some(r.read_int32(bytes)?),
                Ok(32) => msg.reportFlag = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BusiVisitorCountReq {
    fn get_size(&self) -> usize {
        0
        + self.requireuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.operuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mod_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reportFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.requireuin { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.operuin { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.mod_pb { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reportFlag { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BusiVisitorCountRsp {
    pub requireuin: Option<i32>,
    pub totalLike: Option<i32>,
    pub totalView: Option<i32>,
    pub hotValue: Option<i32>,
    pub redValue: Option<i32>,
    pub hotDiff: Option<i32>,
}

impl<'a> MessageRead<'a> for BusiVisitorCountRsp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.requireuin = Some(r.read_int32(bytes)?),
                Ok(16) => msg.totalLike = Some(r.read_int32(bytes)?),
                Ok(24) => msg.totalView = Some(r.read_int32(bytes)?),
                Ok(32) => msg.hotValue = Some(r.read_int32(bytes)?),
                Ok(40) => msg.redValue = Some(r.read_int32(bytes)?),
                Ok(48) => msg.hotDiff = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BusiVisitorCountRsp {
    fn get_size(&self) -> usize {
        0
        + self.requireuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.totalLike.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.totalView.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hotValue.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.redValue.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hotDiff.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.requireuin { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.totalLike { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.totalView { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.hotValue { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.redValue { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.hotDiff { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

