// Automatically generated rust module for 'gate.proto' file

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
pub struct GateCommTaskInfo<'a> {
    pub appid: Option<i32>,
    pub taskData: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GateCommTaskInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.appid = Some(r.read_int32(bytes)?),
                Ok(18) => msg.taskData = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GateCommTaskInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.appid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.taskData.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.appid { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.taskData { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateGetGiftListReq {
    pub uin: Option<i32>,
}

impl<'a> MessageRead<'a> for GateGetGiftListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GateGetGiftListReq {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateGetGiftListRsp<'a> {
    pub giftUrl: Vec<Cow<'a, str>>,
    pub customUrl: Option<Cow<'a, str>>,
    pub desc: Option<Cow<'a, str>>,
    pub isOn: Option<bool>,
}

impl<'a> MessageRead<'a> for GateGetGiftListRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.giftUrl.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.customUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.desc = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.isOn = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GateGetGiftListRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.giftUrl.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.customUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.desc.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.isOn.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.giftUrl { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.customUrl { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.desc { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.isOn { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateGetVipCareReq {
    pub uin: Option<i64>,
}

impl<'a> MessageRead<'a> for GateGetVipCareReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GateGetVipCareReq {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateGetVipCareRsp {
    pub buss: Option<i32>,
    pub notice: Option<i32>,
}

impl<'a> MessageRead<'a> for GateGetVipCareRsp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.buss = Some(r.read_int32(bytes)?),
                Ok(16) => msg.notice = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GateGetVipCareRsp {
    fn get_size(&self) -> usize {
        0
        + self.buss.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.notice.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.buss { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.notice { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateOidbFlagInfo<'a> {
    pub fieled: Option<i32>,
    pub byetsValue: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GateOidbFlagInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fieled = Some(r.read_int32(bytes)?),
                Ok(18) => msg.byetsValue = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GateOidbFlagInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fieled.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.byetsValue.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fieled { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.byetsValue { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GatePrivilegeBaseInfoReq {
    pub uReqUin: Option<i64>,
}

impl<'a> MessageRead<'a> for GatePrivilegeBaseInfoReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uReqUin = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GatePrivilegeBaseInfoReq {
    fn get_size(&self) -> usize {
        0
        + self.uReqUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uReqUin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GatePrivilegeBaseInfoRsp<'a> {
    pub msg: Option<Cow<'a, [u8]>>,
    pub jumpUrl: Option<Cow<'a, [u8]>>,
    pub vOpenPriv: Vec<GatePrivilegeInfo<'a>>,
    pub vClosePriv: Vec<GatePrivilegeInfo<'a>>,
    pub uIsGrayUsr: Option<i32>,
}

impl<'a> MessageRead<'a> for GatePrivilegeBaseInfoRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.jumpUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.vOpenPriv.push(r.read_message::<GatePrivilegeInfo>(bytes)?),
                Ok(34) => msg.vClosePriv.push(r.read_message::<GatePrivilegeInfo>(bytes)?),
                Ok(40) => msg.uIsGrayUsr = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GatePrivilegeBaseInfoRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.jumpUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.vOpenPriv.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.vClosePriv.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.uIsGrayUsr.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msg { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.jumpUrl { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        for s in &self.vOpenPriv { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.vClosePriv { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.uIsGrayUsr { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GatePrivilegeInfo<'a> {
    pub iType: Option<i32>,
    pub iSort: Option<i32>,
    pub iFeeType: Option<i32>,
    pub iLevel: Option<i32>,
    pub iFlag: Option<i32>,
    pub iconUrl: Option<Cow<'a, [u8]>>,
    pub deluxeIconUrl: Option<Cow<'a, [u8]>>,
    pub jumpUrl: Option<Cow<'a, [u8]>>,
    pub iIsBig: Option<i32>,
}

impl<'a> MessageRead<'a> for GatePrivilegeInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.iType = Some(r.read_int32(bytes)?),
                Ok(16) => msg.iSort = Some(r.read_int32(bytes)?),
                Ok(24) => msg.iFeeType = Some(r.read_int32(bytes)?),
                Ok(32) => msg.iLevel = Some(r.read_int32(bytes)?),
                Ok(40) => msg.iFlag = Some(r.read_int32(bytes)?),
                Ok(50) => msg.iconUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.deluxeIconUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.jumpUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.iIsBig = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GatePrivilegeInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.iType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.iSort.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.iFeeType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.iLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.iFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.iconUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.deluxeIconUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.jumpUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.iIsBig.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.iType { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.iSort { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.iFeeType { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.iLevel { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.iFlag { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.iconUrl { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.deluxeIconUrl { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.jumpUrl { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.iIsBig { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateVaProfileGateReq<'a> {
    pub uCmd: Option<i32>,
    pub stPrivilegeReq: Option<GatePrivilegeBaseInfoReq>,
    pub stGiftReq: Option<GateGetGiftListReq>,
    pub taskItem: Vec<GateCommTaskInfo<'a>>,
    pub oidbFlag: Vec<GateOidbFlagInfo<'a>>,
    pub stVipCare: Option<GateGetVipCareReq>,
}

impl<'a> MessageRead<'a> for GateVaProfileGateReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uCmd = Some(r.read_int32(bytes)?),
                Ok(18) => msg.stPrivilegeReq = Some(r.read_message::<GatePrivilegeBaseInfoReq>(bytes)?),
                Ok(26) => msg.stGiftReq = Some(r.read_message::<GateGetGiftListReq>(bytes)?),
                Ok(34) => msg.taskItem.push(r.read_message::<GateCommTaskInfo>(bytes)?),
                Ok(42) => msg.oidbFlag.push(r.read_message::<GateOidbFlagInfo>(bytes)?),
                Ok(50) => msg.stVipCare = Some(r.read_message::<GateGetVipCareReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GateVaProfileGateReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stPrivilegeReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stGiftReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.taskItem.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.oidbFlag.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.stVipCare.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uCmd { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.stPrivilegeReq { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stGiftReq { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.taskItem { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.oidbFlag { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stVipCare { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateQidInfoItem<'a> {
    pub qid: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub color: Option<Cow<'a, str>>,
    pub logoUrl: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for GateQidInfoItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.qid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.color = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.logoUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GateQidInfoItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.qid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.logoUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.qid { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.url { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.color { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.logoUrl { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GateVaProfileGateRsp<'a> {
    pub iRetCode: Option<i32>,
    pub sRetMsg: Option<Cow<'a, [u8]>>,
    pub stPrivilegeRsp: Option<GatePrivilegeBaseInfoRsp<'a>>,
    pub stGiftRsp: Option<GateGetGiftListRsp<'a>>,
    pub taskItem: Vec<GateCommTaskInfo<'a>>,
    pub oidbFlag: Vec<GateOidbFlagInfo<'a>>,
    pub stVipCare: Option<GateGetVipCareRsp>,
    pub qidInfo: Option<GateQidInfoItem<'a>>,
}

impl<'a> MessageRead<'a> for GateVaProfileGateRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.iRetCode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.sRetMsg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.stPrivilegeRsp = Some(r.read_message::<GatePrivilegeBaseInfoRsp>(bytes)?),
                Ok(34) => msg.stGiftRsp = Some(r.read_message::<GateGetGiftListRsp>(bytes)?),
                Ok(42) => msg.taskItem.push(r.read_message::<GateCommTaskInfo>(bytes)?),
                Ok(50) => msg.oidbFlag.push(r.read_message::<GateOidbFlagInfo>(bytes)?),
                Ok(58) => msg.stVipCare = Some(r.read_message::<GateGetVipCareRsp>(bytes)?),
                Ok(74) => msg.qidInfo = Some(r.read_message::<GateQidInfoItem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GateVaProfileGateRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.iRetCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sRetMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.stPrivilegeRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stGiftRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.taskItem.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.oidbFlag.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.stVipCare.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.qidInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.iRetCode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.sRetMsg { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.stPrivilegeRsp { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stGiftRsp { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.taskItem { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.oidbFlag { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stVipCare { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.qidInfo { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

