// Automatically generated rust module for 'oidb.proto' file

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
pub struct OIDBSSOPkg<'a> {
    pub command: i32,
    pub serviceType: i32,
    pub result: i32,
    pub bodybuffer: Cow<'a, [u8]>,
    pub errorMsg: Cow<'a, str>,
    pub clientVersion: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for OIDBSSOPkg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.command = r.read_int32(bytes)?,
                Ok(16) => msg.serviceType = r.read_int32(bytes)?,
                Ok(24) => msg.result = r.read_int32(bytes)?,
                Ok(34) => msg.bodybuffer = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.errorMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.clientVersion = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OIDBSSOPkg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.command == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.command) as u64) }
        + if self.serviceType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.serviceType) as u64) }
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.bodybuffer == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bodybuffer).len()) }
        + if self.errorMsg == "" { 0 } else { 1 + sizeof_len((&self.errorMsg).len()) }
        + if self.clientVersion == "" { 0 } else { 1 + sizeof_len((&self.clientVersion).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.command != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.command))?; }
        if self.serviceType != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.serviceType))?; }
        if self.result != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.result))?; }
        if self.bodybuffer != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.bodybuffer))?; }
        if self.errorMsg != "" { w.write_with_tag(42, |w| w.write_string(&**&self.errorMsg))?; }
        if self.clientVersion != "" { w.write_with_tag(50, |w| w.write_string(&**&self.clientVersion))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8A0RspBody {
    pub optUint64GroupCode: i64,
    pub msgKickResult: Vec<D8A0KickResult>,
}

impl<'a> MessageRead<'a> for D8A0RspBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.optUint64GroupCode = r.read_int64(bytes)?,
                Ok(18) => msg.msgKickResult.push(r.read_message::<D8A0KickResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D8A0RspBody {
    fn get_size(&self) -> usize {
        0
        + if self.optUint64GroupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.optUint64GroupCode) as u64) }
        + self.msgKickResult.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.optUint64GroupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.optUint64GroupCode))?; }
        for s in &self.msgKickResult { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8A0KickResult {
    pub optUint32Result: i32,
    pub optUint64MemberUin: i64,
}

impl<'a> MessageRead<'a> for D8A0KickResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.optUint32Result = r.read_int32(bytes)?,
                Ok(16) => msg.optUint64MemberUin = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D8A0KickResult {
    fn get_size(&self) -> usize {
        0
        + if self.optUint32Result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.optUint32Result) as u64) }
        + if self.optUint64MemberUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.optUint64MemberUin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.optUint32Result != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.optUint32Result))?; }
        if self.optUint64MemberUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.optUint64MemberUin))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8A0KickMemberInfo<'a> {
    pub optUint32Operate: i32,
    pub optUint64MemberUin: i64,
    pub optUint32Flag: i32,
    pub optBytesMsg: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for D8A0KickMemberInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.optUint32Operate = r.read_int32(bytes)?,
                Ok(16) => msg.optUint64MemberUin = r.read_int64(bytes)?,
                Ok(24) => msg.optUint32Flag = r.read_int32(bytes)?,
                Ok(34) => msg.optBytesMsg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8A0KickMemberInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.optUint32Operate == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.optUint32Operate) as u64) }
        + if self.optUint64MemberUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.optUint64MemberUin) as u64) }
        + if self.optUint32Flag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.optUint32Flag) as u64) }
        + if self.optBytesMsg == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.optBytesMsg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.optUint32Operate != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.optUint32Operate))?; }
        if self.optUint64MemberUin != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.optUint64MemberUin))?; }
        if self.optUint32Flag != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.optUint32Flag))?; }
        if self.optBytesMsg != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.optBytesMsg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D8A0ReqBody<'a> {
    pub optUint64GroupCode: i64,
    pub msgKickList: Vec<D8A0KickMemberInfo<'a>>,
    pub kickList: Vec<i64>,
    pub kickFlag: i32,
    pub kickMsg: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for D8A0ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.optUint64GroupCode = r.read_int64(bytes)?,
                Ok(18) => msg.msgKickList.push(r.read_message::<D8A0KickMemberInfo>(bytes)?),
                Ok(26) => msg.kickList = r.read_packed(bytes, |r, bytes| Ok(r.read_int64(bytes)?))?,
                Ok(32) => msg.kickFlag = r.read_int32(bytes)?,
                Ok(42) => msg.kickMsg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D8A0ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.optUint64GroupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.optUint64GroupCode) as u64) }
        + self.msgKickList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.kickList.is_empty() { 0 } else { 1 + sizeof_len(self.kickList.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.kickFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.kickFlag) as u64) }
        + if self.kickMsg == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.kickMsg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.optUint64GroupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.optUint64GroupCode))?; }
        for s in &self.msgKickList { w.write_with_tag(18, |w| w.write_message(s))?; }
        w.write_packed_with_tag(26, &self.kickList, |w, m| w.write_int64(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.kickFlag != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.kickFlag))?; }
        if self.kickMsg != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.kickMsg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D89AReqBody<'a> {
    pub groupCode: i64,
    pub stGroupInfo: Option<D89AGroupinfo<'a>>,
    pub originalOperatorUin: i64,
    pub reqGroupOpenAppid: i32,
}

