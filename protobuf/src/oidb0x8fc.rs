// Automatically generated rust module for 'oidb0x8fc.proto' file

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
pub struct D8FCReqBody<'a> {
    pub groupCode: Option<i64>,
    pub showFlag: Option<i32>,
    pub memLevelInfo: Vec<D8FCMemberInfo<'a>>,
    pub levelName: Vec<D8FCLevelName<'a>>,
    pub updateTime: Option<i32>,
    pub officeMode: Option<i32>,
    pub groupOpenAppid: Option<i32>,
    pub msgClientInfo: Option<D8FCClientInfo<'a>>,
    pub authKey: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D8FCReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_int64(bytes)?),
                Ok(16) => msg.showFlag = Some(r.read_int32(bytes)?),
                Ok(26) => msg.memLevelInfo.push(r.read_message::<D8FCMemberInfo>(bytes)?),
                Ok(34) => msg.levelName.push(r.read_message::<D8FCLevelName>(bytes)?),
                Ok(40) => msg.updateTime = Some(r.read_int32(bytes)?),
                Ok(48) => msg.officeMode = Some(r.read_int32(bytes)?),
                Ok(56) => msg.groupOpenAppid = Some(r.read_int32(bytes)?),
                Ok(66) => msg.msgClientInfo = Some(r.read_message::<D8FCClientInfo>(bytes)?),
                Ok(74) => msg.authKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8FCReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.showFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.memLevelInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.levelName.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.updateTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.officeMode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupOpenAppid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgClientInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.authKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.showFlag { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        for s in &self.memLevelInfo { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.levelName { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.updateTime { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.officeMode { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupOpenAppid { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgClientInfo { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.authKey { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8FCMemberInfo<'a> {
    pub uin: Option<i64>,
    pub point: Option<i32>,
    pub activeDay: Option<i32>,
    pub level: Option<i32>,
    pub specialTitle: Option<Cow<'a, [u8]>>,
    pub specialTitleExpireTime: Option<i32>,
    pub uinName: Option<Cow<'a, [u8]>>,
    pub memberCardName: Option<Cow<'a, [u8]>>,
    pub phone: Option<Cow<'a, [u8]>>,
    pub email: Option<Cow<'a, [u8]>>,
    pub remark: Option<Cow<'a, [u8]>>,
    pub gender: Option<i32>,
    pub job: Option<Cow<'a, [u8]>>,
    pub tribeLevel: Option<i32>,
    pub tribePoint: Option<i32>,
    pub richCardName: Vec<D8FCCardNameElem<'a>>,
    pub commRichCardName: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D8FCMemberInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_int64(bytes)?),
                Ok(16) => msg.point = Some(r.read_int32(bytes)?),
                Ok(24) => msg.activeDay = Some(r.read_int32(bytes)?),
                Ok(32) => msg.level = Some(r.read_int32(bytes)?),
                Ok(42) => msg.specialTitle = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.specialTitleExpireTime = Some(r.read_int32(bytes)?),
                Ok(58) => msg.uinName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.memberCardName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.phone = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.email = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.remark = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.gender = Some(r.read_int32(bytes)?),
                Ok(106) => msg.job = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.tribeLevel = Some(r.read_int32(bytes)?),
                Ok(120) => msg.tribePoint = Some(r.read_int32(bytes)?),
                Ok(130) => msg.richCardName.push(r.read_message::<D8FCCardNameElem>(bytes)?),
                Ok(138) => msg.commRichCardName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8FCMemberInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.point.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.activeDay.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.specialTitle.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.specialTitleExpireTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uinName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.memberCardName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.phone.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.email.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.remark.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.gender.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.job.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.tribeLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tribePoint.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.richCardName.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.commRichCardName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.point { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.activeDay { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.level { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.specialTitle { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.specialTitleExpireTime { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.uinName { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.memberCardName { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.phone { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.email { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.remark { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.gender { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.job { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.tribeLevel { w.write_with_tag(112, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.tribePoint { w.write_with_tag(120, |w| w.write_int32(*s))?; }
        for s in &self.richCardName { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.commRichCardName { w.write_with_tag(138, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8FCCardNameElem<'a> {
    pub enumCardType: Option<i32>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D8FCCardNameElem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.enumCardType = Some(r.read_int32(bytes)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8FCCardNameElem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.enumCardType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.enumCardType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8FCLevelName<'a> {
    pub level: Option<i32>,
    pub name: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for D8FCLevelName<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.level = Some(r.read_int32(bytes)?),
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8FCLevelName<'a> {
    fn get_size(&self) -> usize {
        0
        + self.level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.level { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8FCClientInfo<'a> {
    pub implat: Option<i32>,
    pub ingClientver: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for D8FCClientInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.implat = Some(r.read_int32(bytes)?),
                Ok(18) => msg.ingClientver = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8FCClientInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.implat.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ingClientver.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.implat { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.ingClientver { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8FCCommCardNameBuf<'a> {
    pub richCardName: Vec<D8FCRichCardNameElem<'a>>,
}

impl<'a> MessageRead<'a> for D8FCCommCardNameBuf<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.richCardName.push(r.read_message::<D8FCRichCardNameElem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8FCCommCardNameBuf<'a> {
    fn get_size(&self) -> usize {
        0
        + self.richCardName.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.richCardName { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8FCRichCardNameElem<'a> {
    pub ctrl: Option<Cow<'a, [u8]>>,
    pub text: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D8FCRichCardNameElem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ctrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.text = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8FCRichCardNameElem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ctrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ctrl { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.text { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

