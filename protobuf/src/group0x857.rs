// Automatically generated rust module for 'group0x857.proto' file

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
pub struct NotifyMsgBody<'a> {
    pub optMsgGrayTips: Option<AIOGrayTipsInfo<'a>>,
    pub optMsgRedTips: Option<RedGrayTipsInfo<'a>>,
    pub optMsgRecall: Option<MessageRecallReminder<'a>>,
    pub optGeneralGrayTip: Option<GeneralGrayTipInfo<'a>>,
    pub qqGroupDigestMsg: Option<QQGroupDigestMsg<'a>>,
    pub serviceType: i32,
}

impl<'a> MessageRead<'a> for NotifyMsgBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(42) => msg.optMsgGrayTips = Some(r.read_message::<AIOGrayTipsInfo>(bytes)?),
                Ok(74) => msg.optMsgRedTips = Some(r.read_message::<RedGrayTipsInfo>(bytes)?),
                Ok(90) => msg.optMsgRecall = Some(r.read_message::<MessageRecallReminder>(bytes)?),
                Ok(210) => msg.optGeneralGrayTip = Some(r.read_message::<GeneralGrayTipInfo>(bytes)?),
                Ok(266) => msg.qqGroupDigestMsg = Some(r.read_message::<QQGroupDigestMsg>(bytes)?),
                Ok(104) => msg.serviceType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NotifyMsgBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.optMsgGrayTips.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.optMsgRedTips.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.optMsgRecall.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.optGeneralGrayTip.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.qqGroupDigestMsg.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.serviceType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.serviceType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.optMsgGrayTips { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.optMsgRedTips { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.optMsgRecall { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.optGeneralGrayTip { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.qqGroupDigestMsg { w.write_with_tag(266, |w| w.write_message(s))?; }
        if self.serviceType != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.serviceType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AIOGrayTipsInfo<'a> {
    pub showLatest: u32,
    pub content: Cow<'a, [u8]>,
    pub remind: u32,
    pub brief: Cow<'a, [u8]>,
    pub receiverUin: u64,
    pub reliaoAdminOpt: u32,
}

impl<'a> MessageRead<'a> for AIOGrayTipsInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.showLatest = r.read_uint32(bytes)?,
                Ok(18) => msg.content = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.remind = r.read_uint32(bytes)?,
                Ok(34) => msg.brief = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.receiverUin = r.read_uint64(bytes)?,
                Ok(48) => msg.reliaoAdminOpt = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AIOGrayTipsInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.showLatest == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.showLatest) as u64) }
        + if self.content == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.content).len()) }
        + if self.remind == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.remind) as u64) }
        + if self.brief == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.brief).len()) }
        + if self.receiverUin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.receiverUin) as u64) }
        + if self.reliaoAdminOpt == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.reliaoAdminOpt) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.showLatest != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.showLatest))?; }
        if self.content != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.content))?; }
        if self.remind != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.remind))?; }
        if self.brief != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.brief))?; }
        if self.receiverUin != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.receiverUin))?; }
        if self.reliaoAdminOpt != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.reliaoAdminOpt))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GeneralGrayTipInfo<'a> {
    pub busiType: u64,
    pub busiId: u64,
    pub ctrlFlag: u32,
    pub c2cType: u32,
    pub serviceType: u32,
    pub templId: u64,
    pub msgTemplParam: Vec<TemplParam<'a>>,
    pub content: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for GeneralGrayTipInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.busiType = r.read_uint64(bytes)?,
                Ok(16) => msg.busiId = r.read_uint64(bytes)?,
                Ok(24) => msg.ctrlFlag = r.read_uint32(bytes)?,
                Ok(32) => msg.c2cType = r.read_uint32(bytes)?,
                Ok(40) => msg.serviceType = r.read_uint32(bytes)?,
                Ok(48) => msg.templId = r.read_uint64(bytes)?,
                Ok(58) => msg.msgTemplParam.push(r.read_message::<TemplParam>(bytes)?),
                Ok(66) => msg.content = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GeneralGrayTipInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.busiType == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.busiType) as u64) }
        + if self.busiId == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.busiId) as u64) }
        + if self.ctrlFlag == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.ctrlFlag) as u64) }
        + if self.c2cType == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.c2cType) as u64) }
        + if self.serviceType == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.serviceType) as u64) }
        + if self.templId == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.templId) as u64) }
        + self.msgTemplParam.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.content == "" { 0 } else { 1 + sizeof_len((&self.content).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.busiType != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.busiType))?; }
        if self.busiId != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.busiId))?; }
        if self.ctrlFlag != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.ctrlFlag))?; }
        if self.c2cType != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.c2cType))?; }
        if self.serviceType != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.serviceType))?; }
        if self.templId != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.templId))?; }
        for s in &self.msgTemplParam { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.content != "" { w.write_with_tag(66, |w| w.write_string(&**&self.content))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TemplParam<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TemplParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TemplParam<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.value != "" { w.write_with_tag(18, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageRecallReminder<'a> {
    pub uin: i64,
    pub nickname: Cow<'a, [u8]>,
    pub recalledMsgList: Vec<RecalledMessageMeta>,
    pub reminderContent: Cow<'a, [u8]>,
    pub userdef: Cow<'a, [u8]>,
    pub groupType: i32,
    pub opType: i32,
}

impl<'a> MessageRead<'a> for MessageRecallReminder<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = r.read_int64(bytes)?,
                Ok(18) => msg.nickname = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.recalledMsgList.push(r.read_message::<RecalledMessageMeta>(bytes)?),
                Ok(34) => msg.reminderContent = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.userdef = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.groupType = r.read_int32(bytes)?,
                Ok(56) => msg.opType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MessageRecallReminder<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uin) as u64) }
        + if self.nickname == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.nickname).len()) }
        + self.recalledMsgList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.reminderContent == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.reminderContent).len()) }
        + if self.userdef == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.userdef).len()) }
        + if self.groupType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupType) as u64) }
        + if self.opType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.opType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.uin))?; }
        if self.nickname != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.nickname))?; }
        for s in &self.recalledMsgList { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.reminderContent != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.reminderContent))?; }
        if self.userdef != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.userdef))?; }
        if self.groupType != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.groupType))?; }
        if self.opType != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.opType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RecalledMessageMeta {
    pub seq: i32,
    pub time: i32,
    pub msgRandom: i32,
    pub msgType: i32,
    pub msgFlag: i32,
    pub authorUin: i64,
}

