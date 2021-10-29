// Automatically generated rust module for 'structmsg.proto' file

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
pub struct AddFrdSNInfo {
    pub notSeeDynamic: i32,
    pub setSn: i32,
}

impl<'a> MessageRead<'a> for AddFrdSNInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.notSeeDynamic = r.read_int32(bytes)?,
                Ok(16) => msg.setSn = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AddFrdSNInfo {
    fn get_size(&self) -> usize {
        0
        + if self.notSeeDynamic == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.notSeeDynamic) as u64) }
        + if self.setSn == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.setSn) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.notSeeDynamic != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.notSeeDynamic))?; }
        if self.setSn != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.setSn))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FlagInfo {
    pub grpMsgKickAdmin: i32,
    pub grpMsgHiddenGrp: i32,
    pub grpMsgWordingDown: i32,
    pub frdMsgGetBusiCard: i32,
    pub grpMsgGetOfficialAccount: i32,
    pub grpMsgGetPayInGroup: i32,
    pub frdMsgDiscuss2ManyChat: i32,
    pub grpMsgNotAllowJoinGrpInviteNotFrd: i32,
    pub frdMsgNeedWaitingMsg: i32,
    pub frdMsgUint32NeedAllUnreadMsg: i32,
    pub grpMsgNeedAutoAdminWording: i32,
    pub grpMsgGetTransferGroupMsgFlag: i32,
    pub grpMsgGetQuitPayGroupMsgFlag: i32,
    pub grpMsgSupportInviteAutoJoin: i32,
    pub grpMsgMaskInviteAutoJoin: i32,
    pub grpMsgGetDisbandedByAdmin: i32,
    pub grpMsgGetC2cInviteJoinGroup: i32,
}

impl<'a> MessageRead<'a> for FlagInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.grpMsgKickAdmin = r.read_int32(bytes)?,
                Ok(16) => msg.grpMsgHiddenGrp = r.read_int32(bytes)?,
                Ok(24) => msg.grpMsgWordingDown = r.read_int32(bytes)?,
                Ok(32) => msg.frdMsgGetBusiCard = r.read_int32(bytes)?,
                Ok(40) => msg.grpMsgGetOfficialAccount = r.read_int32(bytes)?,
                Ok(48) => msg.grpMsgGetPayInGroup = r.read_int32(bytes)?,
                Ok(56) => msg.frdMsgDiscuss2ManyChat = r.read_int32(bytes)?,
                Ok(64) => msg.grpMsgNotAllowJoinGrpInviteNotFrd = r.read_int32(bytes)?,
                Ok(72) => msg.frdMsgNeedWaitingMsg = r.read_int32(bytes)?,
                Ok(80) => msg.frdMsgUint32NeedAllUnreadMsg = r.read_int32(bytes)?,
                Ok(88) => msg.grpMsgNeedAutoAdminWording = r.read_int32(bytes)?,
                Ok(96) => msg.grpMsgGetTransferGroupMsgFlag = r.read_int32(bytes)?,
                Ok(104) => msg.grpMsgGetQuitPayGroupMsgFlag = r.read_int32(bytes)?,
                Ok(112) => msg.grpMsgSupportInviteAutoJoin = r.read_int32(bytes)?,
                Ok(120) => msg.grpMsgMaskInviteAutoJoin = r.read_int32(bytes)?,
                Ok(128) => msg.grpMsgGetDisbandedByAdmin = r.read_int32(bytes)?,
                Ok(136) => msg.grpMsgGetC2cInviteJoinGroup = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FlagInfo {
    fn get_size(&self) -> usize {
        0
        + if self.grpMsgKickAdmin == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgKickAdmin) as u64) }
        + if self.grpMsgHiddenGrp == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgHiddenGrp) as u64) }
        + if self.grpMsgWordingDown == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgWordingDown) as u64) }
        + if self.frdMsgGetBusiCard == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.frdMsgGetBusiCard) as u64) }
        + if self.grpMsgGetOfficialAccount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgGetOfficialAccount) as u64) }
        + if self.grpMsgGetPayInGroup == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgGetPayInGroup) as u64) }
        + if self.frdMsgDiscuss2ManyChat == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.frdMsgDiscuss2ManyChat) as u64) }
        + if self.grpMsgNotAllowJoinGrpInviteNotFrd == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgNotAllowJoinGrpInviteNotFrd) as u64) }
        + if self.frdMsgNeedWaitingMsg == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.frdMsgNeedWaitingMsg) as u64) }
        + if self.frdMsgUint32NeedAllUnreadMsg == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.frdMsgUint32NeedAllUnreadMsg) as u64) }
        + if self.grpMsgNeedAutoAdminWording == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgNeedAutoAdminWording) as u64) }
        + if self.grpMsgGetTransferGroupMsgFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgGetTransferGroupMsgFlag) as u64) }
        + if self.grpMsgGetQuitPayGroupMsgFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgGetQuitPayGroupMsgFlag) as u64) }
        + if self.grpMsgSupportInviteAutoJoin == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgSupportInviteAutoJoin) as u64) }
        + if self.grpMsgMaskInviteAutoJoin == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.grpMsgMaskInviteAutoJoin) as u64) }
        + if self.grpMsgGetDisbandedByAdmin == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.grpMsgGetDisbandedByAdmin) as u64) }
        + if self.grpMsgGetC2cInviteJoinGroup == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.grpMsgGetC2cInviteJoinGroup) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.grpMsgKickAdmin != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.grpMsgKickAdmin))?; }
        if self.grpMsgHiddenGrp != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.grpMsgHiddenGrp))?; }
        if self.grpMsgWordingDown != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.grpMsgWordingDown))?; }
        if self.frdMsgGetBusiCard != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.frdMsgGetBusiCard))?; }
        if self.grpMsgGetOfficialAccount != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.grpMsgGetOfficialAccount))?; }
        if self.grpMsgGetPayInGroup != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.grpMsgGetPayInGroup))?; }
        if self.frdMsgDiscuss2ManyChat != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.frdMsgDiscuss2ManyChat))?; }
        if self.grpMsgNotAllowJoinGrpInviteNotFrd != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.grpMsgNotAllowJoinGrpInviteNotFrd))?; }
        if self.frdMsgNeedWaitingMsg != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.frdMsgNeedWaitingMsg))?; }
        if self.frdMsgUint32NeedAllUnreadMsg != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.frdMsgUint32NeedAllUnreadMsg))?; }
        if self.grpMsgNeedAutoAdminWording != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.grpMsgNeedAutoAdminWording))?; }
        if self.grpMsgGetTransferGroupMsgFlag != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.grpMsgGetTransferGroupMsgFlag))?; }
        if self.grpMsgGetQuitPayGroupMsgFlag != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.grpMsgGetQuitPayGroupMsgFlag))?; }
        if self.grpMsgSupportInviteAutoJoin != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.grpMsgSupportInviteAutoJoin))?; }
        if self.grpMsgMaskInviteAutoJoin != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.grpMsgMaskInviteAutoJoin))?; }
        if self.grpMsgGetDisbandedByAdmin != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.grpMsgGetDisbandedByAdmin))?; }
        if self.grpMsgGetC2cInviteJoinGroup != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.grpMsgGetC2cInviteJoinGroup))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FriendInfo<'a> {
    pub msgJointFriend: Cow<'a, str>,
    pub msgBlacklist: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for FriendInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msgJointFriend = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.msgBlacklist = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FriendInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.msgJointFriend == "" { 0 } else { 1 + sizeof_len((&self.msgJointFriend).len()) }
        + if self.msgBlacklist == "" { 0 } else { 1 + sizeof_len((&self.msgBlacklist).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgJointFriend != "" { w.write_with_tag(10, |w| w.write_string(&**&self.msgJointFriend))?; }
        if self.msgBlacklist != "" { w.write_with_tag(18, |w| w.write_string(&**&self.msgBlacklist))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SGroupInfo<'a> {
    pub groupAuthType: i32,
    pub displayAction: i32,
    pub msgAlert: Cow<'a, str>,
    pub msgDetailAlert: Cow<'a, str>,
    pub msgOtherAdminDone: Cow<'a, str>,
    pub appPrivilegeFlag: i32,
}

impl<'a> MessageRead<'a> for SGroupInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupAuthType = r.read_int32(bytes)?,
                Ok(16) => msg.displayAction = r.read_int32(bytes)?,
                Ok(26) => msg.msgAlert = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.msgDetailAlert = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.msgOtherAdminDone = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.appPrivilegeFlag = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SGroupInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupAuthType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupAuthType) as u64) }
        + if self.displayAction == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.displayAction) as u64) }
        + if self.msgAlert == "" { 0 } else { 1 + sizeof_len((&self.msgAlert).len()) }
        + if self.msgDetailAlert == "" { 0 } else { 1 + sizeof_len((&self.msgDetailAlert).len()) }
        + if self.msgOtherAdminDone == "" { 0 } else { 1 + sizeof_len((&self.msgOtherAdminDone).len()) }
        + if self.appPrivilegeFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.appPrivilegeFlag) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupAuthType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.groupAuthType))?; }
        if self.displayAction != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.displayAction))?; }
        if self.msgAlert != "" { w.write_with_tag(26, |w| w.write_string(&**&self.msgAlert))?; }
        if self.msgDetailAlert != "" { w.write_with_tag(34, |w| w.write_string(&**&self.msgDetailAlert))?; }
        if self.msgOtherAdminDone != "" { w.write_with_tag(42, |w| w.write_string(&**&self.msgOtherAdminDone))?; }
        if self.appPrivilegeFlag != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.appPrivilegeFlag))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgInviteExt {
    pub srcType: i32,
    pub srcCode: i64,
    pub waitState: i32,
}

