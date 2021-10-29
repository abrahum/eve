// Automatically generated rust module for 'subMsgType0x27.proto' file

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
pub struct AddGroup<'a> {
    pub groupid: Option<u32>,
    pub sortid: Option<u32>,
    pub groupname: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for AddGroup<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupid = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.sortid = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.groupname = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AddGroup<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sortid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupid { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sortid { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupname { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppointmentNotify<'a> {
    pub fromUin: Option<u64>,
    pub appointId: Option<Cow<'a, str>>,
    pub notifytype: Option<u32>,
    pub tipsContent: Option<Cow<'a, str>>,
    pub unreadCount: Option<u32>,
    pub joinWording: Option<Cow<'a, str>>,
    pub viewWording: Option<Cow<'a, str>>,
    pub sig: Option<Cow<'a, [u8]>>,
    pub eventInfo: Option<Cow<'a, [u8]>>,
    pub nearbyEventInfo: Option<Cow<'a, [u8]>>,
    pub feedEventInfo: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for AppointmentNotify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.appointId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.notifytype = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.tipsContent = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.unreadCount = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.joinWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.viewWording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.eventInfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.nearbyEventInfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.feedEventInfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AppointmentNotify<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.appointId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.notifytype.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tipsContent.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.unreadCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.joinWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.viewWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.eventInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.nearbyEventInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.feedEventInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.appointId { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.notifytype { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.tipsContent { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.unreadCount { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.joinWording { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.viewWording { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.eventInfo { w.write_with_tag(74, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.nearbyEventInfo { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.feedEventInfo { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BinaryMsg<'a> {
    pub opType: Option<u32>,
    pub opValue: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for BinaryMsg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.opType = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.opValue = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BinaryMsg<'a> {
    fn get_size(&self) -> usize {
        0
        + self.opType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.opValue.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.opType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.opValue { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatMatchInfo<'a> {
    pub sig: Option<Cow<'a, [u8]>>,
    pub uin: Option<u64>,
    pub matchUin: Option<u64>,
    pub tipsWording: Option<Cow<'a, [u8]>>,
    pub leftChatTime: Option<u32>,
    pub timeStamp: Option<u64>,
    pub matchExpiredTime: Option<u32>,
    pub c2CExpiredTime: Option<u32>,
    pub matchCount: Option<u32>,
    pub nick: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ChatMatchInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.matchUin = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.tipsWording = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.leftChatTime = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.timeStamp = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.matchExpiredTime = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.c2CExpiredTime = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.matchCount = Some(r.read_uint32(bytes)?),
                Ok(82) => msg.nick = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatMatchInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.matchUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tipsWording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.leftChatTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.timeStamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.matchExpiredTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.c2CExpiredTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.matchCount.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.nick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.sig { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.matchUin { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.tipsWording { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.leftChatTime { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.timeStamp { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.matchExpiredTime { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.c2CExpiredTime { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.matchCount { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.nick { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConfMsgRoamFlag {
    pub confid: Option<u64>,
    pub flag: Option<u32>,
    pub timestamp: Option<u64>,
}

impl<'a> MessageRead<'a> for ConfMsgRoamFlag {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.confid = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.flag = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.timestamp = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ConfMsgRoamFlag {
    fn get_size(&self) -> usize {
        0
        + self.confid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.confid { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DaRenNotify {
    pub uin: Option<u64>,
    pub loginDays: Option<u32>,
    pub days: Option<u32>,
    pub isYestodayLogin: Option<u32>,
    pub isTodayLogin: Option<u32>,
}

impl<'a> MessageRead<'a> for DaRenNotify {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.loginDays = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.days = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.isYestodayLogin = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.isTodayLogin = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DaRenNotify {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.loginDays.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.days.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.isYestodayLogin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.isTodayLogin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.loginDays { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.days { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.isYestodayLogin { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.isTodayLogin { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DelFriend {
    pub uins: Vec<u64>,
}

impl<'a> MessageRead<'a> for DelFriend {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uins.push(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DelFriend {
    fn get_size(&self) -> usize {
        0
        + self.uins.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.uins { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DelGroup {
    pub groupid: Option<u32>,
}

impl<'a> MessageRead<'a> for DelGroup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupid = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DelGroup {
    fn get_size(&self) -> usize {
        0
        + self.groupid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupid { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FanpaiziNotify<'a> {
    pub fromUin: Option<u64>,
    pub fromNick: Option<Cow<'a, str>>,
    pub tipsContent: Option<Cow<'a, [u8]>>,
    pub sig: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for FanpaiziNotify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.fromNick = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.tipsContent = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.sig = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FanpaiziNotify<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fromNick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.tipsContent.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sig.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.fromNick { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.tipsContent { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.sig { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ForwardBody<'a> {
    pub notifyType: Option<u32>,
    pub opType: Option<u32>,
    pub addGroup: Option<AddGroup<'a>>,
    pub delGroup: Option<DelGroup>,
    pub modGroupName: Option<ModGroupName<'a>>,
    pub modGroupSort: Option<ModGroupSort>,
    pub modFriendGroup: Option<ModFriendGroup>,
    pub modProfile: Option<ModProfile<'a>>,
    pub modFriendRemark: Option<ModFriendRemark<'a>>,
    pub modLongNick: Option<ModLongNick<'a>>,
    pub modCustomFace: Option<ModCustomFace>,
    pub modGroupProfile: Option<ModGroupProfile<'a>>,
    pub modGroupMemberProfile: Option<ModGroupMemberProfile<'a>>,
    pub delFriend: Option<DelFriend>,
    pub roamPriv: Option<ModFrdRoamPriv>,
    pub grpMsgRoamFlag: Option<GrpMsgRoamFlag>,
    pub confMsgRoamFlag: Option<ConfMsgRoamFlag>,
    pub modRichLongNick: Option<ModLongNick<'a>>,
    pub binPkg: Option<BinaryMsg<'a>>,
    pub modFriendRings: Option<ModSnsGeneralInfo<'a>>,
    pub modConfProfile: Option<ModConfProfile<'a>>,
    pub modFriendFlag: Option<SnsUpdateFlag>,
    pub appointmentNotify: Option<AppointmentNotify<'a>>,
    pub darenNotify: Option<DaRenNotify>,
    pub newComeinUserNotify: Option<NewComeinUserNotify<'a>>,
    pub pushSearchDev: Option<PushSearchDev<'a>>,
    pub pushReportDev: Option<PushReportDev<'a>>,
    pub qqPayPush: Option<QQPayPush>,
    pub redpointInfo: Option<Cow<'a, [u8]>>,
    pub hotFriendNotify: Option<HotFriendNotify>,
    pub praiseRankNotify: Option<PraiseRankNotify<'a>>,
    pub campusNotify: Option<MQQCampusNotify<'a>>,
    pub modRichLongNickEx: Option<ModLongNick<'a>>,
    pub chatMatchInfo: Option<ChatMatchInfo<'a>>,
    pub frdCustomOnlineStatusChange: Option<FrdCustomOnlineStatusChange>,
    pub fanpanziNotify: Option<FanpaiziNotify<'a>>,
}

impl<'a> MessageRead<'a> for ForwardBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.notifyType = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.opType = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.addGroup = Some(r.read_message::<AddGroup>(bytes)?),
                Ok(34) => msg.delGroup = Some(r.read_message::<DelGroup>(bytes)?),
                Ok(42) => msg.modGroupName = Some(r.read_message::<ModGroupName>(bytes)?),
                Ok(50) => msg.modGroupSort = Some(r.read_message::<ModGroupSort>(bytes)?),
                Ok(58) => msg.modFriendGroup = Some(r.read_message::<ModFriendGroup>(bytes)?),
                Ok(66) => msg.modProfile = Some(r.read_message::<ModProfile>(bytes)?),
                Ok(74) => msg.modFriendRemark = Some(r.read_message::<ModFriendRemark>(bytes)?),
                Ok(82) => msg.modLongNick = Some(r.read_message::<ModLongNick>(bytes)?),
                Ok(90) => msg.modCustomFace = Some(r.read_message::<ModCustomFace>(bytes)?),
                Ok(98) => msg.modGroupProfile = Some(r.read_message::<ModGroupProfile>(bytes)?),
                Ok(106) => msg.modGroupMemberProfile = Some(r.read_message::<ModGroupMemberProfile>(bytes)?),
                Ok(114) => msg.delFriend = Some(r.read_message::<DelFriend>(bytes)?),
                Ok(122) => msg.roamPriv = Some(r.read_message::<ModFrdRoamPriv>(bytes)?),
                Ok(130) => msg.grpMsgRoamFlag = Some(r.read_message::<GrpMsgRoamFlag>(bytes)?),
                Ok(138) => msg.confMsgRoamFlag = Some(r.read_message::<ConfMsgRoamFlag>(bytes)?),
                Ok(146) => msg.modRichLongNick = Some(r.read_message::<ModLongNick>(bytes)?),
                Ok(154) => msg.binPkg = Some(r.read_message::<BinaryMsg>(bytes)?),
                Ok(162) => msg.modFriendRings = Some(r.read_message::<ModSnsGeneralInfo>(bytes)?),
                Ok(170) => msg.modConfProfile = Some(r.read_message::<ModConfProfile>(bytes)?),
                Ok(178) => msg.modFriendFlag = Some(r.read_message::<SnsUpdateFlag>(bytes)?),
                Ok(186) => msg.appointmentNotify = Some(r.read_message::<AppointmentNotify>(bytes)?),
                Ok(202) => msg.darenNotify = Some(r.read_message::<DaRenNotify>(bytes)?),
                Ok(210) => msg.newComeinUserNotify = Some(r.read_message::<NewComeinUserNotify>(bytes)?),
                Ok(1602) => msg.pushSearchDev = Some(r.read_message::<PushSearchDev>(bytes)?),
                Ok(1610) => msg.pushReportDev = Some(r.read_message::<PushReportDev>(bytes)?),
                Ok(1618) => msg.qqPayPush = Some(r.read_message::<QQPayPush>(bytes)?),
                Ok(1626) => msg.redpointInfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(1634) => msg.hotFriendNotify = Some(r.read_message::<HotFriendNotify>(bytes)?),
                Ok(1642) => msg.praiseRankNotify = Some(r.read_message::<PraiseRankNotify>(bytes)?),
                Ok(1682) => msg.campusNotify = Some(r.read_message::<MQQCampusNotify>(bytes)?),
                Ok(1690) => msg.modRichLongNickEx = Some(r.read_message::<ModLongNick>(bytes)?),
                Ok(1698) => msg.chatMatchInfo = Some(r.read_message::<ChatMatchInfo>(bytes)?),
                Ok(1714) => msg.frdCustomOnlineStatusChange = Some(r.read_message::<FrdCustomOnlineStatusChange>(bytes)?),
                Ok(16002) => msg.fanpanziNotify = Some(r.read_message::<FanpaiziNotify>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ForwardBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.notifyType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.opType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.addGroup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.delGroup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modGroupName.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modGroupSort.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modFriendGroup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modProfile.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modFriendRemark.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modLongNick.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modCustomFace.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modGroupProfile.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.modGroupMemberProfile.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.delFriend.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.roamPriv.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.grpMsgRoamFlag.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.confMsgRoamFlag.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.modRichLongNick.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.binPkg.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.modFriendRings.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.modConfProfile.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.modFriendFlag.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.appointmentNotify.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.darenNotify.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.newComeinUserNotify.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.pushSearchDev.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.pushReportDev.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.qqPayPush.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.redpointInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.hotFriendNotify.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.praiseRankNotify.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.campusNotify.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.modRichLongNickEx.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.chatMatchInfo.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.frdCustomOnlineStatusChange.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.fanpanziNotify.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.notifyType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.opType { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.addGroup { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.delGroup { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modGroupName { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modGroupSort { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modFriendGroup { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modProfile { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modFriendRemark { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modLongNick { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modCustomFace { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modGroupProfile { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modGroupMemberProfile { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.delFriend { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.roamPriv { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.grpMsgRoamFlag { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.confMsgRoamFlag { w.write_with_tag(138, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modRichLongNick { w.write_with_tag(146, |w| w.write_message(s))?; }
        if let Some(ref s) = self.binPkg { w.write_with_tag(154, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modFriendRings { w.write_with_tag(162, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modConfProfile { w.write_with_tag(170, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modFriendFlag { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.appointmentNotify { w.write_with_tag(186, |w| w.write_message(s))?; }
        if let Some(ref s) = self.darenNotify { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.newComeinUserNotify { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pushSearchDev { w.write_with_tag(1602, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pushReportDev { w.write_with_tag(1610, |w| w.write_message(s))?; }
        if let Some(ref s) = self.qqPayPush { w.write_with_tag(1618, |w| w.write_message(s))?; }
        if let Some(ref s) = self.redpointInfo { w.write_with_tag(1626, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.hotFriendNotify { w.write_with_tag(1634, |w| w.write_message(s))?; }
        if let Some(ref s) = self.praiseRankNotify { w.write_with_tag(1642, |w| w.write_message(s))?; }
        if let Some(ref s) = self.campusNotify { w.write_with_tag(1682, |w| w.write_message(s))?; }
        if let Some(ref s) = self.modRichLongNickEx { w.write_with_tag(1690, |w| w.write_message(s))?; }
        if let Some(ref s) = self.chatMatchInfo { w.write_with_tag(1698, |w| w.write_message(s))?; }
        if let Some(ref s) = self.frdCustomOnlineStatusChange { w.write_with_tag(1714, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fanpanziNotify { w.write_with_tag(16002, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FrdCustomOnlineStatusChange {
    pub uin: Option<u64>,
}

impl<'a> MessageRead<'a> for FrdCustomOnlineStatusChange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FrdCustomOnlineStatusChange {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FriendGroup {
    pub fuin: Option<u64>,
    pub oldGroupId: Vec<u32>,
    pub newGroupId: Vec<u32>,
}

impl<'a> MessageRead<'a> for FriendGroup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fuin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.oldGroupId.push(r.read_uint32(bytes)?),
                Ok(24) => msg.newGroupId.push(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FriendGroup {
    fn get_size(&self) -> usize {
        0
        + self.fuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.oldGroupId.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.newGroupId.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fuin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        for s in &self.oldGroupId { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        for s in &self.newGroupId { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FriendRemark<'a> {
    pub type_pb: Option<u32>,
    pub fuin: Option<u64>,
    pub rmkName: Option<Cow<'a, [u8]>>,
    pub groupCode: Option<u64>,
}

impl<'a> MessageRead<'a> for FriendRemark<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.fuin = Some(r.read_uint64(bytes)?),
                Ok(26) => msg.rmkName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FriendRemark<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.fuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rmkName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.fuin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.rmkName { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GPS {
    pub lat: Option<i32>,
    pub lon: Option<i32>,
    pub alt: Option<i32>,
    pub type_pb: Option<i32>,
}

impl<'a> MessageRead<'a> for GPS {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.lat = Some(r.read_int32(bytes)?),
                Ok(16) => msg.lon = Some(r.read_int32(bytes)?),
                Ok(24) => msg.alt = Some(r.read_int32(bytes)?),
                Ok(32) => msg.type_pb = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GPS {
    fn get_size(&self) -> usize {
        0
        + self.lat.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lon.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.alt.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.lat { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.lon { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.alt { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMemberProfileInfo<'a> {
    pub field: Option<u32>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GroupMemberProfileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.field = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupMemberProfileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.field { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupProfileInfo<'a> {
    pub field: Option<u32>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for GroupProfileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.field = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupProfileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.field { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupSort {
    pub groupid: Option<u32>,
    pub sortid: Option<u32>,
}

impl<'a> MessageRead<'a> for GroupSort {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupid = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.sortid = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupSort {
    fn get_size(&self) -> usize {
        0
        + self.groupid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sortid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupid { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sortid { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GrpMsgRoamFlag {
    pub groupcode: Option<u64>,
    pub flag: Option<u32>,
    pub timestamp: Option<u64>,
}

impl<'a> MessageRead<'a> for GrpMsgRoamFlag {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupcode = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.flag = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.timestamp = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GrpMsgRoamFlag {
    fn get_size(&self) -> usize {
        0
        + self.groupcode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupcode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HotFriendNotify {
    pub dstUin: Option<u64>,
    pub praiseHotLevel: Option<u32>,
    pub chatHotLevel: Option<u32>,
    pub praiseHotDays: Option<u32>,
    pub chatHotDays: Option<u32>,
    pub closeLevel: Option<u32>,
    pub closeDays: Option<u32>,
    pub praiseFlag: Option<u32>,
    pub chatFlag: Option<u32>,
    pub closeFlag: Option<u32>,
    pub notifyTime: Option<u64>,
    pub lastPraiseTime: Option<u64>,
    pub lastChatTime: Option<u64>,
    pub qzoneHotLevel: Option<u32>,
    pub qzoneHotDays: Option<u32>,
    pub qzoneFlag: Option<u32>,
    pub lastQzoneTime: Option<u64>,
}

impl<'a> MessageRead<'a> for HotFriendNotify {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.dstUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.praiseHotLevel = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.chatHotLevel = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.praiseHotDays = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.chatHotDays = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.closeLevel = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.closeDays = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.praiseFlag = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.chatFlag = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.closeFlag = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.notifyTime = Some(r.read_uint64(bytes)?),
                Ok(96) => msg.lastPraiseTime = Some(r.read_uint64(bytes)?),
                Ok(104) => msg.lastChatTime = Some(r.read_uint64(bytes)?),
                Ok(112) => msg.qzoneHotLevel = Some(r.read_uint32(bytes)?),
                Ok(120) => msg.qzoneHotDays = Some(r.read_uint32(bytes)?),
                Ok(128) => msg.qzoneFlag = Some(r.read_uint32(bytes)?),
                Ok(136) => msg.lastQzoneTime = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HotFriendNotify {
    fn get_size(&self) -> usize {
        0
        + self.dstUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.praiseHotLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chatHotLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.praiseHotDays.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chatHotDays.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.closeLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.closeDays.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.praiseFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chatFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.closeFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.notifyTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastPraiseTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.lastChatTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.qzoneHotLevel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.qzoneHotDays.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.qzoneFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.lastQzoneTime.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.dstUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.praiseHotLevel { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.chatHotLevel { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.praiseHotDays { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.chatHotDays { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.closeLevel { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.closeDays { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.praiseFlag { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.chatFlag { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.closeFlag { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.notifyTime { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.lastPraiseTime { w.write_with_tag(96, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.lastChatTime { w.write_with_tag(104, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.qzoneHotLevel { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.qzoneHotDays { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.qzoneFlag { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.lastQzoneTime { w.write_with_tag(136, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MQQCampusNotify<'a> {
    pub fromUin: Option<u64>,
    pub wording: Option<Cow<'a, str>>,
    pub target: Option<Cow<'a, str>>,
    pub type_pb: Option<u32>,
    pub source: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for MQQCampusNotify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromUin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.wording = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.target = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(42) => msg.source = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MQQCampusNotify<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.wording.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.target.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.wording { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.target { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModConfProfile<'a> {
    pub uin: Option<u64>,
    pub confUin: Option<u32>,
    pub profileInfos: Vec<ProfileInfo<'a>>,
}

impl<'a> MessageRead<'a> for ModConfProfile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.confUin = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.profileInfos.push(r.read_message::<ProfileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModConfProfile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.confUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.profileInfos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.confUin { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        for s in &self.profileInfos { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModCustomFace {
    pub type_pb: Option<u32>,
    pub uin: Option<u64>,
    pub groupCode: Option<u64>,
    pub cmdUin: Option<u64>,
}

impl<'a> MessageRead<'a> for ModCustomFace {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.cmdUin = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ModCustomFace {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cmdUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cmdUin { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModFrdRoamPriv {
    pub roamPriv: Vec<OneRoamPriv>,
}

impl<'a> MessageRead<'a> for ModFrdRoamPriv {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.roamPriv.push(r.read_message::<OneRoamPriv>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ModFrdRoamPriv {
    fn get_size(&self) -> usize {
        0
        + self.roamPriv.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.roamPriv { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModFriendGroup {
    pub frdGroup: Vec<FriendGroup>,
}

impl<'a> MessageRead<'a> for ModFriendGroup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.frdGroup.push(r.read_message::<FriendGroup>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ModFriendGroup {
    fn get_size(&self) -> usize {
        0
        + self.frdGroup.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.frdGroup { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModFriendRemark<'a> {
    pub frdRmk: Vec<FriendRemark<'a>>,
}

impl<'a> MessageRead<'a> for ModFriendRemark<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.frdRmk.push(r.read_message::<FriendRemark>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModFriendRemark<'a> {
    fn get_size(&self) -> usize {
        0
        + self.frdRmk.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.frdRmk { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModGroupMemberProfile<'a> {
    pub groupUin: Option<u64>,
    pub uin: Option<u64>,
    pub groupMemberProfileInfos: Vec<GroupMemberProfileInfo<'a>>,
    pub groupCode: Option<u64>,
}

impl<'a> MessageRead<'a> for ModGroupMemberProfile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(26) => msg.groupMemberProfileInfos.push(r.read_message::<GroupMemberProfileInfo>(bytes)?),
                Ok(32) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModGroupMemberProfile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupMemberProfileInfos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        for s in &self.groupMemberProfileInfos { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModGroupName<'a> {
    pub groupid: Option<u32>,
    pub groupname: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ModGroupName<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupid = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.groupname = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModGroupName<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupid { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupname { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModGroupProfile<'a> {
    pub groupUin: Option<u64>,
    pub groupProfileInfos: Vec<GroupProfileInfo<'a>>,
    pub groupCode: Option<u64>,
    pub cmdUin: Option<u64>,
}

impl<'a> MessageRead<'a> for ModGroupProfile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupUin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.groupProfileInfos.push(r.read_message::<GroupProfileInfo>(bytes)?),
                Ok(24) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.cmdUin = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModGroupProfile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupProfileInfos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cmdUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        for s in &self.groupProfileInfos { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cmdUin { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModGroupSort {
    pub groupsort: Vec<GroupSort>,
}

impl<'a> MessageRead<'a> for ModGroupSort {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.groupsort.push(r.read_message::<GroupSort>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ModGroupSort {
    fn get_size(&self) -> usize {
        0
        + self.groupsort.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.groupsort { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModLongNick<'a> {
    pub uin: Option<u64>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ModLongNick<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModLongNick<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModProfile<'a> {
    pub uin: Option<u64>,
    pub profileInfos: Vec<ProfileInfo<'a>>,
}

impl<'a> MessageRead<'a> for ModProfile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.profileInfos.push(r.read_message::<ProfileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModProfile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.profileInfos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        for s in &self.profileInfos { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModSnsGeneralInfo<'a> {
    pub snsGeneralInfos: Vec<SnsUpateBuffer<'a>>,
}

impl<'a> MessageRead<'a> for ModSnsGeneralInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.snsGeneralInfos.push(r.read_message::<SnsUpateBuffer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModSnsGeneralInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.snsGeneralInfos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.snsGeneralInfos { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SubMsg0x27Body<'a> {
    pub modInfos: Vec<ForwardBody<'a>>,
}

impl<'a> MessageRead<'a> for SubMsg0x27Body<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.modInfos.push(r.read_message::<ForwardBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SubMsg0x27Body<'a> {
    fn get_size(&self) -> usize {
        0
        + self.modInfos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.modInfos { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewComeinUser<'a> {
    pub uin: Option<u64>,
    pub isFrd: Option<u32>,
    pub remark: Option<Cow<'a, [u8]>>,
    pub nick: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for NewComeinUser<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.isFrd = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.remark = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.nick = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NewComeinUser<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.isFrd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.remark.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.nick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.isFrd { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.remark { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.nick { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewComeinUserNotify<'a> {
    pub msgType: Option<u32>,
    pub ongNotify: Option<bool>,
    pub pushTime: Option<u32>,
    pub newComeinUser: Option<NewComeinUser<'a>>,
    pub newGroup: Option<NewGroup<'a>>,
    pub newGroupUser: Option<NewGroupUser<'a>>,
}

impl<'a> MessageRead<'a> for NewComeinUserNotify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgType = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.ongNotify = Some(r.read_bool(bytes)?),
                Ok(24) => msg.pushTime = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.newComeinUser = Some(r.read_message::<NewComeinUser>(bytes)?),
                Ok(42) => msg.newGroup = Some(r.read_message::<NewGroup>(bytes)?),
                Ok(50) => msg.newGroupUser = Some(r.read_message::<NewGroupUser>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NewComeinUserNotify<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ongNotify.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pushTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.newComeinUser.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.newGroup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.newGroupUser.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msgType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.ongNotify { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.pushTime { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.newComeinUser { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.newGroup { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.newGroupUser { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewGroup<'a> {
    pub groupCode: Option<u64>,
    pub groupName: Option<Cow<'a, [u8]>>,
    pub ownerUin: Option<u64>,
    pub ownerNick: Option<Cow<'a, [u8]>>,
    pub distance: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for NewGroup<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(18) => msg.groupName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.ownerUin = Some(r.read_uint64(bytes)?),
                Ok(34) => msg.ownerNick = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.distance = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NewGroup<'a> {
    fn get_size(&self) -> usize {
        0
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ownerUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ownerNick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.distance.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.groupCode { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupName { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.ownerUin { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ownerNick { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.distance { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewGroupUser<'a> {
    pub uin: Option<u64>,
    pub sex: Option<i32>,
    pub age: Option<i32>,
    pub nick: Option<Cow<'a, str>>,
    pub distance: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for NewGroupUser<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.sex = Some(r.read_int32(bytes)?),
                Ok(24) => msg.age = Some(r.read_int32(bytes)?),
                Ok(34) => msg.nick = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.distance = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NewGroupUser<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sex.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.age.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.nick.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.distance.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.sex { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.age { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.nick { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.distance { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OneRoamPriv {
    pub fuin: Option<u64>,
    pub privTag: Option<u32>,
    pub privValue: Option<u32>,
}

impl<'a> MessageRead<'a> for OneRoamPriv {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fuin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.privTag = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.privValue = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OneRoamPriv {
    fn get_size(&self) -> usize {
        0
        + self.fuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.privTag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.privValue.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fuin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.privTag { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.privValue { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PraiseRankNotify<'a> {
    pub isChampion: Option<u32>,
    pub rankNum: Option<u32>,
    pub msg: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for PraiseRankNotify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(88) => msg.isChampion = Some(r.read_uint32(bytes)?),
                Ok(96) => msg.rankNum = Some(r.read_uint32(bytes)?),
                Ok(106) => msg.msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PraiseRankNotify<'a> {
    fn get_size(&self) -> usize {
        0
        + self.isChampion.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rankNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.isChampion { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.rankNum { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.msg { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProfileInfo<'a> {
    pub field: Option<u32>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ProfileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.field = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProfileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.field.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.field { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PushReportDev<'a> {
    pub msgType: Option<u32>,
    pub cookie: Option<Cow<'a, [u8]>>,
    pub reportMaxNum: Option<u32>,
    pub sn: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for PushReportDev<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgType = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.cookie = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.reportMaxNum = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.sn = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PushReportDev<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cookie.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.reportMaxNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sn.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msgType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.cookie { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.reportMaxNum { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sn { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PushSearchDev<'a> {
    pub msgType: Option<u32>,
    pub gpsInfo: Option<GPS>,
    pub devTime: Option<u32>,
    pub pushTime: Option<u32>,
    pub din: Option<u64>,
    pub data: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for PushSearchDev<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msgType = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.gpsInfo = Some(r.read_message::<GPS>(bytes)?),
                Ok(24) => msg.devTime = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.pushTime = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.din = Some(r.read_uint64(bytes)?),
                Ok(50) => msg.data = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PushSearchDev<'a> {
    fn get_size(&self) -> usize {
        0
        + self.msgType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.gpsInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.devTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pushTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.din.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msgType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.gpsInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.devTime { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.pushTime { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.din { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQPayPush {
    pub uin: Option<u64>,
    pub payOk: Option<bool>,
}

impl<'a> MessageRead<'a> for QQPayPush {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.payOk = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for QQPayPush {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.payOk.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.payOk { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SnsUpateBuffer<'a> {
    pub uin: Option<u64>,
    pub code: Option<u64>,
    pub result: Option<u32>,
    pub snsUpdateItem: Vec<SnsUpdateItem<'a>>,
    pub idlist: Vec<u32>,
}

impl<'a> MessageRead<'a> for SnsUpateBuffer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.code = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(3202) => msg.snsUpdateItem.push(r.read_message::<SnsUpdateItem>(bytes)?),
                Ok(3208) => msg.idlist.push(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SnsUpateBuffer<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.code.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.snsUpdateItem.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.idlist.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.code { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        for s in &self.snsUpdateItem { w.write_with_tag(3202, |w| w.write_message(s))?; }
        for s in &self.idlist { w.write_with_tag(3208, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SnsUpdateFlag {
    pub updateSnsFlag: Vec<SnsUpdateOneFlag>,
}

impl<'a> MessageRead<'a> for SnsUpdateFlag {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.updateSnsFlag.push(r.read_message::<SnsUpdateOneFlag>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SnsUpdateFlag {
    fn get_size(&self) -> usize {
        0
        + self.updateSnsFlag.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.updateSnsFlag { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SnsUpdateItem<'a> {
    pub updateSnsType: Option<u32>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for SnsUpdateItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.updateSnsType = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SnsUpdateItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.updateSnsType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.updateSnsType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SnsUpdateOneFlag {
    pub XUin: Option<u64>,
    pub id: Option<u64>,
    pub flag: Option<u32>,
}

impl<'a> MessageRead<'a> for SnsUpdateOneFlag {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.XUin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.id = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.flag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SnsUpdateOneFlag {
    fn get_size(&self) -> usize {
        0
        + self.XUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.XUin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.id { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