impl<'a> MessageRead<'a> for D89AReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = r.read_int64(bytes)?,
                Ok(18) => msg.stGroupInfo = Some(r.read_message::<D89AGroupinfo>(bytes)?),
                Ok(24) => msg.originalOperatorUin = r.read_int64(bytes)?,
                Ok(32) => msg.reqGroupOpenAppid = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D89AReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + self.stGroupInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.originalOperatorUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.originalOperatorUin) as u64) }
        + if self.reqGroupOpenAppid == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqGroupOpenAppid) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupCode != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.groupCode))?; }
        if let Some(ref s) = self.stGroupInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.originalOperatorUin != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.originalOperatorUin))?; }
        if self.reqGroupOpenAppid != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.reqGroupOpenAppid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D89AGroupinfo<'a> {
    pub groupExtAdmNum: i32,
    pub flag: i32,
    pub ingGroupName: Cow<'a, [u8]>,
    pub ingGroupMemo: Cow<'a, [u8]>,
    pub ingGroupFingerMemo: Cow<'a, [u8]>,
    pub ingGroupAioSkinUrl: Cow<'a, [u8]>,
    pub ingGroupBoardSkinUrl: Cow<'a, [u8]>,
    pub ingGroupCoverSkinUrl: Cow<'a, [u8]>,
    pub groupGrade: i32,
    pub activeMemberNum: i32,
    pub certificationType: i32,
    pub ingCertificationText: Cow<'a, [u8]>,
    pub ingGroupRichFingerMemo: Cow<'a, [u8]>,
    pub stGroupNewguidelines: Option<D89AGroupNewGuidelinesInfo<'a>>,
    pub groupFace: i32,
    pub addOption: i32,
    pub groupTypeFlag: i32,
    pub stringGroupTag: Cow<'a, [u8]>,
    pub msgGroupGeoInfo: Option<D89AGroupGeoInfo<'a>>,
    pub groupClassExt: i32,
    pub ingGroupClassText: Cow<'a, [u8]>,
    pub appPrivilegeFlag: i32,
    pub appPrivilegeMask: i32,
    pub stGroupExInfo: Option<D89AGroupExInfoOnly>,
    pub groupSecLevel: i32,
    pub groupSecLevelInfo: i32,
    pub subscriptionUin: i64,
    pub allowMemberInvite: i32,
    pub ingGroupQuestion: Cow<'a, [u8]>,
    pub ingGroupAnswer: Cow<'a, [u8]>,
    pub groupFlagext3: i32,
    pub groupFlagext3Mask: i32,
    pub groupOpenAppid: i32,
    pub noFingerOpenFlag: i32,
    pub noCodeFingerOpenFlag: i32,
    pub rootId: i64,
    pub msgLimitFrequency: i32,
    pub shutupTime: mod_D89AGroupinfo::OneOfshutupTime,
}

