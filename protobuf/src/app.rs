// Automatically generated rust module for 'app.proto' file

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
pub struct GetAppInfoByIdReq<'a> {
    pub appId: Cow<'a, str>,
    pub needVersionInfo: i32,
}

impl<'a> MessageRead<'a> for GetAppInfoByIdReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.appId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.needVersionInfo = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetAppInfoByIdReq<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.appId == "" { 0 } else { 1 + sizeof_len((&self.appId).len()) }
        + if self.needVersionInfo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.needVersionInfo) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.appId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.appId))?; }
        if self.needVersionInfo != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.needVersionInfo))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetAppInfoByIdRsp<'a> {
    pub appInfo: Option<ApiAppInfo<'a>>,
}

impl<'a> MessageRead<'a> for GetAppInfoByIdRsp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.appInfo = Some(r.read_message::<ApiAppInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetAppInfoByIdRsp<'a> {
    fn get_size(&self) -> usize {
        0
        + self.appInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.appInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApiAppInfo<'a> {
    pub appId: Cow<'a, str>,
    pub appName: Cow<'a, str>,
    pub icon: Cow<'a, str>,
    pub downloadUrl: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub desc: Cow<'a, str>,
    pub type_pb: i32,
    pub baseLibMiniVersion: Cow<'a, str>,
    pub subPkgs: Vec<AppSubPkgInfo<'a>>,
    pub domain: Option<DomainConfig<'a>>,
}

impl<'a> MessageRead<'a> for ApiAppInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.appId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.appName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.icon = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.downloadUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.desc = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.type_pb = r.read_int32(bytes)?,
                Ok(74) => msg.baseLibMiniVersion = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.subPkgs.push(r.read_message::<AppSubPkgInfo>(bytes)?),
                Ok(98) => msg.domain = Some(r.read_message::<DomainConfig>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApiAppInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.appId == "" { 0 } else { 1 + sizeof_len((&self.appId).len()) }
        + if self.appName == "" { 0 } else { 1 + sizeof_len((&self.appName).len()) }
        + if self.icon == "" { 0 } else { 1 + sizeof_len((&self.icon).len()) }
        + if self.downloadUrl == "" { 0 } else { 1 + sizeof_len((&self.downloadUrl).len()) }
        + if self.version == "" { 0 } else { 1 + sizeof_len((&self.version).len()) }
        + if self.desc == "" { 0 } else { 1 + sizeof_len((&self.desc).len()) }
        + if self.type_pb == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.baseLibMiniVersion == "" { 0 } else { 1 + sizeof_len((&self.baseLibMiniVersion).len()) }
        + self.subPkgs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.domain.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.appId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.appId))?; }
        if self.appName != "" { w.write_with_tag(18, |w| w.write_string(&**&self.appName))?; }
        if self.icon != "" { w.write_with_tag(26, |w| w.write_string(&**&self.icon))?; }
        if self.downloadUrl != "" { w.write_with_tag(34, |w| w.write_string(&**&self.downloadUrl))?; }
        if self.version != "" { w.write_with_tag(42, |w| w.write_string(&**&self.version))?; }
        if self.desc != "" { w.write_with_tag(50, |w| w.write_string(&**&self.desc))?; }
        if self.type_pb != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.type_pb))?; }
        if self.baseLibMiniVersion != "" { w.write_with_tag(74, |w| w.write_string(&**&self.baseLibMiniVersion))?; }
        for s in &self.subPkgs { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.domain { w.write_with_tag(98, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppSubPkgInfo<'a> {
    pub subPkgName: Cow<'a, str>,
    pub downloadUrl: Cow<'a, str>,
    pub independent: i32,
    pub fileSize: i32,
}

impl<'a> MessageRead<'a> for AppSubPkgInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subPkgName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.downloadUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.independent = r.read_int32(bytes)?,
                Ok(32) => msg.fileSize = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AppSubPkgInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.subPkgName == "" { 0 } else { 1 + sizeof_len((&self.subPkgName).len()) }
        + if self.downloadUrl == "" { 0 } else { 1 + sizeof_len((&self.downloadUrl).len()) }
        + if self.independent == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.independent) as u64) }
        + if self.fileSize == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subPkgName != "" { w.write_with_tag(10, |w| w.write_string(&**&self.subPkgName))?; }
        if self.downloadUrl != "" { w.write_with_tag(18, |w| w.write_string(&**&self.downloadUrl))?; }
        if self.independent != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.independent))?; }
        if self.fileSize != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.fileSize))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DomainConfig<'a> {
    pub requestDomain: Vec<Cow<'a, str>>,
    pub socketDomain: Vec<Cow<'a, str>>,
    pub uploadFileDomain: Vec<Cow<'a, str>>,
    pub downloadFileDomain: Vec<Cow<'a, str>>,
    pub businessDomain: Vec<Cow<'a, str>>,
    pub udpIpList: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for DomainConfig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requestDomain.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.socketDomain.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.uploadFileDomain.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.downloadFileDomain.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.businessDomain.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.udpIpList.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DomainConfig<'a> {
    fn get_size(&self) -> usize {
        0
        + self.requestDomain.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.socketDomain.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.uploadFileDomain.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.downloadFileDomain.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.businessDomain.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.udpIpList.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.requestDomain { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.socketDomain { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.uploadFileDomain { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.downloadFileDomain { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.businessDomain { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        for s in &self.udpIpList { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

