// Automatically generated rust module for 'cmd0x3f6.proto' file

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
pub struct C3F6ReqBody<'a> {
    pub subCmd: Option<u32>,
    pub crmCommonHead: Option<C3F6CRMMsgHead<'a>>,
    pub subcmdLoginProcessCompleteReqBody: Option<QDUserLoginProcessCompleteReqBody<'a>>,
}

impl<'a> MessageRead<'a> for C3F6ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subCmd = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.crmCommonHead = Some(r.read_message::<C3F6CRMMsgHead>(bytes)?),
                Ok(338) => msg.subcmdLoginProcessCompleteReqBody = Some(r.read_message::<QDUserLoginProcessCompleteReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C3F6ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.crmCommonHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.subcmdLoginProcessCompleteReqBody.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subCmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.crmCommonHead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.subcmdLoginProcessCompleteReqBody { w.write_with_tag(338, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C3F6RspBody<'a> {
    pub subCmd: Option<u32>,
    pub crmCommonHead: Option<C3F6CRMMsgHead<'a>>,
    pub subcmdLoginProcessCompleteRspBody: Option<QDUserLoginProcessCompleteRspBody<'a>>,
}

impl<'a> MessageRead<'a> for C3F6RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subCmd = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.crmCommonHead = Some(r.read_message::<C3F6CRMMsgHead>(bytes)?),
                Ok(338) => msg.subcmdLoginProcessCompleteRspBody = Some(r.read_message::<QDUserLoginProcessCompleteRspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C3F6RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.crmCommonHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.subcmdLoginProcessCompleteRspBody.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subCmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.crmCommonHead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.subcmdLoginProcessCompleteRspBody { w.write_with_tag(338, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QDUserLoginProcessCompleteReqBody<'a> {
    pub kfext: Option<u64>,
    pub pubno: Option<u32>,
    pub buildno: Option<u32>,
    pub terminalType: Option<u32>,
    pub status: Option<u32>,
    pub loginTime: Option<u32>,
    pub hardwareInfo: Option<Cow<'a, str>>,
    pub softwareInfo: Option<Cow<'a, str>>,
    pub guid: Option<Cow<'a, [u8]>>,
    pub appName: Option<Cow<'a, str>>,
    pub subAppId: Option<u32>,
}

impl<'a> MessageRead<'a> for QDUserLoginProcessCompleteReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.kfext = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.pubno = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.buildno = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.terminalType = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.status = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.loginTime = Some(r.read_uint32(bytes)?),
                Ok(58) => msg.hardwareInfo = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.softwareInfo = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.guid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.appName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.subAppId = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QDUserLoginProcessCompleteReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.kfext.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pubno.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.buildno.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.terminalType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.loginTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hardwareInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.softwareInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.guid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.appName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.subAppId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.kfext { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.pubno { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.buildno { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.terminalType { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.loginTime { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.hardwareInfo { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.softwareInfo { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.guid { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.appName { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.subAppId { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QDUserLoginProcessCompleteRspBody<'a> {
    pub ret: Option<RetInfo<'a>>,
    pub url: Option<Cow<'a, str>>,
    pub mobile: Option<Cow<'a, str>>,
    pub externalMobile: Option<Cow<'a, str>>,
    pub dataAnalysisPriv: Option<bool>,
    pub deviceLock: Option<bool>,
    pub modulePrivilege: Option<u64>,
    pub moduleSubPrivilege: Vec<u32>,
    pub masterSet: Option<u32>,
    pub extSet: Option<u32>,
    pub corpConfProperty: Option<u64>,
    pub corpuin: Option<u64>,
    pub kfaccount: Option<u64>,
    pub securityLevel: Option<u32>,
    pub msgTitle: Option<Cow<'a, str>>,
    pub succNoticeMsg: Option<Cow<'a, str>>,
    pub nameAccount: Option<u64>,
    pub crmMigrateFlag: Option<u32>,
    pub extuinName: Option<Cow<'a, str>>,
    pub openAccountTime: Option<u32>,
}

impl<'a> MessageRead<'a> for QDUserLoginProcessCompleteRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ret = Some(r.read_message::<RetInfo>(bytes)?),
                Ok(18) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.mobile = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.externalMobile = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.dataAnalysisPriv = Some(r.read_bool(bytes)?),
                Ok(48) => msg.deviceLock = Some(r.read_bool(bytes)?),
                Ok(56) => msg.modulePrivilege = Some(r.read_uint64(bytes)?),
                Ok(64) => msg.moduleSubPrivilege.push(r.read_uint32(bytes)?),
                Ok(72) => msg.masterSet = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.extSet = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.corpConfProperty = Some(r.read_uint64(bytes)?),
                Ok(96) => msg.corpuin = Some(r.read_uint64(bytes)?),
                Ok(104) => msg.kfaccount = Some(r.read_uint64(bytes)?),
                Ok(112) => msg.securityLevel = Some(r.read_uint32(bytes)?),
                Ok(122) => msg.msgTitle = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.succNoticeMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(136) => msg.nameAccount = Some(r.read_uint64(bytes)?),
                Ok(144) => msg.crmMigrateFlag = Some(r.read_uint32(bytes)?),
                Ok(154) => msg.extuinName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(160) => msg.openAccountTime = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QDUserLoginProcessCompleteRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ret.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.mobile.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.externalMobile.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.dataAnalysisPriv.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.deviceLock.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.modulePrivilege.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.moduleSubPrivilege.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.masterSet.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.extSet.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.corpConfProperty.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.corpuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.kfaccount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.securityLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgTitle.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.succNoticeMsg.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.nameAccount.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.crmMigrateFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.extuinName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.openAccountTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ret { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.url { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.mobile { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.externalMobile { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.dataAnalysisPriv { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.deviceLock { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.modulePrivilege { w.write_with_tag(56, |w| w.write_uint64(*s))?; }
        for s in &self.moduleSubPrivilege { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.masterSet { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.extSet { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.corpConfProperty { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.corpuin { w.write_with_tag(96, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.kfaccount { w.write_with_tag(104, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.securityLevel { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.msgTitle { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.succNoticeMsg { w.write_with_tag(130, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.nameAccount { w.write_with_tag(136, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.crmMigrateFlag { w.write_with_tag(144, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.extuinName { w.write_with_tag(154, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.openAccountTime { w.write_with_tag(160, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RetInfo<'a> {
    pub retCode: Option<u32>,
    pub errorMsg: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for RetInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.errorMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RetInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errorMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retCode { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.errorMsg { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C3F6CRMMsgHead<'a> {
    pub crmSubCmd: Option<u32>,
    pub headLen: Option<u32>,
    pub verNo: Option<u32>,
    pub kfUin: Option<u64>,
    pub seq: Option<u32>,
    pub packNum: Option<u32>,
    pub curPack: Option<u32>,
    pub bufSig: Option<Cow<'a, str>>,
    pub clienttype: Option<u32>,
    pub laborUin: Option<u64>,
    pub laborName: Option<Cow<'a, str>>,
    pub kfaccount: Option<u64>,
    pub traceId: Option<Cow<'a, str>>,
    pub appId: Option<u32>,
}

impl<'a> MessageRead<'a> for C3F6CRMMsgHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.crmSubCmd = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.headLen = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.verNo = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.kfUin = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.seq = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.packNum = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.curPack = Some(r.read_uint32(bytes)?),
                Ok(66) => msg.bufSig = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.clienttype = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.laborUin = Some(r.read_uint64(bytes)?),
                Ok(90) => msg.laborName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.kfaccount = Some(r.read_uint64(bytes)?),
                Ok(106) => msg.traceId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C3F6CRMMsgHead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.crmSubCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.headLen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.verNo.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.kfUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.seq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.packNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.curPack.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bufSig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clienttype.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.laborUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.laborName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.kfaccount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.traceId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.crmSubCmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.headLen { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.verNo { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.kfUin { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.seq { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.packNum { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.curPack { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.bufSig { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clienttype { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.laborUin { w.write_with_tag(80, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.laborName { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.kfaccount { w.write_with_tag(96, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.traceId { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.appId { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