impl<'a> MessageRead<'a> for RecalledMessageMeta {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.seq = r.read_int32(bytes)?,
                Ok(16) => msg.time = r.read_int32(bytes)?,
                Ok(24) => msg.msgRandom = r.read_int32(bytes)?,
                Ok(32) => msg.msgType = r.read_int32(bytes)?,
                Ok(40) => msg.msgFlag = r.read_int32(bytes)?,
                Ok(48) => msg.authorUin = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RecalledMessageMeta {
    fn get_size(&self) -> usize {
        0
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.time) as u64) }
        + if self.msgRandom == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgRandom) as u64) }
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.msgFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgFlag) as u64) }
        + if self.authorUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.authorUin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.seq != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.seq))?; }
        if self.time != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.time))?; }
        if self.msgRandom != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.msgRandom))?; }
        if self.msgType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.msgType))?; }
        if self.msgFlag != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.msgFlag))?; }
        if self.authorUin != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.authorUin))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RedGrayTipsInfo<'a> {
    pub showLatest: u32,
    pub senderUin: u64,
    pub receiverUin: u64,
    pub senderRichContent: Cow<'a, str>,
    pub receiverRichContent: Cow<'a, str>,
    pub authKey: Cow<'a, [u8]>,
    pub msgType: i32,
    pub luckyFlag: u32,
    pub hideFlag: u32,
    pub luckyUin: u64,
}

