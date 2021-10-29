// Automatically generated rust module for 'oidb0xe07.proto' file

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
pub struct DE07ReqBody<'a> {
    pub version: i32,
    pub client: i32,
    pub entrance: i32,
    pub ocrReqBody: Option<OCRReqBody<'a>>,
}

impl<'a> MessageRead<'a> for DE07ReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_int32(bytes)?,
                Ok(16) => msg.client = r.read_int32(bytes)?,
                Ok(24) => msg.entrance = r.read_int32(bytes)?,
                Ok(82) => msg.ocrReqBody = Some(r.read_message::<OCRReqBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DE07ReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.client == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.client) as u64) }
        + if self.entrance == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.entrance) as u64) }
        + self.ocrReqBody.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.version))?; }
        if self.client != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.client))?; }
        if self.entrance != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.entrance))?; }
        if let Some(ref s) = self.ocrReqBody { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OCRReqBody<'a> {
    pub imageUrl: Cow<'a, str>,
    pub languageType: Cow<'a, str>,
    pub scene: Cow<'a, str>,
    pub originMd5: Cow<'a, str>,
    pub afterCompressMd5: Cow<'a, str>,
    pub afterCompressFileSize: i32,
    pub afterCompressWeight: i32,
    pub afterCompressHeight: i32,
    pub isCut: bool,
}

impl<'a> MessageRead<'a> for OCRReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.imageUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.languageType = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.scene = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.originMd5 = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.afterCompressMd5 = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(96) => msg.afterCompressFileSize = r.read_int32(bytes)?,
                Ok(104) => msg.afterCompressWeight = r.read_int32(bytes)?,
                Ok(112) => msg.afterCompressHeight = r.read_int32(bytes)?,
                Ok(120) => msg.isCut = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OCRReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.imageUrl == "" { 0 } else { 1 + sizeof_len((&self.imageUrl).len()) }
        + if self.languageType == "" { 0 } else { 1 + sizeof_len((&self.languageType).len()) }
        + if self.scene == "" { 0 } else { 1 + sizeof_len((&self.scene).len()) }
        + if self.originMd5 == "" { 0 } else { 1 + sizeof_len((&self.originMd5).len()) }
        + if self.afterCompressMd5 == "" { 0 } else { 1 + sizeof_len((&self.afterCompressMd5).len()) }
        + if self.afterCompressFileSize == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.afterCompressFileSize) as u64) }
        + if self.afterCompressWeight == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.afterCompressWeight) as u64) }
        + if self.afterCompressHeight == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.afterCompressHeight) as u64) }
        + if self.isCut == false { 0 } else { 1 + sizeof_varint(*(&self.isCut) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.imageUrl != "" { w.write_with_tag(10, |w| w.write_string(&**&self.imageUrl))?; }
        if self.languageType != "" { w.write_with_tag(18, |w| w.write_string(&**&self.languageType))?; }
        if self.scene != "" { w.write_with_tag(26, |w| w.write_string(&**&self.scene))?; }
        if self.originMd5 != "" { w.write_with_tag(82, |w| w.write_string(&**&self.originMd5))?; }
        if self.afterCompressMd5 != "" { w.write_with_tag(90, |w| w.write_string(&**&self.afterCompressMd5))?; }
        if self.afterCompressFileSize != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.afterCompressFileSize))?; }
        if self.afterCompressWeight != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.afterCompressWeight))?; }
        if self.afterCompressHeight != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.afterCompressHeight))?; }
        if self.isCut != false { w.write_with_tag(120, |w| w.write_bool(*&self.isCut))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DE07RspBody<'a> {
    pub retCode: i32,
    pub errMsg: Cow<'a, str>,
    pub wording: Cow<'a, str>,
    pub ocrRspBody: Option<OCRRspBody<'a>>,
}

