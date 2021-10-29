// Automatically generated rust module for 'register_proxy.proto' file

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
pub struct DiscussList {
    pub discussCode: Option<u64>,
    pub discussSeq: Option<u64>,
    pub memberSeq: Option<u64>,
    pub infoSeq: Option<u64>,
    pub bHotGroup: Option<bool>,
    pub redpackTime: Option<u64>,
    pub hasMsg: Option<bool>,
    pub dicussFlag: Option<i64>,
}

impl<'a> MessageRead<'a> for DiscussList {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.discussCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.discussSeq = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.memberSeq = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.infoSeq = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.bHotGroup = Some(r.read_bool(bytes)?),
                Ok(48) => msg.redpackTime = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.hasMsg = Some(r.read_bool(bytes)?),
                Ok(64) => msg.dicussFlag = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DiscussList {
    fn get_size(&self) -> usize {
        0
        + self.discussCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.discussSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.memberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.infoSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bHotGroup.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.redpackTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hasMsg.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.dicussFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.discussCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.discussSeq { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.memberSeq { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.infoSeq { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.bHotGroup { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.redpackTime { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.hasMsg { w.write_with_tag(56, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.dicussFlag { w.write_with_tag(64, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupList {
    pub groupCode: Option<u64>,
    pub groupSeq: Option<u64>,
    pub memberSeq: Option<u64>,
    pub mask: Option<u64>,
    pub redpackTime: Option<u64>,
    pub hasMsg: Option<bool>,
    pub groupFlag: Option<i64>,
    pub groupType: Option<u64>,
    pub groupNameSeq: Option<u32>,
    pub groupMemberSeq: Option<u32>,
    pub uinFlagEx2: Option<u32>,
    pub importantMsgLatestSeq: Option<u32>,
}

impl<'a> MessageRead<'a> for GroupList {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.groupSeq = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.memberSeq = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.mask = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.redpackTime = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.hasMsg = Some(r.read_bool(bytes)?),
                Ok(56) => msg.groupFlag = Some(r.read_int64(bytes)?),
                Ok(64) => msg.groupType = Some(r.read_uint64(bytes)?),
                Ok(72) => msg.groupNameSeq = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.groupMemberSeq = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.uinFlagEx2 = Some(r.read_uint32(bytes)?),
                Ok(96) => msg.importantMsgLatestSeq = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupList {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.memberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mask.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.redpackTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hasMsg.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupNameSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupMemberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uinFlagEx2.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.importantMsgLatestSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupSeq { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.memberSeq { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mask { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.redpackTime { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.hasMsg { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.groupFlag { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.groupType { w.write_with_tag(64, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupNameSeq { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupMemberSeq { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uinFlagEx2 { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.importantMsgLatestSeq { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SvcPbResponsePullDisMsgProxy<'a> {
    pub memberSeq: Option<u64>,
    pub content: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for SvcPbResponsePullDisMsgProxy<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.memberSeq = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.content = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SvcPbResponsePullDisMsgProxy<'a> {
    fn get_size(&self) -> usize {
        0
        + self.memberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.memberSeq { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.content { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SvcRegisterProxyMsgResp<'a> {
    pub result: Option<u32>,
    pub errMsg: Option<Cow<'a, [u8]>>,
    pub flag: Option<u32>,
    pub seq: Option<u32>,
    pub info: Option<SvcResponseMsgInfo>,
    pub groupList: Vec<GroupList>,
    pub discussList: Vec<DiscussList>,
    pub groupMsg: Vec<SvcResponsePbPullGroupMsgProxy<'a>>,
    pub discussMsg: Vec<SvcPbResponsePullDisMsgProxy<'a>>,
    pub c2CMsg: Option<Cow<'a, [u8]>>,
    pub pubAccountMsg: Option<Cow<'a, [u8]>>,
    pub discussListFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for SvcRegisterProxyMsgResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.errMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.flag = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.seq = Some(r.read_uint32(bytes)?),
                Ok(42) => msg.info = Some(r.read_message::<SvcResponseMsgInfo>(bytes)?),
                Ok(50) => msg.groupList.push(r.read_message::<GroupList>(bytes)?),
                Ok(58) => msg.discussList.push(r.read_message::<DiscussList>(bytes)?),
                Ok(66) => msg.groupMsg.push(r.read_message::<SvcResponsePbPullGroupMsgProxy>(bytes)?),
                Ok(74) => msg.discussMsg.push(r.read_message::<SvcPbResponsePullDisMsgProxy>(bytes)?),
                Ok(82) => msg.c2CMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.pubAccountMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.discussListFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SvcRegisterProxyMsgResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.seq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.discussList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupMsg.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.discussMsg.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.c2CMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pubAccountMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.discussListFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errMsg { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.seq { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.info { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.groupList { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.discussList { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.groupMsg { w.write_with_tag(66, |w| w.write_message(s))?; }
        for s in &self.discussMsg { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.c2CMsg { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pubAccountMsg { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.discussListFlag { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SvcResponseMsgInfo {
    pub groupNum: Option<u32>,
    pub discussNum: Option<u32>,
}

impl<'a> MessageRead<'a> for SvcResponseMsgInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupNum = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.discussNum = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SvcResponseMsgInfo {
    fn get_size(&self) -> usize {
        0
        + self.groupNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.discussNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupNum { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.discussNum { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SvcResponsePbPullGroupMsgProxy<'a> {
    pub memberSeq: Option<u64>,
    pub content: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for SvcResponsePbPullGroupMsgProxy<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.memberSeq = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.content = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SvcResponsePbPullGroupMsgProxy<'a> {
    fn get_size(&self) -> usize {
        0
        + self.memberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.memberSeq { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.content { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