impl<'a> MessageRead<'a> for RedGrayTipsInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.showLatest = r.read_uint32(bytes)?,
                Ok(16) => msg.senderUin = r.read_uint64(bytes)?,
                Ok(24) => msg.receiverUin = r.read_uint64(bytes)?,
                Ok(34) => msg.senderRichContent = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.receiverRichContent = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.authKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.msgType = r.read_sint32(bytes)?,
                Ok(64) => msg.luckyFlag = r.read_uint32(bytes)?,
                Ok(72) => msg.hideFlag = r.read_uint32(bytes)?,
                Ok(96) => msg.luckyUin = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RedGrayTipsInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.showLatest == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.showLatest) as u64) }
        + if self.senderUin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.senderUin) as u64) }
        + if self.receiverUin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.receiverUin) as u64) }
        + if self.senderRichContent == "" { 0 } else { 1 + sizeof_len((&self.senderRichContent).len()) }
        + if self.receiverRichContent == "" { 0 } else { 1 + sizeof_len((&self.receiverRichContent).len()) }
        + if self.authKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.authKey).len()) }
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_sint32(*(&self.msgType)) }
        + if self.luckyFlag == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.luckyFlag) as u64) }
        + if self.hideFlag == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.hideFlag) as u64) }
        + if self.luckyUin == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.luckyUin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.showLatest != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.showLatest))?; }
        if self.senderUin != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.senderUin))?; }
        if self.receiverUin != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.receiverUin))?; }
        if self.senderRichContent != "" { w.write_with_tag(34, |w| w.write_string(&**&self.senderRichContent))?; }
        if self.receiverRichContent != "" { w.write_with_tag(42, |w| w.write_string(&**&self.receiverRichContent))?; }
        if self.authKey != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.authKey))?; }
        if self.msgType != 0i32 { w.write_with_tag(56, |w| w.write_sint32(*&self.msgType))?; }
        if self.luckyFlag != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.luckyFlag))?; }
        if self.hideFlag != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.hideFlag))?; }
        if self.luckyUin != 0u64 { w.write_with_tag(96, |w| w.write_uint64(*&self.luckyUin))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQGroupDigestMsg<'a> {
    pub groupCode: u64,
    pub seq: u32,
    pub random: u32,
    pub opType: i32,
    pub sender: u64,
    pub digestOper: u64,
    pub opTime: u32,
    pub lastestMsgSeq: u32,
    pub operNick: Cow<'a, [u8]>,
    pub senderNick: Cow<'a, [u8]>,
    pub extInfo: i32,
}

impl<'a> MessageRead<'a> for QQGroupDigestMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_uint64(bytes)?,
                Ok(16) => msg.seq = r.read_uint32(bytes)?,
                Ok(24) => msg.random = r.read_uint32(bytes)?,
                Ok(32) => msg.opType = r.read_int32(bytes)?,
                Ok(40) => msg.sender = r.read_uint64(bytes)?,
                Ok(48) => msg.digestOper = r.read_uint64(bytes)?,
                Ok(56) => msg.opTime = r.read_uint32(bytes)?,
                Ok(64) => msg.lastestMsgSeq = r.read_uint32(bytes)?,
                Ok(74) => msg.operNick = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.senderNick = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(88) => msg.extInfo = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QQGroupDigestMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.seq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.random == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.random) as u64) }
        + if self.opType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.opType) as u64) }
        + if self.sender == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.sender) as u64) }
        + if self.digestOper == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.digestOper) as u64) }
        + if self.opTime == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.opTime) as u64) }
        + if self.lastestMsgSeq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.lastestMsgSeq) as u64) }
        + if self.operNick == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.operNick).len()) }
        + if self.senderNick == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.senderNick).len()) }
        + if self.extInfo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.extInfo) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.groupCode))?; }
        if self.seq != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.seq))?; }
        if self.random != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.random))?; }
        if self.opType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.opType))?; }
        if self.sender != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.sender))?; }
        if self.digestOper != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.digestOper))?; }
        if self.opTime != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.opTime))?; }
        if self.lastestMsgSeq != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.lastestMsgSeq))?; }
        if self.operNick != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.operNick))?; }
        if self.senderNick != Cow::Borrowed(b"") { w.write_with_tag(82, |w| w.write_bytes(&**&self.senderNick))?; }
        if self.extInfo != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.extInfo))?; }
        Ok(())
    }
}

