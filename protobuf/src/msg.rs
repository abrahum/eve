// Automatically generated rust module for 'msg.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyncFlag {
    START = 0,
    CONTINUME = 1,
    STOP = 2,
}

impl Default for SyncFlag {
    fn default() -> Self {
        SyncFlag::START
    }
}

impl From<i32> for SyncFlag {
    fn from(i: i32) -> Self {
        match i {
            0 => SyncFlag::START,
            1 => SyncFlag::CONTINUME,
            2 => SyncFlag::STOP,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SyncFlag {
    fn from(s: &'a str) -> Self {
        match s {
            "START" => SyncFlag::START,
            "CONTINUME" => SyncFlag::CONTINUME,
            "STOP" => SyncFlag::STOP,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetMessageRequest<'a> {
    pub syncFlag: Option<SyncFlag>,
    pub syncCookie: Option<Cow<'a, [u8]>>,
    pub rambleFlag: Option<i32>,
    pub latestRambleNumber: Option<i32>,
    pub otherRambleNumber: Option<i32>,
    pub onlineSyncFlag: Option<i32>,
    pub contextFlag: Option<i32>,
    pub whisperSessionId: Option<i32>,
    pub msgReqType: Option<i32>,
    pub pubaccountCookie: Option<Cow<'a, [u8]>>,
    pub msgCtrlBuf: Option<Cow<'a, [u8]>>,
    pub serverBuf: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GetMessageRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.syncFlag = Some(r.read_enum(bytes)?),
                Ok(18) => msg.syncCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.rambleFlag = Some(r.read_int32(bytes)?),
                Ok(32) => msg.latestRambleNumber = Some(r.read_int32(bytes)?),
                Ok(40) => msg.otherRambleNumber = Some(r.read_int32(bytes)?),
                Ok(48) => msg.onlineSyncFlag = Some(r.read_int32(bytes)?),
                Ok(56) => msg.contextFlag = Some(r.read_int32(bytes)?),
                Ok(64) => msg.whisperSessionId = Some(r.read_int32(bytes)?),
                Ok(72) => msg.msgReqType = Some(r.read_int32(bytes)?),
                Ok(82) => msg.pubaccountCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.msgCtrlBuf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.serverBuf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetMessageRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.syncFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.syncCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.rambleFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.latestRambleNumber.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.otherRambleNumber.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.onlineSyncFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.contextFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.whisperSessionId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgReqType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pubaccountCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.msgCtrlBuf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.serverBuf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.syncFlag { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.syncCookie { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.rambleFlag { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.latestRambleNumber { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.otherRambleNumber { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.onlineSyncFlag { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.contextFlag { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.whisperSessionId { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgReqType { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pubaccountCookie { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.msgCtrlBuf { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.serverBuf { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SendMessageRequest<'a> {
    pub routingHead: Option<RoutingHead<'a>>,
    pub contentHead: Option<ContentHead>,
    pub msgBody: Option<MessageBody<'a>>,
    pub msgSeq: Option<i32>,
    pub msgRand: Option<i32>,
    pub syncCookie: Option<Cow<'a, [u8]>>,
    pub msgVia: Option<i32>,
    pub dataStatist: Option<i32>,
    pub msgCtrl: Option<MsgCtrl>,
    pub multiSendSeq: Option<i32>,
}

impl<'a> MessageRead<'a> for SendMessageRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.routingHead = Some(r.read_message::<RoutingHead>(bytes)?),
                Ok(18) => msg.contentHead = Some(r.read_message::<ContentHead>(bytes)?),
                Ok(26) => msg.msgBody = Some(r.read_message::<MessageBody>(bytes)?),
                Ok(32) => msg.msgSeq = Some(r.read_int32(bytes)?),
                Ok(40) => msg.msgRand = Some(r.read_int32(bytes)?),
                Ok(50) => msg.syncCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.msgVia = Some(r.read_int32(bytes)?),
                Ok(72) => msg.dataStatist = Some(r.read_int32(bytes)?),
                Ok(98) => msg.msgCtrl = Some(r.read_message::<MsgCtrl>(bytes)?),
                Ok(112) => msg.multiSendSeq = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SendMessageRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.routingHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.contentHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgBody.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgRand.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.syncCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.msgVia.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.dataStatist.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgCtrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.multiSendSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.routingHead { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.contentHead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgBody { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgSeq { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgRand { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.syncCookie { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.msgVia { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.dataStatist { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgCtrl { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.multiSendSeq { w.write_with_tag(112, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SendMessageResponse<'a> {
    pub result: Option<i32>,
    pub errMsg: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for SendMessageResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_int32(bytes)?),
                Ok(18) => msg.errMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SendMessageResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.errMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgWithDrawReq<'a> {
    pub c2cWithDraw: Vec<C2CMsgWithDrawReq<'a>>,
    pub groupWithDraw: Vec<GroupMsgWithDrawReq<'a>>,
}

impl<'a> MessageRead<'a> for MsgWithDrawReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.c2cWithDraw.push(r.read_message::<C2CMsgWithDrawReq>(bytes)?),
                Ok(18) => msg.groupWithDraw.push(r.read_message::<GroupMsgWithDrawReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MsgWithDrawReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.c2cWithDraw.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupWithDraw.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.c2cWithDraw { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.groupWithDraw { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C2CMsgWithDrawReq<'a> {
    pub msgInfo: Vec<C2CMsgInfo<'a>>,
    pub longMessageFlag: Option<i32>,
    pub reserved: Option<Cow<'a, [u8]>>,
    pub subCmd: Option<i32>,
}

impl<'a> MessageRead<'a> for C2CMsgWithDrawReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msgInfo.push(r.read_message::<C2CMsgInfo>(bytes)?),
                Ok(16) => msg.longMessageFlag = Some(r.read_int32(bytes)?),
                Ok(26) => msg.reserved = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.subCmd = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C2CMsgWithDrawReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msgInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.longMessageFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reserved.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.msgInfo { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.longMessageFlag { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reserved { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.subCmd { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMsgWithDrawReq<'a> {
    pub subCmd: Option<i32>,
    pub groupType: Option<i32>,
    pub groupCode: Option<i64>,
    pub msgList: Vec<GroupMsgInfo>,
    pub userDef: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GroupMsgWithDrawReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subCmd = Some(r.read_int32(bytes)?),
                Ok(16) => msg.groupType = Some(r.read_int32(bytes)?),
                Ok(24) => msg.groupCode = Some(r.read_int64(bytes)?),
                Ok(34) => msg.msgList.push(r.read_message::<GroupMsgInfo>(bytes)?),
                Ok(42) => msg.userDef = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupMsgWithDrawReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.userDef.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subCmd { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupType { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        for s in &self.msgList { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.userDef { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgWithDrawResp<'a> {
    pub c2cWithDraw: Vec<C2CMsgWithDrawResp<'a>>,
    pub groupWithDraw: Vec<GroupMsgWithDrawResp<'a>>,
}

impl<'a> MessageRead<'a> for MsgWithDrawResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.c2cWithDraw.push(r.read_message::<C2CMsgWithDrawResp>(bytes)?),
                Ok(18) => msg.groupWithDraw.push(r.read_message::<GroupMsgWithDrawResp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MsgWithDrawResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.c2cWithDraw.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupWithDraw.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.c2cWithDraw { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.groupWithDraw { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C2CMsgWithDrawResp<'a> {
    pub result: Option<i32>,
    pub errMsg: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for C2CMsgWithDrawResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_int32(bytes)?),
                Ok(18) => msg.errMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C2CMsgWithDrawResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.errMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMsgWithDrawResp<'a> {
    pub result: Option<i32>,
    pub errMsg: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for GroupMsgWithDrawResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_int32(bytes)?),
                Ok(18) => msg.errMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupMsgWithDrawResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.errMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMsgInfo {
    pub msgSeq: Option<i32>,
    pub msgRandom: Option<i32>,
    pub msgType: Option<i32>,
}

impl<'a> MessageRead<'a> for GroupMsgInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgSeq = Some(r.read_int32(bytes)?),
                Ok(16) => msg.msgRandom = Some(r.read_int32(bytes)?),
                Ok(24) => msg.msgType = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupMsgInfo {
    fn get_size(&self) -> usize {
        0
        + self.msgSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgRandom.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msgSeq { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgRandom { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgType { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C2CMsgInfo<'a> {
    pub fromUin: Option<i64>,
    pub toUin: Option<i64>,
    pub msgSeq: Option<i32>,
    pub msgUid: Option<i64>,
    pub msgTime: Option<i64>,
    pub msgRandom: Option<i32>,
    pub pkgNum: Option<i32>,
    pub pkgIndex: Option<i32>,
    pub divSeq: Option<i32>,
    pub msgType: Option<i32>,
    pub routingHead: Option<RoutingHead<'a>>,
}

impl<'a> MessageRead<'a> for C2CMsgInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_int64(bytes)?),
                Ok(16) => msg.toUin = Some(r.read_int64(bytes)?),
                Ok(24) => msg.msgSeq = Some(r.read_int32(bytes)?),
                Ok(32) => msg.msgUid = Some(r.read_int64(bytes)?),
                Ok(40) => msg.msgTime = Some(r.read_int64(bytes)?),
                Ok(48) => msg.msgRandom = Some(r.read_int32(bytes)?),
                Ok(56) => msg.pkgNum = Some(r.read_int32(bytes)?),
                Ok(64) => msg.pkgIndex = Some(r.read_int32(bytes)?),
                Ok(72) => msg.divSeq = Some(r.read_int32(bytes)?),
                Ok(80) => msg.msgType = Some(r.read_int32(bytes)?),
                Ok(162) => msg.routingHead = Some(r.read_message::<RoutingHead>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C2CMsgInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgUid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgRandom.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pkgNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pkgIndex.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.divSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.routingHead.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgSeq { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgUid { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgTime { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgRandom { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pkgNum { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pkgIndex { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.divSeq { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgType { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.routingHead { w.write_with_tag(162, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoutingHead<'a> {
    pub c2c: Option<C2C>,
    pub grp: Option<Grp>,
    pub grpTmp: Option<GrpTmp>,
    pub wpaTmp: Option<WPATmp<'a>>,
}

impl<'a> MessageRead<'a> for RoutingHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.c2c = Some(r.read_message::<C2C>(bytes)?),
                Ok(18) => msg.grp = Some(r.read_message::<Grp>(bytes)?),
                Ok(26) => msg.grpTmp = Some(r.read_message::<GrpTmp>(bytes)?),
                Ok(50) => msg.wpaTmp = Some(r.read_message::<WPATmp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RoutingHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.c2c.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.grp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.grpTmp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.wpaTmp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.c2c { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.grp { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.grpTmp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.wpaTmp { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WPATmp<'a> {
    pub toUin: Option<u64>,
    pub sig: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for WPATmp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.toUin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for WPATmp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.toUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C2C {
    pub toUin: Option<i64>,
}

impl<'a> MessageRead<'a> for C2C {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.toUin = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for C2C {
    fn get_size(&self) -> usize {
        0
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.toUin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Grp {
    pub groupCode: Option<i64>,
}

impl<'a> MessageRead<'a> for Grp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Grp {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GrpTmp {
    pub groupUin: Option<i64>,
    pub toUin: Option<i64>,
}

impl<'a> MessageRead<'a> for GrpTmp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupUin = Some(r.read_int64(bytes)?),
                Ok(16) => msg.toUin = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GrpTmp {
    fn get_size(&self) -> usize {
        0
        + self.groupUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupUin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgCtrl {
    pub msgFlag: Option<i32>,
}

impl<'a> MessageRead<'a> for MsgCtrl {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgFlag = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MsgCtrl {
    fn get_size(&self) -> usize {
        0
        + self.msgFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msgFlag { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetMessageResponse<'a> {
    pub result: Option<i32>,
    pub errorMessage: Option<Cow<'a, str>>,
    pub syncCookie: Option<Cow<'a, [u8]>>,
    pub syncFlag: Option<SyncFlag>,
    pub uinPairMsgs: Vec<UinPairMessage<'a>>,
    pub bindUin: Option<i64>,
    pub msgRspType: Option<i32>,
    pub pubAccountCookie: Option<Cow<'a, [u8]>>,
    pub isPartialSync: Option<bool>,
    pub msgCtrlBuf: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GetMessageResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_int32(bytes)?),
                Ok(18) => msg.errorMessage = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.syncCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.syncFlag = Some(r.read_enum(bytes)?),
                Ok(42) => msg.uinPairMsgs.push(r.read_message::<UinPairMessage>(bytes)?),
                Ok(48) => msg.bindUin = Some(r.read_int64(bytes)?),
                Ok(56) => msg.msgRspType = Some(r.read_int32(bytes)?),
                Ok(66) => msg.pubAccountCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.isPartialSync = Some(r.read_bool(bytes)?),
                Ok(82) => msg.msgCtrlBuf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetMessageResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errorMessage.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.syncCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.syncFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uinPairMsgs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.bindUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgRspType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pubAccountCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.isPartialSync.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgCtrlBuf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.errorMessage { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.syncCookie { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.syncFlag { w.write_with_tag(32, |w| w.write_enum(*s as i32))?; }
        for s in &self.uinPairMsgs { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.bindUin { w.write_with_tag(48, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgRspType { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pubAccountCookie { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.isPartialSync { w.write_with_tag(72, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.msgCtrlBuf { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PushMessagePacket<'a> {
    pub message: Option<Message<'a>>,
    pub svrip: Option<i32>,
    pub pushToken: Option<Cow<'a, [u8]>>,
    pub pingFLag: Option<i32>,
    pub generalFlag: Option<i32>,
}

impl<'a> MessageRead<'a> for PushMessagePacket<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.message = Some(r.read_message::<Message>(bytes)?),
                Ok(16) => msg.svrip = Some(r.read_int32(bytes)?),
                Ok(26) => msg.pushToken = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.pingFLag = Some(r.read_int32(bytes)?),
                Ok(72) => msg.generalFlag = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PushMessagePacket<'a> {
    fn get_size(&self) -> usize {
        0
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.svrip.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pushToken.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pingFLag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.generalFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.message { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.svrip { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pushToken { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pingFLag { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.generalFlag { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UinPairMessage<'a> {
    pub lastReadTime: Option<i32>,
    pub peerUin: Option<i64>,
    pub msgCompleted: Option<i32>,
    pub messages: Vec<Message<'a>>,
}

impl<'a> MessageRead<'a> for UinPairMessage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.lastReadTime = Some(r.read_int32(bytes)?),
                Ok(16) => msg.peerUin = Some(r.read_int64(bytes)?),
                Ok(24) => msg.msgCompleted = Some(r.read_int32(bytes)?),
                Ok(34) => msg.messages.push(r.read_message::<Message>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UinPairMessage<'a> {
    fn get_size(&self) -> usize {
        0
        + self.lastReadTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.peerUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgCompleted.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.messages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.lastReadTime { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.peerUin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgCompleted { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        for s in &self.messages { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Message<'a> {
    pub head: Option<MessageHead<'a>>,
    pub content: Option<ContentHead>,
    pub body: Option<MessageBody<'a>>,
}

impl<'a> MessageRead<'a> for Message<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.head = Some(r.read_message::<MessageHead>(bytes)?),
                Ok(18) => msg.content = Some(r.read_message::<ContentHead>(bytes)?),
                Ok(26) => msg.body = Some(r.read_message::<MessageBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Message<'a> {
    fn get_size(&self) -> usize {
        0
        + self.head.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.body.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.head { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.content { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.body { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageBody<'a> {
    pub richText: Option<RichText<'a>>,
    pub msgContent: Option<Cow<'a, [u8]>>,
    pub msgEncryptContent: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for MessageBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.richText = Some(r.read_message::<RichText>(bytes)?),
                Ok(18) => msg.msgContent = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.msgEncryptContent = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MessageBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.richText.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgContent.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.msgEncryptContent.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.richText { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgContent { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.msgEncryptContent { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RichText<'a> {
    pub attr: Option<Attr<'a>>,
    pub elems: Vec<Elem<'a>>,
    pub notOnlineFile: Option<NotOnlineFile<'a>>,
    pub ptt: Option<Ptt<'a>>,
}

impl<'a> MessageRead<'a> for RichText<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.attr = Some(r.read_message::<Attr>(bytes)?),
                Ok(18) => msg.elems.push(r.read_message::<Elem>(bytes)?),
                Ok(26) => msg.notOnlineFile = Some(r.read_message::<NotOnlineFile>(bytes)?),
                Ok(34) => msg.ptt = Some(r.read_message::<Ptt>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RichText<'a> {
    fn get_size(&self) -> usize {
        0
        + self.attr.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.elems.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.notOnlineFile.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ptt.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.attr { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.elems { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.notOnlineFile { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ptt { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Elem<'a> {
    pub text: Option<Text<'a>>,
    pub face: Option<Face<'a>>,
    pub onlineImage: Option<OnlineImage<'a>>,
    pub notOnlineImage: Option<NotOnlineImage<'a>>,
    pub transElemInfo: Option<TransElem<'a>>,
    pub customFace: Option<CustomFace<'a>>,
    pub richMsg: Option<RichMsg<'a>>,
    pub groupFile: Option<GroupFile<'a>>,
    pub extraInfo: Option<ExtraInfo<'a>>,
    pub videoFile: Option<VideoFile<'a>>,
    pub anonGroupMsg: Option<AnonymousGroupMessage<'a>>,
    pub QQWalletMsg: Option<QQWalletMsg<'a>>,
    pub customElem: Option<CustomElem<'a>>,
    pub generalFlags: Option<GeneralFlags<'a>>,
    pub srcMsg: Option<SourceMsg<'a>>,
    pub lightApp: Option<LightAppElem<'a>>,
    pub commonElem: Option<CommonElem<'a>>,
}

impl<'a> MessageRead<'a> for Elem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = Some(r.read_message::<Text>(bytes)?),
                Ok(18) => msg.face = Some(r.read_message::<Face>(bytes)?),
                Ok(26) => msg.onlineImage = Some(r.read_message::<OnlineImage>(bytes)?),
                Ok(34) => msg.notOnlineImage = Some(r.read_message::<NotOnlineImage>(bytes)?),
                Ok(42) => msg.transElemInfo = Some(r.read_message::<TransElem>(bytes)?),
                Ok(66) => msg.customFace = Some(r.read_message::<CustomFace>(bytes)?),
                Ok(98) => msg.richMsg = Some(r.read_message::<RichMsg>(bytes)?),
                Ok(106) => msg.groupFile = Some(r.read_message::<GroupFile>(bytes)?),
                Ok(130) => msg.extraInfo = Some(r.read_message::<ExtraInfo>(bytes)?),
                Ok(154) => msg.videoFile = Some(r.read_message::<VideoFile>(bytes)?),
                Ok(170) => msg.anonGroupMsg = Some(r.read_message::<AnonymousGroupMessage>(bytes)?),
                Ok(194) => msg.QQWalletMsg = Some(r.read_message::<QQWalletMsg>(bytes)?),
                Ok(250) => msg.customElem = Some(r.read_message::<CustomElem>(bytes)?),
                Ok(298) => msg.generalFlags = Some(r.read_message::<GeneralFlags>(bytes)?),
                Ok(362) => msg.srcMsg = Some(r.read_message::<SourceMsg>(bytes)?),
                Ok(410) => msg.lightApp = Some(r.read_message::<LightAppElem>(bytes)?),
                Ok(426) => msg.commonElem = Some(r.read_message::<CommonElem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Elem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.face.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.onlineImage.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.notOnlineImage.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.transElemInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.customFace.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.richMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupFile.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.extraInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.videoFile.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.anonGroupMsg.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.QQWalletMsg.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.customElem.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.generalFlags.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.srcMsg.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.lightApp.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.commonElem.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.text { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.face { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.onlineImage { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.notOnlineImage { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.transElemInfo { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.customFace { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.richMsg { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupFile { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.extraInfo { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.videoFile { w.write_with_tag(154, |w| w.write_message(s))?; }
        if let Some(ref s) = self.anonGroupMsg { w.write_with_tag(170, |w| w.write_message(s))?; }
        if let Some(ref s) = self.QQWalletMsg { w.write_with_tag(194, |w| w.write_message(s))?; }
        if let Some(ref s) = self.customElem { w.write_with_tag(250, |w| w.write_message(s))?; }
        if let Some(ref s) = self.generalFlags { w.write_with_tag(298, |w| w.write_message(s))?; }
        if let Some(ref s) = self.srcMsg { w.write_with_tag(362, |w| w.write_message(s))?; }
        if let Some(ref s) = self.lightApp { w.write_with_tag(410, |w| w.write_message(s))?; }
        if let Some(ref s) = self.commonElem { w.write_with_tag(426, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommonElem<'a> {
    pub serviceType: Option<i32>,
    pub pbElem: Option<Cow<'a, [u8]>>,
    pub businessType: Option<i32>,
}

impl<'a> MessageRead<'a> for CommonElem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.serviceType = Some(r.read_int32(bytes)?),
                Ok(18) => msg.pbElem = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.businessType = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CommonElem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.serviceType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pbElem.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.businessType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.serviceType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pbElem { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.businessType { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQWalletMsg<'a> {
    pub aioBody: Option<QQWalletAioBody<'a>>,
}

impl<'a> MessageRead<'a> for QQWalletMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.aioBody = Some(r.read_message::<QQWalletAioBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QQWalletMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + self.aioBody.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.aioBody { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQWalletAioBody<'a> {
    pub sendUin: Option<u64>,
    pub sender: Option<QQWalletAioElem<'a>>,
    pub receiver: Option<QQWalletAioElem<'a>>,
    pub ChannelId: Option<i32>,
    pub templateId: Option<i32>,
    pub resend: Option<u32>,
    pub msgPriority: Option<u32>,
    pub redType: Option<i32>,
    pub billNo: Option<Cow<'a, [u8]>>,
    pub authKey: Option<Cow<'a, [u8]>>,
    pub sessionType: Option<i32>,
    pub msgType: Option<i32>,
    pub envelOpeId: Option<i32>,
    pub name: Option<Cow<'a, [u8]>>,
    pub confType: Option<i32>,
    pub msgFrom: Option<i32>,
    pub pcBody: Option<Cow<'a, [u8]>>,
    pub index: Option<Cow<'a, [u8]>>,
    pub redChannel: Option<u32>,
    pub grapUin: Vec<u64>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for QQWalletAioBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.sendUin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.sender = Some(r.read_message::<QQWalletAioElem>(bytes)?),
                Ok(26) => msg.receiver = Some(r.read_message::<QQWalletAioElem>(bytes)?),
                Ok(32) => msg.ChannelId = Some(r.read_sint32(bytes)?),
                Ok(40) => msg.templateId = Some(r.read_sint32(bytes)?),
                Ok(48) => msg.resend = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.msgPriority = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.redType = Some(r.read_sint32(bytes)?),
                Ok(74) => msg.billNo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.authKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.sessionType = Some(r.read_sint32(bytes)?),
                Ok(96) => msg.msgType = Some(r.read_sint32(bytes)?),
                Ok(104) => msg.envelOpeId = Some(r.read_sint32(bytes)?),
                Ok(114) => msg.name = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.confType = Some(r.read_sint32(bytes)?),
                Ok(128) => msg.msgFrom = Some(r.read_sint32(bytes)?),
                Ok(138) => msg.pcBody = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(146) => msg.index = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(152) => msg.redChannel = Some(r.read_uint32(bytes)?),
                Ok(160) => msg.grapUin.push(r.read_uint64(bytes)?),
                Ok(170) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QQWalletAioBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.sendUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sender.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.receiver.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ChannelId.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.templateId.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.resend.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgPriority.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.redType.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.billNo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.authKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sessionType.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.envelOpeId.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.confType.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.msgFrom.as_ref().map_or(0, |m| 2 + sizeof_sint32(*(m)))
        + self.pcBody.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.index.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.redChannel.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.grapUin.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.sendUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sender { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.receiver { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ChannelId { w.write_with_tag(32, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.templateId { w.write_with_tag(40, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.resend { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.msgPriority { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.redType { w.write_with_tag(64, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.billNo { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.authKey { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sessionType { w.write_with_tag(88, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.msgType { w.write_with_tag(96, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.envelOpeId { w.write_with_tag(104, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.confType { w.write_with_tag(120, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.msgFrom { w.write_with_tag(128, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.pcBody { w.write_with_tag(138, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.index { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.redChannel { w.write_with_tag(152, |w| w.write_uint32(*s))?; }
        for s in &self.grapUin { w.write_with_tag(160, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(170, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQWalletAioElem<'a> {
    pub background: Option<u32>,
    pub icon: Option<u32>,
    pub title: Option<Cow<'a, str>>,
    pub subtitle: Option<Cow<'a, str>>,
    pub content: Option<Cow<'a, str>>,
    pub linkUrl: Option<Cow<'a, [u8]>>,
    pub blackStripe: Option<Cow<'a, [u8]>>,
    pub notice: Option<Cow<'a, [u8]>>,
    pub titleColor: Option<u32>,
    pub subtitleColor: Option<u32>,
    pub actionsPriority: Option<Cow<'a, [u8]>>,
    pub jumpUrl: Option<Cow<'a, [u8]>>,
    pub nativeIos: Option<Cow<'a, [u8]>>,
    pub nativeAndroid: Option<Cow<'a, [u8]>>,
    pub iconUrl: Option<Cow<'a, [u8]>>,
    pub contentColor: Option<u32>,
    pub contentBgColor: Option<u32>,
    pub aioImageLeft: Option<Cow<'a, [u8]>>,
    pub aioImageRight: Option<Cow<'a, [u8]>>,
    pub cftImage: Option<Cow<'a, [u8]>>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for QQWalletAioElem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.background = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.icon = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.title = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.subtitle = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.content = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.linkUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.blackStripe = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.notice = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.titleColor = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.subtitleColor = Some(r.read_uint32(bytes)?),
                Ok(90) => msg.actionsPriority = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.jumpUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.nativeIos = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(114) => msg.nativeAndroid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.iconUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.contentColor = Some(r.read_uint32(bytes)?),
                Ok(136) => msg.contentBgColor = Some(r.read_uint32(bytes)?),
                Ok(146) => msg.aioImageLeft = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(154) => msg.aioImageRight = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(162) => msg.cftImage = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(170) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QQWalletAioElem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.background.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.icon.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.title.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.subtitle.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.linkUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.blackStripe.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.notice.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.titleColor.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.subtitleColor.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.actionsPriority.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.jumpUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.nativeIos.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.nativeAndroid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.iconUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.contentColor.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.contentBgColor.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.aioImageLeft.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.aioImageRight.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.cftImage.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.background { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.icon { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.title { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.subtitle { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.content { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.linkUrl { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.blackStripe { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.notice { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.titleColor { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.subtitleColor { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.actionsPriority { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.jumpUrl { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.nativeIos { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.nativeAndroid { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.iconUrl { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.contentColor { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.contentBgColor { w.write_with_tag(136, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.aioImageLeft { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.aioImageRight { w.write_with_tag(154, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.cftImage { w.write_with_tag(162, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(170, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RichMsg<'a> {
    pub template1: Option<Cow<'a, [u8]>>,
    pub serviceId: Option<i32>,
    pub msgResId: Option<Cow<'a, [u8]>>,
    pub rand: Option<i32>,
    pub seq: Option<i32>,
}

impl<'a> MessageRead<'a> for RichMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.template1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.serviceId = Some(r.read_int32(bytes)?),
                Ok(26) => msg.msgResId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.rand = Some(r.read_int32(bytes)?),
                Ok(40) => msg.seq = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RichMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + self.template1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.serviceId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgResId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.rand.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.seq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.template1 { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.serviceId { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgResId { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.rand { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.seq { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CustomElem<'a> {
    pub desc: Option<Cow<'a, [u8]>>,
    pub data: Option<Cow<'a, [u8]>>,
    pub enumType: Option<i32>,
    pub ext: Option<Cow<'a, [u8]>>,
    pub sound: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for CustomElem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.desc = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.enumType = Some(r.read_int32(bytes)?),
                Ok(34) => msg.ext = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.sound = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CustomElem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.desc.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.enumType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ext.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sound.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.desc { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.enumType { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.ext { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sound { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub str_pb: Option<Cow<'a, str>>,
    pub link: Option<Cow<'a, str>>,
    pub attr6Buf: Option<Cow<'a, [u8]>>,
    pub attr7Buf: Option<Cow<'a, [u8]>>,
    pub buf: Option<Cow<'a, [u8]>>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.str_pb = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.link = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.attr6Buf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.attr7Buf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.buf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Text<'a> {
    fn get_size(&self) -> usize {
        0
        + self.str_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.link.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.attr6Buf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.attr7Buf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.buf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pbReserve.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.str_pb { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.link { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.attr6Buf { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.attr7Buf { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.buf { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Attr<'a> {
    pub codePage: Option<i32>,
    pub time: Option<i32>,
    pub random: Option<i32>,
    pub color: Option<i32>,
    pub size: Option<i32>,
    pub effect: Option<i32>,
    pub charSet: Option<i32>,
    pub pitchAndFamily: Option<i32>,
    pub fontName: Option<Cow<'a, str>>,
    pub reserveData: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for Attr<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.codePage = Some(r.read_int32(bytes)?),
                Ok(16) => msg.time = Some(r.read_int32(bytes)?),
                Ok(24) => msg.random = Some(r.read_int32(bytes)?),
                Ok(32) => msg.color = Some(r.read_int32(bytes)?),
                Ok(40) => msg.size = Some(r.read_int32(bytes)?),
                Ok(48) => msg.effect = Some(r.read_int32(bytes)?),
                Ok(56) => msg.charSet = Some(r.read_int32(bytes)?),
                Ok(64) => msg.pitchAndFamily = Some(r.read_int32(bytes)?),
                Ok(74) => msg.fontName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.reserveData = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Attr<'a> {
    fn get_size(&self) -> usize {
        0
        + self.codePage.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.random.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.effect.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.charSet.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pitchAndFamily.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fontName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.reserveData.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.codePage { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.random { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.color { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.effect { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.charSet { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pitchAndFamily { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fontName { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.reserveData { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ptt<'a> {
    pub fileType: Option<i32>,
    pub srcUin: Option<i64>,
    pub fileUuid: Option<Cow<'a, [u8]>>,
    pub fileMd5: Option<Cow<'a, [u8]>>,
    pub fileName: Option<Cow<'a, str>>,
    pub fileSize: Option<i32>,
    pub reserve: Option<Cow<'a, [u8]>>,
    pub fileId: Option<i32>,
    pub serverIp: Option<i32>,
    pub serverPort: Option<i32>,
    pub boolValid: Option<bool>,
    pub signature: Option<Cow<'a, [u8]>>,
    pub shortcut: Option<Cow<'a, [u8]>>,
    pub fileKey: Option<Cow<'a, [u8]>>,
    pub magicPttIndex: Option<i32>,
    pub voiceSwitch: Option<i32>,
    pub pttUrl: Option<Cow<'a, [u8]>>,
    pub groupFileKey: Option<Cow<'a, [u8]>>,
    pub time: Option<i32>,
    pub downPara: Option<Cow<'a, [u8]>>,
    pub format: Option<i32>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
    pub bytesPttUrls: Vec<Cow<'a, [u8]>>,
    pub downloadFlag: Option<i32>,
}

impl<'a> MessageRead<'a> for Ptt<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fileType = Some(r.read_int32(bytes)?),
                Ok(16) => msg.srcUin = Some(r.read_int64(bytes)?),
                Ok(26) => msg.fileUuid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.fileMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.fileName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.fileSize = Some(r.read_int32(bytes)?),
                Ok(58) => msg.reserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.fileId = Some(r.read_int32(bytes)?),
                Ok(72) => msg.serverIp = Some(r.read_int32(bytes)?),
                Ok(80) => msg.serverPort = Some(r.read_int32(bytes)?),
                Ok(88) => msg.boolValid = Some(r.read_bool(bytes)?),
                Ok(98) => msg.signature = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.shortcut = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(114) => msg.fileKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.magicPttIndex = Some(r.read_int32(bytes)?),
                Ok(128) => msg.voiceSwitch = Some(r.read_int32(bytes)?),
                Ok(138) => msg.pttUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(146) => msg.groupFileKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(152) => msg.time = Some(r.read_int32(bytes)?),
                Ok(162) => msg.downPara = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(232) => msg.format = Some(r.read_int32(bytes)?),
                Ok(242) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(250) => msg.bytesPttUrls.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(256) => msg.downloadFlag = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Ptt<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.srcUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileUuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reserve.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serverIp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serverPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.boolValid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.signature.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.shortcut.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.magicPttIndex.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.voiceSwitch.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.pttUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupFileKey.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.time.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.downPara.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.format.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.bytesPttUrls.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.downloadFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.srcUin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.fileUuid { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileMd5 { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileName { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fileSize { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reserve { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.serverIp { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.serverPort { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.boolValid { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.signature { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.shortcut { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileKey { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.magicPttIndex { w.write_with_tag(120, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.voiceSwitch { w.write_with_tag(128, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pttUrl { w.write_with_tag(138, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupFileKey { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(152, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.downPara { w.write_with_tag(162, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.format { w.write_with_tag(232, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(242, |w| w.write_bytes(&**s))?; }
        for s in &self.bytesPttUrls { w.write_with_tag(250, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.downloadFlag { w.write_with_tag(256, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OnlineImage<'a> {
    pub guid: Option<Cow<'a, [u8]>>,
    pub filePath: Option<Cow<'a, [u8]>>,
    pub oldVerSendFile: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for OnlineImage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.guid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.filePath = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.oldVerSendFile = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OnlineImage<'a> {
    fn get_size(&self) -> usize {
        0
        + self.guid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.filePath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.oldVerSendFile.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.guid { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.filePath { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.oldVerSendFile { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotOnlineImage<'a> {
    pub filePath: Option<Cow<'a, str>>,
    pub fileLen: Option<i32>,
    pub downloadPath: Option<Cow<'a, str>>,
    pub oldVerSendFile: Option<Cow<'a, [u8]>>,
    pub imgType: Option<i32>,
    pub previewsImage: Option<Cow<'a, [u8]>>,
    pub picMd5: Option<Cow<'a, [u8]>>,
    pub picHeight: Option<i32>,
    pub picWidth: Option<i32>,
    pub resId: Option<Cow<'a, str>>,
    pub flag: Option<Cow<'a, [u8]>>,
    pub thumbUrl: Option<Cow<'a, str>>,
    pub original: Option<i32>,
    pub bigUrl: Option<Cow<'a, str>>,
    pub origUrl: Option<Cow<'a, str>>,
    pub bizType: Option<i32>,
    pub result: Option<i32>,
    pub index: Option<i32>,
    pub opFaceBuf: Option<Cow<'a, [u8]>>,
    pub oldPicMd5: Option<bool>,
    pub thumbWidth: Option<i32>,
    pub thumbHeight: Option<i32>,
    pub fileId: Option<i32>,
    pub showLen: Option<i32>,
    pub downloadLen: Option<i32>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for NotOnlineImage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.filePath = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.fileLen = Some(r.read_int32(bytes)?),
                Ok(26) => msg.downloadPath = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.oldVerSendFile = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.imgType = Some(r.read_int32(bytes)?),
                Ok(50) => msg.previewsImage = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.picMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.picHeight = Some(r.read_int32(bytes)?),
                Ok(72) => msg.picWidth = Some(r.read_int32(bytes)?),
                Ok(82) => msg.resId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.flag = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.thumbUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(104) => msg.original = Some(r.read_int32(bytes)?),
                Ok(114) => msg.bigUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.origUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.bizType = Some(r.read_int32(bytes)?),
                Ok(136) => msg.result = Some(r.read_int32(bytes)?),
                Ok(144) => msg.index = Some(r.read_int32(bytes)?),
                Ok(154) => msg.opFaceBuf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160) => msg.oldPicMd5 = Some(r.read_bool(bytes)?),
                Ok(168) => msg.thumbWidth = Some(r.read_int32(bytes)?),
                Ok(176) => msg.thumbHeight = Some(r.read_int32(bytes)?),
                Ok(184) => msg.fileId = Some(r.read_int32(bytes)?),
                Ok(192) => msg.showLen = Some(r.read_int32(bytes)?),
                Ok(200) => msg.downloadLen = Some(r.read_int32(bytes)?),
                Ok(234) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NotOnlineImage<'a> {
    fn get_size(&self) -> usize {
        0
        + self.filePath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileLen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.downloadPath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.oldVerSendFile.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.imgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.previewsImage.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.picMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.picHeight.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.picWidth.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.resId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.thumbUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.original.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bigUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.origUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.bizType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.index.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.opFaceBuf.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.oldPicMd5.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.thumbWidth.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.thumbHeight.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.fileId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.showLen.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.downloadLen.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.filePath { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.fileLen { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.downloadPath { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.oldVerSendFile { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.imgType { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.previewsImage { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.picMd5 { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.picHeight { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.picWidth { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.resId { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.thumbUrl { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.original { w.write_with_tag(104, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.bigUrl { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.origUrl { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bizType { w.write_with_tag(128, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.index { w.write_with_tag(144, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.opFaceBuf { w.write_with_tag(154, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.oldPicMd5 { w.write_with_tag(160, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.thumbWidth { w.write_with_tag(168, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.thumbHeight { w.write_with_tag(176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(184, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.showLen { w.write_with_tag(192, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.downloadLen { w.write_with_tag(200, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(234, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotOnlineFile<'a> {
    pub fileType: Option<i32>,
    pub sig: Option<Cow<'a, [u8]>>,
    pub fileUuid: Option<Cow<'a, [u8]>>,
    pub fileMd5: Option<Cow<'a, [u8]>>,
    pub fileName: Option<Cow<'a, [u8]>>,
    pub fileSize: Option<i64>,
    pub note: Option<Cow<'a, [u8]>>,
    pub reserved: Option<i32>,
    pub subcmd: Option<i32>,
    pub microCloud: Option<i32>,
    pub bytesFileUrls: Vec<Cow<'a, [u8]>>,
    pub downloadFlag: Option<i32>,
    pub dangerEvel: Option<i32>,
    pub lifeTime: Option<i32>,
    pub uploadTime: Option<i32>,
    pub absFileType: Option<i32>,
    pub clientType: Option<i32>,
    pub expireTime: Option<i32>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for NotOnlineFile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fileType = Some(r.read_int32(bytes)?),
                Ok(18) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.fileUuid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.fileMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.fileName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.fileSize = Some(r.read_int64(bytes)?),
                Ok(58) => msg.note = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.reserved = Some(r.read_int32(bytes)?),
                Ok(72) => msg.subcmd = Some(r.read_int32(bytes)?),
                Ok(80) => msg.microCloud = Some(r.read_int32(bytes)?),
                Ok(90) => msg.bytesFileUrls.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.downloadFlag = Some(r.read_int32(bytes)?),
                Ok(400) => msg.dangerEvel = Some(r.read_int32(bytes)?),
                Ok(408) => msg.lifeTime = Some(r.read_int32(bytes)?),
                Ok(416) => msg.uploadTime = Some(r.read_int32(bytes)?),
                Ok(424) => msg.absFileType = Some(r.read_int32(bytes)?),
                Ok(432) => msg.clientType = Some(r.read_int32(bytes)?),
                Ok(440) => msg.expireTime = Some(r.read_int32(bytes)?),
                Ok(450) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NotOnlineFile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileUuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.note.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.reserved.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.subcmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.microCloud.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bytesFileUrls.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.downloadFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.dangerEvel.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.lifeTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.uploadTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.absFileType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.clientType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.expireTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileUuid { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileMd5 { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileName { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileSize { w.write_with_tag(48, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.note { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.reserved { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.subcmd { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.microCloud { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        for s in &self.bytesFileUrls { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.downloadFlag { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.dangerEvel { w.write_with_tag(400, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.lifeTime { w.write_with_tag(408, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.uploadTime { w.write_with_tag(416, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.absFileType { w.write_with_tag(424, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.clientType { w.write_with_tag(432, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.expireTime { w.write_with_tag(440, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(450, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransElem<'a> {
    pub elemType: Option<i32>,
    pub elemValue: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for TransElem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.elemType = Some(r.read_int32(bytes)?),
                Ok(18) => msg.elemValue = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransElem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.elemType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.elemValue.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.elemType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.elemValue { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExtraInfo<'a> {
    pub nick: Option<Cow<'a, [u8]>>,
    pub groupCard: Option<Cow<'a, [u8]>>,
    pub level: Option<i32>,
    pub flags: Option<i32>,
    pub groupMask: Option<i32>,
    pub msgTailId: Option<i32>,
    pub senderTitle: Option<Cow<'a, [u8]>>,
    pub apnsTips: Option<Cow<'a, [u8]>>,
    pub uin: Option<i64>,
    pub msgStateFlag: Option<i32>,
    pub apnsSoundType: Option<i32>,
    pub newGroupFlag: Option<i32>,
}

impl<'a> MessageRead<'a> for ExtraInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.nick = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.groupCard = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.level = Some(r.read_int32(bytes)?),
                Ok(32) => msg.flags = Some(r.read_int32(bytes)?),
                Ok(40) => msg.groupMask = Some(r.read_int32(bytes)?),
                Ok(48) => msg.msgTailId = Some(r.read_int32(bytes)?),
                Ok(58) => msg.senderTitle = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.apnsTips = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.uin = Some(r.read_int64(bytes)?),
                Ok(80) => msg.msgStateFlag = Some(r.read_int32(bytes)?),
                Ok(88) => msg.apnsSoundType = Some(r.read_int32(bytes)?),
                Ok(96) => msg.newGroupFlag = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ExtraInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.nick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupCard.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flags.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupMask.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgTailId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.senderTitle.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.apnsTips.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgStateFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.apnsSoundType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.newGroupFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.nick { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupCard { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.level { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.flags { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupMask { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgTailId { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.senderTitle { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.apnsTips { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(72, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgStateFlag { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.apnsSoundType { w.write_with_tag(88, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.newGroupFlag { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupFile<'a> {
    pub filename: Option<Cow<'a, [u8]>>,
    pub fileSize: Option<i64>,
    pub fileId: Option<Cow<'a, [u8]>>,
    pub batchId: Option<Cow<'a, [u8]>>,
    pub fileKey: Option<Cow<'a, [u8]>>,
    pub mark: Option<Cow<'a, [u8]>>,
    pub sequence: Option<i64>,
    pub batchItemId: Option<Cow<'a, [u8]>>,
    pub feedMsgTime: Option<i32>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GroupFile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.filename = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.fileSize = Some(r.read_int64(bytes)?),
                Ok(26) => msg.fileId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.batchId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.fileKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.mark = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.sequence = Some(r.read_int64(bytes)?),
                Ok(66) => msg.batchItemId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.feedMsgTime = Some(r.read_int32(bytes)?),
                Ok(82) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupFile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.filename.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.batchId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.mark.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sequence.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.batchItemId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.feedMsgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pbReserve.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.filename { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileSize { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.batchId { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileKey { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.mark { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sequence { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.batchItemId { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.feedMsgTime { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AnonymousGroupMessage<'a> {
    pub flags: Option<i32>,
    pub anonId: Option<Cow<'a, [u8]>>,
    pub anonNick: Option<Cow<'a, [u8]>>,
    pub headPortrait: Option<i32>,
    pub expireTime: Option<i32>,
    pub bubbleId: Option<i32>,
    pub rankColor: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for AnonymousGroupMessage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.flags = Some(r.read_int32(bytes)?),
                Ok(18) => msg.anonId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.anonNick = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.headPortrait = Some(r.read_int32(bytes)?),
                Ok(40) => msg.expireTime = Some(r.read_int32(bytes)?),
                Ok(48) => msg.bubbleId = Some(r.read_int32(bytes)?),
                Ok(58) => msg.rankColor = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AnonymousGroupMessage<'a> {
    fn get_size(&self) -> usize {
        0
        + self.flags.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.anonId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.anonNick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.headPortrait.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.expireTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bubbleId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rankColor.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.flags { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.anonId { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.anonNick { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.headPortrait { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.expireTime { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.bubbleId { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.rankColor { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct VideoFile<'a> {
    pub fileUuid: Option<Cow<'a, [u8]>>,
    pub fileMd5: Option<Cow<'a, [u8]>>,
    pub fileName: Option<Cow<'a, [u8]>>,
    pub fileFormat: Option<i32>,
    pub fileTime: Option<i32>,
    pub fileSize: Option<i32>,
    pub thumbWidth: Option<i32>,
    pub thumbHeight: Option<i32>,
    pub thumbFileMd5: Option<Cow<'a, [u8]>>,
    pub source: Option<Cow<'a, [u8]>>,
    pub thumbFileSize: Option<i32>,
    pub busiType: Option<i32>,
    pub fromChatType: Option<i32>,
    pub toChatType: Option<i32>,
    pub boolSupportProgressive: Option<bool>,
    pub fileWidth: Option<i32>,
    pub fileHeight: Option<i32>,
    pub subBusiType: Option<i32>,
    pub videoAttr: Option<i32>,
    pub bytesThumbFileUrls: Vec<Cow<'a, [u8]>>,
    pub bytesVideoFileUrls: Vec<Cow<'a, [u8]>>,
    pub thumbDownloadFlag: Option<i32>,
    pub videoDownloadFlag: Option<i32>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for VideoFile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileUuid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.fileMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.fileName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.fileFormat = Some(r.read_int32(bytes)?),
                Ok(40) => msg.fileTime = Some(r.read_int32(bytes)?),
                Ok(48) => msg.fileSize = Some(r.read_int32(bytes)?),
                Ok(56) => msg.thumbWidth = Some(r.read_int32(bytes)?),
                Ok(64) => msg.thumbHeight = Some(r.read_int32(bytes)?),
                Ok(74) => msg.thumbFileMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.source = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.thumbFileSize = Some(r.read_int32(bytes)?),
                Ok(96) => msg.busiType = Some(r.read_int32(bytes)?),
                Ok(104) => msg.fromChatType = Some(r.read_int32(bytes)?),
                Ok(112) => msg.toChatType = Some(r.read_int32(bytes)?),
                Ok(120) => msg.boolSupportProgressive = Some(r.read_bool(bytes)?),
                Ok(128) => msg.fileWidth = Some(r.read_int32(bytes)?),
                Ok(136) => msg.fileHeight = Some(r.read_int32(bytes)?),
                Ok(144) => msg.subBusiType = Some(r.read_int32(bytes)?),
                Ok(152) => msg.videoAttr = Some(r.read_int32(bytes)?),
                Ok(162) => msg.bytesThumbFileUrls.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(170) => msg.bytesVideoFileUrls.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(176) => msg.thumbDownloadFlag = Some(r.read_int32(bytes)?),
                Ok(184) => msg.videoDownloadFlag = Some(r.read_int32(bytes)?),
                Ok(194) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for VideoFile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileUuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileFormat.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.thumbWidth.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.thumbHeight.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.thumbFileMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.thumbFileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.busiType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromChatType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toChatType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.boolSupportProgressive.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileWidth.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.fileHeight.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.subBusiType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.videoAttr.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.bytesThumbFileUrls.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.bytesVideoFileUrls.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.thumbDownloadFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.videoDownloadFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileUuid { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileMd5 { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileName { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileFormat { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fileTime { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fileSize { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.thumbWidth { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.thumbHeight { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.thumbFileMd5 { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.thumbFileSize { w.write_with_tag(88, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.busiType { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fromChatType { w.write_with_tag(104, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.toChatType { w.write_with_tag(112, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.boolSupportProgressive { w.write_with_tag(120, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.fileWidth { w.write_with_tag(128, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fileHeight { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.subBusiType { w.write_with_tag(144, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.videoAttr { w.write_with_tag(152, |w| w.write_int32(*s))?; }
        for s in &self.bytesThumbFileUrls { w.write_with_tag(162, |w| w.write_bytes(&**s))?; }
        for s in &self.bytesVideoFileUrls { w.write_with_tag(170, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.thumbDownloadFlag { w.write_with_tag(176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.videoDownloadFlag { w.write_with_tag(184, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(194, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SourceMsg<'a> {
    pub origSeqs: Vec<i32>,
    pub senderUin: Option<i64>,
    pub time: Option<i32>,
    pub flag: Option<i32>,
    pub elems: Vec<Elem<'a>>,
    pub type_pb: Option<i32>,
    pub richMsg: Option<Cow<'a, [u8]>>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
    pub srcMsg: Option<Cow<'a, [u8]>>,
    pub toUin: Option<i64>,
    pub troopName: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for SourceMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.origSeqs.push(r.read_int32(bytes)?),
                Ok(16) => msg.senderUin = Some(r.read_int64(bytes)?),
                Ok(24) => msg.time = Some(r.read_int32(bytes)?),
                Ok(32) => msg.flag = Some(r.read_int32(bytes)?),
                Ok(42) => msg.elems.push(r.read_message::<Elem>(bytes)?),
                Ok(48) => msg.type_pb = Some(r.read_int32(bytes)?),
                Ok(58) => msg.richMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.srcMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(80) => msg.toUin = Some(r.read_int64(bytes)?),
                Ok(90) => msg.troopName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SourceMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + self.origSeqs.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.senderUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.elems.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.richMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pbReserve.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.srcMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.troopName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.origSeqs { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.senderUin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        for s in &self.elems { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.richMsg { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.srcMsg { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(80, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.troopName { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Face<'a> {
    pub index: Option<i32>,
    pub old: Option<Cow<'a, [u8]>>,
    pub buf: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for Face<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = Some(r.read_int32(bytes)?),
                Ok(18) => msg.old = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.buf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Face<'a> {
    fn get_size(&self) -> usize {
        0
        + self.index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.old.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.buf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.index { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.old { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.buf { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LightAppElem<'a> {
    pub data: Option<Cow<'a, [u8]>>,
    pub msgResid: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for LightAppElem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.msgResid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LightAppElem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.msgResid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.data { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.msgResid { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CustomFace<'a> {
    pub guid: Option<Cow<'a, [u8]>>,
    pub filePath: Option<Cow<'a, str>>,
    pub shortcut: Option<Cow<'a, str>>,
    pub buffer: Option<Cow<'a, [u8]>>,
    pub flag: Option<Cow<'a, [u8]>>,
    pub oldData: Option<Cow<'a, [u8]>>,
    pub fileId: Option<i32>,
    pub serverIp: Option<i32>,
    pub serverPort: Option<i32>,
    pub fileType: Option<i32>,
    pub signature: Option<Cow<'a, [u8]>>,
    pub useful: Option<i32>,
    pub md5: Option<Cow<'a, [u8]>>,
    pub thumbUrl: Option<Cow<'a, str>>,
    pub bigUrl: Option<Cow<'a, str>>,
    pub origUrl: Option<Cow<'a, str>>,
    pub bizType: Option<i32>,
    pub repeatIndex: Option<i32>,
    pub repeatImage: Option<i32>,
    pub imageType: Option<i32>,
    pub index: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub source: Option<i32>,
    pub size: Option<i32>,
    pub origin: Option<i32>,
    pub thumbWidth: Option<i32>,
    pub thumbHeight: Option<i32>,
    pub showLen: Option<i32>,
    pub downloadLen: Option<i32>,
    pub _400Url: Option<Cow<'a, str>>,
    pub _400Width: Option<i32>,
    pub _400Height: Option<i32>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for CustomFace<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.guid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.filePath = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.shortcut = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.buffer = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.flag = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.oldData = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.fileId = Some(r.read_int32(bytes)?),
                Ok(64) => msg.serverIp = Some(r.read_int32(bytes)?),
                Ok(72) => msg.serverPort = Some(r.read_int32(bytes)?),
                Ok(80) => msg.fileType = Some(r.read_int32(bytes)?),
                Ok(90) => msg.signature = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.useful = Some(r.read_int32(bytes)?),
                Ok(106) => msg.md5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(114) => msg.thumbUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.bigUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.origUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(136) => msg.bizType = Some(r.read_int32(bytes)?),
                Ok(144) => msg.repeatIndex = Some(r.read_int32(bytes)?),
                Ok(152) => msg.repeatImage = Some(r.read_int32(bytes)?),
                Ok(160) => msg.imageType = Some(r.read_int32(bytes)?),
                Ok(168) => msg.index = Some(r.read_int32(bytes)?),
                Ok(176) => msg.width = Some(r.read_int32(bytes)?),
                Ok(184) => msg.height = Some(r.read_int32(bytes)?),
                Ok(192) => msg.source = Some(r.read_int32(bytes)?),
                Ok(200) => msg.size = Some(r.read_int32(bytes)?),
                Ok(208) => msg.origin = Some(r.read_int32(bytes)?),
                Ok(216) => msg.thumbWidth = Some(r.read_int32(bytes)?),
                Ok(224) => msg.thumbHeight = Some(r.read_int32(bytes)?),
                Ok(232) => msg.showLen = Some(r.read_int32(bytes)?),
                Ok(240) => msg.downloadLen = Some(r.read_int32(bytes)?),
                Ok(250) => msg._400Url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(256) => msg._400Width = Some(r.read_int32(bytes)?),
                Ok(264) => msg._400Height = Some(r.read_int32(bytes)?),
                Ok(274) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CustomFace<'a> {
    fn get_size(&self) -> usize {
        0
        + self.guid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.filePath.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.shortcut.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.buffer.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.oldData.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serverIp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serverPort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.signature.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.useful.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.md5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.thumbUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.bigUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.origUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.bizType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.repeatIndex.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.repeatImage.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.imageType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.index.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.width.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.height.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.source.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.size.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.origin.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.thumbWidth.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.thumbHeight.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.showLen.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.downloadLen.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self._400Url.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self._400Width.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self._400Height.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.guid { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.filePath { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.shortcut { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.buffer { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.oldData { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.serverIp { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.serverPort { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fileType { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.signature { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.useful { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.md5 { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.thumbUrl { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bigUrl { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.origUrl { w.write_with_tag(130, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bizType { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.repeatIndex { w.write_with_tag(144, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.repeatImage { w.write_with_tag(152, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.imageType { w.write_with_tag(160, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.index { w.write_with_tag(168, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.width { w.write_with_tag(176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.height { w.write_with_tag(184, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(192, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(200, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.origin { w.write_with_tag(208, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.thumbWidth { w.write_with_tag(216, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.thumbHeight { w.write_with_tag(224, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.showLen { w.write_with_tag(232, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.downloadLen { w.write_with_tag(240, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self._400Url { w.write_with_tag(250, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self._400Width { w.write_with_tag(256, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self._400Height { w.write_with_tag(264, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(274, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContentHead {
    pub pkgNum: Option<i32>,
    pub pkgIndex: Option<i32>,
    pub divSeq: Option<i32>,
    pub autoReply: Option<i32>,
}

impl<'a> MessageRead<'a> for ContentHead {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.pkgNum = Some(r.read_int32(bytes)?),
                Ok(16) => msg.pkgIndex = Some(r.read_int32(bytes)?),
                Ok(24) => msg.divSeq = Some(r.read_int32(bytes)?),
                Ok(32) => msg.autoReply = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ContentHead {
    fn get_size(&self) -> usize {
        0
        + self.pkgNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pkgIndex.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.divSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.autoReply.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.pkgNum { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pkgIndex { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.divSeq { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.autoReply { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageHead<'a> {
    pub fromUin: Option<i64>,
    pub toUin: Option<i64>,
    pub msgType: Option<i32>,
    pub c2cCmd: Option<i32>,
    pub msgSeq: Option<i32>,
    pub msgTime: Option<i32>,
    pub msgUid: Option<i64>,
    pub c2cTmpMsgHead: Option<C2CTempMessageHead<'a>>,
    pub groupInfo: Option<GroupInfo<'a>>,
    pub fromAppid: Option<i32>,
    pub fromInstid: Option<i32>,
    pub userActive: Option<i32>,
    pub discussInfo: Option<DiscussInfo<'a>>,
    pub fromNick: Option<Cow<'a, str>>,
    pub authUin: Option<i64>,
    pub authNick: Option<Cow<'a, str>>,
    pub msgFlag: Option<i32>,
    pub authRemark: Option<Cow<'a, str>>,
    pub groupName: Option<Cow<'a, str>>,
    pub mutiltransHead: Option<MutilTransHead>,
    pub msgInstCtrl: Option<InstCtrl>,
    pub publicAccountGroupSendFlag: Option<i32>,
    pub wseqInC2cMsghead: Option<i32>,
    pub cpid: Option<i64>,
    pub extGroupKeyInfo: Option<ExtGroupKeyInfo>,
    pub multiCompatibleText: Option<Cow<'a, str>>,
    pub authSex: Option<i32>,
    pub isSrcMsg: Option<bool>,
}

impl<'a> MessageRead<'a> for MessageHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_int64(bytes)?),
                Ok(16) => msg.toUin = Some(r.read_int64(bytes)?),
                Ok(24) => msg.msgType = Some(r.read_int32(bytes)?),
                Ok(32) => msg.c2cCmd = Some(r.read_int32(bytes)?),
                Ok(40) => msg.msgSeq = Some(r.read_int32(bytes)?),
                Ok(48) => msg.msgTime = Some(r.read_int32(bytes)?),
                Ok(56) => msg.msgUid = Some(r.read_int64(bytes)?),
                Ok(66) => msg.c2cTmpMsgHead = Some(r.read_message::<C2CTempMessageHead>(bytes)?),
                Ok(74) => msg.groupInfo = Some(r.read_message::<GroupInfo>(bytes)?),
                Ok(80) => msg.fromAppid = Some(r.read_int32(bytes)?),
                Ok(88) => msg.fromInstid = Some(r.read_int32(bytes)?),
                Ok(96) => msg.userActive = Some(r.read_int32(bytes)?),
                Ok(106) => msg.discussInfo = Some(r.read_message::<DiscussInfo>(bytes)?),
                Ok(114) => msg.fromNick = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.authUin = Some(r.read_int64(bytes)?),
                Ok(130) => msg.authNick = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(136) => msg.msgFlag = Some(r.read_int32(bytes)?),
                Ok(146) => msg.authRemark = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(154) => msg.groupName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(162) => msg.mutiltransHead = Some(r.read_message::<MutilTransHead>(bytes)?),
                Ok(170) => msg.msgInstCtrl = Some(r.read_message::<InstCtrl>(bytes)?),
                Ok(176) => msg.publicAccountGroupSendFlag = Some(r.read_int32(bytes)?),
                Ok(184) => msg.wseqInC2cMsghead = Some(r.read_int32(bytes)?),
                Ok(192) => msg.cpid = Some(r.read_int64(bytes)?),
                Ok(202) => msg.extGroupKeyInfo = Some(r.read_message::<ExtGroupKeyInfo>(bytes)?),
                Ok(210) => msg.multiCompatibleText = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(216) => msg.authSex = Some(r.read_int32(bytes)?),
                Ok(224) => msg.isSrcMsg = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MessageHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.c2cCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgUid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.c2cTmpMsgHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fromAppid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromInstid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.userActive.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.discussInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fromNick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.authUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.authNick.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.msgFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.authRemark.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.mutiltransHead.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.msgInstCtrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.publicAccountGroupSendFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.wseqInC2cMsghead.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cpid.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.extGroupKeyInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.multiCompatibleText.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.authSex.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.isSrcMsg.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgType { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.c2cCmd { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgSeq { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgTime { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgUid { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.c2cTmpMsgHead { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupInfo { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fromAppid { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fromInstid { w.write_with_tag(88, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.userActive { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.discussInfo { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fromNick { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.authUin { w.write_with_tag(120, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.authNick { w.write_with_tag(130, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.msgFlag { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.authRemark { w.write_with_tag(146, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.groupName { w.write_with_tag(154, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.mutiltransHead { w.write_with_tag(162, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgInstCtrl { w.write_with_tag(170, |w| w.write_message(s))?; }
        if let Some(ref s) = self.publicAccountGroupSendFlag { w.write_with_tag(176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.wseqInC2cMsghead { w.write_with_tag(184, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.cpid { w.write_with_tag(192, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.extGroupKeyInfo { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.multiCompatibleText { w.write_with_tag(210, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.authSex { w.write_with_tag(216, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.isSrcMsg { w.write_with_tag(224, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupInfo<'a> {
    pub groupCode: Option<i64>,
    pub groupType: Option<i32>,
    pub groupInfoSeq: Option<i64>,
    pub groupCard: Option<Cow<'a, str>>,
    pub groupRank: Option<Cow<'a, [u8]>>,
    pub groupLevel: Option<i32>,
    pub groupCardType: Option<i32>,
    pub groupName: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GroupInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_int64(bytes)?),
                Ok(16) => msg.groupType = Some(r.read_int32(bytes)?),
                Ok(24) => msg.groupInfoSeq = Some(r.read_int64(bytes)?),
                Ok(34) => msg.groupCard = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.groupRank = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.groupLevel = Some(r.read_int32(bytes)?),
                Ok(56) => msg.groupCardType = Some(r.read_int32(bytes)?),
                Ok(66) => msg.groupName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupInfoSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCard.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupRank.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCardType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.groupType { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupInfoSeq { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.groupCard { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.groupRank { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupLevel { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupCardType { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupName { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiscussInfo<'a> {
    pub discussUin: Option<i64>,
    pub discussType: Option<i32>,
    pub discussInfoSeq: Option<i64>,
    pub discussRemark: Option<Cow<'a, [u8]>>,
    pub discussName: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for DiscussInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.discussUin = Some(r.read_int64(bytes)?),
                Ok(16) => msg.discussType = Some(r.read_int32(bytes)?),
                Ok(24) => msg.discussInfoSeq = Some(r.read_int64(bytes)?),
                Ok(34) => msg.discussRemark = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.discussName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DiscussInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.discussUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.discussType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.discussInfoSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.discussRemark.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.discussName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.discussUin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.discussType { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.discussInfoSeq { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.discussRemark { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.discussName { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MutilTransHead {
    pub status: Option<i32>,
    pub msgId: Option<i32>,
}

impl<'a> MessageRead<'a> for MutilTransHead {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = Some(r.read_int32(bytes)?),
                Ok(16) => msg.msgId = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MutilTransHead {
    fn get_size(&self) -> usize {
        0
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.status { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgId { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C2CTempMessageHead<'a> {
    pub c2cType: Option<i32>,
    pub serviceType: Option<i32>,
    pub groupUin: Option<i64>,
    pub groupCode: Option<i64>,
    pub sig: Option<Cow<'a, [u8]>>,
    pub sigType: Option<i32>,
    pub fromPhone: Option<Cow<'a, str>>,
    pub toPhone: Option<Cow<'a, str>>,
    pub lockDisplay: Option<i32>,
    pub directionFlag: Option<i32>,
    pub reserved: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for C2CTempMessageHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.c2cType = Some(r.read_int32(bytes)?),
                Ok(16) => msg.serviceType = Some(r.read_int32(bytes)?),
                Ok(24) => msg.groupUin = Some(r.read_int64(bytes)?),
                Ok(32) => msg.groupCode = Some(r.read_int64(bytes)?),
                Ok(42) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.sigType = Some(r.read_int32(bytes)?),
                Ok(58) => msg.fromPhone = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.toPhone = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.lockDisplay = Some(r.read_int32(bytes)?),
                Ok(80) => msg.directionFlag = Some(r.read_int32(bytes)?),
                Ok(90) => msg.reserved = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C2CTempMessageHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.c2cType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.serviceType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sigType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromPhone.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.toPhone.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.lockDisplay.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.directionFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reserved.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.c2cType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.serviceType { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupUin { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sigType { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fromPhone { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.toPhone { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.lockDisplay { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.directionFlag { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reserved { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct InstCtrl {
    pub msgSendToInst: Vec<InstInfo>,
    pub msgExcludeInst: Vec<InstInfo>,
    pub msgFromInst: Option<InstInfo>,
}

impl<'a> MessageRead<'a> for InstCtrl {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msgSendToInst.push(r.read_message::<InstInfo>(bytes)?),
                Ok(18) => msg.msgExcludeInst.push(r.read_message::<InstInfo>(bytes)?),
                Ok(26) => msg.msgFromInst = Some(r.read_message::<InstInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for InstCtrl {
    fn get_size(&self) -> usize {
        0
        + self.msgSendToInst.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgExcludeInst.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgFromInst.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.msgSendToInst { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.msgExcludeInst { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgFromInst { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct InstInfo {
    pub apppid: Option<i32>,
    pub instid: Option<i32>,
    pub platform: Option<i32>,
    pub enumDeviceType: Option<i32>,
}

impl<'a> MessageRead<'a> for InstInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.apppid = Some(r.read_int32(bytes)?),
                Ok(16) => msg.instid = Some(r.read_int32(bytes)?),
                Ok(24) => msg.platform = Some(r.read_int32(bytes)?),
                Ok(80) => msg.enumDeviceType = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for InstInfo {
    fn get_size(&self) -> usize {
        0
        + self.apppid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.instid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.platform.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.enumDeviceType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.apppid { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.instid { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.platform { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.enumDeviceType { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExtGroupKeyInfo {
    pub curMaxSeq: Option<i32>,
    pub curTime: Option<i64>,
}

impl<'a> MessageRead<'a> for ExtGroupKeyInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.curMaxSeq = Some(r.read_int32(bytes)?),
                Ok(16) => msg.curTime = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ExtGroupKeyInfo {
    fn get_size(&self) -> usize {
        0
        + self.curMaxSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.curTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.curMaxSeq { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.curTime { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SyncCookie {
    pub time1: Option<i64>,
    pub time: Option<i64>,
    pub ran1: Option<i64>,
    pub ran2: Option<i64>,
    pub const1: Option<i64>,
    pub const2: Option<i64>,
    pub const3: Option<i64>,
    pub lastSyncTime: Option<i64>,
    pub const4: Option<i64>,
}

impl<'a> MessageRead<'a> for SyncCookie {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.time1 = Some(r.read_int64(bytes)?),
                Ok(16) => msg.time = Some(r.read_int64(bytes)?),
                Ok(24) => msg.ran1 = Some(r.read_int64(bytes)?),
                Ok(32) => msg.ran2 = Some(r.read_int64(bytes)?),
                Ok(40) => msg.const1 = Some(r.read_int64(bytes)?),
                Ok(88) => msg.const2 = Some(r.read_int64(bytes)?),
                Ok(96) => msg.const3 = Some(r.read_int64(bytes)?),
                Ok(104) => msg.lastSyncTime = Some(r.read_int64(bytes)?),
                Ok(112) => msg.const4 = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SyncCookie {
    fn get_size(&self) -> usize {
        0
        + self.time1.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ran1.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ran2.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.const1.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.const2.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.const3.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastSyncTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.const4.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.time1 { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ran1 { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ran2 { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.const1 { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.const2 { w.write_with_tag(88, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.const3 { w.write_with_tag(96, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.lastSyncTime { w.write_with_tag(104, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.const4 { w.write_with_tag(112, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransMsgInfo<'a> {
    pub fromUin: Option<i64>,
    pub toUin: Option<i64>,
    pub msgType: Option<i32>,
    pub msgSubtype: Option<i32>,
    pub msgSeq: Option<i32>,
    pub msgUid: Option<i64>,
    pub msgTime: Option<i32>,
    pub realMsgTime: Option<i32>,
    pub nickName: Option<Cow<'a, str>>,
    pub msgData: Option<Cow<'a, [u8]>>,
    pub svrIp: Option<i32>,
    pub extGroupKeyInfo: Option<ExtGroupKeyInfo>,
    pub generalFlag: Option<i32>,
}

impl<'a> MessageRead<'a> for TransMsgInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_int64(bytes)?),
                Ok(16) => msg.toUin = Some(r.read_int64(bytes)?),
                Ok(24) => msg.msgType = Some(r.read_int32(bytes)?),
                Ok(32) => msg.msgSubtype = Some(r.read_int32(bytes)?),
                Ok(40) => msg.msgSeq = Some(r.read_int32(bytes)?),
                Ok(48) => msg.msgUid = Some(r.read_int64(bytes)?),
                Ok(56) => msg.msgTime = Some(r.read_int32(bytes)?),
                Ok(64) => msg.realMsgTime = Some(r.read_int32(bytes)?),
                Ok(74) => msg.nickName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.msgData = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.svrIp = Some(r.read_int32(bytes)?),
                Ok(98) => msg.extGroupKeyInfo = Some(r.read_message::<ExtGroupKeyInfo>(bytes)?),
                Ok(136) => msg.generalFlag = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransMsgInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgSubtype.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgUid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.realMsgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.nickName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.msgData.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.svrIp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.extGroupKeyInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.generalFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.toUin { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgType { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgSubtype { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgSeq { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msgUid { w.write_with_tag(48, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.msgTime { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.realMsgTime { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.nickName { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.msgData { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.svrIp { w.write_with_tag(88, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.extGroupKeyInfo { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.generalFlag { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GeneralFlags<'a> {
    pub bubbleDiyTextId: Option<i32>,
    pub groupFlagNew: Option<i32>,
    pub uin: Option<i64>,
    pub rpId: Option<Cow<'a, [u8]>>,
    pub prpFold: Option<i32>,
    pub longTextFlag: Option<i32>,
    pub longTextResid: Option<Cow<'a, str>>,
    pub groupType: Option<i32>,
    pub toUinFlag: Option<i32>,
    pub glamourLevel: Option<i32>,
    pub memberLevel: Option<i32>,
    pub groupRankSeq: Option<i64>,
    pub olympicTorch: Option<i32>,
    pub babyqGuideMsgCookie: Option<Cow<'a, [u8]>>,
    pub uin32ExpertFlag: Option<i32>,
    pub bubbleSubId: Option<i32>,
    pub pendantId: Option<i64>,
    pub rpIndex: Option<Cow<'a, [u8]>>,
    pub pbReserve: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GeneralFlags<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.bubbleDiyTextId = Some(r.read_int32(bytes)?),
                Ok(16) => msg.groupFlagNew = Some(r.read_int32(bytes)?),
                Ok(24) => msg.uin = Some(r.read_int64(bytes)?),
                Ok(34) => msg.rpId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.prpFold = Some(r.read_int32(bytes)?),
                Ok(48) => msg.longTextFlag = Some(r.read_int32(bytes)?),
                Ok(58) => msg.longTextResid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.groupType = Some(r.read_int32(bytes)?),
                Ok(72) => msg.toUinFlag = Some(r.read_int32(bytes)?),
                Ok(80) => msg.glamourLevel = Some(r.read_int32(bytes)?),
                Ok(88) => msg.memberLevel = Some(r.read_int32(bytes)?),
                Ok(96) => msg.groupRankSeq = Some(r.read_int64(bytes)?),
                Ok(104) => msg.olympicTorch = Some(r.read_int32(bytes)?),
                Ok(114) => msg.babyqGuideMsgCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(120) => msg.uin32ExpertFlag = Some(r.read_int32(bytes)?),
                Ok(128) => msg.bubbleSubId = Some(r.read_int32(bytes)?),
                Ok(136) => msg.pendantId = Some(r.read_int64(bytes)?),
                Ok(146) => msg.rpIndex = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(154) => msg.pbReserve = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GeneralFlags<'a> {
    fn get_size(&self) -> usize {
        0
        + self.bubbleDiyTextId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupFlagNew.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rpId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.prpFold.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.longTextFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.longTextResid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toUinFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.glamourLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.memberLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupRankSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.olympicTorch.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.babyqGuideMsgCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.uin32ExpertFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bubbleSubId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.pendantId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.rpIndex.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.pbReserve.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.bubbleDiyTextId { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupFlagNew { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.rpId { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.prpFold { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.longTextFlag { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.longTextResid { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.groupType { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.toUinFlag { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.glamourLevel { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.memberLevel { w.write_with_tag(88, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupRankSeq { w.write_with_tag(96, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.olympicTorch { w.write_with_tag(104, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.babyqGuideMsgCookie { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.uin32ExpertFlag { w.write_with_tag(120, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.bubbleSubId { w.write_with_tag(128, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pendantId { w.write_with_tag(136, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.rpIndex { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pbReserve { w.write_with_tag(154, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbMultiMsgItem<'a> {
    pub fileName: Option<Cow<'a, str>>,
    pub buffer: Option<PbMultiMsgNew<'a>>,
}

impl<'a> MessageRead<'a> for PbMultiMsgItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.buffer = Some(r.read_message::<PbMultiMsgNew>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbMultiMsgItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.buffer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileName { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.buffer { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbMultiMsgNew<'a> {
    pub msg: Vec<Message<'a>>,
}

impl<'a> MessageRead<'a> for PbMultiMsgNew<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg.push(r.read_message::<Message>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbMultiMsgNew<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msg.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.msg { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbMultiMsgTransmit<'a> {
    pub msg: Vec<Message<'a>>,
    pub pbItemList: Vec<PbMultiMsgItem<'a>>,
}

impl<'a> MessageRead<'a> for PbMultiMsgTransmit<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg.push(r.read_message::<Message>(bytes)?),
                Ok(18) => msg.pbItemList.push(r.read_message::<PbMultiMsgItem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbMultiMsgTransmit<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msg.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.pbItemList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.msg { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.pbItemList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgElemInfo_servtype3<'a> {
    pub flash_troop_pic: Option<CustomFace<'a>>,
    pub flash_c2c_pic: Option<NotOnlineImage<'a>>,
}

impl<'a> MessageRead<'a> for MsgElemInfo_servtype3<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.flash_troop_pic = Some(r.read_message::<CustomFace>(bytes)?),
                Ok(18) => msg.flash_c2c_pic = Some(r.read_message::<NotOnlineImage>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MsgElemInfo_servtype3<'a> {
    fn get_size(&self) -> usize {
        0
        + self.flash_troop_pic.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.flash_c2c_pic.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.flash_troop_pic { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.flash_c2c_pic { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgElemInfo_servtype33<'a> {
    pub index: Option<u32>,
    pub text: Option<Cow<'a, [u8]>>,
    pub compat: Option<Cow<'a, [u8]>>,
    pub buf: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for MsgElemInfo_servtype33<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.text = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.compat = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.buf = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MsgElemInfo_servtype33<'a> {
    fn get_size(&self) -> usize {
        0
        + self.index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.compat.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.buf.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.index { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.text { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.compat { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.buf { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SubMsgType0x4Body<'a> {
    pub notOnlineFile: Option<NotOnlineFile<'a>>,
    pub msgTime: Option<u32>,
    pub onlineFileForPolyToOffline: Option<u32>,
}

impl<'a> MessageRead<'a> for SubMsgType0x4Body<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.notOnlineFile = Some(r.read_message::<NotOnlineFile>(bytes)?),
                Ok(16) => msg.msgTime = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.onlineFileForPolyToOffline = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SubMsgType0x4Body<'a> {
    fn get_size(&self) -> usize {
        0
        + self.notOnlineFile.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.onlineFileForPolyToOffline.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.notOnlineFile { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgTime { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.onlineFileForPolyToOffline { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResvAttr<'a> {
    pub imageBizType: Option<u32>,
    pub image_show: Option<AnimationImageShow<'a>>,
}

impl<'a> MessageRead<'a> for ResvAttr<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.imageBizType = Some(r.read_uint32(bytes)?),
                Ok(58) => msg.image_show = Some(r.read_message::<AnimationImageShow>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResvAttr<'a> {
    fn get_size(&self) -> usize {
        0
        + self.imageBizType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.image_show.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.imageBizType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.image_show { w.write_with_tag(58, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AnimationImageShow<'a> {
    pub effect_id: Option<i32>,
    pub animation_param: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for AnimationImageShow<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.effect_id = Some(r.read_int32(bytes)?),
                Ok(18) => msg.animation_param = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AnimationImageShow<'a> {
    fn get_size(&self) -> usize {
        0
        + self.effect_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.animation_param.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.effect_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.animation_param { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UinTypeUserDef<'a> {
    pub fromUinType: Option<i32>,
    pub fromGroupCode: Option<i64>,
    pub fileUuid: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for UinTypeUserDef<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUinType = Some(r.read_int32(bytes)?),
                Ok(16) => msg.fromGroupCode = Some(r.read_int64(bytes)?),
                Ok(26) => msg.fileUuid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UinTypeUserDef<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUinType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromGroupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fileUuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUinType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.fromGroupCode { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.fileUuid { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetGroupMsgReq {
    pub groupCode: Option<u64>,
    pub beginSeq: Option<u64>,
    pub endSeq: Option<u64>,
    pub filter: Option<u32>,
    pub memberSeq: Option<u64>,
    pub publicGroup: Option<bool>,
    pub shieldFlag: Option<u32>,
    pub saveTrafficFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for GetGroupMsgReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.beginSeq = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.endSeq = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.filter = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.memberSeq = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.publicGroup = Some(r.read_bool(bytes)?),
                Ok(56) => msg.shieldFlag = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.saveTrafficFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetGroupMsgReq {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.beginSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.endSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.filter.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.memberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.publicGroup.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.shieldFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.saveTrafficFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.beginSeq { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.endSeq { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.filter { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.memberSeq { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.publicGroup { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.shieldFlag { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.saveTrafficFlag { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetGroupMsgResp<'a> {
    pub result: Option<u32>,
    pub errmsg: Option<Cow<'a, str>>,
    pub groupCode: Option<u64>,
    pub returnBeginSeq: Option<u64>,
    pub returnEndSeq: Option<u64>,
    pub msg: Vec<Message<'a>>,
}

impl<'a> MessageRead<'a> for GetGroupMsgResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.errmsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.returnBeginSeq = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.returnEndSeq = Some(r.read_uint64(bytes)?),
                Ok(50) => msg.msg.push(r.read_message::<Message>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetGroupMsgResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errmsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.returnBeginSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.returnEndSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msg.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errmsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.returnBeginSeq { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.returnEndSeq { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        for s in &self.msg { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbGetOneDayRoamMsgReq {
    pub peerUin: Option<u64>,
    pub lastMsgTime: Option<u64>,
    pub random: Option<u64>,
    pub readCnt: Option<u32>,
}

impl<'a> MessageRead<'a> for PbGetOneDayRoamMsgReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.peerUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.lastMsgTime = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.random = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.readCnt = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PbGetOneDayRoamMsgReq {
    fn get_size(&self) -> usize {
        0
        + self.peerUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastMsgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.random.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.readCnt.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.peerUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.lastMsgTime { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.random { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.readCnt { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbGetOneDayRoamMsgResp<'a> {
    pub result: Option<u32>,
    pub errMsg: Option<Cow<'a, str>>,
    pub peerUin: Option<u64>,
    pub lastMsgTime: Option<u64>,
    pub random: Option<u64>,
    pub msg: Vec<Message<'a>>,
    pub isComplete: Option<u32>,
}

impl<'a> MessageRead<'a> for PbGetOneDayRoamMsgResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.errMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.peerUin = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.lastMsgTime = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.random = Some(r.read_uint64(bytes)?),
                Ok(50) => msg.msg.push(r.read_message::<Message>(bytes)?),
                Ok(56) => msg.isComplete = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbGetOneDayRoamMsgResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.peerUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastMsgTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.random.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msg.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.isComplete.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.peerUin { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.lastMsgTime { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.random { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        for s in &self.msg { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.isComplete { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbPushMsg<'a> {
    pub msg: Option<Message<'a>>,
    pub svrip: Option<i32>,
    pub pushToken: Option<Cow<'a, [u8]>>,
    pub pingFlag: Option<u32>,
    pub generalFlag: Option<u32>,
    pub bindUin: Option<u64>,
}

impl<'a> MessageRead<'a> for PbPushMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = Some(r.read_message::<Message>(bytes)?),
                Ok(16) => msg.svrip = Some(r.read_int32(bytes)?),
                Ok(26) => msg.pushToken = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.pingFlag = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.generalFlag = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.bindUin = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbPushMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msg.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.svrip.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pushToken.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pingFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.generalFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bindUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msg { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.svrip { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pushToken { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pingFlag { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.generalFlag { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.bindUin { w.write_with_tag(80, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