impl<'a> MessageRead<'a> for D89AGroupinfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupExtAdmNum = r.read_int32(bytes)?,
                Ok(16) => msg.flag = r.read_int32(bytes)?,
                Ok(26) => msg.ingGroupName = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.ingGroupMemo = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.ingGroupFingerMemo = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.ingGroupAioSkinUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.ingGroupBoardSkinUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.ingGroupCoverSkinUrl = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.groupGrade = r.read_int32(bytes)?,
                Ok(80) => msg.activeMemberNum = r.read_int32(bytes)?,
                Ok(88) => msg.certificationType = r.read_int32(bytes)?,
                Ok(98) => msg.ingCertificationText = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(106) => msg.ingGroupRichFingerMemo = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.stGroupNewguidelines = Some(r.read_message::<D89AGroupNewGuidelinesInfo>(bytes)?),
                Ok(120) => msg.groupFace = r.read_int32(bytes)?,
                Ok(128) => msg.addOption = r.read_int32(bytes)?,
                Ok(144) => msg.groupTypeFlag = r.read_int32(bytes)?,
                Ok(154) => msg.stringGroupTag = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(162) => msg.msgGroupGeoInfo = Some(r.read_message::<D89AGroupGeoInfo>(bytes)?),
                Ok(168) => msg.groupClassExt = r.read_int32(bytes)?,
                Ok(178) => msg.ingGroupClassText = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(184) => msg.appPrivilegeFlag = r.read_int32(bytes)?,
                Ok(192) => msg.appPrivilegeMask = r.read_int32(bytes)?,
                Ok(202) => msg.stGroupExInfo = Some(r.read_message::<D89AGroupExInfoOnly>(bytes)?),
                Ok(208) => msg.groupSecLevel = r.read_int32(bytes)?,
                Ok(216) => msg.groupSecLevelInfo = r.read_int32(bytes)?,
                Ok(224) => msg.subscriptionUin = r.read_int64(bytes)?,
                Ok(232) => msg.allowMemberInvite = r.read_int32(bytes)?,
                Ok(242) => msg.ingGroupQuestion = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(250) => msg.ingGroupAnswer = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(256) => msg.groupFlagext3 = r.read_int32(bytes)?,
                Ok(264) => msg.groupFlagext3Mask = r.read_int32(bytes)?,
                Ok(272) => msg.groupOpenAppid = r.read_int32(bytes)?,
                Ok(280) => msg.noFingerOpenFlag = r.read_int32(bytes)?,
                Ok(288) => msg.noCodeFingerOpenFlag = r.read_int32(bytes)?,
                Ok(296) => msg.rootId = r.read_int64(bytes)?,
                Ok(304) => msg.msgLimitFrequency = r.read_int32(bytes)?,
                Ok(136) => msg.shutupTime = mod_D89AGroupinfo::OneOfshutupTime::val(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D89AGroupinfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupExtAdmNum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupExtAdmNum) as u64) }
        + if self.flag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.flag) as u64) }
        + if self.ingGroupName == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGroupName).len()) }
        + if self.ingGroupMemo == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGroupMemo).len()) }
        + if self.ingGroupFingerMemo == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGroupFingerMemo).len()) }
        + if self.ingGroupAioSkinUrl == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGroupAioSkinUrl).len()) }
        + if self.ingGroupBoardSkinUrl == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGroupBoardSkinUrl).len()) }
        + if self.ingGroupCoverSkinUrl == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGroupCoverSkinUrl).len()) }
        + if self.groupGrade == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupGrade) as u64) }
        + if self.activeMemberNum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.activeMemberNum) as u64) }
        + if self.certificationType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.certificationType) as u64) }
        + if self.ingCertificationText == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingCertificationText).len()) }
        + if self.ingGroupRichFingerMemo == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGroupRichFingerMemo).len()) }
        + self.stGroupNewguidelines.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.groupFace == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupFace) as u64) }
        + if self.addOption == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.addOption) as u64) }
        + if self.groupTypeFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupTypeFlag) as u64) }
        + if self.stringGroupTag == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.stringGroupTag).len()) }
        + self.msgGroupGeoInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.groupClassExt == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupClassExt) as u64) }
        + if self.ingGroupClassText == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.ingGroupClassText).len()) }
        + if self.appPrivilegeFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.appPrivilegeFlag) as u64) }
        + if self.appPrivilegeMask == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.appPrivilegeMask) as u64) }
        + self.stGroupExInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.groupSecLevel == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupSecLevel) as u64) }
        + if self.groupSecLevelInfo == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupSecLevelInfo) as u64) }
        + if self.subscriptionUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.subscriptionUin) as u64) }
        + if self.allowMemberInvite == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.allowMemberInvite) as u64) }
        + if self.ingGroupQuestion == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.ingGroupQuestion).len()) }
        + if self.ingGroupAnswer == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.ingGroupAnswer).len()) }
        + if self.groupFlagext3 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupFlagext3) as u64) }
        + if self.groupFlagext3Mask == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupFlagext3Mask) as u64) }
        + if self.groupOpenAppid == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupOpenAppid) as u64) }
        + if self.noFingerOpenFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.noFingerOpenFlag) as u64) }
        + if self.noCodeFingerOpenFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.noCodeFingerOpenFlag) as u64) }
        + if self.rootId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.rootId) as u64) }
        + if self.msgLimitFrequency == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.msgLimitFrequency) as u64) }
        + match self.shutupTime {
            mod_D89AGroupinfo::OneOfshutupTime::val(ref m) => 2 + sizeof_varint(*(m) as u64),
            mod_D89AGroupinfo::OneOfshutupTime::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupExtAdmNum != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.groupExtAdmNum))?; }
        if self.flag != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.flag))?; }
        if self.ingGroupName != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.ingGroupName))?; }
        if self.ingGroupMemo != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.ingGroupMemo))?; }
        if self.ingGroupFingerMemo != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.ingGroupFingerMemo))?; }
        if self.ingGroupAioSkinUrl != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.ingGroupAioSkinUrl))?; }
        if self.ingGroupBoardSkinUrl != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.ingGroupBoardSkinUrl))?; }
        if self.ingGroupCoverSkinUrl != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.ingGroupCoverSkinUrl))?; }
        if self.groupGrade != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.groupGrade))?; }
        if self.activeMemberNum != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.activeMemberNum))?; }
        if self.certificationType != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.certificationType))?; }
        if self.ingCertificationText != Cow::Borrowed(b"") { w.write_with_tag(98, |w| w.write_bytes(&**&self.ingCertificationText))?; }
        if self.ingGroupRichFingerMemo != Cow::Borrowed(b"") { w.write_with_tag(106, |w| w.write_bytes(&**&self.ingGroupRichFingerMemo))?; }
        if let Some(ref s) = self.stGroupNewguidelines { w.write_with_tag(114, |w| w.write_message(s))?; }
        if self.groupFace != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.groupFace))?; }
        if self.addOption != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.addOption))?; }
        if self.groupTypeFlag != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.groupTypeFlag))?; }
        if self.stringGroupTag != Cow::Borrowed(b"") { w.write_with_tag(154, |w| w.write_bytes(&**&self.stringGroupTag))?; }
        if let Some(ref s) = self.msgGroupGeoInfo { w.write_with_tag(162, |w| w.write_message(s))?; }
        if self.groupClassExt != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.groupClassExt))?; }
        if self.ingGroupClassText != Cow::Borrowed(b"") { w.write_with_tag(178, |w| w.write_bytes(&**&self.ingGroupClassText))?; }
        if self.appPrivilegeFlag != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.appPrivilegeFlag))?; }
        if self.appPrivilegeMask != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.appPrivilegeMask))?; }
        if let Some(ref s) = self.stGroupExInfo { w.write_with_tag(202, |w| w.write_message(s))?; }
        if self.groupSecLevel != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.groupSecLevel))?; }
        if self.groupSecLevelInfo != 0i32 { w.write_with_tag(216, |w| w.write_int32(*&self.groupSecLevelInfo))?; }
        if self.subscriptionUin != 0i64 { w.write_with_tag(224, |w| w.write_int64(*&self.subscriptionUin))?; }
        if self.allowMemberInvite != 0i32 { w.write_with_tag(232, |w| w.write_int32(*&self.allowMemberInvite))?; }
        if self.ingGroupQuestion != Cow::Borrowed(b"") { w.write_with_tag(242, |w| w.write_bytes(&**&self.ingGroupQuestion))?; }
        if self.ingGroupAnswer != Cow::Borrowed(b"") { w.write_with_tag(250, |w| w.write_bytes(&**&self.ingGroupAnswer))?; }
        if self.groupFlagext3 != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.groupFlagext3))?; }
        if self.groupFlagext3Mask != 0i32 { w.write_with_tag(264, |w| w.write_int32(*&self.groupFlagext3Mask))?; }
        if self.groupOpenAppid != 0i32 { w.write_with_tag(272, |w| w.write_int32(*&self.groupOpenAppid))?; }
        if self.noFingerOpenFlag != 0i32 { w.write_with_tag(280, |w| w.write_int32(*&self.noFingerOpenFlag))?; }
        if self.noCodeFingerOpenFlag != 0i32 { w.write_with_tag(288, |w| w.write_int32(*&self.noCodeFingerOpenFlag))?; }
        if self.rootId != 0i64 { w.write_with_tag(296, |w| w.write_int64(*&self.rootId))?; }
        if self.msgLimitFrequency != 0i32 { w.write_with_tag(304, |w| w.write_int32(*&self.msgLimitFrequency))?; }
        match self.shutupTime {            mod_D89AGroupinfo::OneOfshutupTime::val(ref m) => { w.write_with_tag(136, |w| w.write_int32(*m))? },
            mod_D89AGroupinfo::OneOfshutupTime::None => {},
    }        Ok(())
    }
}

