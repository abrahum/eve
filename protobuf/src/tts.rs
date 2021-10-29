// Automatically generated rust module for 'tts.proto' file

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
pub struct TtsRspBody<'a> {
    pub ret_code: u32,
    pub session_id: Cow<'a, str>,
    pub out_seq: u32,
    pub voice_data: Vec<TtsVoiceItem<'a>>,
    pub islast: bool,
    pub pcm_sample_rate: u32,
    pub opus_sample_rate: u32,
    pub opus_channels: u32,
    pub opus_bit_rate: u32,
    pub opus_frame_size: u32,
}

impl<'a> MessageRead<'a> for TtsRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ret_code = r.read_uint32(bytes)?,
                Ok(18) => msg.session_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.out_seq = r.read_uint32(bytes)?,
                Ok(34) => msg.voice_data.push(r.read_message::<TtsVoiceItem>(bytes)?),
                Ok(40) => msg.islast = r.read_bool(bytes)?,
                Ok(48) => msg.pcm_sample_rate = r.read_uint32(bytes)?,
                Ok(56) => msg.opus_sample_rate = r.read_uint32(bytes)?,
                Ok(64) => msg.opus_channels = r.read_uint32(bytes)?,
                Ok(72) => msg.opus_bit_rate = r.read_uint32(bytes)?,
                Ok(80) => msg.opus_frame_size = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TtsRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.ret_code == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.ret_code) as u64) }
        + if self.session_id == "" { 0 } else { 1 + sizeof_len((&self.session_id).len()) }
        + if self.out_seq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.out_seq) as u64) }
        + self.voice_data.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.islast == false { 0 } else { 1 + sizeof_varint(*(&self.islast) as u64) }
        + if self.pcm_sample_rate == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.pcm_sample_rate) as u64) }
        + if self.opus_sample_rate == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.opus_sample_rate) as u64) }
        + if self.opus_channels == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.opus_channels) as u64) }
        + if self.opus_bit_rate == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.opus_bit_rate) as u64) }
        + if self.opus_frame_size == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.opus_frame_size) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ret_code != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.ret_code))?; }
        if self.session_id != "" { w.write_with_tag(18, |w| w.write_string(&**&self.session_id))?; }
        if self.out_seq != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.out_seq))?; }
        for s in &self.voice_data { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.islast != false { w.write_with_tag(40, |w| w.write_bool(*&self.islast))?; }
        if self.pcm_sample_rate != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.pcm_sample_rate))?; }
        if self.opus_sample_rate != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.opus_sample_rate))?; }
        if self.opus_channels != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.opus_channels))?; }
        if self.opus_bit_rate != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.opus_bit_rate))?; }
        if self.opus_frame_size != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.opus_frame_size))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TtsVoiceItem<'a> {
    pub voice: Cow<'a, [u8]>,
    pub seq: u32,
}

impl<'a> MessageRead<'a> for TtsVoiceItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.voice = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.seq = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TtsVoiceItem<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.voice == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.voice).len()) }
        + if self.seq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.voice != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.voice))?; }
        if self.seq != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.seq))?; }
        Ok(())
    }
}