impl<'a> MessageRead<'a> for MsgInviteExt {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.srcType = r.read_int32(bytes)?,
                Ok(16) => msg.srcCode = r.read_int64(bytes)?,
                Ok(24) => msg.waitState = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MsgInviteExt {
    fn get_size(&self) -> usize {
        0
        + if self.srcType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.srcType) as u64) }
        + if self.srcCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.srcCode) as u64) }
        + if self.waitState == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.waitState) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.srcType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.srcType))?; }
        if self.srcCode != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.srcCode))?; }
        if self.waitState != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.waitState))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MsgPayGroupExt {
    pub joinGrpTime: i64,
    pub quitGrpTime: i64,
}

impl<'a> MessageRead<'a> for MsgPayGroupExt {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.joinGrpTime = r.read_int64(bytes)?,
                Ok(16) => msg.quitGrpTime = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MsgPayGroupExt {
    fn get_size(&self) -> usize {
        0
        + if self.joinGrpTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.joinGrpTime) as u64) }
        + if self.quitGrpTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.quitGrpTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.joinGrpTime != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.joinGrpTime))?; }
        if self.quitGrpTime != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.quitGrpTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqNextSystemMsg {
    pub msgNum: i32,
    pub followingFriendSeq: i64,
    pub followingGroupSeq: i64,
    pub checktype: i32,
    pub flag: Option<FlagInfo>,
    pub language: i32,
    pub version: i32,
    pub friendMsgTypeFlag: i64,
}

impl<'a> MessageRead<'a> for ReqNextSystemMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgNum = r.read_int32(bytes)?,
                Ok(16) => msg.followingFriendSeq = r.read_int64(bytes)?,
                Ok(24) => msg.followingGroupSeq = r.read_int64(bytes)?,
                Ok(32) => msg.checktype = r.read_int32(bytes)?,
                Ok(42) => msg.flag = Some(r.read_message::<FlagInfo>(bytes)?),
                Ok(48) => msg.language = r.read_int32(bytes)?,
                Ok(56) => msg.version = r.read_int32(bytes)?,
                Ok(64) => msg.friendMsgTypeFlag = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReqNextSystemMsg {
    fn get_size(&self) -> usize {
        0
        + if self.msgNum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgNum) as u64) }
        + if self.followingFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingFriendSeq) as u64) }
        + if self.followingGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingGroupSeq) as u64) }
        + if self.checktype == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.checktype) as u64) }
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.language == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.language) as u64) }
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.friendMsgTypeFlag == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.friendMsgTypeFlag) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgNum != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.msgNum))?; }
        if self.followingFriendSeq != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.followingFriendSeq))?; }
        if self.followingGroupSeq != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.followingGroupSeq))?; }
        if self.checktype != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.checktype))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.language != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.language))?; }
        if self.version != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.version))?; }
        if self.friendMsgTypeFlag != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.friendMsgTypeFlag))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqSystemMsg {
    pub msgNum: i32,
    pub latestFriendSeq: i64,
    pub latestGroupSeq: i64,
    pub version: i32,
    pub language: i32,
}