pub mod mod_D89AGroupinfo {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfshutupTime {
    val(i32),
    None,
}

impl Default for OneOfshutupTime {
    fn default() -> Self {
        OneOfshutupTime::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D89AGroupNewGuidelinesInfo<'a> {
    pub boolEnabled: bool,
    pub ingContent: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for D89AGroupNewGuidelinesInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.boolEnabled = r.read_bool(bytes)?,
                Ok(18) => msg.ingContent = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D89AGroupNewGuidelinesInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.boolEnabled == false { 0 } else { 1 + sizeof_varint(*(&self.boolEnabled) as u64) }
        + if self.ingContent == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingContent).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.boolEnabled != false { w.write_with_tag(8, |w| w.write_bool(*&self.boolEnabled))?; }
        if self.ingContent != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.ingContent))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D89AGroupExInfoOnly {
    pub tribeId: i32,
    pub moneyForAddGroup: i32,
}

impl<'a> MessageRead<'a> for D89AGroupExInfoOnly {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.tribeId = r.read_int32(bytes)?,
                Ok(16) => msg.moneyForAddGroup = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D89AGroupExInfoOnly {
    fn get_size(&self) -> usize {
        0
        + if self.tribeId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.tribeId) as u64) }
        + if self.moneyForAddGroup == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.moneyForAddGroup) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.tribeId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.tribeId))?; }
        if self.moneyForAddGroup != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.moneyForAddGroup))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D89AGroupGeoInfo<'a> {
    pub cityId: i32,
    pub longtitude: i64,
    pub latitude: i64,
    pub ingGeoContent: Cow<'a, [u8]>,
    pub poiId: i64,
}

