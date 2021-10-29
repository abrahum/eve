// Automatically generated rust module for 'oidb0x769.proto' file

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
pub struct CPU<'a> {
    pub model: Option<Cow<'a, str>>,
    pub cores: Option<u32>,
    pub frequency: Option<u32>,
}

impl<'a> MessageRead<'a> for CPU<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.model = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.cores = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.frequency = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CPU<'a> {
    fn get_size(&self) -> usize {
        0
        + self.model.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.cores.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.frequency.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.model { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.cores { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.frequency { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Camera {
    pub primary: Option<u64>,
    pub secondary: Option<u64>,
    pub flash: Option<bool>,
}

impl<'a> MessageRead<'a> for Camera {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.primary = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.secondary = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.flash = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Camera {
    fn get_size(&self) -> usize {
        0
        + self.primary.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.secondary.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.flash.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.primary { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.secondary { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.flash { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D769ConfigSeq {
    pub type_pb: Option<u32>,
    pub version: Option<u32>,
}

impl<'a> MessageRead<'a> for D769ConfigSeq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.version = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for D769ConfigSeq {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Content<'a> {
    pub taskId: Option<u32>,
    pub compress: Option<u32>,
    pub content: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for Content<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.taskId = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.compress = Some(r.read_uint32(bytes)?),
                Ok(82) => msg.content = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Content<'a> {
    fn get_size(&self) -> usize {
        0
        + self.taskId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.compress.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.taskId { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.compress { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.content { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D769DeviceInfo<'a> {
    pub brand: Option<Cow<'a, str>>,
    pub model: Option<Cow<'a, str>>,
    pub os: Option<C41219OS<'a>>,
    pub cpu: Option<CPU<'a>>,
    pub memory: Option<Memory>,
    pub storage: Option<Storage>,
    pub screen: Option<Screen<'a>>,
    pub camera: Option<Camera>,
}

impl<'a> MessageRead<'a> for D769DeviceInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.brand = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.model = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.os = Some(r.read_message::<C41219OS>(bytes)?),
                Ok(34) => msg.cpu = Some(r.read_message::<CPU>(bytes)?),
                Ok(42) => msg.memory = Some(r.read_message::<Memory>(bytes)?),
                Ok(50) => msg.storage = Some(r.read_message::<Storage>(bytes)?),
                Ok(58) => msg.screen = Some(r.read_message::<Screen>(bytes)?),
                Ok(66) => msg.camera = Some(r.read_message::<Camera>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D769DeviceInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.brand.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.model.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.os.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.cpu.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.memory.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.storage.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.screen.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.camera.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.brand { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.model { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.os { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cpu { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.memory { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.storage { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.screen { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.camera { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Memory {
    pub total: Option<u64>,
    pub process: Option<u64>,
}

impl<'a> MessageRead<'a> for Memory {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.total = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.process = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Memory {
    fn get_size(&self) -> usize {
        0
        + self.total.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.process.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.total { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.process { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct C41219OS<'a> {
    pub type_pb: Option<u32>,
    pub version: Option<Cow<'a, str>>,
    pub sdk: Option<Cow<'a, str>>,
    pub kernel: Option<Cow<'a, str>>,
    pub rom: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for C41219OS<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.sdk = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.kernel = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.rom = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for C41219OS<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sdk.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.kernel.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.rom.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.sdk { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.kernel { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.rom { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QueryUinPackageUsageReq {
    pub type_pb: Option<u32>,
    pub uinFileSize: Option<u64>,
}

impl<'a> MessageRead<'a> for QueryUinPackageUsageReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.uinFileSize = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for QueryUinPackageUsageReq {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uinFileSize.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.uinFileSize { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QueryUinPackageUsageRsp<'a> {
    pub status: Option<u32>,
    pub leftUinNum: Option<u64>,
    pub maxUinNum: Option<u64>,
    pub proportion: Option<u32>,
    pub uinPackageUsedList: Vec<UinPackageUsedInfo<'a>>,
}

impl<'a> MessageRead<'a> for QueryUinPackageUsageRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.leftUinNum = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.maxUinNum = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.proportion = Some(r.read_uint32(bytes)?),
                Ok(82) => msg.uinPackageUsedList.push(r.read_message::<UinPackageUsedInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QueryUinPackageUsageRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.leftUinNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.maxUinNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.proportion.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uinPackageUsedList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.status { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.leftUinNum { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.maxUinNum { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.proportion { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        for s in &self.uinPackageUsedList { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D769ReqBody<'a> {
    pub configList: Vec<D769ConfigSeq>,
    pub deviceInfo: Option<D769DeviceInfo<'a>>,
    pub info: Option<Cow<'a, str>>,
    pub province: Option<Cow<'a, str>>,
    pub city: Option<Cow<'a, str>>,
    pub reqDebugMsg: Option<i32>,
    pub queryUinPackageUsageReq: Option<QueryUinPackageUsageReq>,
}

impl<'a> MessageRead<'a> for D769ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.configList.push(r.read_message::<D769ConfigSeq>(bytes)?),
                Ok(18) => msg.deviceInfo = Some(r.read_message::<D769DeviceInfo>(bytes)?),
                Ok(26) => msg.info = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.province = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.city = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.reqDebugMsg = Some(r.read_int32(bytes)?),
                Ok(810) => msg.queryUinPackageUsageReq = Some(r.read_message::<QueryUinPackageUsageReq>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D769ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.configList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.deviceInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.info.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.province.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.city.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.reqDebugMsg.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.queryUinPackageUsageReq.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.configList { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.deviceInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.info { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.province { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.city { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.reqDebugMsg { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.queryUinPackageUsageReq { w.write_with_tag(810, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D769RspBody<'a> {
    pub result: Option<u32>,
    pub configList: Vec<D769ConfigSeq>,
    pub queryUinPackageUsageRsp: Option<QueryUinPackageUsageRsp<'a>>,
}

impl<'a> MessageRead<'a> for D769RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.configList.push(r.read_message::<D769ConfigSeq>(bytes)?),
                Ok(810) => msg.queryUinPackageUsageRsp = Some(r.read_message::<QueryUinPackageUsageRsp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D769RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.configList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.queryUinPackageUsageRsp.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.configList { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.queryUinPackageUsageRsp { w.write_with_tag(810, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Screen<'a> {
    pub model: Option<Cow<'a, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub dpi: Option<u32>,
    pub multiTouch: Option<bool>,
}

impl<'a> MessageRead<'a> for Screen<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.model = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.width = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.height = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.dpi = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.multiTouch = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Screen<'a> {
    fn get_size(&self) -> usize {
        0
        + self.model.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.width.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.height.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.dpi.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.multiTouch.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.model { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.width { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.height { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.dpi { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.multiTouch { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Storage {
    pub builtin: Option<u64>,
    pub external: Option<u64>,
}

impl<'a> MessageRead<'a> for Storage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.builtin = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.external = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Storage {
    fn get_size(&self) -> usize {
        0
        + self.builtin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.external.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.builtin { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.external { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UinPackageUsedInfo<'a> {
    pub ruleId: Option<u32>,
    pub author: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub uinNum: Option<u64>,
}

impl<'a> MessageRead<'a> for UinPackageUsedInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ruleId = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.author = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.uinNum = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UinPackageUsedInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ruleId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.author.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.uinNum.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ruleId { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.author { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.url { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.uinNum { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