impl<'a> MessageRead<'a> for ReqSystemMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgNum = r.read_int32(bytes)?,
                Ok(16) => msg.latestFriendSeq = r.read_int64(bytes)?,
                Ok(24) => msg.latestGroupSeq = r.read_int64(bytes)?,
                Ok(32) => msg.version = r.read_int32(bytes)?,
                Ok(40) => msg.language = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReqSystemMsg {
    fn get_size(&self) -> usize {
        0
        + if self.msgNum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgNum) as u64) }
        + if self.latestFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestFriendSeq) as u64) }
        + if self.latestGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestGroupSeq) as u64) }
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.language == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.language) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgNum != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.msgNum))?; }
        if self.latestFriendSeq != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.latestFriendSeq))?; }
        if self.latestGroupSeq != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.latestGroupSeq))?; }
        if self.version != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.version))?; }
        if self.language != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.language))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqSystemMsgAction<'a> {
    pub msgType: i32,
    pub msgSeq: i64,
    pub reqUin: i64,
    pub subType: i32,
    pub srcId: i32,
    pub subSrcId: i32,
    pub groupMsgType: i32,
    pub actionInfo: Option<SystemMsgActionInfo<'a>>,
    pub language: i32,
}

impl<'a> MessageRead<'a> for ReqSystemMsgAction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgType = r.read_int32(bytes)?,
                Ok(16) => msg.msgSeq = r.read_int64(bytes)?,
                Ok(24) => msg.reqUin = r.read_int64(bytes)?,
                Ok(32) => msg.subType = r.read_int32(bytes)?,
                Ok(40) => msg.srcId = r.read_int32(bytes)?,
                Ok(48) => msg.subSrcId = r.read_int32(bytes)?,
                Ok(56) => msg.groupMsgType = r.read_int32(bytes)?,
                Ok(66) => msg.actionInfo = Some(r.read_message::<SystemMsgActionInfo>(bytes)?),
                Ok(72) => msg.language = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReqSystemMsgAction<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.msgSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgSeq) as u64) }
        + if self.reqUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.reqUin) as u64) }
        + if self.subType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subType) as u64) }
        + if self.srcId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.srcId) as u64) }
        + if self.subSrcId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subSrcId) as u64) }
        + if self.groupMsgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupMsgType) as u64) }
        + self.actionInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.language == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.language) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.msgType))?; }
        if self.msgSeq != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.msgSeq))?; }
        if self.reqUin != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.reqUin))?; }
        if self.subType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.subType))?; }
        if self.srcId != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.srcId))?; }
        if self.subSrcId != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.subSrcId))?; }
        if self.groupMsgType != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.groupMsgType))?; }
        if let Some(ref s) = self.actionInfo { w.write_with_tag(66, |w| w.write_message(s))?; }
        if self.language != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.language))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqSystemMsgNew {
    pub msgNum: i32,
    pub latestFriendSeq: i64,
    pub latestGroupSeq: i64,
    pub version: i32,
    pub checktype: i32,
    pub flag: Option<FlagInfo>,
    pub language: i32,
    pub isGetFrdRibbon: bool,
    pub isGetGrpRibbon: bool,
    pub friendMsgTypeFlag: i64,
    pub reqMsgType: i32,
}

impl<'a> MessageRead<'a> for ReqSystemMsgNew {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgNum = r.read_int32(bytes)?,
                Ok(16) => msg.latestFriendSeq = r.read_int64(bytes)?,
                Ok(24) => msg.latestGroupSeq = r.read_int64(bytes)?,
                Ok(32) => msg.version = r.read_int32(bytes)?,
                Ok(40) => msg.checktype = r.read_int32(bytes)?,
                Ok(50) => msg.flag = Some(r.read_message::<FlagInfo>(bytes)?),
                Ok(56) => msg.language = r.read_int32(bytes)?,
                Ok(64) => msg.isGetFrdRibbon = r.read_bool(bytes)?,
                Ok(72) => msg.isGetGrpRibbon = r.read_bool(bytes)?,
                Ok(80) => msg.friendMsgTypeFlag = r.read_int64(bytes)?,
                Ok(88) => msg.reqMsgType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReqSystemMsgNew {
    fn get_size(&self) -> usize {
        0
        + if self.msgNum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgNum) as u64) }
        + if self.latestFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestFriendSeq) as u64) }
        + if self.latestGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestGroupSeq) as u64) }
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.checktype == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.checktype) as u64) }
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.language == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.language) as u64) }
        + if self.isGetFrdRibbon == false { 0 } else { 1 + sizeof_varint(*(&self.isGetFrdRibbon) as u64) }
        + if self.isGetGrpRibbon == false { 0 } else { 1 + sizeof_varint(*(&self.isGetGrpRibbon) as u64) }
        + if self.friendMsgTypeFlag == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.friendMsgTypeFlag) as u64) }
        + if self.reqMsgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqMsgType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgNum != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.msgNum))?; }
        if self.latestFriendSeq != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.latestFriendSeq))?; }
        if self.latestGroupSeq != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.latestGroupSeq))?; }
        if self.version != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.version))?; }
        if self.checktype != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.checktype))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.language != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.language))?; }
        if self.isGetFrdRibbon != false { w.write_with_tag(64, |w| w.write_bool(*&self.isGetFrdRibbon))?; }
        if self.isGetGrpRibbon != false { w.write_with_tag(72, |w| w.write_bool(*&self.isGetGrpRibbon))?; }
        if self.friendMsgTypeFlag != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.friendMsgTypeFlag))?; }
        if self.reqMsgType != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.reqMsgType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReqSystemMsgRead {
    pub latestFriendSeq: i64,
    pub latestGroupSeq: i64,
    pub type_pb: i32,
    pub checktype: i32,
}

