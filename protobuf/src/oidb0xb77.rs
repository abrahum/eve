// Automatically generated rust module for 'oidb0xb77.proto' file

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
pub struct DB77ReqBody<'a> {
    pub appId: u64,
    pub appType: u32,
    pub msgStyle: u32,
    pub senderUin: u64,
    pub clientInfo: Option<DB77ClientInfo<'a>>,
    pub textMsg: Cow<'a, str>,
    pub extInfo: Option<DB77ExtInfo<'a>>,
    pub sendType: u32,
    pub recvUin: u64,
    pub richMsgBody: Option<DB77RichMsgBody<'a>>,
}

impl<'a> MessageRead<'a> for DB77ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.appId = r.read_uint64(bytes)?,
                Ok(16) => msg.appType = r.read_uint32(bytes)?,
                Ok(24) => msg.msgStyle = r.read_uint32(bytes)?,
                Ok(32) => msg.senderUin = r.read_uint64(bytes)?,
                Ok(42) => msg.clientInfo = Some(r.read_message::<DB77ClientInfo>(bytes)?),
                Ok(50) => msg.textMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.extInfo = Some(r.read_message::<DB77ExtInfo>(bytes)?),
                Ok(80) => msg.sendType = r.read_uint32(bytes)?,
                Ok(88) => msg.recvUin = r.read_uint64(bytes)?,
                Ok(98) => msg.richMsgBody = Some(r.read_message::<DB77RichMsgBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DB77ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.appId == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.appId) as u64) }
        + if self.appType == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.appType) as u64) }
        + if self.msgStyle == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.msgStyle) as u64) }
        + if self.senderUin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + self.clientInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.textMsg == "" { 0 } else { 1 + sizeof_len((&self.textMsg).len()) }
        + self.extInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.sendType == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.sendType) as u64) }
        + if self.recvUin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.recvUin) as u64) }
        + self.richMsgBody.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.appId != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.appId))?; }
        if self.appType != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.appType))?; }
        if self.msgStyle != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.msgStyle))?; }
        if self.senderUin != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.senderUin))?; }
        if let Some(ref s) = self.clientInfo { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.textMsg != "" { w.write_with_tag(50, |w| w.write_string(&**&self.textMsg))?; }
        if let Some(ref s) = self.extInfo { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.sendType != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.sendType))?; }
        if self.recvUin != 0u64 { w.write_with_tag(88, |w| w.write_uint64(*&self.recvUin))?; }
        if let Some(ref s) = self.richMsgBody { w.write_with_tag(98, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DB77ClientInfo<'a> {
    pub platform: u32,
    pub sdkVersion: Cow<'a, str>,
    pub androidPackageName: Cow<'a, str>,
    pub androidSignature: Cow<'a, str>,
    pub iosBundleId: Cow<'a, str>,
    pub pcSign: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DB77ClientInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.platform = r.read_uint32(bytes)?,
                Ok(18) => msg.sdkVersion = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.androidPackageName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.androidSignature = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.iosBundleId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.pcSign = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DB77ClientInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.platform == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.platform) as u64) }
        + if self.sdkVersion == "" { 0 } else { 1 + sizeof_len((&self.sdkVersion).len()) }
        + if self.androidPackageName == "" { 0 } else { 1 + sizeof_len((&self.androidPackageName).len()) }
        + if self.androidSignature == "" { 0 } else { 1 + sizeof_len((&self.androidSignature).len()) }
        + if self.iosBundleId == "" { 0 } else { 1 + sizeof_len((&self.iosBundleId).len()) }
        + if self.pcSign == "" { 0 } else { 1 + sizeof_len((&self.pcSign).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.platform != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.platform))?; }
        if self.sdkVersion != "" { w.write_with_tag(18, |w| w.write_string(&**&self.sdkVersion))?; }
        if self.androidPackageName != "" { w.write_with_tag(26, |w| w.write_string(&**&self.androidPackageName))?; }
        if self.androidSignature != "" { w.write_with_tag(34, |w| w.write_string(&**&self.androidSignature))?; }
        if self.iosBundleId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.iosBundleId))?; }
        if self.pcSign != "" { w.write_with_tag(50, |w| w.write_string(&**&self.pcSign))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DB77ExtInfo<'a> {
    pub customFeatureId: Vec<u32>,
    pub apnsWording: Cow<'a, str>,
    pub groupSaveDbFlag: u32,
    pub receiverAppId: u32,
    pub msgSeq: u64,
}

impl<'a> MessageRead<'a> for DB77ExtInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(90) => msg.customFeatureId = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(98) => msg.apnsWording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(104) => msg.groupSaveDbFlag = r.read_uint32(bytes)?,
                Ok(112) => msg.receiverAppId = r.read_uint32(bytes)?,
                Ok(120) => msg.msgSeq = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DB77ExtInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.customFeatureId.is_empty() { 0 } else { 1 + sizeof_len(self.customFeatureId.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.apnsWording == "" { 0 } else { 1 + sizeof_len((&self.apnsWording).len()) }
        + if self.groupSaveDbFlag == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.groupSaveDbFlag) as u64) }
        + if self.receiverAppId == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.receiverAppId) as u64) }
        + if self.msgSeq == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.msgSeq) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(90, &self.customFeatureId, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.apnsWording != "" { w.write_with_tag(98, |w| w.write_string(&**&self.apnsWording))?; }
        if self.groupSaveDbFlag != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.groupSaveDbFlag))?; }
        if self.receiverAppId != 0u32 { w.write_with_tag(112, |w| w.write_uint32(*&self.receiverAppId))?; }
        if self.msgSeq != 0u64 { w.write_with_tag(120, |w| w.write_uint64(*&self.msgSeq))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DB77RichMsgBody<'a> {
    pub title: Cow<'a, str>,
    pub summary: Cow<'a, str>,
    pub brief: Cow<'a, str>,
    pub url: Cow<'a, str>,
    pub pictureUrl: Cow<'a, str>,
    pub action: Cow<'a, str>,
    pub musicUrl: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DB77RichMsgBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(82) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.summary = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(98) => msg.brief = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(106) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.pictureUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(122) => msg.action = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(130) => msg.musicUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DB77RichMsgBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.summary == "" { 0 } else { 1 + sizeof_len((&self.summary).len()) }
        + if self.brief == "" { 0 } else { 1 + sizeof_len((&self.brief).len()) }
        + if self.url == "" { 0 } else { 1 + sizeof_len((&self.url).len()) }
        + if self.pictureUrl == "" { 0 } else { 1 + sizeof_len((&self.pictureUrl).len()) }
        + if self.action == "" { 0 } else { 1 + sizeof_len((&self.action).len()) }
        + if self.musicUrl == "" { 0 } else { 2 + sizeof_len((&self.musicUrl).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.title != "" { w.write_with_tag(82, |w| w.write_string(&**&self.title))?; }
        if self.summary != "" { w.write_with_tag(90, |w| w.write_string(&**&self.summary))?; }
        if self.brief != "" { w.write_with_tag(98, |w| w.write_string(&**&self.brief))?; }
        if self.url != "" { w.write_with_tag(106, |w| w.write_string(&**&self.url))?; }
        if self.pictureUrl != "" { w.write_with_tag(114, |w| w.write_string(&**&self.pictureUrl))?; }
        if self.action != "" { w.write_with_tag(122, |w| w.write_string(&**&self.action))?; }
        if self.musicUrl != "" { w.write_with_tag(130, |w| w.write_string(&**&self.musicUrl))?; }
        Ok(())
    }
}

