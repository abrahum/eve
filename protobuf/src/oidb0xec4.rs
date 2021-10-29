// Automatically generated rust module for 'oidb0xec4.proto' file

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
pub struct Comment<'a> {
    pub id: Option<Cow<'a, str>>,
    pub comment: Option<Cow<'a, str>>,
    pub time: Option<u64>,
    pub fromUin: Option<u64>,
    pub toUin: Option<u64>,
    pub replyId: Option<Cow<'a, str>>,
    pub fromNick: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Comment<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.comment = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.time = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.toUin = Some(r.read_uint64(bytes)?),
                Ok(50) => msg.replyId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.fromNick = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Comment<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.comment.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.replyId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fromNick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.comment { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fromUin { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.replyId { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fromNick { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Praise<'a> {
    pub fromUin: Option<u64>,
    pub toUin: Option<u64>,
    pub time: Option<u64>,
    pub fromNick: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Praise<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.toUin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.time = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.fromNick = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Praise<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromNick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fromNick { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Quest<'a> {
    pub id: Option<Cow<'a, str>>,
    pub quest: Option<Cow<'a, str>>,
    pub questUin: Option<u64>,
    pub time: Option<u64>,
    pub ans: Option<Cow<'a, str>>,
    pub ansTime: Option<u64>,
    pub comment: Vec<Comment<'a>>,
    pub praise: Vec<Praise<'a>>,
    pub praiseNum: Option<u64>,
    pub likeKey: Option<Cow<'a, str>>,
    pub systemId: Option<u64>,
    pub commentNum: Option<u64>,
    pub showType: Option<u64>,
    pub showTimes: Option<u64>,
    pub beenPraised: Option<u64>,
    pub questRead: Option<bool>,
    pub ansShowType: Option<u64>,
}

impl<'a> MessageRead<'a> for Quest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.quest = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.questUin = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.time = Some(r.read_uint64(bytes)?),
                Ok(42) => msg.ans = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.ansTime = Some(r.read_uint64(bytes)?),
                Ok(58) => msg.comment.push(r.read_message::<Comment>(bytes)?),
                Ok(66) => msg.praise.push(r.read_message::<Praise>(bytes)?),
                Ok(72) => msg.praiseNum = Some(r.read_uint64(bytes)?),
                Ok(82) => msg.likeKey = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.systemId = Some(r.read_uint64(bytes)?),
                Ok(96) => msg.commentNum = Some(r.read_uint64(bytes)?),
                Ok(104) => msg.showType = Some(r.read_uint64(bytes)?),
                Ok(112) => msg.showTimes = Some(r.read_uint64(bytes)?),
                Ok(120) => msg.beenPraised = Some(r.read_uint64(bytes)?),
                Ok(128) => msg.questRead = Some(r.read_bool(bytes)?),
                Ok(136) => msg.ansShowType = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Quest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.quest.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.questUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ans.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ansTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.comment.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.praise.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.praiseNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.likeKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.systemId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.commentNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.showType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.showTimes.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.beenPraised.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.questRead.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.ansShowType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.quest { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.questUin { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ans { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.ansTime { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        for s in &self.comment { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.praise { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.praiseNum { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.likeKey { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.systemId { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.commentNum { w.write_with_tag(96, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.showType { w.write_with_tag(104, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.showTimes { w.write_with_tag(112, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.beenPraised { w.write_with_tag(120, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.questRead { w.write_with_tag(128, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.ansShowType { w.write_with_tag(136, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DEC4ReqBody<'a> {
    pub uin: Option<u64>,
    pub questNum: Option<u64>,
    pub commentNum: Option<u64>,
    pub cookie: Option<Cow<'a, [u8]>>,
    pub fetchType: Option<u32>,
}

impl<'a> MessageRead<'a> for DEC4ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.questNum = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.commentNum = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.cookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.fetchType = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DEC4ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.questNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.commentNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fetchType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.questNum { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.commentNum { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cookie { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fetchType { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DEC4RspBody<'a> {
    pub quest: Vec<Quest<'a>>,
    pub isFetchOver: Option<bool>,
    pub totalQuestNum: Option<u32>,
    pub cookie: Option<Cow<'a, [u8]>>,
    pub ret: Option<u32>,
    pub answeredQuestNum: Option<u32>,
}

impl<'a> MessageRead<'a> for DEC4RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.quest.push(r.read_message::<Quest>(bytes)?),
                Ok(16) => msg.isFetchOver = Some(r.read_bool(bytes)?),
                Ok(24) => msg.totalQuestNum = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.cookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.ret = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.answeredQuestNum = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DEC4RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.quest.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.isFetchOver.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.totalQuestNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ret.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.answeredQuestNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.quest { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.isFetchOver { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.totalQuestNum { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cookie { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.ret { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.answeredQuestNum { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