impl<'a> MessageRead<'a> for ReqSystemMsgRead {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.latestFriendSeq = r.read_int64(bytes)?,
                Ok(16) => msg.latestGroupSeq = r.read_int64(bytes)?,
                Ok(24) => msg.type_pb = r.read_int32(bytes)?,
                Ok(32) => msg.checktype = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReqSystemMsgRead {
    fn get_size(&self) -> usize {
        0
        + if self.latestFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestFriendSeq) as u64) }
        + if self.latestGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestGroupSeq) as u64) }
        + if self.type_pb == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.checktype == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.checktype) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.latestFriendSeq != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.latestFriendSeq))?; }
        if self.latestGroupSeq != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.latestGroupSeq))?; }
        if self.type_pb != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.type_pb))?; }
        if self.checktype != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.checktype))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspHead<'a> {
    pub result: i32,
    pub msgFail: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RspHead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = r.read_int32(bytes)?,
                Ok(18) => msg.msgFail = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspHead<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.result == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.result) as u64) }
        + if self.msgFail == "" { 0 } else { 1 + sizeof_len((&self.msgFail).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.result != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.result))?; }
        if self.msgFail != "" { w.write_with_tag(18, |w| w.write_string(&**&self.msgFail))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspNextSystemMsg<'a> {
    pub head: Option<RspHead<'a>>,
    pub msgs: Vec<StructMsg<'a>>,
    pub followingFriendSeq: i64,
    pub followingGroupSeq: i64,
    pub checktype: i32,
    pub gameNick: Cow<'a, str>,
    pub undecidForQim: Cow<'a, [u8]>,
    pub unReadCount3: i32,
}

impl<'a> MessageRead<'a> for RspNextSystemMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.head = Some(r.read_message::<RspHead>(bytes)?),
                Ok(18) => msg.msgs.push(r.read_message::<StructMsg>(bytes)?),
                Ok(24) => msg.followingFriendSeq = r.read_int64(bytes)?,
                Ok(32) => msg.followingGroupSeq = r.read_int64(bytes)?,
                Ok(40) => msg.checktype = r.read_int32(bytes)?,
                Ok(802) => msg.gameNick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(810) => msg.undecidForQim = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(816) => msg.unReadCount3 = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspNextSystemMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + self.head.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.followingFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingFriendSeq) as u64) }
        + if self.followingGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingGroupSeq) as u64) }
        + if self.checktype == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.checktype) as u64) }
        + if self.gameNick == "" { 0 } else { 2 + sizeof_len((&self.gameNick).len()) }
        + if self.undecidForQim == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.undecidForQim).len()) }
        + if self.unReadCount3 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.unReadCount3) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.head { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.msgs { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.followingFriendSeq != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.followingFriendSeq))?; }
        if self.followingGroupSeq != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.followingGroupSeq))?; }
        if self.checktype != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.checktype))?; }
        if self.gameNick != "" { w.write_with_tag(802, |w| w.write_string(&**&self.gameNick))?; }
        if self.undecidForQim != Cow::Borrowed(b"") { w.write_with_tag(810, |w| w.write_bytes(&**&self.undecidForQim))?; }
        if self.unReadCount3 != 0i32 { w.write_with_tag(816, |w| w.write_int32(*&self.unReadCount3))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspSystemMsg<'a> {
    pub head: Option<RspHead<'a>>,
    pub msgs: Vec<StructMsg<'a>>,
    pub unreadCount: i32,
    pub latestFriendSeq: i64,
    pub latestGroupSeq: i64,
    pub followingFriendSeq: i64,
    pub followingGroupSeq: i64,
    pub msgDisplay: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RspSystemMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.head = Some(r.read_message::<RspHead>(bytes)?),
                Ok(18) => msg.msgs.push(r.read_message::<StructMsg>(bytes)?),
                Ok(24) => msg.unreadCount = r.read_int32(bytes)?,
                Ok(32) => msg.latestFriendSeq = r.read_int64(bytes)?,
                Ok(40) => msg.latestGroupSeq = r.read_int64(bytes)?,
                Ok(48) => msg.followingFriendSeq = r.read_int64(bytes)?,
                Ok(56) => msg.followingGroupSeq = r.read_int64(bytes)?,
                Ok(66) => msg.msgDisplay = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspSystemMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + self.head.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.unreadCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.unreadCount) as u64) }
        + if self.latestFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestFriendSeq) as u64) }
        + if self.latestGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestGroupSeq) as u64) }
        + if self.followingFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingFriendSeq) as u64) }
        + if self.followingGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingGroupSeq) as u64) }
        + if self.msgDisplay == "" { 0 } else { 1 + sizeof_len((&self.msgDisplay).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.head { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.msgs { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.unreadCount != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.unreadCount))?; }
        if self.latestFriendSeq != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.latestFriendSeq))?; }
        if self.latestGroupSeq != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.latestGroupSeq))?; }
        if self.followingFriendSeq != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.followingFriendSeq))?; }
        if self.followingGroupSeq != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.followingGroupSeq))?; }
        if self.msgDisplay != "" { w.write_with_tag(66, |w| w.write_string(&**&self.msgDisplay))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspSystemMsgAction<'a> {
    pub head: Option<RspHead<'a>>,
    pub msgDetail: Cow<'a, str>,
    pub type_pb: i32,
    pub msgInvalidDecided: Cow<'a, str>,
    pub remarkResult: i32,
}

impl<'a> MessageRead<'a> for RspSystemMsgAction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.head = Some(r.read_message::<RspHead>(bytes)?),
                Ok(18) => msg.msgDetail = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.type_pb = r.read_int32(bytes)?,
                Ok(42) => msg.msgInvalidDecided = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.remarkResult = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspSystemMsgAction<'a> {
    fn get_size(&self) -> usize {
        0
        + self.head.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.msgDetail == "" { 0 } else { 1 + sizeof_len((&self.msgDetail).len()) }
        + if self.type_pb == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.msgInvalidDecided == "" { 0 } else { 1 + sizeof_len((&self.msgInvalidDecided).len()) }
        + if self.remarkResult == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.remarkResult) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.head { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.msgDetail != "" { w.write_with_tag(18, |w| w.write_string(&**&self.msgDetail))?; }
        if self.type_pb != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.type_pb))?; }
        if self.msgInvalidDecided != "" { w.write_with_tag(42, |w| w.write_string(&**&self.msgInvalidDecided))?; }
        if self.remarkResult != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.remarkResult))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspSystemMsgNew<'a> {
    pub head: Option<RspHead<'a>>,
    pub unreadFriendCount: i32,
    pub unreadGroupCount: i32,
    pub latestFriendSeq: i64,
    pub latestGroupSeq: i64,
    pub followingFriendSeq: i64,
    pub followingGroupSeq: i64,
    pub friendmsgs: Vec<StructMsg<'a>>,
    pub groupmsgs: Vec<StructMsg<'a>>,
    pub msgRibbonFriend: Option<StructMsg<'a>>,
    pub msgRibbonGroup: Option<StructMsg<'a>>,
    pub msgDisplay: Cow<'a, str>,
    pub grpMsgDisplay: Cow<'a, str>,
    pub over: i32,
    pub checktype: i32,
    pub gameNick: Cow<'a, str>,
    pub undecidForQim: Cow<'a, [u8]>,
    pub unReadCount3: i32,
}

