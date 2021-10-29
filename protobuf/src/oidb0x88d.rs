// Automatically generated rust module for 'oidb0x88d.proto' file

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
pub struct D88DGroupHeadPortraitInfo {
    pub picId: Option<u32>,
}

impl<'a> MessageRead<'a> for D88DGroupHeadPortraitInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.picId = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D88DGroupHeadPortraitInfo {
    fn get_size(&self) -> usize {
        0
        + self.picId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.picId { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D88DGroupHeadPortrait {
    pub picCount: Option<u32>,
    pub msgInfo: Vec<D88DGroupHeadPortraitInfo>,
    pub defaultId: Option<u32>,
    pub verifyingPicCnt: Option<u32>,
    pub msgVerifyingPicInfo: Vec<D88DGroupHeadPortraitInfo>,
}

impl<'a> MessageRead<'a> for D88DGroupHeadPortrait {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.picCount = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.msgInfo.push(r.read_message::<D88DGroupHeadPortraitInfo>(bytes)?),
                Ok(24) => msg.defaultId = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.verifyingPicCnt = Some(r.read_uint32(bytes)?),
                Ok(42) => msg.msgVerifyingPicInfo.push(r.read_message::<D88DGroupHeadPortraitInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D88DGroupHeadPortrait {
    fn get_size(&self) -> usize {
        0
        + self.picCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.defaultId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.verifyingPicCnt.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msgVerifyingPicInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.picCount { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.msgInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.defaultId { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.verifyingPicCnt { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        for s in &self.msgVerifyingPicInfo { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D88DGroupExInfoOnly {
    pub tribeId: Option<u32>,
    pub moneyForAddGroup: Option<u32>,
}

impl<'a> MessageRead<'a> for D88DGroupExInfoOnly {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.tribeId = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.moneyForAddGroup = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D88DGroupExInfoOnly {
    fn get_size(&self) -> usize {
        0
        + self.tribeId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.moneyForAddGroup.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.tribeId { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.moneyForAddGroup { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D88DGroupInfo<'a> {
    pub groupOwner: Option<u64>,
    pub groupCreateTime: Option<u32>,
    pub groupFlag: Option<u32>,
    pub groupFlagExt: Option<u32>,
    pub groupMemberMaxNum: Option<u32>,
    pub groupMemberNum: Option<u32>,
    pub groupOption: Option<u32>,
    pub groupClassExt: Option<u32>,
    pub groupSpecialClass: Option<u32>,
    pub groupLevel: Option<u32>,
    pub groupFace: Option<u32>,
    pub groupDefaultPage: Option<u32>,
    pub groupInfoSeq: Option<u32>,
    pub groupRoamingTime: Option<u32>,
    pub groupName: Option<Cow<'a, [u8]>>,
    pub groupMemo: Option<Cow<'a, [u8]>>,
    pub groupFingerMemo: Option<Cow<'a, [u8]>>,
    pub groupClassText: Option<Cow<'a, [u8]>>,
    pub groupAllianceCode: Vec<u32>,
    pub groupExtraAadmNum: Option<u32>,
    pub groupUin: Option<u64>,
    pub groupCurMsgSeq: Option<u32>,
    pub groupLastMsgTime: Option<u32>,
    pub groupQuestion: Option<Cow<'a, [u8]>>,
    pub groupAnswer: Option<Cow<'a, [u8]>>,
    pub groupVisitorMaxNum: Option<u32>,
    pub groupVisitorCurNum: Option<u32>,
    pub levelNameSeq: Option<u32>,
    pub groupAdminMaxNum: Option<u32>,
    pub groupAioSkinTimestamp: Option<u32>,
    pub groupBoardSkinTimestamp: Option<u32>,
    pub groupAioSkinUrl: Option<Cow<'a, [u8]>>,
    pub groupBoardSkinUrl: Option<Cow<'a, [u8]>>,
    pub groupCoverSkinTimestamp: Option<u32>,
    pub groupCoverSkinUrl: Option<Cow<'a, [u8]>>,
    pub groupGrade: Option<u32>,
    pub activeMemberNum: Option<u32>,
    pub certificationType: Option<u32>,
    pub certificationText: Option<Cow<'a, [u8]>>,
    pub groupRichFingerMemo: Option<Cow<'a, [u8]>>,
    pub tagRecord: Vec<D88DTagRecord<'a>>,
    pub groupGeoInfo: Option<D88DGroupGeoInfo<'a>>,
    pub headPortraitSeq: Option<u32>,
    pub msgHeadPortrait: Option<D88DGroupHeadPortrait>,
    pub shutupTimestamp: Option<u32>,
    pub shutupTimestampMe: Option<u32>,
    pub createSourceFlag: Option<u32>,
    pub cmduinMsgSeq: Option<u32>,
    pub cmduinJoinTime: Option<u32>,
    pub cmduinUinFlag: Option<u32>,
    pub cmduinFlagEx: Option<u32>,
    pub cmduinNewMobileFlag: Option<u32>,
    pub cmduinReadMsgSeq: Option<u32>,
    pub cmduinLastMsgTime: Option<u32>,
    pub groupTypeFlag: Option<u32>,
    pub appPrivilegeFlag: Option<u32>,
    pub stGroupExInfo: Option<D88DGroupExInfoOnly>,
    pub groupSecLevel: Option<u32>,
    pub groupSecLevelInfo: Option<u32>,
    pub cmduinPrivilege: Option<u32>,
    pub poidInfo: Option<Cow<'a, [u8]>>,
    pub cmduinFlagEx2: Option<u32>,
    pub confUin: Option<u64>,
    pub confMaxMsgSeq: Option<u32>,
    pub confToGroupTime: Option<u32>,
    pub passwordRedbagTime: Option<u32>,
    pub subscriptionUin: Option<u64>,
    pub memberListChangeSeq: Option<u32>,
    pub membercardSeq: Option<u32>,
    pub rootId: Option<u64>,
    pub parentId: Option<u64>,
    pub teamSeq: Option<u32>,
    pub historyMsgBeginTime: Option<u64>,
    pub inviteNoAuthNumLimit: Option<u64>,
    pub cmduinHistoryMsgSeq: Option<u32>,
    pub cmduinJoinMsgSeq: Option<u32>,
    pub groupFlagext3: Option<u32>,
    pub groupOpenAppid: Option<u32>,
    pub isConfGroup: Option<u32>,
    pub isModifyConfGroupFace: Option<u32>,
    pub isModifyConfGroupName: Option<u32>,
    pub noFingerOpenFlag: Option<u32>,
    pub noCodeFingerOpenFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for D88DGroupInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupOwner = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.groupCreateTime = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.groupFlag = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.groupFlagExt = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.groupMemberMaxNum = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.groupMemberNum = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.groupOption = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.groupClassExt = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.groupSpecialClass = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.groupLevel = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.groupFace = Some(r.read_uint32(bytes)?),
                Ok(96) => msg.groupDefaultPage = Some(r.read_uint32(bytes)?),
                Ok(104) => msg.groupInfoSeq = Some(r.read_uint32(bytes)?),
                Ok(112) => msg.groupRoamingTime = Some(r.read_uint32(bytes)?),
                Ok(122) => msg.groupName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.groupMemo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(138) => msg.groupFingerMemo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(146) => msg.groupClassText = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(152) => msg.groupAllianceCode.push(r.read_uint32(bytes)?),
                Ok(160) => msg.groupExtraAadmNum = Some(r.read_uint32(bytes)?),
                Ok(168) => msg.groupUin = Some(r.read_uint64(bytes)?),
                Ok(176) => msg.groupCurMsgSeq = Some(r.read_uint32(bytes)?),
                Ok(184) => msg.groupLastMsgTime = Some(r.read_uint32(bytes)?),
                Ok(194) => msg.groupQuestion = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(202) => msg.groupAnswer = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(208) => msg.groupVisitorMaxNum = Some(r.read_uint32(bytes)?),
                Ok(216) => msg.groupVisitorCurNum = Some(r.read_uint32(bytes)?),
                Ok(224) => msg.levelNameSeq = Some(r.read_uint32(bytes)?),
                Ok(232) => msg.groupAdminMaxNum = Some(r.read_uint32(bytes)?),
                Ok(240) => msg.groupAioSkinTimestamp = Some(r.read_uint32(bytes)?),
                Ok(248) => msg.groupBoardSkinTimestamp = Some(r.read_uint32(bytes)?),
                Ok(258) => msg.groupAioSkinUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(266) => msg.groupBoardSkinUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(272) => msg.groupCoverSkinTimestamp = Some(r.read_uint32(bytes)?),
                Ok(282) => msg.groupCoverSkinUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(288) => msg.groupGrade = Some(r.read_uint32(bytes)?),
                Ok(296) => msg.activeMemberNum = Some(r.read_uint32(bytes)?),
                Ok(304) => msg.certificationType = Some(r.read_uint32(bytes)?),
                Ok(314) => msg.certificationText = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(322) => msg.groupRichFingerMemo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(330) => msg.tagRecord.push(r.read_message::<D88DTagRecord>(bytes)?),
                Ok(338) => msg.groupGeoInfo = Some(r.read_message::<D88DGroupGeoInfo>(bytes)?),
                Ok(344) => msg.headPortraitSeq = Some(r.read_uint32(bytes)?),
                Ok(354) => msg.msgHeadPortrait = Some(r.read_message::<D88DGroupHeadPortrait>(bytes)?),
                Ok(360) => msg.shutupTimestamp = Some(r.read_uint32(bytes)?),
                Ok(368) => msg.shutupTimestampMe = Some(r.read_uint32(bytes)?),
                Ok(376) => msg.createSourceFlag = Some(r.read_uint32(bytes)?),
                Ok(384) => msg.cmduinMsgSeq = Some(r.read_uint32(bytes)?),
                Ok(392) => msg.cmduinJoinTime = Some(r.read_uint32(bytes)?),
                Ok(400) => msg.cmduinUinFlag = Some(r.read_uint32(bytes)?),
                Ok(408) => msg.cmduinFlagEx = Some(r.read_uint32(bytes)?),
                Ok(416) => msg.cmduinNewMobileFlag = Some(r.read_uint32(bytes)?),
                Ok(424) => msg.cmduinReadMsgSeq = Some(r.read_uint32(bytes)?),
                Ok(432) => msg.cmduinLastMsgTime = Some(r.read_uint32(bytes)?),
                Ok(440) => msg.groupTypeFlag = Some(r.read_uint32(bytes)?),
                Ok(448) => msg.appPrivilegeFlag = Some(r.read_uint32(bytes)?),
                Ok(458) => msg.stGroupExInfo = Some(r.read_message::<D88DGroupExInfoOnly>(bytes)?),
                Ok(464) => msg.groupSecLevel = Some(r.read_uint32(bytes)?),
                Ok(472) => msg.groupSecLevelInfo = Some(r.read_uint32(bytes)?),
                Ok(480) => msg.cmduinPrivilege = Some(r.read_uint32(bytes)?),
                Ok(490) => msg.poidInfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(496) => msg.cmduinFlagEx2 = Some(r.read_uint32(bytes)?),
                Ok(504) => msg.confUin = Some(r.read_uint64(bytes)?),
                Ok(512) => msg.confMaxMsgSeq = Some(r.read_uint32(bytes)?),
                Ok(520) => msg.confToGroupTime = Some(r.read_uint32(bytes)?),
                Ok(528) => msg.passwordRedbagTime = Some(r.read_uint32(bytes)?),
                Ok(536) => msg.subscriptionUin = Some(r.read_uint64(bytes)?),
                Ok(544) => msg.memberListChangeSeq = Some(r.read_uint32(bytes)?),
                Ok(552) => msg.membercardSeq = Some(r.read_uint32(bytes)?),
                Ok(560) => msg.rootId = Some(r.read_uint64(bytes)?),
                Ok(568) => msg.parentId = Some(r.read_uint64(bytes)?),
                Ok(576) => msg.teamSeq = Some(r.read_uint32(bytes)?),
                Ok(584) => msg.historyMsgBeginTime = Some(r.read_uint64(bytes)?),
                Ok(592) => msg.inviteNoAuthNumLimit = Some(r.read_uint64(bytes)?),
                Ok(600) => msg.cmduinHistoryMsgSeq = Some(r.read_uint32(bytes)?),
                Ok(608) => msg.cmduinJoinMsgSeq = Some(r.read_uint32(bytes)?),
                Ok(616) => msg.groupFlagext3 = Some(r.read_uint32(bytes)?),
                Ok(624) => msg.groupOpenAppid = Some(r.read_uint32(bytes)?),
                Ok(632) => msg.isConfGroup = Some(r.read_uint32(bytes)?),
                Ok(640) => msg.isModifyConfGroupFace = Some(r.read_uint32(bytes)?),
                Ok(648) => msg.isModifyConfGroupName = Some(r.read_uint32(bytes)?),
                Ok(656) => msg.noFingerOpenFlag = Some(r.read_uint32(bytes)?),
                Ok(664) => msg.noCodeFingerOpenFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D88DGroupInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupOwner.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCreateTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupFlagExt.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupMemberMaxNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupMemberNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupOption.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupClassExt.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupSpecialClass.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupFace.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupDefaultPage.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupInfoSeq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupRoamingTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupMemo.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupFingerMemo.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupClassText.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupAllianceCode.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.groupExtraAadmNum.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupUin.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupCurMsgSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupLastMsgTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupQuestion.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupAnswer.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupVisitorMaxNum.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupVisitorCurNum.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.levelNameSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupAdminMaxNum.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupAioSkinTimestamp.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupBoardSkinTimestamp.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupAioSkinUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupBoardSkinUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupCoverSkinTimestamp.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupCoverSkinUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupGrade.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.activeMemberNum.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.certificationType.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.certificationText.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.groupRichFingerMemo.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.tagRecord.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupGeoInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.headPortraitSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.msgHeadPortrait.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.shutupTimestamp.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.shutupTimestampMe.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.createSourceFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinMsgSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinJoinTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinUinFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinFlagEx.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinNewMobileFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinReadMsgSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinLastMsgTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupTypeFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.appPrivilegeFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.stGroupExInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.groupSecLevel.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupSecLevelInfo.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinPrivilege.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.poidInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.cmduinFlagEx2.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.confUin.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.confMaxMsgSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.confToGroupTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.passwordRedbagTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.subscriptionUin.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.memberListChangeSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.membercardSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.rootId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.parentId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.teamSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.historyMsgBeginTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.inviteNoAuthNumLimit.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinHistoryMsgSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cmduinJoinMsgSeq.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupFlagext3.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.groupOpenAppid.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.isConfGroup.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.isModifyConfGroupFace.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.isModifyConfGroupName.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.noFingerOpenFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.noCodeFingerOpenFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupOwner { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupCreateTime { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupFlag { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupFlagExt { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupMemberMaxNum { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupMemberNum { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupOption { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupClassExt { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupSpecialClass { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupLevel { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupFace { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupDefaultPage { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupInfoSeq { w.write_with_tag(104, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupRoamingTime { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupName { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupMemo { w.write_with_tag(130, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupFingerMemo { w.write_with_tag(138, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupClassText { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        for s in &self.groupAllianceCode { w.write_with_tag(152, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupExtraAadmNum { w.write_with_tag(160, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupUin { w.write_with_tag(168, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupCurMsgSeq { w.write_with_tag(176, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupLastMsgTime { w.write_with_tag(184, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupQuestion { w.write_with_tag(194, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupAnswer { w.write_with_tag(202, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupVisitorMaxNum { w.write_with_tag(208, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupVisitorCurNum { w.write_with_tag(216, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.levelNameSeq { w.write_with_tag(224, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupAdminMaxNum { w.write_with_tag(232, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupAioSkinTimestamp { w.write_with_tag(240, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupBoardSkinTimestamp { w.write_with_tag(248, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupAioSkinUrl { w.write_with_tag(258, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupBoardSkinUrl { w.write_with_tag(266, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupCoverSkinTimestamp { w.write_with_tag(272, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupCoverSkinUrl { w.write_with_tag(282, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupGrade { w.write_with_tag(288, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.activeMemberNum { w.write_with_tag(296, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.certificationType { w.write_with_tag(304, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.certificationText { w.write_with_tag(314, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupRichFingerMemo { w.write_with_tag(322, |w| w.write_bytes(&**s))?; }
        for s in &self.tagRecord { w.write_with_tag(330, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupGeoInfo { w.write_with_tag(338, |w| w.write_message(s))?; }
        if let Some(ref s) = self.headPortraitSeq { w.write_with_tag(344, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.msgHeadPortrait { w.write_with_tag(354, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shutupTimestamp { w.write_with_tag(360, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.shutupTimestampMe { w.write_with_tag(368, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.createSourceFlag { w.write_with_tag(376, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinMsgSeq { w.write_with_tag(384, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinJoinTime { w.write_with_tag(392, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinUinFlag { w.write_with_tag(400, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinFlagEx { w.write_with_tag(408, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinNewMobileFlag { w.write_with_tag(416, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinReadMsgSeq { w.write_with_tag(424, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinLastMsgTime { w.write_with_tag(432, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupTypeFlag { w.write_with_tag(440, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.appPrivilegeFlag { w.write_with_tag(448, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.stGroupExInfo { w.write_with_tag(458, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupSecLevel { w.write_with_tag(464, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupSecLevelInfo { w.write_with_tag(472, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinPrivilege { w.write_with_tag(480, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.poidInfo { w.write_with_tag(490, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.cmduinFlagEx2 { w.write_with_tag(496, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.confUin { w.write_with_tag(504, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.confMaxMsgSeq { w.write_with_tag(512, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.confToGroupTime { w.write_with_tag(520, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.passwordRedbagTime { w.write_with_tag(528, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.subscriptionUin { w.write_with_tag(536, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.memberListChangeSeq { w.write_with_tag(544, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.membercardSeq { w.write_with_tag(552, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.rootId { w.write_with_tag(560, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.parentId { w.write_with_tag(568, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.teamSeq { w.write_with_tag(576, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.historyMsgBeginTime { w.write_with_tag(584, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.inviteNoAuthNumLimit { w.write_with_tag(592, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cmduinHistoryMsgSeq { w.write_with_tag(600, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cmduinJoinMsgSeq { w.write_with_tag(608, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupFlagext3 { w.write_with_tag(616, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupOpenAppid { w.write_with_tag(624, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.isConfGroup { w.write_with_tag(632, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.isModifyConfGroupFace { w.write_with_tag(640, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.isModifyConfGroupName { w.write_with_tag(648, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.noFingerOpenFlag { w.write_with_tag(656, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.noCodeFingerOpenFlag { w.write_with_tag(664, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqGroupInfo<'a> {
    pub groupCode: Option<u64>,
    pub stgroupinfo: Option<D88DGroupInfo<'a>>,
    pub lastGetGroupNameTime: Option<u32>,
}

impl<'a> MessageRead<'a> for ReqGroupInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.stgroupinfo = Some(r.read_message::<D88DGroupInfo>(bytes)?),
                Ok(24) => msg.lastGetGroupNameTime = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReqGroupInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stgroupinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.lastGetGroupNameTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.stgroupinfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.lastGetGroupNameTime { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D88DReqBody<'a> {
    pub appId: Option<u32>,
    pub reqGroupInfo: Vec<ReqGroupInfo<'a>>,
    pub pcClientVersion: Option<u32>,
}

impl<'a> MessageRead<'a> for D88DReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.appId = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.reqGroupInfo.push(r.read_message::<ReqGroupInfo>(bytes)?),
                Ok(24) => msg.pcClientVersion = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D88DReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.appId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reqGroupInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.pcClientVersion.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.appId { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.reqGroupInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pcClientVersion { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspGroupInfo<'a> {
    pub groupCode: Option<u64>,
    pub result: Option<u32>,
    pub groupInfo: Option<D88DGroupInfo<'a>>,
}

impl<'a> MessageRead<'a> for RspGroupInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.groupInfo = Some(r.read_message::<D88DGroupInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspGroupInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupInfo { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D88DRspBody<'a> {
    pub rspGroupInfo: Vec<RspGroupInfo<'a>>,
    pub strErrorInfo: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D88DRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.rspGroupInfo.push(r.read_message::<RspGroupInfo>(bytes)?),
                Ok(18) => msg.strErrorInfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D88DRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.rspGroupInfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.strErrorInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.rspGroupInfo { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.strErrorInfo { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D88DTagRecord<'a> {
    pub fromUin: Option<u64>,
    pub groupCode: Option<u64>,
    pub tagId: Option<Cow<'a, [u8]>>,
    pub setTime: Option<u64>,
    pub goodNum: Option<u32>,
    pub badNum: Option<u32>,
    pub tagLen: Option<u32>,
    pub tagValue: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for D88DTagRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(26) => msg.tagId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.setTime = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.goodNum = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.badNum = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.tagLen = Some(r.read_uint32(bytes)?),
                Ok(66) => msg.tagValue = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D88DTagRecord<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tagId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.setTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.goodNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.badNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tagLen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tagValue.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.tagId { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.setTime { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.goodNum { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.badNum { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.tagLen { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.tagValue { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D88DGroupGeoInfo<'a> {
    pub owneruin: Option<u64>,
    pub settime: Option<u32>,
    pub cityid: Option<u32>,
    pub longitude: Option<i64>,
    pub latitude: Option<i64>,
    pub geocontent: Option<Cow<'a, [u8]>>,
    pub poiId: Option<u64>,
}

impl<'a> MessageRead<'a> for D88DGroupGeoInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.owneruin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.settime = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.cityid = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.longitude = Some(r.read_int64(bytes)?),
                Ok(40) => msg.latitude = Some(r.read_int64(bytes)?),
                Ok(50) => msg.geocontent = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.poiId = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D88DGroupGeoInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.owneruin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.settime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cityid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.longitude.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.latitude.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.geocontent.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.poiId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.owneruin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.settime { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cityid { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.longitude { w.write_with_tag(32, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.latitude { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.geocontent { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.poiId { w.write_with_tag(56, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

