// Automatically generated rust module for 'oidb0x5eb.proto' file

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
pub struct D5EBReqBody<'a> {
    pub uins: Vec<u64>,
    pub max_package_size: Option<u32>,
    pub openid: Vec<Cow<'a, [u8]>>,
    pub appid: Option<u32>,
    pub reqNick: Option<u32>,
    pub reqCountry: Option<u32>,
    pub reqProvince: Option<i32>,
    pub reqGender: Option<i32>,
    pub reqAllow: Option<i32>,
    pub reqFaceId: Option<i32>,
    pub reqCity: Option<i32>,
    pub reqConstellation: Option<i32>,
    pub reqCommonPlace1: Option<i32>,
    pub reqMss3Bitmapextra: Option<i32>,
    pub reqBirthday: Option<i32>,
    pub reqCityId: Option<i32>,
    pub reqLang1: Option<i32>,
    pub reqLang2: Option<i32>,
    pub reqLang3: Option<i32>,
    pub reqAge: Option<i32>,
    pub reqCityZoneId: Option<i32>,
    pub reqOin: Option<i32>,
    pub reqBubbleId: Option<i32>,
    pub reqMss2Identity: Option<i32>,
    pub reqMss1Service: Option<i32>,
    pub reqLflag: Option<i32>,
    pub reqExtFlag: Option<i32>,
    pub reqBasicSvrFlag: Option<i32>,
    pub reqBasicCliFlag: Option<i32>,
    pub reqFullBirthday: Option<i32>,
    pub reqFullAge: Option<i32>,
    pub reqSimpleUpdateTime: Option<i32>,
    pub reqMssUpdateTime: Option<i32>,
    pub reqPstnMultiCallTime: Option<i32>,
    pub reqPstnMultiLastGuideRechargeTime: Option<i32>,
    pub reqPstnC2cCallTime: Option<i32>,
    pub reqPstnC2cLastGuideRechargeTime: Option<i32>,
    pub reqGroupMemCreditFlag: Option<i32>,
    pub reqFaceAddonId: Option<i32>,
    pub reqMusicGene: Option<i32>,
    pub reqStrangerNick: Option<i32>,
    pub reqStrangerDeclare: Option<i32>,
    pub reqLoveStatus: Option<i32>,
    pub reqProfession: Option<i32>,
    pub reqVasColorringFlag: Option<i32>,
    pub reqCharm: Option<i32>,
    pub reqApolloTimestamp: Option<i32>,
    pub reqVasFontIdFlag: Option<i32>,
    pub reqGlobalGroupLevel: Option<i32>,
    pub reqInvite2groupAutoAgreeFlag: Option<i32>,
    pub reqSubaccountDisplayThirdQqFlag: Option<i32>,
    pub notifyPartakeLikeRankingListFlag: Option<i32>,
    pub reqLightalkSwitch: Option<i32>,
    pub reqMusicRingVisible: Option<i32>,
    pub reqMusicRingAutoplay: Option<i32>,
    pub reqMusicRingRedpoint: Option<i32>,
    pub torchDisableFlag: Option<i32>,
    pub reqVasMagicfontFlag: Option<i32>,
    pub reqVipFlag: Option<i32>,
    pub reqAuthFlag: Option<i32>,
    pub reqForbidFlag: Option<i32>,
    pub reqGodForbid: Option<i32>,
    pub reqGodFlag: Option<i32>,
    pub reqCharmLevel: Option<i32>,
    pub reqCharmShown: Option<i32>,
    pub reqFreshnewsNotifyFlag: Option<i32>,
    pub reqApolloVipLevel: Option<i32>,
    pub reqApolloVipFlag: Option<i32>,
    pub reqPstnC2cVip: Option<i32>,
    pub reqPstnMultiVip: Option<i32>,
    pub reqPstnEverC2cVip: Option<i32>,
    pub reqPstnEverMultiVip: Option<i32>,
    pub reqPstnMultiTryFlag: Option<i32>,
    pub reqPstnC2cTryFlag: Option<i32>,
    pub reqSubscribeNearbyassistantSwitch: Option<i32>,
    pub reqTorchbearerFlag: Option<i32>,
    pub preloadDisableFlag: Option<i32>,
    pub reqMedalwallFlag: Option<i32>,
    pub notifyOnLikeRankingListFlag: Option<i32>,
    pub reqApolloStatus: Option<i32>,
}

