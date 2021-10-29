// Automatically generated rust module for 'oidb0xe5b.proto' file

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
pub struct LifeAchievementItem<'a> {
    pub achievementId: Option<u32>,
    pub achievementTitle: Option<Cow<'a, str>>,
    pub achievementIcon: Option<Cow<'a, str>>,
    pub hasPraised: Option<bool>,
    pub praiseNum: Option<u32>,
    pub achievementContent: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for LifeAchievementItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.achievementId = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.achievementTitle = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.achievementIcon = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.hasPraised = Some(r.read_bool(bytes)?),
                Ok(40) => msg.praiseNum = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.achievementContent = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LifeAchievementItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.achievementId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.achievementTitle.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.achievementIcon.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hasPraised.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.praiseNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.achievementContent.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.achievementId { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.achievementTitle { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.achievementIcon { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hasPraised { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.praiseNum { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.achievementContent { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DE5BReqBody {
    pub uin: Option<u64>,
    pub achievementId: Vec<u32>,
    pub maxCount: Option<u32>,
    pub reqAchievementContent: Option<bool>,
}

impl<'a> MessageRead<'a> for DE5BReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.achievementId.push(r.read_uint32(bytes)?),
                Ok(24) => msg.maxCount = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.reqAchievementContent = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DE5BReqBody {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.achievementId.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.maxCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reqAchievementContent.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        for s in &self.achievementId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.maxCount { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.reqAchievementContent { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DE5BRspBody<'a> {
    pub achievementTotalCount: Option<u32>,
    pub lifeAchItem: Vec<LifeAchievementItem<'a>>,
    pub achievementOpenid: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for DE5BRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.achievementTotalCount = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.lifeAchItem.push(r.read_message::<LifeAchievementItem>(bytes)?),
                Ok(26) => msg.achievementOpenid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DE5BRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.achievementTotalCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lifeAchItem.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.achievementOpenid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.achievementTotalCount { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.lifeAchItem { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.achievementOpenid { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

