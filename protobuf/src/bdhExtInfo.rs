// Automatically generated rust module for 'bdhExtInfo.proto' file

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
pub struct CommFileExtReq<'a> {
    pub actionType: Option<u32>,
    pub uuid: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for CommFileExtReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.actionType = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.uuid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CommFileExtReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.actionType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.actionType { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uuid { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommFileExtRsp<'a> {
    pub retcode: Option<i32>,
    pub downloadUrl: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for CommFileExtRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retcode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.downloadUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CommFileExtRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retcode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.downloadUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retcode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.downloadUrl { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PicInfo<'a> {
    pub idx: Option<u32>,
    pub size: Option<u32>,
    pub binMd5: Option<Cow<'a, [u8]>>,
    pub type_pb: Option<u32>,
}

impl<'a> MessageRead<'a> for PicInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.idx = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.size = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.binMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PicInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.idx.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.binMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.idx { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.binMd5 { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQVoiceExtReq<'a> {
    pub qid: Option<Cow<'a, [u8]>>,
    pub fmt: Option<u32>,
    pub rate: Option<u32>,
    pub bits: Option<u32>,
    pub channel: Option<u32>,
    pub pinyin: Option<u32>,
}

impl<'a> MessageRead<'a> for QQVoiceExtReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.qid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.fmt = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.rate = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.bits = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.channel = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.pinyin = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QQVoiceExtReq<'a> {
    fn get_size(&self) -> usize {
        0
        + self.qid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fmt.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rate.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bits.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.channel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pinyin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.qid { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fmt { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.rate { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.bits { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.channel { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.pinyin { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQVoiceExtRsp<'a> {
    pub qid: Option<Cow<'a, [u8]>>,
    pub retcode: Option<i32>,
    pub result: Vec<QQVoiceResult<'a>>,
}

impl<'a> MessageRead<'a> for QQVoiceExtRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.qid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.retcode = Some(r.read_int32(bytes)?),
                Ok(26) => msg.result.push(r.read_message::<QQVoiceResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QQVoiceExtRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.qid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.retcode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.result.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.qid { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.retcode { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        for s in &self.result { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QQVoiceResult<'a> {
    pub text: Option<Cow<'a, [u8]>>,
    pub pinyin: Option<Cow<'a, [u8]>>,
    pub source: Option<u32>,
}

impl<'a> MessageRead<'a> for QQVoiceResult<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.pinyin = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.source = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QQVoiceResult<'a> {
    fn get_size(&self) -> usize {
        0
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pinyin.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.text { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.pinyin { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoReqExtInfo<'a> {
    pub cmd: Option<u32>,
    pub sessionId: Option<u64>,
    pub thumbinfo: Option<PicInfo<'a>>,
    pub videoinfo: Option<VideoInfo<'a>>,
    pub shortvideoSureReq: Option<ShortVideoSureReqInfo<'a>>,
    pub isMergeCmdBeforeData: Option<bool>,
}

impl<'a> MessageRead<'a> for ShortVideoReqExtInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cmd = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.sessionId = Some(r.read_uint64(bytes)?),
                Ok(26) => msg.thumbinfo = Some(r.read_message::<PicInfo>(bytes)?),
                Ok(34) => msg.videoinfo = Some(r.read_message::<VideoInfo>(bytes)?),
                Ok(42) => msg.shortvideoSureReq = Some(r.read_message::<ShortVideoSureReqInfo>(bytes)?),
                Ok(48) => msg.isMergeCmdBeforeData = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoReqExtInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.cmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sessionId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.thumbinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.videoinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.shortvideoSureReq.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.isMergeCmdBeforeData.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.cmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sessionId { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.thumbinfo { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.videoinfo { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shortvideoSureReq { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.isMergeCmdBeforeData { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoRspExtInfo<'a> {
    pub cmd: Option<u32>,
    pub sessionId: Option<u64>,
    pub retcode: Option<i32>,
    pub errinfo: Option<Cow<'a, [u8]>>,
    pub thumbinfo: Option<PicInfo<'a>>,
    pub videoinfo: Option<VideoInfo<'a>>,
    pub shortvideoSureRsp: Option<ShortVideoSureRspInfo<'a>>,
    pub retryFlag: Option<u32>,
}

impl<'a> MessageRead<'a> for ShortVideoRspExtInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cmd = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.sessionId = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.retcode = Some(r.read_int32(bytes)?),
                Ok(34) => msg.errinfo = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.thumbinfo = Some(r.read_message::<PicInfo>(bytes)?),
                Ok(50) => msg.videoinfo = Some(r.read_message::<VideoInfo>(bytes)?),
                Ok(58) => msg.shortvideoSureRsp = Some(r.read_message::<ShortVideoSureRspInfo>(bytes)?),
                Ok(64) => msg.retryFlag = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoRspExtInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.cmd.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sessionId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.retcode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.errinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.thumbinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.videoinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.shortvideoSureRsp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.retryFlag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.cmd { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.sessionId { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.retcode { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.errinfo { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.thumbinfo { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.videoinfo { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shortvideoSureRsp { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.retryFlag { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoSureReqInfo<'a> {
    pub fromuin: Option<u64>,
    pub chatType: Option<u32>,
    pub touin: Option<u64>,
    pub groupCode: Option<u64>,
    pub clientType: Option<u32>,
    pub thumbinfo: Option<PicInfo<'a>>,
    pub mergeVideoinfo: Vec<VideoInfo<'a>>,
    pub dropVideoinfo: Vec<VideoInfo<'a>>,
    pub businessType: Option<u32>,
    pub subBusinessType: Option<u32>,
}

impl<'a> MessageRead<'a> for ShortVideoSureReqInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.fromuin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.chatType = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.touin = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.groupCode = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.clientType = Some(r.read_uint32(bytes)?),
                Ok(50) => msg.thumbinfo = Some(r.read_message::<PicInfo>(bytes)?),
                Ok(58) => msg.mergeVideoinfo.push(r.read_message::<VideoInfo>(bytes)?),
                Ok(66) => msg.dropVideoinfo.push(r.read_message::<VideoInfo>(bytes)?),
                Ok(72) => msg.businessType = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.subBusinessType = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoSureReqInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fromuin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chatType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.touin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.groupCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.clientType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.thumbinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mergeVideoinfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.dropVideoinfo.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.businessType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.subBusinessType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fromuin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.chatType { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.touin { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.groupCode { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.clientType { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.thumbinfo { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.mergeVideoinfo { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.dropVideoinfo { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.businessType { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.subBusinessType { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShortVideoSureRspInfo<'a> {
    pub fileid: Option<Cow<'a, [u8]>>,
    pub ukey: Option<Cow<'a, [u8]>>,
    pub videoinfo: Option<VideoInfo<'a>>,
    pub mergeCost: Option<u32>,
}

impl<'a> MessageRead<'a> for ShortVideoSureRspInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.ukey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.videoinfo = Some(r.read_message::<VideoInfo>(bytes)?),
                Ok(32) => msg.mergeCost = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShortVideoSureRspInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ukey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.videoinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mergeCost.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileid { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.ukey { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.videoinfo { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mergeCost { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StoryVideoExtReq { }

impl<'a> MessageRead<'a> for StoryVideoExtReq {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for StoryVideoExtReq { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StoryVideoExtRsp<'a> {
    pub retcode: Option<i32>,
    pub msg: Option<Cow<'a, [u8]>>,
    pub cdnUrl: Option<Cow<'a, [u8]>>,
    pub fileKey: Option<Cow<'a, [u8]>>,
    pub fileId: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for StoryVideoExtRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retcode = Some(r.read_int32(bytes)?),
                Ok(18) => msg.msg = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.cdnUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.fileKey = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.fileId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StoryVideoExtRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.retcode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.msg.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.cdnUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileKey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.fileId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.retcode { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.msg { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.cdnUrl { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileKey { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fileId { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UploadPicExtInfo<'a> {
    pub fileResid: Option<Cow<'a, [u8]>>,
    pub downloadUrl: Option<Cow<'a, [u8]>>,
    pub thumbDownloadUrl: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for UploadPicExtInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileResid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.downloadUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.thumbDownloadUrl = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UploadPicExtInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileResid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.downloadUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.thumbDownloadUrl.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileResid { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.downloadUrl { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.thumbDownloadUrl { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct VideoInfo<'a> {
    pub idx: Option<u32>,
    pub size: Option<u32>,
    pub binMd5: Option<Cow<'a, [u8]>>,
    pub format: Option<u32>,
    pub resLen: Option<u32>,
    pub resWidth: Option<u32>,
    pub time: Option<u32>,
    pub starttime: Option<u64>,
    pub isAudio: Option<u32>,
}

impl<'a> MessageRead<'a> for VideoInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.idx = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.size = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.binMd5 = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.format = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.resLen = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.resWidth = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.time = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.starttime = Some(r.read_uint64(bytes)?),
                Ok(72) => msg.isAudio = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for VideoInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.idx.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.binMd5.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.format.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.resLen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.resWidth.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.starttime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.isAudio.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.idx { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.binMd5 { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.format { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.resLen { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.resWidth { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.starttime { w.write_with_tag(64, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.isAudio { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