impl<'a> MessageRead<'a> for D5EBReqBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uins.push(r.read_uint64(bytes)?),
                Ok(24) => msg.max_package_size = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.openid.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.appid = Some(r.read_uint32(bytes)?),
                Ok(160016) => msg.reqNick = Some(r.read_uint32(bytes)?),
                Ok(160024) => msg.reqCountry = Some(r.read_uint32(bytes)?),
                Ok(160032) => msg.reqProvince = Some(r.read_int32(bytes)?),
                Ok(160072) => msg.reqGender = Some(r.read_int32(bytes)?),
                Ok(160112) => msg.reqAllow = Some(r.read_int32(bytes)?),
                Ok(160120) => msg.reqFaceId = Some(r.read_int32(bytes)?),
                Ok(160160) => msg.reqCity = Some(r.read_int32(bytes)?),
                Ok(160176) => msg.reqConstellation = Some(r.read_int32(bytes)?),
                Ok(160216) => msg.reqCommonPlace1 = Some(r.read_int32(bytes)?),
                Ok(160240) => msg.reqMss3Bitmapextra = Some(r.read_int32(bytes)?),
                Ok(160248) => msg.reqBirthday = Some(r.read_int32(bytes)?),
                Ok(160256) => msg.reqCityId = Some(r.read_int32(bytes)?),
                Ok(160264) => msg.reqLang1 = Some(r.read_int32(bytes)?),
                Ok(160272) => msg.reqLang2 = Some(r.read_int32(bytes)?),
                Ok(160280) => msg.reqLang3 = Some(r.read_int32(bytes)?),
                Ok(160296) => msg.reqAge = Some(r.read_int32(bytes)?),
                Ok(160328) => msg.reqCityZoneId = Some(r.read_int32(bytes)?),
                Ok(160448) => msg.reqOin = Some(r.read_int32(bytes)?),
                Ok(160472) => msg.reqBubbleId = Some(r.read_int32(bytes)?),
                Ok(168008) => msg.reqMss2Identity = Some(r.read_int32(bytes)?),
                Ok(168016) => msg.reqMss1Service = Some(r.read_int32(bytes)?),
                Ok(168024) => msg.reqLflag = Some(r.read_int32(bytes)?),
                Ok(168032) => msg.reqExtFlag = Some(r.read_int32(bytes)?),
                Ok(168048) => msg.reqBasicSvrFlag = Some(r.read_int32(bytes)?),
                Ok(168056) => msg.reqBasicCliFlag = Some(r.read_int32(bytes)?),
                Ok(208032) => msg.reqFullBirthday = Some(r.read_int32(bytes)?),
                Ok(208040) => msg.reqFullAge = Some(r.read_int32(bytes)?),
                Ok(208080) => msg.reqSimpleUpdateTime = Some(r.read_int32(bytes)?),
                Ok(208088) => msg.reqMssUpdateTime = Some(r.read_int32(bytes)?),
                Ok(208096) => msg.reqPstnMultiCallTime = Some(r.read_int32(bytes)?),
                Ok(208104) => msg.reqPstnMultiLastGuideRechargeTime = Some(r.read_int32(bytes)?),
                Ok(208112) => msg.reqPstnC2cCallTime = Some(r.read_int32(bytes)?),
                Ok(208120) => msg.reqPstnC2cLastGuideRechargeTime = Some(r.read_int32(bytes)?),
                Ok(216176) => msg.reqGroupMemCreditFlag = Some(r.read_int32(bytes)?),
                Ok(216200) => msg.reqFaceAddonId = Some(r.read_int32(bytes)?),
                Ok(216208) => msg.reqMusicGene = Some(r.read_int32(bytes)?),
                Ok(216272) => msg.reqStrangerNick = Some(r.read_int32(bytes)?),
                Ok(216280) => msg.reqStrangerDeclare = Some(r.read_int32(bytes)?),
                Ok(216288) => msg.reqLoveStatus = Some(r.read_int32(bytes)?),
                Ok(216296) => msg.reqProfession = Some(r.read_int32(bytes)?),
                Ok(216328) => msg.reqVasColorringFlag = Some(r.read_int32(bytes)?),
                Ok(216416) => msg.reqCharm = Some(r.read_int32(bytes)?),
                Ok(216472) => msg.reqApolloTimestamp = Some(r.read_int32(bytes)?),
                Ok(217608) => msg.reqVasFontIdFlag = Some(r.read_int32(bytes)?),
                Ok(217664) => msg.reqGlobalGroupLevel = Some(r.read_int32(bytes)?),
                Ok(322768) => msg.reqInvite2groupAutoAgreeFlag = Some(r.read_int32(bytes)?),
                Ok(322784) => msg.reqSubaccountDisplayThirdQqFlag = Some(r.read_int32(bytes)?),
                Ok(322800) => msg.notifyPartakeLikeRankingListFlag = Some(r.read_int32(bytes)?),
                Ok(324048) => msg.reqLightalkSwitch = Some(r.read_int32(bytes)?),
                Ok(324056) => msg.reqMusicRingVisible = Some(r.read_int32(bytes)?),
                Ok(324064) => msg.reqMusicRingAutoplay = Some(r.read_int32(bytes)?),
                Ok(324072) => msg.reqMusicRingRedpoint = Some(r.read_int32(bytes)?),
                Ok(324200) => msg.torchDisableFlag = Some(r.read_int32(bytes)?),
                Ok(324240) => msg.reqVasMagicfontFlag = Some(r.read_int32(bytes)?),
                Ok(334048) => msg.reqVipFlag = Some(r.read_int32(bytes)?),
                Ok(334264) => msg.reqAuthFlag = Some(r.read_int32(bytes)?),
                Ok(334272) => msg.reqForbidFlag = Some(r.read_int32(bytes)?),
                Ok(334432) => msg.reqGodForbid = Some(r.read_int32(bytes)?),
                Ok(334440) => msg.reqGodFlag = Some(r.read_int32(bytes)?),
                Ok(335600) => msg.reqCharmLevel = Some(r.read_int32(bytes)?),
                Ok(335784) => msg.reqCharmShown = Some(r.read_int32(bytes)?),
                Ok(335944) => msg.reqFreshnewsNotifyFlag = Some(r.read_int32(bytes)?),
                Ok(335992) => msg.reqApolloVipLevel = Some(r.read_int32(bytes)?),
                Ok(336024) => msg.reqApolloVipFlag = Some(r.read_int32(bytes)?),
                Ok(336040) => msg.reqPstnC2cVip = Some(r.read_int32(bytes)?),
                Ok(336048) => msg.reqPstnMultiVip = Some(r.read_int32(bytes)?),
                Ok(336056) => msg.reqPstnEverC2cVip = Some(r.read_int32(bytes)?),
                Ok(336064) => msg.reqPstnEverMultiVip = Some(r.read_int32(bytes)?),
                Ok(336088) => msg.reqPstnMultiTryFlag = Some(r.read_int32(bytes)?),
                Ok(336096) => msg.reqPstnC2cTryFlag = Some(r.read_int32(bytes)?),
                Ok(336192) => msg.reqSubscribeNearbyassistantSwitch = Some(r.read_int32(bytes)?),
                Ok(336408) => msg.reqTorchbearerFlag = Some(r.read_int32(bytes)?),
                Ok(336584) => msg.preloadDisableFlag = Some(r.read_int32(bytes)?),
                Ok(336600) => msg.reqMedalwallFlag = Some(r.read_int32(bytes)?),
                Ok(336736) => msg.notifyOnLikeRankingListFlag = Some(r.read_int32(bytes)?),
                Ok(343840) => msg.reqApolloStatus = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D5EBReqBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uins.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.max_package_size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.openid.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.appid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.reqNick.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCountry.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqProvince.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqGender.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqAllow.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqFaceId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCity.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqConstellation.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCommonPlace1.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMss3Bitmapextra.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqBirthday.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCityId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqLang1.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqLang2.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqLang3.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqAge.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCityZoneId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqOin.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqBubbleId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMss2Identity.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMss1Service.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqLflag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqExtFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqBasicSvrFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqBasicCliFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqFullBirthday.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqFullAge.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqSimpleUpdateTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMssUpdateTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnMultiCallTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnMultiLastGuideRechargeTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnC2cCallTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnC2cLastGuideRechargeTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqGroupMemCreditFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqFaceAddonId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMusicGene.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqStrangerNick.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqStrangerDeclare.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqLoveStatus.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqProfession.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqVasColorringFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCharm.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqApolloTimestamp.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqVasFontIdFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqGlobalGroupLevel.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqInvite2groupAutoAgreeFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqSubaccountDisplayThirdQqFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.notifyPartakeLikeRankingListFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqLightalkSwitch.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMusicRingVisible.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMusicRingAutoplay.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMusicRingRedpoint.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.torchDisableFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqVasMagicfontFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqVipFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqAuthFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqForbidFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqGodForbid.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqGodFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCharmLevel.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqCharmShown.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqFreshnewsNotifyFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqApolloVipLevel.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqApolloVipFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnC2cVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnMultiVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnEverC2cVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnEverMultiVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnMultiTryFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqPstnC2cTryFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqSubscribeNearbyassistantSwitch.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqTorchbearerFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.preloadDisableFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMedalwallFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.notifyOnLikeRankingListFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqApolloStatus.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.uins { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.max_package_size { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        for s in &self.openid { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.appid { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.reqNick { w.write_with_tag(160016, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.reqCountry { w.write_with_tag(160024, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.reqProvince { w.write_with_tag(160032, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqGender { w.write_with_tag(160072, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqAllow { w.write_with_tag(160112, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqFaceId { w.write_with_tag(160120, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqCity { w.write_with_tag(160160, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqConstellation { w.write_with_tag(160176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqCommonPlace1 { w.write_with_tag(160216, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMss3Bitmapextra { w.write_with_tag(160240, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqBirthday { w.write_with_tag(160248, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqCityId { w.write_with_tag(160256, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqLang1 { w.write_with_tag(160264, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqLang2 { w.write_with_tag(160272, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqLang3 { w.write_with_tag(160280, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqAge { w.write_with_tag(160296, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqCityZoneId { w.write_with_tag(160328, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqOin { w.write_with_tag(160448, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqBubbleId { w.write_with_tag(160472, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMss2Identity { w.write_with_tag(168008, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMss1Service { w.write_with_tag(168016, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqLflag { w.write_with_tag(168024, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqExtFlag { w.write_with_tag(168032, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqBasicSvrFlag { w.write_with_tag(168048, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqBasicCliFlag { w.write_with_tag(168056, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqFullBirthday { w.write_with_tag(208032, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqFullAge { w.write_with_tag(208040, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqSimpleUpdateTime { w.write_with_tag(208080, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMssUpdateTime { w.write_with_tag(208088, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnMultiCallTime { w.write_with_tag(208096, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnMultiLastGuideRechargeTime { w.write_with_tag(208104, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnC2cCallTime { w.write_with_tag(208112, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnC2cLastGuideRechargeTime { w.write_with_tag(208120, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqGroupMemCreditFlag { w.write_with_tag(216176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqFaceAddonId { w.write_with_tag(216200, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMusicGene { w.write_with_tag(216208, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqStrangerNick { w.write_with_tag(216272, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqStrangerDeclare { w.write_with_tag(216280, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqLoveStatus { w.write_with_tag(216288, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqProfession { w.write_with_tag(216296, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqVasColorringFlag { w.write_with_tag(216328, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqCharm { w.write_with_tag(216416, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqApolloTimestamp { w.write_with_tag(216472, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqVasFontIdFlag { w.write_with_tag(217608, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqGlobalGroupLevel { w.write_with_tag(217664, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqInvite2groupAutoAgreeFlag { w.write_with_tag(322768, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqSubaccountDisplayThirdQqFlag { w.write_with_tag(322784, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.notifyPartakeLikeRankingListFlag { w.write_with_tag(322800, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqLightalkSwitch { w.write_with_tag(324048, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMusicRingVisible { w.write_with_tag(324056, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMusicRingAutoplay { w.write_with_tag(324064, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMusicRingRedpoint { w.write_with_tag(324072, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.torchDisableFlag { w.write_with_tag(324200, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqVasMagicfontFlag { w.write_with_tag(324240, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqVipFlag { w.write_with_tag(334048, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqAuthFlag { w.write_with_tag(334264, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqForbidFlag { w.write_with_tag(334272, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqGodForbid { w.write_with_tag(334432, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqGodFlag { w.write_with_tag(334440, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqCharmLevel { w.write_with_tag(335600, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqCharmShown { w.write_with_tag(335784, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqFreshnewsNotifyFlag { w.write_with_tag(335944, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqApolloVipLevel { w.write_with_tag(335992, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqApolloVipFlag { w.write_with_tag(336024, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnC2cVip { w.write_with_tag(336040, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnMultiVip { w.write_with_tag(336048, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnEverC2cVip { w.write_with_tag(336056, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnEverMultiVip { w.write_with_tag(336064, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnMultiTryFlag { w.write_with_tag(336088, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqPstnC2cTryFlag { w.write_with_tag(336096, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqSubscribeNearbyassistantSwitch { w.write_with_tag(336192, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqTorchbearerFlag { w.write_with_tag(336408, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.preloadDisableFlag { w.write_with_tag(336584, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMedalwallFlag { w.write_with_tag(336600, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.notifyOnLikeRankingListFlag { w.write_with_tag(336736, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqApolloStatus { w.write_with_tag(343840, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct D5EBRspBody<'a> {
    pub uinData: Vec<UdcUinData<'a>>,
    pub unfinishedUins: Vec<i64>,
}

impl<'a> MessageRead<'a> for D5EBRspBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(90) => msg.uinData.push(r.read_message::<UdcUinData>(bytes)?),
                Ok(96) => msg.unfinishedUins.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for D5EBRspBody<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uinData.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.unfinishedUins.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.uinData { w.write_with_tag(90, |w| w.write_message(s))?; }
        for s in &self.unfinishedUins { w.write_with_tag(96, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UdcUinData<'a> {
    pub uin: Option<i64>,
    pub openid: Option<Cow<'a, [u8]>>,
    pub nick: Option<Cow<'a, [u8]>>,
    pub country: Option<Cow<'a, [u8]>>,
    pub province: Option<Cow<'a, [u8]>>,
    pub gender: Option<i32>,
    pub allow: Option<i32>,
    pub faceId: Option<i32>,
    pub city: Option<Cow<'a, [u8]>>,
    pub constellation: Option<i32>,
    pub commonPlace1: Option<i32>,
    pub mss3Bitmapextra: Option<Cow<'a, [u8]>>,
    pub birthday: Option<Cow<'a, [u8]>>,
    pub cityId: Option<Cow<'a, [u8]>>,
    pub lang1: Option<i32>,
    pub lang2: Option<i32>,
    pub lang3: Option<i32>,
    pub age: Option<i32>,
    pub cityZoneId: Option<i32>,
    pub oin: Option<i32>,
    pub bubbleId: Option<i32>,
    pub mss2Identity: Option<Cow<'a, [u8]>>,
    pub mss1Service: Option<Cow<'a, [u8]>>,
    pub lflag: Option<i32>,
    pub extFlag: Option<i32>,
    pub basicSvrFlag: Option<Cow<'a, [u8]>>,
    pub basicCliFlag: Option<Cow<'a, [u8]>>,
    pub fullBirthday: Option<Cow<'a, [u8]>>,
    pub fullAge: Option<Cow<'a, [u8]>>,
    pub simpleUpdateTime: Option<i32>,
    pub mssUpdateTime: Option<i32>,
    pub pstnMultiCallTime: Option<i32>,
    pub pstnMultiLastGuideRechargeTime: Option<i32>,
    pub pstnC2cCallTime: Option<i32>,
    pub pstnC2cLastGuideRechargeTime: Option<i32>,
    pub groupMemCreditFlag: Option<i32>,
    pub faceAddonId: Option<i64>,
    pub musicGene: Option<Cow<'a, [u8]>>,
    pub strangerNick: Option<Cow<'a, [u8]>>,
    pub strangerDeclare: Option<Cow<'a, [u8]>>,
    pub loveStatus: Option<i32>,
    pub profession: Option<i32>,
    pub vasColorringId: Option<i32>,
    pub charm: Option<i32>,
    pub apolloTimestamp: Option<i32>,
    pub vasFontId: Option<i32>,
    pub globalGroupLevel: Option<i32>,
    pub reqInvite2groupAutoAgreeFlag: Option<i32>,
    pub subaccountDisplayThirdQqFlag: Option<i32>,
    pub notifyPartakeLikeRankingListFlag: Option<i32>,
    pub lightalkSwitch: Option<i32>,
    pub musicRingVisible: Option<i32>,
    pub musicRingAutoplay: Option<i32>,
    pub musicRingRedpoint: Option<i32>,
    pub torchDisableFlag: Option<i32>,
    pub vasMagicfontFlag: Option<i32>,
    pub vipFlag: Option<i32>,
    pub authFlag: Option<i32>,
    pub forbidFlag: Option<i32>,
    pub godForbid: Option<i32>,
    pub godFlag: Option<i32>,
    pub charmLevel: Option<i32>,
    pub charmShown: Option<i32>,
    pub freshnewsNotifyFlag: Option<i32>,
    pub apolloVipLevel: Option<i32>,
    pub apolloVipFlag: Option<i32>,
    pub pstnC2cVip: Option<i32>,
    pub pstnMultiVip: Option<i32>,
    pub pstnEverC2cVip: Option<i32>,
    pub pstnEverMultiVip: Option<i32>,
    pub pstnMultiTryFlag: Option<i32>,
    pub pstnC2cTryFlag: Option<i32>,
    pub subscribeNearbyassistantSwitch: Option<i32>,
    pub torchbearerFlag: Option<i32>,
    pub preloadDisableFlag: Option<i32>,
    pub reqMedalwallFlag: Option<i32>,
    pub notifyOnLikeRankingListFlag: Option<i32>,
    pub apolloStatus: Option<i32>,
}

impl<'a> MessageRead<'a> for UdcUinData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.uin = Some(r.read_int64(bytes)?),
                Ok(34) => msg.openid = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160018) => msg.nick = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160026) => msg.country = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160034) => msg.province = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160072) => msg.gender = Some(r.read_int32(bytes)?),
                Ok(160112) => msg.allow = Some(r.read_int32(bytes)?),
                Ok(160120) => msg.faceId = Some(r.read_int32(bytes)?),
                Ok(160162) => msg.city = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160176) => msg.constellation = Some(r.read_int32(bytes)?),
                Ok(160216) => msg.commonPlace1 = Some(r.read_int32(bytes)?),
                Ok(160242) => msg.mss3Bitmapextra = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160250) => msg.birthday = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160258) => msg.cityId = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(160264) => msg.lang1 = Some(r.read_int32(bytes)?),
                Ok(160272) => msg.lang2 = Some(r.read_int32(bytes)?),
                Ok(160280) => msg.lang3 = Some(r.read_int32(bytes)?),
                Ok(160296) => msg.age = Some(r.read_int32(bytes)?),
                Ok(160328) => msg.cityZoneId = Some(r.read_int32(bytes)?),
                Ok(160448) => msg.oin = Some(r.read_int32(bytes)?),
                Ok(160472) => msg.bubbleId = Some(r.read_int32(bytes)?),
                Ok(168010) => msg.mss2Identity = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(168018) => msg.mss1Service = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(168024) => msg.lflag = Some(r.read_int32(bytes)?),
                Ok(168032) => msg.extFlag = Some(r.read_int32(bytes)?),
                Ok(168050) => msg.basicSvrFlag = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(168058) => msg.basicCliFlag = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(208034) => msg.fullBirthday = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(208042) => msg.fullAge = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(208080) => msg.simpleUpdateTime = Some(r.read_int32(bytes)?),
                Ok(208088) => msg.mssUpdateTime = Some(r.read_int32(bytes)?),
                Ok(208096) => msg.pstnMultiCallTime = Some(r.read_int32(bytes)?),
                Ok(208104) => msg.pstnMultiLastGuideRechargeTime = Some(r.read_int32(bytes)?),
                Ok(208112) => msg.pstnC2cCallTime = Some(r.read_int32(bytes)?),
                Ok(208120) => msg.pstnC2cLastGuideRechargeTime = Some(r.read_int32(bytes)?),
                Ok(216176) => msg.groupMemCreditFlag = Some(r.read_int32(bytes)?),
                Ok(216200) => msg.faceAddonId = Some(r.read_int64(bytes)?),
                Ok(216210) => msg.musicGene = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(216274) => msg.strangerNick = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(216282) => msg.strangerDeclare = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(216288) => msg.loveStatus = Some(r.read_int32(bytes)?),
                Ok(216296) => msg.profession = Some(r.read_int32(bytes)?),
                Ok(216328) => msg.vasColorringId = Some(r.read_int32(bytes)?),
                Ok(216416) => msg.charm = Some(r.read_int32(bytes)?),
                Ok(216472) => msg.apolloTimestamp = Some(r.read_int32(bytes)?),
                Ok(217608) => msg.vasFontId = Some(r.read_int32(bytes)?),
                Ok(217664) => msg.globalGroupLevel = Some(r.read_int32(bytes)?),
                Ok(322768) => msg.reqInvite2groupAutoAgreeFlag = Some(r.read_int32(bytes)?),
                Ok(322784) => msg.subaccountDisplayThirdQqFlag = Some(r.read_int32(bytes)?),
                Ok(322800) => msg.notifyPartakeLikeRankingListFlag = Some(r.read_int32(bytes)?),
                Ok(324048) => msg.lightalkSwitch = Some(r.read_int32(bytes)?),
                Ok(324056) => msg.musicRingVisible = Some(r.read_int32(bytes)?),
                Ok(324064) => msg.musicRingAutoplay = Some(r.read_int32(bytes)?),
                Ok(324072) => msg.musicRingRedpoint = Some(r.read_int32(bytes)?),
                Ok(324200) => msg.torchDisableFlag = Some(r.read_int32(bytes)?),
                Ok(324240) => msg.vasMagicfontFlag = Some(r.read_int32(bytes)?),
                Ok(334048) => msg.vipFlag = Some(r.read_int32(bytes)?),
                Ok(334264) => msg.authFlag = Some(r.read_int32(bytes)?),
                Ok(334272) => msg.forbidFlag = Some(r.read_int32(bytes)?),
                Ok(334432) => msg.godForbid = Some(r.read_int32(bytes)?),
                Ok(334440) => msg.godFlag = Some(r.read_int32(bytes)?),
                Ok(335600) => msg.charmLevel = Some(r.read_int32(bytes)?),
                Ok(335784) => msg.charmShown = Some(r.read_int32(bytes)?),
                Ok(335944) => msg.freshnewsNotifyFlag = Some(r.read_int32(bytes)?),
                Ok(335992) => msg.apolloVipLevel = Some(r.read_int32(bytes)?),
                Ok(336024) => msg.apolloVipFlag = Some(r.read_int32(bytes)?),
                Ok(336040) => msg.pstnC2cVip = Some(r.read_int32(bytes)?),
                Ok(336048) => msg.pstnMultiVip = Some(r.read_int32(bytes)?),
                Ok(336056) => msg.pstnEverC2cVip = Some(r.read_int32(bytes)?),
                Ok(336064) => msg.pstnEverMultiVip = Some(r.read_int32(bytes)?),
                Ok(336088) => msg.pstnMultiTryFlag = Some(r.read_int32(bytes)?),
                Ok(336096) => msg.pstnC2cTryFlag = Some(r.read_int32(bytes)?),
                Ok(336192) => msg.subscribeNearbyassistantSwitch = Some(r.read_int32(bytes)?),
                Ok(336408) => msg.torchbearerFlag = Some(r.read_int32(bytes)?),
                Ok(336584) => msg.preloadDisableFlag = Some(r.read_int32(bytes)?),
                Ok(336600) => msg.reqMedalwallFlag = Some(r.read_int32(bytes)?),
                Ok(336736) => msg.notifyOnLikeRankingListFlag = Some(r.read_int32(bytes)?),
                Ok(343840) => msg.apolloStatus = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UdcUinData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uin.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.openid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.nick.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.country.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.province.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.gender.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.allow.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.faceId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.city.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.constellation.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.commonPlace1.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.mss3Bitmapextra.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.birthday.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.cityId.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.lang1.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.lang2.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.lang3.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.age.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.cityZoneId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.oin.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.bubbleId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.mss2Identity.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.mss1Service.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.lflag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.extFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.basicSvrFlag.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.basicCliFlag.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.fullBirthday.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.fullAge.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.simpleUpdateTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.mssUpdateTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnMultiCallTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnMultiLastGuideRechargeTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnC2cCallTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnC2cLastGuideRechargeTime.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.groupMemCreditFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.faceAddonId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.musicGene.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.strangerNick.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.strangerDeclare.as_ref().map_or(0, |m| 3 + sizeof_len((m).len()))
        + self.loveStatus.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.profession.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.vasColorringId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.charm.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.apolloTimestamp.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.vasFontId.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.globalGroupLevel.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqInvite2groupAutoAgreeFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.subaccountDisplayThirdQqFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.notifyPartakeLikeRankingListFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.lightalkSwitch.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.musicRingVisible.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.musicRingAutoplay.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.musicRingRedpoint.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.torchDisableFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.vasMagicfontFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.vipFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.authFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.forbidFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.godForbid.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.godFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.charmLevel.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.charmShown.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.freshnewsNotifyFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.apolloVipLevel.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.apolloVipFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnC2cVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnMultiVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnEverC2cVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnEverMultiVip.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnMultiTryFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.pstnC2cTryFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.subscribeNearbyassistantSwitch.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.torchbearerFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.preloadDisableFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.reqMedalwallFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.notifyOnLikeRankingListFlag.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
        + self.apolloStatus.as_ref().map_or(0, |m| 3 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uin { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.openid { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.nick { w.write_with_tag(160018, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.country { w.write_with_tag(160026, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.province { w.write_with_tag(160034, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.gender { w.write_with_tag(160072, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.allow { w.write_with_tag(160112, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.faceId { w.write_with_tag(160120, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.city { w.write_with_tag(160162, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.constellation { w.write_with_tag(160176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.commonPlace1 { w.write_with_tag(160216, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.mss3Bitmapextra { w.write_with_tag(160242, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.birthday { w.write_with_tag(160250, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.cityId { w.write_with_tag(160258, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.lang1 { w.write_with_tag(160264, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.lang2 { w.write_with_tag(160272, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.lang3 { w.write_with_tag(160280, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.age { w.write_with_tag(160296, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.cityZoneId { w.write_with_tag(160328, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.oin { w.write_with_tag(160448, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.bubbleId { w.write_with_tag(160472, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.mss2Identity { w.write_with_tag(168010, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.mss1Service { w.write_with_tag(168018, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.lflag { w.write_with_tag(168024, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.extFlag { w.write_with_tag(168032, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.basicSvrFlag { w.write_with_tag(168050, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.basicCliFlag { w.write_with_tag(168058, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fullBirthday { w.write_with_tag(208034, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.fullAge { w.write_with_tag(208042, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.simpleUpdateTime { w.write_with_tag(208080, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.mssUpdateTime { w.write_with_tag(208088, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnMultiCallTime { w.write_with_tag(208096, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnMultiLastGuideRechargeTime { w.write_with_tag(208104, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnC2cCallTime { w.write_with_tag(208112, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnC2cLastGuideRechargeTime { w.write_with_tag(208120, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.groupMemCreditFlag { w.write_with_tag(216176, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.faceAddonId { w.write_with_tag(216200, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.musicGene { w.write_with_tag(216210, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.strangerNick { w.write_with_tag(216274, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.strangerDeclare { w.write_with_tag(216282, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.loveStatus { w.write_with_tag(216288, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.profession { w.write_with_tag(216296, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.vasColorringId { w.write_with_tag(216328, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.charm { w.write_with_tag(216416, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.apolloTimestamp { w.write_with_tag(216472, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.vasFontId { w.write_with_tag(217608, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.globalGroupLevel { w.write_with_tag(217664, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqInvite2groupAutoAgreeFlag { w.write_with_tag(322768, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.subaccountDisplayThirdQqFlag { w.write_with_tag(322784, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.notifyPartakeLikeRankingListFlag { w.write_with_tag(322800, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.lightalkSwitch { w.write_with_tag(324048, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.musicRingVisible { w.write_with_tag(324056, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.musicRingAutoplay { w.write_with_tag(324064, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.musicRingRedpoint { w.write_with_tag(324072, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.torchDisableFlag { w.write_with_tag(324200, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.vasMagicfontFlag { w.write_with_tag(324240, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.vipFlag { w.write_with_tag(334048, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.authFlag { w.write_with_tag(334264, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.forbidFlag { w.write_with_tag(334272, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.godForbid { w.write_with_tag(334432, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.godFlag { w.write_with_tag(334440, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.charmLevel { w.write_with_tag(335600, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.charmShown { w.write_with_tag(335784, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.freshnewsNotifyFlag { w.write_with_tag(335944, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.apolloVipLevel { w.write_with_tag(335992, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.apolloVipFlag { w.write_with_tag(336024, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnC2cVip { w.write_with_tag(336040, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnMultiVip { w.write_with_tag(336048, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnEverC2cVip { w.write_with_tag(336056, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnEverMultiVip { w.write_with_tag(336064, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnMultiTryFlag { w.write_with_tag(336088, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pstnC2cTryFlag { w.write_with_tag(336096, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.subscribeNearbyassistantSwitch { w.write_with_tag(336192, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.torchbearerFlag { w.write_with_tag(336408, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.preloadDisableFlag { w.write_with_tag(336584, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.reqMedalwallFlag { w.write_with_tag(336600, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.notifyOnLikeRankingListFlag { w.write_with_tag(336736, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.apolloStatus { w.write_with_tag(343840, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

