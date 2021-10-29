// Automatically generated rust module for 'accountsearch.proto' file

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
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl<'a> MessageRead<'a> for Location {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.latitude = Some(r.read_double(bytes)?),
                Ok(17) => msg.longitude = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Location {
    fn get_size(&self) -> usize {
        0
        + self.latitude.as_ref().map_or(0, |_| 1 + 8)
        + self.longitude.as_ref().map_or(0, |_| 1 + 8)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.latitude { w.write_with_tag(9, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.longitude { w.write_with_tag(17, |w| w.write_double(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResultItem<'a> {
    pub feedId: Option<Cow<'a, [u8]>>,
    pub name: Option<Cow<'a, [u8]>>,
    pub picUrl: Option<Cow<'a, [u8]>>,
    pub jmpUrl: Option<Cow<'a, [u8]>>,
    pub feedType: Option<Cow<'a, [u8]>>,
    pub summary: Option<Cow<'a, [u8]>>,
    pub hasVideo: Option<Cow<'a, [u8]>>,
    pub phtotUpdate: Option<Cow<'a, [u8]>>,
    pub uin: Option<u64>,
    pub resultId: Option<Cow<'a, [u8]>>,
    pub ftime: Option<u32>,
    pub nickName: Option<Cow<'a, [u8]>>,
    pub picUrlList: Vec<Cow<'a, [u8]>>,
    pub totalPicNum: Option<u32>,
}

impl<'a> MessageRead<'a> for ResultItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.feedId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.name = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.picUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.jmpUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.feedType = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.summary = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.hasVideo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.phtotUpdate = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(82) => msg.resultId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.ftime = Some(r.read_uint32(bytes)?),
                Ok(98) => msg.nickName = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.picUrlList.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.totalPicNum = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResultItem<'a> {
    fn get_size(&self) -> usize {
        0
        + self.feedId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.picUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.jmpUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.feedType.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.summary.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hasVideo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.phtotUpdate.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.resultId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ftime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.nickName.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.picUrlList.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.totalPicNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.feedId { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.picUrl { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.jmpUrl { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.feedType { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.summary { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.hasVideo { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.phtotUpdate { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.uin { w.write_with_tag(72, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.resultId { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.ftime { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.nickName { w.write_with_tag(98, |w| w.write_bytes(&**s))?; }
        for s in &self.picUrlList { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.totalPicNum { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct hotwordrecord<'a> {
    pub hotword: Option<Cow<'a, str>>,
    pub hotwordType: Option<u32>,
    pub hotwordCoverUrl: Option<Cow<'a, str>>,
    pub hotwordTitle: Option<Cow<'a, str>>,
    pub hotwordDescription: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for hotwordrecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hotword = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.hotwordType = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.hotwordCoverUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.hotwordTitle = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.hotwordDescription = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for hotwordrecord<'a> {
    fn get_size(&self) -> usize {
        0
        + self.hotword.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hotwordType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hotwordCoverUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hotwordTitle.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hotwordDescription.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.hotword { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hotwordType { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.hotwordCoverUrl { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hotwordTitle { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hotwordDescription { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccountSearchRecord<'a> {
    pub uin: Option<u64>,
    pub code: Option<u64>,
    pub source: Option<u32>,
    pub name: Option<Cow<'a, str>>,
    pub sex: Option<u32>,
    pub age: Option<u32>,
    pub accout: Option<Cow<'a, str>>,
    pub brief: Option<Cow<'a, str>>,
    pub number: Option<u32>,
    pub flag: Option<u64>,
    pub relation: Option<u64>,
    pub mobile: Option<Cow<'a, str>>,
    pub sign: Option<Cow<'a, [u8]>>,
    pub country: Option<u32>,
    pub province: Option<u32>,
    pub city: Option<u32>,
    pub classIndex: Option<u32>,
    pub className: Option<Cow<'a, str>>,
    pub countryName: Option<Cow<'a, str>>,
    pub provinceName: Option<Cow<'a, str>>,
    pub cityName: Option<Cow<'a, str>>,
    pub accountFlag: Option<u32>,
    pub titleImage: Option<Cow<'a, str>>,
    pub articleShortUrl: Option<Cow<'a, str>>,
    pub articleCreateTime: Option<Cow<'a, str>>,
    pub articleAuthor: Option<Cow<'a, str>>,
    pub accountId: Option<u64>,
    pub videoAccount: Option<u32>,
    pub videoArticle: Option<u32>,
    pub uinPrivilege: Option<i32>,
    pub joinGroupAuth: Option<Cow<'a, [u8]>>,
    pub token: Option<Cow<'a, [u8]>>,
    pub richflag1_59: Option<u32>,
    pub richflag4_409: Option<u32>,
}

impl<'a> MessageRead<'a> for AccountSearchRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.code = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.source = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.sex = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.age = Some(r.read_uint32(bytes)?),
                Ok(58) => msg.accout = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.brief = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.number = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.flag = Some(r.read_uint64(bytes)?),
                Ok(88) => msg.relation = Some(r.read_uint64(bytes)?),
                Ok(98) => msg.mobile = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.sign = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.country = Some(r.read_uint32(bytes)?),
                Ok(120) => msg.province = Some(r.read_uint32(bytes)?),
                Ok(128) => msg.city = Some(r.read_uint32(bytes)?),
                Ok(136) => msg.classIndex = Some(r.read_uint32(bytes)?),
                Ok(146) => msg.className = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(154) => msg.countryName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(162) => msg.provinceName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(170) => msg.cityName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(176) => msg.accountFlag = Some(r.read_uint32(bytes)?),
                Ok(186) => msg.titleImage = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(194) => msg.articleShortUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(202) => msg.articleCreateTime = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(210) => msg.articleAuthor = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(216) => msg.accountId = Some(r.read_uint64(bytes)?),
                Ok(248) => msg.videoAccount = Some(r.read_uint32(bytes)?),
                Ok(256) => msg.videoArticle = Some(r.read_uint32(bytes)?),
                Ok(264) => msg.uinPrivilege = Some(r.read_int32(bytes)?),
                Ok(274) => msg.joinGroupAuth = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(4002) => msg.token = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(324824) => msg.richflag1_59 = Some(r.read_uint32(bytes)?),
                Ok(339272) => msg.richflag4_409 = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AccountSearchRecord<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.code.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sex.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.age.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.accout.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.brief.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.number.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.relation.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mobile.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sign.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.country.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.province.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.city.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.classIndex.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.className.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.countryName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.provinceName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.cityName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.accountFlag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.titleImage.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.articleShortUrl.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.articleCreateTime.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.articleAuthor.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.accountId.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.videoAccount.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.videoArticle.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.uinPrivilege.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.joinGroupAuth.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.token.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.richflag1_59.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.richflag4_409.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.code { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.sex { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.age { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.accout { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.brief { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.number { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.flag { w.write_with_tag(80, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.relation { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.mobile { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.sign { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.country { w.write_with_tag(112, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.province { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.city { w.write_with_tag(128, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.classIndex { w.write_with_tag(136, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.className { w.write_with_tag(146, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.countryName { w.write_with_tag(154, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.provinceName { w.write_with_tag(162, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.cityName { w.write_with_tag(170, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.accountFlag { w.write_with_tag(176, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.titleImage { w.write_with_tag(186, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.articleShortUrl { w.write_with_tag(194, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.articleCreateTime { w.write_with_tag(202, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.articleAuthor { w.write_with_tag(210, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.accountId { w.write_with_tag(216, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.videoAccount { w.write_with_tag(248, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.videoArticle { w.write_with_tag(256, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uinPrivilege { w.write_with_tag(264, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.joinGroupAuth { w.write_with_tag(274, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.token { w.write_with_tag(4002, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.richflag1_59 { w.write_with_tag(324824, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.richflag4_409 { w.write_with_tag(339272, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccountSearch<'a> {
    pub start: Option<i32>,
    pub count: Option<u32>,
    pub end: Option<u32>,
    pub keyword: Option<Cow<'a, str>>,
    pub list: Vec<AccountSearchRecord<'a>>,
    pub highlight: Vec<Cow<'a, str>>,
    pub userLocation: Option<Location>,
    pub locationGroup: Option<bool>,
    pub filtertype: Option<i32>,
    pub hotwordRecord: Option<hotwordrecord<'a>>,
    pub articleMoreUrl: Option<Cow<'a, str>>,
    pub resultItems: Vec<ResultItem<'a>>,
    pub keywordSuicide: Option<bool>,
    pub exactSearch: Option<bool>,
}

impl<'a> MessageRead<'a> for AccountSearch<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.start = Some(r.read_int32(bytes)?),
                Ok(16) => msg.count = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.end = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.keyword = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.list.push(r.read_message::<AccountSearchRecord>(bytes)?),
                Ok(50) => msg.highlight.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.userLocation = Some(r.read_message::<Location>(bytes)?),
                Ok(88) => msg.locationGroup = Some(r.read_bool(bytes)?),
                Ok(96) => msg.filtertype = Some(r.read_int32(bytes)?),
                Ok(114) => msg.hotwordRecord = Some(r.read_message::<hotwordrecord>(bytes)?),
                Ok(122) => msg.articleMoreUrl = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.resultItems.push(r.read_message::<ResultItem>(bytes)?),
                Ok(136) => msg.keywordSuicide = Some(r.read_bool(bytes)?),
                Ok(144) => msg.exactSearch = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AccountSearch<'a> {
    fn get_size(&self) -> usize {
        0
        + self.start.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.end.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.keyword.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.highlight.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.userLocation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.locationGroup.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.filtertype.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hotwordRecord.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.articleMoreUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.resultItems.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.keywordSuicide.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.exactSearch.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.start { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.count { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.end { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.keyword { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.list { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.highlight { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.userLocation { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.locationGroup { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.filtertype { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.hotwordRecord { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.articleMoreUrl { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        for s in &self.resultItems { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.keywordSuicide { w.write_with_tag(136, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.exactSearch { w.write_with_tag(144, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

