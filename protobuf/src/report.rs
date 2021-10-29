// Automatically generated rust module for 'report.proto' file

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
pub struct PbMsgReadedReportReq<'a> {
    pub grpReadReport: Vec<PbGroupReadedReportReq>,
    pub disReadReport: Vec<PbDiscussReadedReportReq>,
    pub c2CReadReport: Option<PbC2CReadedReportReq<'a>>,
}

impl<'a> MessageRead<'a> for PbMsgReadedReportReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.grpReadReport.push(r.read_message::<PbGroupReadedReportReq>(bytes)?),
                Ok(18) => msg.disReadReport.push(r.read_message::<PbDiscussReadedReportReq>(bytes)?),
                Ok(26) => msg.c2CReadReport = Some(r.read_message::<PbC2CReadedReportReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbMsgReadedReportReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.grpReadReport.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.disReadReport.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.c2CReadReport.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.grpReadReport { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.disReadReport { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.c2CReadReport { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbMsgReadedReportResp<'a> {
    pub grpReadReport: Vec<PbGroupReadedReportResp<'a>>,
    pub disReadReport: Vec<PbDiscussReadedReportResp<'a>>,
    pub c2CReadReport: Option<PbC2CReadedReportResp<'a>>,
}

impl<'a> MessageRead<'a> for PbMsgReadedReportResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.grpReadReport.push(r.read_message::<PbGroupReadedReportResp>(bytes)?),
                Ok(18) => msg.disReadReport.push(r.read_message::<PbDiscussReadedReportResp>(bytes)?),
                Ok(26) => msg.c2CReadReport = Some(r.read_message::<PbC2CReadedReportResp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbMsgReadedReportResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.grpReadReport.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.disReadReport.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.c2CReadReport.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.grpReadReport { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.disReadReport { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.c2CReadReport { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbGroupReadedReportReq {
    pub groupCode: Option<u64>,
    pub lastReadSeq: Option<u64>,
}

impl<'a> MessageRead<'a> for PbGroupReadedReportReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.lastReadSeq = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PbGroupReadedReportReq {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastReadSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.lastReadSeq { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbDiscussReadedReportReq {
    pub confUin: Option<u64>,
    pub lastReadSeq: Option<u64>,
}

impl<'a> MessageRead<'a> for PbDiscussReadedReportReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.confUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.lastReadSeq = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PbDiscussReadedReportReq {
    fn get_size(&self) -> usize {
        0
        + self.confUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastReadSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.confUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.lastReadSeq { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbC2CReadedReportReq<'a> {
    pub syncCookie: Option<Cow<'a, [u8]>>,
    pub pairInfo: Vec<UinPairReadInfo<'a>>,
}

impl<'a> MessageRead<'a> for PbC2CReadedReportReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.syncCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.pairInfo.push(r.read_message::<UinPairReadInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbC2CReadedReportReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.syncCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pairInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.syncCookie { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        for s in &self.pairInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UinPairReadInfo<'a> {
    pub peerUin: Option<u64>,
    pub lastReadTime: Option<u32>,
    pub crmSig: Option<Cow<'a, [u8]>>,
    pub peerType: Option<u32>,
    pub chatType: Option<u32>,
    pub cpid: Option<u64>,
    pub aioType: Option<u32>,
    pub toTinyId: Option<u64>,
}

impl<'a> MessageRead<'a> for UinPairReadInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.peerUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.lastReadTime = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.crmSig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.peerType = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.chatType = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.cpid = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.aioType = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.toTinyId = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UinPairReadInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.peerUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastReadTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.crmSig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.peerType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chatType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cpid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.aioType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.toTinyId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.peerUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.lastReadTime { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.crmSig { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.peerType { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.chatType { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cpid { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.aioType { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.toTinyId { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbGroupReadedReportResp<'a> {
    pub result: Option<u32>,
    pub errmsg: Option<Cow<'a, str>>,
    pub groupCode: Option<u64>,
    pub memberSeq: Option<u64>,
    pub groupMsgSeq: Option<u64>,
}

impl<'a> MessageRead<'a> for PbGroupReadedReportResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.errmsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.memberSeq = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.groupMsgSeq = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbGroupReadedReportResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errmsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.memberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupMsgSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errmsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.memberSeq { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupMsgSeq { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbDiscussReadedReportResp<'a> {
    pub result: Option<u32>,
    pub errmsg: Option<Cow<'a, str>>,
    pub confUin: Option<u64>,
    pub memberSeq: Option<u64>,
    pub confSeq: Option<u64>,
}

impl<'a> MessageRead<'a> for PbDiscussReadedReportResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.errmsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.confUin = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.memberSeq = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.confSeq = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbDiscussReadedReportResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errmsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.confUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.memberSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.confSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errmsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.confUin { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.memberSeq { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.confSeq { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PbC2CReadedReportResp<'a> {
    pub result: Option<u32>,
    pub errmsg: Option<Cow<'a, str>>,
    pub syncCookie: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for PbC2CReadedReportResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.errmsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.syncCookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PbC2CReadedReportResp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errmsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.syncCookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errmsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.syncCookie { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