impl<'a> MessageRead<'a> for DE07RspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.retCode = r.read_int32(bytes)?,
                Ok(18) => msg.errMsg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.wording = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.ocrRspBody = Some(r.read_message::<OCRRspBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DE07RspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.retCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.retCode) as u64) }
        + if self.errMsg == "" { 0 } else { 1 + sizeof_len((&self.errMsg).len()) }
        + if self.wording == "" { 0 } else { 1 + sizeof_len((&self.wording).len()) }
        + self.ocrRspBody.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.retCode != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.retCode))?; }
        if self.errMsg != "" { w.write_with_tag(18, |w| w.write_string(&**&self.errMsg))?; }
        if self.wording != "" { w.write_with_tag(26, |w| w.write_string(&**&self.wording))?; }
        if let Some(ref s) = self.ocrRspBody { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TextDetection<'a> {
    pub detectedText: Cow<'a, str>,
    pub confidence: i32,
    pub polygon: Option<Polygon>,
    pub advancedInfo: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TextDetection<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.detectedText = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.confidence = r.read_int32(bytes)?,
                Ok(26) => msg.polygon = Some(r.read_message::<Polygon>(bytes)?),
                Ok(34) => msg.advancedInfo = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TextDetection<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.detectedText == "" { 0 } else { 1 + sizeof_len((&self.detectedText).len()) }
        + if self.confidence == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.confidence) as u64) }
        + self.polygon.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.advancedInfo == "" { 0 } else { 1 + sizeof_len((&self.advancedInfo).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.detectedText != "" { w.write_with_tag(10, |w| w.write_string(&**&self.detectedText))?; }
        if self.confidence != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.confidence))?; }
        if let Some(ref s) = self.polygon { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.advancedInfo != "" { w.write_with_tag(34, |w| w.write_string(&**&self.advancedInfo))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Polygon {
    pub coordinates: Vec<Coordinate>,
}

impl<'a> MessageRead<'a> for Polygon {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.coordinates.push(r.read_message::<Coordinate>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Polygon {
    fn get_size(&self) -> usize {
        0
        + self.coordinates.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.coordinates { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Coordinate {
    pub X: i32,
    pub Y: i32,
}

impl<'a> MessageRead<'a> for Coordinate {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.X = r.read_int32(bytes)?,
                Ok(16) => msg.Y = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Coordinate {
    fn get_size(&self) -> usize {
        0
        + if self.X == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.X) as u64) }
        + if self.Y == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Y) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.X != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.X))?; }
        if self.Y != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Y))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Language<'a> {
    pub language: Cow<'a, str>,
    pub languageDesc: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Language<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.languageDesc = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Language<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.language == "" { 0 } else { 1 + sizeof_len((&self.language).len()) }
        + if self.languageDesc == "" { 0 } else { 1 + sizeof_len((&self.languageDesc).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.language != "" { w.write_with_tag(10, |w| w.write_string(&**&self.language))?; }
        if self.languageDesc != "" { w.write_with_tag(18, |w| w.write_string(&**&self.languageDesc))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OCRRspBody<'a> {
    pub textDetections: Vec<TextDetection<'a>>,
    pub language: Cow<'a, str>,
    pub requestId: Cow<'a, str>,
    pub ocrLanguageList: Vec<Cow<'a, str>>,
    pub dstTranslateLanguageList: Vec<Cow<'a, str>>,
    pub languageList: Vec<Language<'a>>,
    pub afterCompressWeight: i32,
    pub afterCompressHeight: i32,
}

impl<'a> MessageRead<'a> for OCRRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.textDetections.push(r.read_message::<TextDetection>(bytes)?),
                Ok(18) => msg.language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.requestId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(810) => msg.ocrLanguageList.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(818) => msg.dstTranslateLanguageList.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(826) => msg.languageList.push(r.read_message::<Language>(bytes)?),
                Ok(888) => msg.afterCompressWeight = r.read_int32(bytes)?,
                Ok(896) => msg.afterCompressHeight = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OCRRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.textDetections.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.language == "" { 0 } else { 1 + sizeof_len((&self.language).len()) }
        + if self.requestId == "" { 0 } else { 1 + sizeof_len((&self.requestId).len()) }
        + self.ocrLanguageList.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.dstTranslateLanguageList.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.languageList.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.afterCompressWeight == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.afterCompressWeight) as u64) }
        + if self.afterCompressHeight == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.afterCompressHeight) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.textDetections { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.language != "" { w.write_with_tag(18, |w| w.write_string(&**&self.language))?; }
        if self.requestId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.requestId))?; }
        for s in &self.ocrLanguageList { w.write_with_tag(810, |w| w.write_string(&**s))?; }
        for s in &self.dstTranslateLanguageList { w.write_with_tag(818, |w| w.write_string(&**s))?; }
        for s in &self.languageList { w.write_with_tag(826, |w| w.write_message(s))?; }
        if self.afterCompressWeight != 0i32 { w.write_with_tag(888, |w| w.write_int32(*&self.afterCompressWeight))?; }
        if self.afterCompressHeight != 0i32 { w.write_with_tag(896, |w| w.write_int32(*&self.afterCompressHeight))?; }
        Ok(())
    }
}