impl<'a> MessageRead<'a> for RspSystemMsgNew<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.head = Some(r.read_message::<RspHead>(bytes)?),
                Ok(16) => msg.unreadFriendCount = r.read_int32(bytes)?,
                Ok(24) => msg.unreadGroupCount = r.read_int32(bytes)?,
                Ok(32) => msg.latestFriendSeq = r.read_int64(bytes)?,
                Ok(40) => msg.latestGroupSeq = r.read_int64(bytes)?,
                Ok(48) => msg.followingFriendSeq = r.read_int64(bytes)?,
                Ok(56) => msg.followingGroupSeq = r.read_int64(bytes)?,
                Ok(74) => msg.friendmsgs.push(r.read_message::<StructMsg>(bytes)?),
                Ok(82) => msg.groupmsgs.push(r.read_message::<StructMsg>(bytes)?),
                Ok(90) => msg.msgRibbonFriend = Some(r.read_message::<StructMsg>(bytes)?),
                Ok(98) => msg.msgRibbonGroup = Some(r.read_message::<StructMsg>(bytes)?),
                Ok(106) => msg.msgDisplay = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.grpMsgDisplay = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(120) => msg.over = r.read_int32(bytes)?,
                Ok(160) => msg.checktype = r.read_int32(bytes)?,
                Ok(802) => msg.gameNick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(810) => msg.undecidForQim = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(816) => msg.unReadCount3 = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspSystemMsgNew<'a> {
    fn get_size(&self) -> usize {
        0
        + self.head.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.unreadFriendCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.unreadFriendCount) as u64) }
        + if self.unreadGroupCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.unreadGroupCount) as u64) }
        + if self.latestFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestFriendSeq) as u64) }
        + if self.latestGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.latestGroupSeq) as u64) }
        + if self.followingFriendSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingFriendSeq) as u64) }
        + if self.followingGroupSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.followingGroupSeq) as u64) }
        + self.friendmsgs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupmsgs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.msgRibbonFriend.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.msgRibbonGroup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.msgDisplay == "" { 0 } else { 1 + sizeof_len((&self.msgDisplay).len()) }
        + if self.grpMsgDisplay == "" { 0 } else { 1 + sizeof_len((&self.grpMsgDisplay).len()) }
        + if self.over == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.over) as u64) }
        + if self.checktype == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.checktype) as u64) }
        + if self.gameNick == "" { 0 } else { 2 + sizeof_len((&self.gameNick).len()) }
        + if self.undecidForQim == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.undecidForQim).len()) }
        + if self.unReadCount3 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.unReadCount3) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.head { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.unreadFriendCount != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.unreadFriendCount))?; }
        if self.unreadGroupCount != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.unreadGroupCount))?; }
        if self.latestFriendSeq != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.latestFriendSeq))?; }
        if self.latestGroupSeq != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.latestGroupSeq))?; }
        if self.followingFriendSeq != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.followingFriendSeq))?; }
        if self.followingGroupSeq != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.followingGroupSeq))?; }
        for s in &self.friendmsgs { w.write_with_tag(74, |w| w.write_message(s))?; }
        for s in &self.groupmsgs { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgRibbonFriend { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgRibbonGroup { w.write_with_tag(98, |w| w.write_message(s))?; }
        if self.msgDisplay != "" { w.write_with_tag(106, |w| w.write_string(&**&self.msgDisplay))?; }
        if self.grpMsgDisplay != "" { w.write_with_tag(114, |w| w.write_string(&**&self.grpMsgDisplay))?; }
        if self.over != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.over))?; }
        if self.checktype != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.checktype))?; }
        if self.gameNick != "" { w.write_with_tag(802, |w| w.write_string(&**&self.gameNick))?; }
        if self.undecidForQim != Cow::Borrowed(b"") { w.write_with_tag(810, |w| w.write_bytes(&**&self.undecidForQim))?; }
        if self.unReadCount3 != 0i32 { w.write_with_tag(816, |w| w.write_int32(*&self.unReadCount3))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RspSystemMsgRead<'a> {
    pub head: Option<RspHead<'a>>,
    pub type_pb: i32,
    pub checktype: i32,
}

impl<'a> MessageRead<'a> for RspSystemMsgRead<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.head = Some(r.read_message::<RspHead>(bytes)?),
                Ok(16) => msg.type_pb = r.read_int32(bytes)?,
                Ok(24) => msg.checktype = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RspSystemMsgRead<'a> {
    fn get_size(&self) -> usize {
        0
        + self.head.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.type_pb == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.checktype == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.checktype) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.head { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.type_pb != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.type_pb))?; }
        if self.checktype != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.checktype))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StructMsg<'a> {
    pub version: i32,
    pub msgType: i32,
    pub msgSeq: i64,
    pub msgTime: i64,
    pub reqUin: i64,
    pub unreadFlag: i32,
    pub msg: Option<SystemMsg<'a>>,
}