impl<'a> MessageRead<'a> for D89AGroupGeoInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cityId = r.read_int32(bytes)?,
                Ok(16) => msg.longtitude = r.read_int64(bytes)?,
                Ok(24) => msg.latitude = r.read_int64(bytes)?,
                Ok(34) => msg.ingGeoContent = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.poiId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D89AGroupGeoInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.cityId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.cityId) as u64) }
        + if self.longtitude == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.longtitude) as u64) }
        + if self.latitude == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latitude) as u64) }
        + if self.ingGeoContent == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.ingGeoContent).len()) }
        + if self.poiId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.poiId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cityId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.cityId))?; }
        if self.longtitude != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.longtitude))?; }
        if self.latitude != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.latitude))?; }
        if self.ingGeoContent != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.ingGeoContent))?; }
        if self.poiId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.poiId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DED3ReqBody {
    pub toUin: i64,
    pub groupCode: i64,
    pub msgSeq: i32,
    pub msgRand: i32,
    pub aioUin: i64,
}

impl<'a> MessageRead<'a> for DED3ReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.toUin = r.read_int64(bytes)?,
                Ok(16) => msg.groupCode = r.read_int64(bytes)?,
                Ok(24) => msg.msgSeq = r.read_int32(bytes)?,
                Ok(32) => msg.msgRand = r.read_int32(bytes)?,
                Ok(40) => msg.aioUin = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DED3ReqBody {
    fn get_size(&self) -> usize {
        0
        + if self.toUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.toUin) as u64) }
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.msgSeq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgSeq) as u64) }
        + if self.msgRand == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgRand) as u64) }
        + if self.aioUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.aioUin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.toUin != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.toUin))?; }
        if self.groupCode != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.groupCode))?; }
        if self.msgSeq != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.msgSeq))?; }
        if self.msgRand != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.msgRand))?; }
        if self.aioUin != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.aioUin))?; }
        Ok(())
    }
}

