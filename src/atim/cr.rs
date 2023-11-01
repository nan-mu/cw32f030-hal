#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type COMP_R = crate::BitReader;
#[doc = "Field `COMP` writer - desc COMP"]
pub type COMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CT` reader - desc CT"]
pub type CT_R = crate::BitReader;
#[doc = "Field `CT` writer - desc CT"]
pub type CT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM2S` reader - desc PWM2S"]
pub type PWM2S_R = crate::BitReader;
#[doc = "Field `PWM2S` writer - desc PWM2S"]
pub type PWM2S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PRS_R = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PRS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BUFPEN` reader - desc BUFPEN"]
pub type BUFPEN_R = crate::BitReader;
#[doc = "Field `BUFPEN` writer - desc BUFPEN"]
pub type BUFPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - Please keep 10/11"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Please keep 10/11"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ONESHOT` reader - desc ONESHOT"]
pub type ONESHOT_R = crate::BitReader;
#[doc = "Field `ONESHOT` writer - desc ONESHOT"]
pub type ONESHOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCCS` reader - desc OCCS"]
pub type OCCS_R = crate::BitReader;
#[doc = "Field `OCCS` writer - desc OCCS"]
pub type OCCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `URS` reader - desc URS"]
pub type URS_R = crate::BitReader;
#[doc = "Field `URS` writer - desc URS"]
pub type URS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BIE` reader - desc BIE"]
pub type BIE_R = crate::BitReader;
#[doc = "Field `BIE` writer - desc BIE"]
pub type BIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CISA` reader - desc CISA"]
pub type CISA_R = crate::FieldReader;
#[doc = "Field `CISA` writer - desc CISA"]
pub type CISA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `OCCE` reader - desc OCCE"]
pub type OCCE_R = crate::BitReader;
#[doc = "Field `OCCE` writer - desc OCCE"]
pub type OCCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TG` reader - desc TG"]
pub type TG_R = crate::BitReader;
#[doc = "Field `TG` writer - desc TG"]
pub type TG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UG` reader - desc UG"]
pub type UG_R = crate::BitReader;
#[doc = "Field `UG` writer - desc UG"]
pub type UG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BG` reader - desc BG"]
pub type BG_R = crate::BitReader;
#[doc = "Field `BG` writer - desc BG"]
pub type BG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVE` reader - desc OVE"]
pub type OVE_R = crate::BitReader;
#[doc = "Field `OVE` writer - desc OVE"]
pub type OVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNDE` reader - desc UNDE"]
pub type UNDE_R = crate::BitReader;
#[doc = "Field `UNDE` writer - desc UNDE"]
pub type UNDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PWM2S"]
    #[inline(always)]
    pub fn pwm2s(&self) -> PWM2S_R {
        PWM2S_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc BUFPEN"]
    #[inline(always)]
    pub fn bufpen(&self) -> BUFPEN_R {
        BUFPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Please keep 10/11"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OCCS"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc URS"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - desc CISA"]
    #[inline(always)]
    pub fn cisa(&self) -> CISA_R {
        CISA_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - desc OCCE"]
    #[inline(always)]
    pub fn occe(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TG"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc UG"]
    #[inline(always)]
    pub fn ug(&self) -> UG_R {
        UG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc BG"]
    #[inline(always)]
    pub fn bg(&self) -> BG_R {
        BG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc OVE"]
    #[inline(always)]
    pub fn ove(&self) -> OVE_R {
        OVE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc UNDE"]
    #[inline(always)]
    pub fn unde(&self) -> UNDE_R {
        UNDE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - desc COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<CR_SPEC, 1> {
        COMP_W::new(self)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<CR_SPEC, 2> {
        CT_W::new(self)
    }
    #[doc = "Bit 3 - desc PWM2S"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2s(&mut self) -> PWM2S_W<CR_SPEC, 3> {
        PWM2S_W::new(self)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<CR_SPEC, 4> {
        PRS_W::new(self)
    }
    #[doc = "Bit 7 - desc BUFPEN"]
    #[inline(always)]
    #[must_use]
    pub fn bufpen(&mut self) -> BUFPEN_W<CR_SPEC, 7> {
        BUFPEN_W::new(self)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<CR_SPEC, 10> {
        UIE_W::new(self)
    }
    #[doc = "Bits 12:13 - Please keep 10/11"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CR_SPEC, 12> {
        MODE_W::new(self)
    }
    #[doc = "Bit 14 - desc ONESHOT"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> ONESHOT_W<CR_SPEC, 14> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bit 16 - desc OCCS"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<CR_SPEC, 16> {
        OCCS_W::new(self)
    }
    #[doc = "Bit 17 - desc URS"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<CR_SPEC, 17> {
        URS_W::new(self)
    }
    #[doc = "Bit 19 - desc TIE"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<CR_SPEC, 19> {
        TIE_W::new(self)
    }
    #[doc = "Bit 20 - desc BIE"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<CR_SPEC, 20> {
        BIE_W::new(self)
    }
    #[doc = "Bits 21:22 - desc CISA"]
    #[inline(always)]
    #[must_use]
    pub fn cisa(&mut self) -> CISA_W<CR_SPEC, 21> {
        CISA_W::new(self)
    }
    #[doc = "Bit 23 - desc OCCE"]
    #[inline(always)]
    #[must_use]
    pub fn occe(&mut self) -> OCCE_W<CR_SPEC, 23> {
        OCCE_W::new(self)
    }
    #[doc = "Bit 24 - desc TG"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<CR_SPEC, 24> {
        TG_W::new(self)
    }
    #[doc = "Bit 25 - desc UG"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<CR_SPEC, 25> {
        UG_W::new(self)
    }
    #[doc = "Bit 26 - desc BG"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<CR_SPEC, 26> {
        BG_W::new(self)
    }
    #[doc = "Bit 27 - desc DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CR_SPEC, 27> {
        DIR_W::new(self)
    }
    #[doc = "Bit 28 - desc OVE"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OVE_W<CR_SPEC, 28> {
        OVE_W::new(self)
    }
    #[doc = "Bit 29 - desc UNDE"]
    #[inline(always)]
    #[must_use]
    pub fn unde(&mut self) -> UNDE_W<CR_SPEC, 29> {
        UNDE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "desc CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
