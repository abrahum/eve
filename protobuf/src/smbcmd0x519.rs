// Automatically generated rust module for 'smbcmd0x519.proto' file

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
pub struct C519CRMMsgHead<'a> {
    pub crmSubCmd: Option<u32>,
    pub headLen: Option<u32>,
    pub verNo: Option<u32>,
    pub kfUin: Option<u64>,
    pub seq: Option<u32>,
    pub packNum: Option<u32>,
    pub curPack: Option<u32>,
    pub bufSig: Option<Cow<'a, str>>,
    pub pubQq: Option<u64>,
    pub clienttype: Option<u32>,
    pub laborUin: Option<u64>,
    pub laborName: Option<Cow<'a, str>>,
    pub puin: Option<u64>,
}

impl<'a> MessageRead<'a> for C519CRMMsgHead<'a> {
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
                Ok(72) => msg.pubQq = Some(r.read_uint64(bytes)?),
                Ok(80) => msg.clienttype = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.laborUin = Some(r.read_uint64(bytes)?),
                Ok(98) => msg.laborName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(104) => msg.puin = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C519CRMMsgHead<'a> {
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
        + self.pubQq.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clienttype.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.laborUin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.laborName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.puin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
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
        if let Some(ref s) = self.pubQq { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.clienttype { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.laborUin { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.laborName { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.puin { w.write_with_tag(104, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetNavigationMenuReqBody {
    pub puin: Option<u64>,
    pub uin: Option<u64>,
    pub verNo: Option<u32>,
}

impl<'a> MessageRead<'a> for GetNavigationMenuReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.puin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.verNo = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetNavigationMenuReqBody {
    fn get_size(&self) -> usize {
        0
        + self.puin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.verNo.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.puin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.verNo { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetNavigationMenuRspBody<'a> {
    pub ret: Option<C519RetInfo<'a>>,
    pub isShow: Option<i32>,
    pub uctMsg: Option<Cow<'a, str>>,
    pub verNo: Option<u32>,
}

impl<'a> MessageRead<'a> for GetNavigationMenuRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ret = Some(r.read_message::<C519RetInfo>(bytes)?),
                Ok(16) => msg.isShow = Some(r.read_int32(bytes)?),
                Ok(26) => msg.uctMsg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.verNo = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetNavigationMenuRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ret.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.isShow.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uctMsg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.verNo.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ret { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.isShow { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.uctMsg { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.verNo { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C519ReqBody<'a> {
    pub subCmd: Option<u32>,
    pub crmCommonHead: Option<C519CRMMsgHead<'a>>,
    pub getAddressDetailListReqBody: Option<GetAddressDetailListReqBody>,
    pub getNavigationMenuReq: Option<GetNavigationMenuReqBody>,
}

impl<'a> MessageRead<'a> for C519ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subCmd = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.crmCommonHead = Some(r.read_message::<C519CRMMsgHead>(bytes)?),
                Ok(266) => msg.getAddressDetailListReqBody = Some(r.read_message::<GetAddressDetailListReqBody>(bytes)?),
                Ok(282) => msg.getNavigationMenuReq = Some(r.read_message::<GetNavigationMenuReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C519ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.crmCommonHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.getAddressDetailListReqBody.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.getNavigationMenuReq.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subCmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.crmCommonHead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.getAddressDetailListReqBody { w.write_with_tag(266, |w| w.write_message(s))?; }
        if let Some(ref s) = self.getNavigationMenuReq { w.write_with_tag(282, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C519RetInfo<'a> {
    pub retCode: Option<u32>,
    pub errorMsg: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for C519RetInfo<'a> {
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

impl<'a> MessageWrite for C519RetInfo<'a> {
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
pub struct C519RspBody<'a> {
    pub subCmd: Option<u32>,
    pub crmCommonHead: Option<C519CRMMsgHead<'a>>,
    pub getAddressDetailListRspBody: Option<GetAddressDetailListRspBody<'a>>,
    pub getNavigationMenuRsp: Option<GetNavigationMenuRspBody<'a>>,
}

impl<'a> MessageRead<'a> for C519RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.subCmd = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.crmCommonHead = Some(r.read_message::<C519CRMMsgHead>(bytes)?),
                Ok(266) => msg.getAddressDetailListRspBody = Some(r.read_message::<GetAddressDetailListRspBody>(bytes)?),
                Ok(282) => msg.getNavigationMenuRsp = Some(r.read_message::<GetNavigationMenuRspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C519RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.subCmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.crmCommonHead.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.getAddressDetailListRspBody.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.getNavigationMenuRsp.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subCmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.crmCommonHead { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.getAddressDetailListRspBody { w.write_with_tag(266, |w| w.write_message(s))?; }
        if let Some(ref s) = self.getNavigationMenuRsp { w.write_with_tag(282, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetAddressDetailListReqBody {
    pub timestamp: Option<u32>,
    pub timestamp2: Option<u64>,
}

impl<'a> MessageRead<'a> for GetAddressDetailListReqBody {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.timestamp = Some(r.read_fixed32(bytes)?),
                Ok(17) => msg.timestamp2 = Some(r.read_fixed64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetAddressDetailListReqBody {
    fn get_size(&self) -> usize {
        0
        + self.timestamp.as_ref().map_or(0, |_| 1 + 4)
        + self.timestamp2.as_ref().map_or(0, |_| 1 + 8)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.timestamp { w.write_with_tag(13, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.timestamp2 { w.write_with_tag(17, |w| w.write_fixed64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetAddressDetailListRspBody<'a> {
    pub ret: Option<C519RetInfo<'a>>,
    pub timestamp: Option<u32>,
    pub full: Option<bool>,
    pub addressDetail: Vec<AddressDetail<'a>>,
    pub timestamp2: Option<u64>,
}

impl<'a> MessageRead<'a> for GetAddressDetailListRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ret = Some(r.read_message::<C519RetInfo>(bytes)?),
                Ok(21) => msg.timestamp = Some(r.read_fixed32(bytes)?),
                Ok(24) => msg.full = Some(r.read_bool(bytes)?),
                Ok(34) => msg.addressDetail.push(r.read_message::<AddressDetail>(bytes)?),
                Ok(41) => msg.timestamp2 = Some(r.read_fixed64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetAddressDetailListRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ret.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.timestamp.as_ref().map_or(0, |_| 1 + 4)
        + self.full.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.addressDetail.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.timestamp2.as_ref().map_or(0, |_| 1 + 8)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ret { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(21, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.full { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        for s in &self.addressDetail { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.timestamp2 { w.write_with_tag(41, |w| w.write_fixed64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddressDetail<'a> {
    pub aid: Option<u32>,
    pub modifyTime: Option<u32>,
    pub createTime: Option<u32>,
    pub status: Option<u32>,
    pub groupid: Option<u32>,
    pub addGroupName: Option<Cow<'a, [u8]>>,
    pub name: Option<Cow<'a, [u8]>>,
    pub gender: Option<u32>,
    pub birthday: Option<u32>,
    pub company0: Option<Cow<'a, [u8]>>,
    pub companyPosition0: Option<Cow<'a, [u8]>>,
    pub company1: Option<Cow<'a, [u8]>>,
    pub companyPosition1: Option<Cow<'a, [u8]>>,
    pub fixedPhone0: Option<Cow<'a, [u8]>>,
    pub fixedPhone1: Option<Cow<'a, [u8]>>,
    pub email0: Option<Cow<'a, [u8]>>,
    pub email1: Option<Cow<'a, [u8]>>,
    pub fax0: Option<Cow<'a, [u8]>>,
    pub fax1: Option<Cow<'a, [u8]>>,
    pub comment: Option<Cow<'a, [u8]>>,
    pub headUrl: Option<Cow<'a, [u8]>>,
    pub mobilePhone: Vec<AddressMobileInfo<'a>>,
    pub mobilePhoneUpdated: Option<bool>,
    pub qq: Vec<AddressQQinfo>,
    pub qqPhoneUpdated: Option<bool>,
    pub modifyTime2: Option<u64>,
    pub clientRegion: Option<NewBizClientRegion<'a>>,
    pub clientRegionCode: Option<NewBizClientRegionCode>,
}

impl<'a> MessageRead<'a> for AddressDetail<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.aid = Some(r.read_uint32(bytes)?),
                Ok(21) => msg.modifyTime = Some(r.read_fixed32(bytes)?),
                Ok(29) => msg.createTime = Some(r.read_fixed32(bytes)?),
                Ok(32) => msg.status = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.groupid = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.addGroupName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.name = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(64) => msg.gender = Some(r.read_uint32(bytes)?),
                Ok(77) => msg.birthday = Some(r.read_fixed32(bytes)?),
                Ok(82) => msg.company0 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.companyPosition0 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.company1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.companyPosition1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(114) => msg.fixedPhone0 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.fixedPhone1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.email0 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(138) => msg.email1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(146) => msg.fax0 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(154) => msg.fax1 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(162) => msg.comment = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(170) => msg.headUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(178) => msg.mobilePhone.push(r.read_message::<AddressMobileInfo>(bytes)?),
                Ok(184) => msg.mobilePhoneUpdated = Some(r.read_bool(bytes)?),
                Ok(194) => msg.qq.push(r.read_message::<AddressQQinfo>(bytes)?),
                Ok(200) => msg.qqPhoneUpdated = Some(r.read_bool(bytes)?),
                Ok(209) => msg.modifyTime2 = Some(r.read_fixed64(bytes)?),
                Ok(218) => msg.clientRegion = Some(r.read_message::<NewBizClientRegion>(bytes)?),
                Ok(226) => msg.clientRegionCode = Some(r.read_message::<NewBizClientRegionCode>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AddressDetail<'a> {
    fn get_size(&self) -> usize {
        0
        + self.aid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.modifyTime.as_ref().map_or(0, |_| 1 + 4)
        + self.createTime.as_ref().map_or(0, |_| 1 + 4)
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.addGroupName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.gender.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.birthday.as_ref().map_or(0, |_| 1 + 4)
        + self.company0.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.companyPosition0.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.company1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.companyPosition1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fixedPhone0.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fixedPhone1.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.email0.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.email1.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.fax0.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.fax1.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.comment.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.headUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.mobilePhone.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.mobilePhoneUpdated.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.qq.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.qqPhoneUpdated.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.modifyTime2.as_ref().map_or(0, |_| 2 + 8)
        + self.clientRegion.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.clientRegionCode.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.aid { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.modifyTime { w.write_with_tag(21, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.createTime { w.write_with_tag(29, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.groupid { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.addGroupName { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.gender { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.birthday { w.write_with_tag(77, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.company0 { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.companyPosition0 { w.write_with_tag(90, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.company1 { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.companyPosition1 { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fixedPhone0 { w.write_with_tag(114, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fixedPhone1 { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.email0 { w.write_with_tag(130, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.email1 { w.write_with_tag(138, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fax0 { w.write_with_tag(146, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fax1 { w.write_with_tag(154, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.comment { w.write_with_tag(162, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.headUrl { w.write_with_tag(170, |w| w.write_bytes(&**s))?; }
        for s in &self.mobilePhone { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mobilePhoneUpdated { w.write_with_tag(184, |w| w.write_bool(*s))?; }
        for s in &self.qq { w.write_with_tag(194, |w| w.write_message(s))?; }
        if let Some(ref s) = self.qqPhoneUpdated { w.write_with_tag(200, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.modifyTime2 { w.write_with_tag(209, |w| w.write_fixed64(*s))?; }
        if let Some(ref s) = self.clientRegion { w.write_with_tag(218, |w| w.write_message(s))?; }
        if let Some(ref s) = self.clientRegionCode { w.write_with_tag(226, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddressMobileInfo<'a> {
    pub index: Option<u32>,
    pub account: Option<Cow<'a, [u8]>>,
    pub formattedAccount: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for AddressMobileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.account = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.formattedAccount = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AddressMobileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.account.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.formattedAccount.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.index { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.account { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.formattedAccount { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddressQQinfo {
    pub index: Option<u32>,
    pub account: Option<u64>,
}

impl<'a> MessageRead<'a> for AddressQQinfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.account = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AddressQQinfo {
    fn get_size(&self) -> usize {
        0
        + self.index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.account.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.index { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.account { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewBizClientRegion<'a> {
    pub clientNation: Option<Cow<'a, str>>,
    pub clientProvince: Option<Cow<'a, str>>,
    pub clientCity: Option<Cow<'a, str>>,
    pub clientRegion: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for NewBizClientRegion<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.clientNation = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.clientProvince = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientCity = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.clientRegion = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NewBizClientRegion<'a> {
    fn get_size(&self) -> usize {
        0
        + self.clientNation.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientProvince.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientCity.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientRegion.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.clientNation { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientProvince { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientCity { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientRegion { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewBizClientRegionCode {
    pub nationid: Option<u64>,
    pub provinceid: Option<u64>,
    pub cityid: Option<u64>,
    pub regionid: Option<u64>,
}

impl<'a> MessageRead<'a> for NewBizClientRegionCode {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.nationid = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.provinceid = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.cityid = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.regionid = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NewBizClientRegionCode {
    fn get_size(&self) -> usize {
        0
        + self.nationid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.provinceid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cityid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.regionid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.nationid { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.provinceid { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.cityid { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.regionid { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