impl<'a> MessageRead<'a> for StructMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_int32(bytes)?,
                Ok(16) => msg.msgType = r.read_int32(bytes)?,
                Ok(24) => msg.msgSeq = r.read_int64(bytes)?,
                Ok(32) => msg.msgTime = r.read_int64(bytes)?,
                Ok(40) => msg.reqUin = r.read_int64(bytes)?,
                Ok(48) => msg.unreadFlag = r.read_int32(bytes)?,
                Ok(402) => msg.msg = Some(r.read_message::<SystemMsg>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StructMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.msgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.msgType) as u64) }
        + if self.msgSeq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgSeq) as u64) }
        + if self.msgTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.msgTime) as u64) }
        + if self.reqUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.reqUin) as u64) }
        + if self.unreadFlag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.unreadFlag) as u64) }
        + self.msg.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.version))?; }
        if self.msgType != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.msgType))?; }
        if self.msgSeq != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.msgSeq))?; }
        if self.msgTime != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.msgTime))?; }
        if self.reqUin != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.reqUin))?; }
        if self.unreadFlag != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.unreadFlag))?; }
        if let Some(ref s) = self.msg { w.write_with_tag(402, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SystemMsg<'a> {
    pub subType: i32,
    pub msgTitle: Cow<'a, str>,
    pub msgDescribe: Cow<'a, str>,
    pub msgAdditional: Cow<'a, str>,
    pub msgSource: Cow<'a, str>,
    pub msgDecided: Cow<'a, str>,
    pub srcId: i32,
    pub subSrcId: i32,
    pub actions: Vec<SystemMsgAction<'a>>,
    pub groupCode: i64,
    pub actionUin: i64,
    pub groupMsgType: i32,
    pub groupInviterRole: i32,
    pub friendInfo: Option<FriendInfo<'a>>,
    pub groupInfo: Option<SGroupInfo<'a>>,
    pub actorUin: i64,
    pub msgActorDescribe: Cow<'a, str>,
    pub msgAdditionalList: Cow<'a, str>,
    pub relation: i32,
    pub reqsubtype: i32,
    pub cloneUin: i64,
    pub discussUin: i64,
    pub eimGroupId: i64,
    pub msgInviteExtinfo: Option<MsgInviteExt>,
    pub msgPayGroupExtinfo: Option<MsgPayGroupExt>,
    pub sourceFlag: i32,
    pub gameNick: Cow<'a, [u8]>,
    pub gameMsg: Cow<'a, [u8]>,
    pub groupFlagext3: i32,
    pub groupOwnerUin: i64,
    pub doubtFlag: i32,
    pub warningTips: Cow<'a, [u8]>,
    pub nameMore: Cow<'a, [u8]>,
    pub reqUinFaceid: i32,
    pub reqUinNick: Cow<'a, str>,
    pub groupName: Cow<'a, str>,
    pub actionUinNick: Cow<'a, str>,
    pub msgQna: Cow<'a, str>,
    pub msgDetail: Cow<'a, str>,
    pub groupExtFlag: i32,
    pub actorUinNick: Cow<'a, str>,
    pub picUrl: Cow<'a, str>,
    pub cloneUinNick: Cow<'a, str>,
    pub reqUinBusinessCard: Cow<'a, str>,
    pub eimGroupIdName: Cow<'a, str>,
    pub reqUinPreRemark: Cow<'a, str>,
    pub actionUinQqNick: Cow<'a, str>,
    pub actionUinRemark: Cow<'a, str>,
    pub reqUinGender: i32,
    pub reqUinAge: i32,
    pub c2cInviteJoinGroupFlag: i32,
    pub cardSwitch: i32,
}

impl<'a> MessageRead<'a> for SystemMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subType = r.read_int32(bytes)?,
                Ok(18) => msg.msgTitle = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.msgDescribe = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.msgAdditional = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.msgSource = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.msgDecided = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.srcId = r.read_int32(bytes)?,
                Ok(64) => msg.subSrcId = r.read_int32(bytes)?,
                Ok(74) => msg.actions.push(r.read_message::<SystemMsgAction>(bytes)?),
                Ok(80) => msg.groupCode = r.read_int64(bytes)?,
                Ok(88) => msg.actionUin = r.read_int64(bytes)?,
                Ok(96) => msg.groupMsgType = r.read_int32(bytes)?,
                Ok(104) => msg.groupInviterRole = r.read_int32(bytes)?,
                Ok(114) => msg.friendInfo = Some(r.read_message::<FriendInfo>(bytes)?),
                Ok(122) => msg.groupInfo = Some(r.read_message::<SGroupInfo>(bytes)?),
                Ok(128) => msg.actorUin = r.read_int64(bytes)?,
                Ok(138) => msg.msgActorDescribe = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(146) => msg.msgAdditionalList = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(152) => msg.relation = r.read_int32(bytes)?,
                Ok(160) => msg.reqsubtype = r.read_int32(bytes)?,
                Ok(168) => msg.cloneUin = r.read_int64(bytes)?,
                Ok(176) => msg.discussUin = r.read_int64(bytes)?,
                Ok(184) => msg.eimGroupId = r.read_int64(bytes)?,
                Ok(194) => msg.msgInviteExtinfo = Some(r.read_message::<MsgInviteExt>(bytes)?),
                Ok(202) => msg.msgPayGroupExtinfo = Some(r.read_message::<MsgPayGroupExt>(bytes)?),
                Ok(208) => msg.sourceFlag = r.read_int32(bytes)?,
                Ok(218) => msg.gameNick = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(226) => msg.gameMsg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(232) => msg.groupFlagext3 = r.read_int32(bytes)?,
                Ok(240) => msg.groupOwnerUin = r.read_int64(bytes)?,
                Ok(248) => msg.doubtFlag = r.read_int32(bytes)?,
                Ok(258) => msg.warningTips = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(266) => msg.nameMore = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(400) => msg.reqUinFaceid = r.read_int32(bytes)?,
                Ok(410) => msg.reqUinNick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(418) => msg.groupName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(426) => msg.actionUinNick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(434) => msg.msgQna = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(442) => msg.msgDetail = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(456) => msg.groupExtFlag = r.read_int32(bytes)?,
                Ok(466) => msg.actorUinNick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(474) => msg.picUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(482) => msg.cloneUinNick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(490) => msg.reqUinBusinessCard = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(506) => msg.eimGroupIdName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(514) => msg.reqUinPreRemark = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(522) => msg.actionUinQqNick = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(530) => msg.actionUinRemark = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(536) => msg.reqUinGender = r.read_int32(bytes)?,
                Ok(544) => msg.reqUinAge = r.read_int32(bytes)?,
                Ok(552) => msg.c2cInviteJoinGroupFlag = r.read_int32(bytes)?,
                Ok(808) => msg.cardSwitch = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SystemMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.subType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subType) as u64) }
        + if self.msgTitle == "" { 0 } else { 1 + sizeof_len((&self.msgTitle).len()) }
        + if self.msgDescribe == "" { 0 } else { 1 + sizeof_len((&self.msgDescribe).len()) }
        + if self.msgAdditional == "" { 0 } else { 1 + sizeof_len((&self.msgAdditional).len()) }
        + if self.msgSource == "" { 0 } else { 1 + sizeof_len((&self.msgSource).len()) }
        + if self.msgDecided == "" { 0 } else { 1 + sizeof_len((&self.msgDecided).len()) }
        + if self.srcId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.srcId) as u64) }
        + if self.subSrcId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.subSrcId) as u64) }
        + self.actions.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.actionUin == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.actionUin) as u64) }
        + if self.groupMsgType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupMsgType) as u64) }
        + if self.groupInviterRole == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.groupInviterRole) as u64) }
        + self.friendInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.groupInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.actorUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.actorUin) as u64) }
        + if self.msgActorDescribe == "" { 0 } else { 2 + sizeof_len((&self.msgActorDescribe).len()) }
        + if self.msgAdditionalList == "" { 0 } else { 2 + sizeof_len((&self.msgAdditionalList).len()) }
        + if self.relation == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.relation) as u64) }
        + if self.reqsubtype == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.reqsubtype) as u64) }
        + if self.cloneUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.cloneUin) as u64) }
        + if self.discussUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.discussUin) as u64) }
        + if self.eimGroupId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.eimGroupId) as u64) }
        + self.msgInviteExtinfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.msgPayGroupExtinfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.sourceFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.sourceFlag) as u64) }
        + if self.gameNick == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.gameNick).len()) }
        + if self.gameMsg == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.gameMsg).len()) }
        + if self.groupFlagext3 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupFlagext3) as u64) }
        + if self.groupOwnerUin == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.groupOwnerUin) as u64) }
        + if self.doubtFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.doubtFlag) as u64) }
        + if self.warningTips == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.warningTips).len()) }
        + if self.nameMore == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.nameMore).len()) }
        + if self.reqUinFaceid == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.reqUinFaceid) as u64) }
        + if self.reqUinNick == "" { 0 } else { 2 + sizeof_len((&self.reqUinNick).len()) }
        + if self.groupName == "" { 0 } else { 2 + sizeof_len((&self.groupName).len()) }
        + if self.actionUinNick == "" { 0 } else { 2 + sizeof_len((&self.actionUinNick).len()) }
        + if self.msgQna == "" { 0 } else { 2 + sizeof_len((&self.msgQna).len()) }
        + if self.msgDetail == "" { 0 } else { 2 + sizeof_len((&self.msgDetail).len()) }
        + if self.groupExtFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupExtFlag) as u64) }
        + if self.actorUinNick == "" { 0 } else { 2 + sizeof_len((&self.actorUinNick).len()) }
        + if self.picUrl == "" { 0 } else { 2 + sizeof_len((&self.picUrl).len()) }
        + if self.cloneUinNick == "" { 0 } else { 2 + sizeof_len((&self.cloneUinNick).len()) }
        + if self.reqUinBusinessCard == "" { 0 } else { 2 + sizeof_len((&self.reqUinBusinessCard).len()) }
        + if self.eimGroupIdName == "" { 0 } else { 2 + sizeof_len((&self.eimGroupIdName).len()) }
        + if self.reqUinPreRemark == "" { 0 } else { 2 + sizeof_len((&self.reqUinPreRemark).len()) }
        + if self.actionUinQqNick == "" { 0 } else { 2 + sizeof_len((&self.actionUinQqNick).len()) }
        + if self.actionUinRemark == "" { 0 } else { 2 + sizeof_len((&self.actionUinRemark).len()) }
        + if self.reqUinGender == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.reqUinGender) as u64) }
        + if self.reqUinAge == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.reqUinAge) as u64) }
        + if self.c2cInviteJoinGroupFlag == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.c2cInviteJoinGroupFlag) as u64) }
        + if self.cardSwitch == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.cardSwitch) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subType != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.subType))?; }
        if self.msgTitle != "" { w.write_with_tag(18, |w| w.write_string(&**&self.msgTitle))?; }
        if self.msgDescribe != "" { w.write_with_tag(26, |w| w.write_string(&**&self.msgDescribe))?; }
        if self.msgAdditional != "" { w.write_with_tag(34, |w| w.write_string(&**&self.msgAdditional))?; }
        if self.msgSource != "" { w.write_with_tag(42, |w| w.write_string(&**&self.msgSource))?; }
        if self.msgDecided != "" { w.write_with_tag(50, |w| w.write_string(&**&self.msgDecided))?; }
        if self.srcId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.srcId))?; }
        if self.subSrcId != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.subSrcId))?; }
        for s in &self.actions { w.write_with_tag(74, |w| w.write_message(s))?; }
        if self.groupCode != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.groupCode))?; }
        if self.actionUin != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.actionUin))?; }
        if self.groupMsgType != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.groupMsgType))?; }
        if self.groupInviterRole != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.groupInviterRole))?; }
        if let Some(ref s) = self.friendInfo { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupInfo { w.write_with_tag(122, |w| w.write_message(s))?; }
        if self.actorUin != 0i64 { w.write_with_tag(128, |w| w.write_int64(*&self.actorUin))?; }
        if self.msgActorDescribe != "" { w.write_with_tag(138, |w| w.write_string(&**&self.msgActorDescribe))?; }
        if self.msgAdditionalList != "" { w.write_with_tag(146, |w| w.write_string(&**&self.msgAdditionalList))?; }
        if self.relation != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.relation))?; }
        if self.reqsubtype != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.reqsubtype))?; }
        if self.cloneUin != 0i64 { w.write_with_tag(168, |w| w.write_int64(*&self.cloneUin))?; }
        if self.discussUin != 0i64 { w.write_with_tag(176, |w| w.write_int64(*&self.discussUin))?; }
        if self.eimGroupId != 0i64 { w.write_with_tag(184, |w| w.write_int64(*&self.eimGroupId))?; }
        if let Some(ref s) = self.msgInviteExtinfo { w.write_with_tag(194, |w| w.write_message(s))?; }
        if let Some(ref s) = self.msgPayGroupExtinfo { w.write_with_tag(202, |w| w.write_message(s))?; }
        if self.sourceFlag != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.sourceFlag))?; }
        if self.gameNick != Cow::Borrowed(b"") { w.write_with_tag(218, |w| w.write_bytes(&**&self.gameNick))?; }
        if self.gameMsg != Cow::Borrowed(b"") { w.write_with_tag(226, |w| w.write_bytes(&**&self.gameMsg))?; }
        if self.groupFlagext3 != 0i32 { w.write_with_tag(232, |w| w.write_int32(*&self.groupFlagext3))?; }
        if self.groupOwnerUin != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.groupOwnerUin))?; }
        if self.doubtFlag != 0i32 { w.write_with_tag(248, |w| w.write_int32(*&self.doubtFlag))?; }
        if self.warningTips != Cow::Borrowed(b"") { w.write_with_tag(258, |w| w.write_bytes(&**&self.warningTips))?; }
        if self.nameMore != Cow::Borrowed(b"") { w.write_with_tag(266, |w| w.write_bytes(&**&self.nameMore))?; }
        if self.reqUinFaceid != 0i32 { w.write_with_tag(400, |w| w.write_int32(*&self.reqUinFaceid))?; }
        if self.reqUinNick != "" { w.write_with_tag(410, |w| w.write_string(&**&self.reqUinNick))?; }
        if self.groupName != "" { w.write_with_tag(418, |w| w.write_string(&**&self.groupName))?; }
        if self.actionUinNick != "" { w.write_with_tag(426, |w| w.write_string(&**&self.actionUinNick))?; }
        if self.msgQna != "" { w.write_with_tag(434, |w| w.write_string(&**&self.msgQna))?; }
        if self.msgDetail != "" { w.write_with_tag(442, |w| w.write_string(&**&self.msgDetail))?; }
        if self.groupExtFlag != 0i32 { w.write_with_tag(456, |w| w.write_int32(*&self.groupExtFlag))?; }
        if self.actorUinNick != "" { w.write_with_tag(466, |w| w.write_string(&**&self.actorUinNick))?; }
        if self.picUrl != "" { w.write_with_tag(474, |w| w.write_string(&**&self.picUrl))?; }
        if self.cloneUinNick != "" { w.write_with_tag(482, |w| w.write_string(&**&self.cloneUinNick))?; }
        if self.reqUinBusinessCard != "" { w.write_with_tag(490, |w| w.write_string(&**&self.reqUinBusinessCard))?; }
        if self.eimGroupIdName != "" { w.write_with_tag(506, |w| w.write_string(&**&self.eimGroupIdName))?; }
        if self.reqUinPreRemark != "" { w.write_with_tag(514, |w| w.write_string(&**&self.reqUinPreRemark))?; }
        if self.actionUinQqNick != "" { w.write_with_tag(522, |w| w.write_string(&**&self.actionUinQqNick))?; }
        if self.actionUinRemark != "" { w.write_with_tag(530, |w| w.write_string(&**&self.actionUinRemark))?; }
        if self.reqUinGender != 0i32 { w.write_with_tag(536, |w| w.write_int32(*&self.reqUinGender))?; }
        if self.reqUinAge != 0i32 { w.write_with_tag(544, |w| w.write_int32(*&self.reqUinAge))?; }
        if self.c2cInviteJoinGroupFlag != 0i32 { w.write_with_tag(552, |w| w.write_int32(*&self.c2cInviteJoinGroupFlag))?; }
        if self.cardSwitch != 0i32 { w.write_with_tag(808, |w| w.write_int32(*&self.cardSwitch))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SystemMsgAction<'a> {
    pub name: Cow<'a, str>,
    pub result: Cow<'a, str>,
    pub action: i32,
    pub actionInfo: Option<SystemMsgActionInfo<'a>>,
    pub detailName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SystemMsgAction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.result = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.action = r.read_int32(bytes)?,
                Ok(34) => msg.actionInfo = Some(r.read_message::<SystemMsgActionInfo>(bytes)?),
                Ok(42) => msg.detailName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SystemMsgAction<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.result == "" { 0 } else { 1 + sizeof_len((&self.result).len()) }
        + if self.action == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.action) as u64) }
        + self.actionInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.detailName == "" { 0 } else { 1 + sizeof_len((&self.detailName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.result != "" { w.write_with_tag(18, |w| w.write_string(&**&self.result))?; }
        if self.action != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.action))?; }
        if let Some(ref s) = self.actionInfo { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.detailName != "" { w.write_with_tag(42, |w| w.write_string(&**&self.detailName))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SystemMsgActionInfo<'a> {
    pub type_pb: i32,
    pub groupCode: i64,
    pub sig: Cow<'a, [u8]>,
    pub msg: Cow<'a, str>,
    pub groupId: i32,
    pub remark: Cow<'a, str>,
    pub blacklist: bool,
    pub addFrdSNInfo: Option<AddFrdSNInfo>,
}

impl<'a> MessageRead<'a> for SystemMsgActionInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_int32(bytes)?,
                Ok(16) => msg.groupCode = r.read_int64(bytes)?,
                Ok(26) => msg.sig = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(402) => msg.msg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(408) => msg.groupId = r.read_int32(bytes)?,
                Ok(418) => msg.remark = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(424) => msg.blacklist = r.read_bool(bytes)?,
                Ok(434) => msg.addFrdSNInfo = Some(r.read_message::<AddFrdSNInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SystemMsgActionInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.groupCode == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.groupCode) as u64) }
        + if self.sig == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sig).len()) }
        + if self.msg == "" { 0 } else { 2 + sizeof_len((&self.msg).len()) }
        + if self.groupId == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.groupId) as u64) }
        + if self.remark == "" { 0 } else { 2 + sizeof_len((&self.remark).len()) }
        + if self.blacklist == false { 0 } else { 2 + sizeof_varint(*(&self.blacklist) as u64) }
        + self.addFrdSNInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.type_pb))?; }
        if self.groupCode != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.groupCode))?; }
        if self.sig != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.sig))?; }
        if self.msg != "" { w.write_with_tag(402, |w| w.write_string(&**&self.msg))?; }
        if self.groupId != 0i32 { w.write_with_tag(408, |w| w.write_int32(*&self.groupId))?; }
        if self.remark != "" { w.write_with_tag(418, |w| w.write_string(&**&self.remark))?; }
        if self.blacklist != false { w.write_with_tag(424, |w| w.write_bool(*&self.blacklist))?; }
        if let Some(ref s) = self.addFrdSNInfo { w.write_with_tag(434, |w| w.write_message(s))?; }
        Ok(())
    }
}

