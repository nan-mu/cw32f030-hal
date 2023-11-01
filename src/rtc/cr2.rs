#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `AWTSRC` reader - desc AWTSRC"]
pub type AWTSRC_R = crate::FieldReader;
#[doc = "Field `AWTSRC` writer - desc AWTSRC"]
pub type AWTSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TAMPEDGE` reader - desc TAMPEDGE"]
pub type TAMPEDGE_R = crate::BitReader;
#[doc = "Field `TAMPEDGE` writer - desc TAMPEDGE"]
pub type TAMPEDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTCOUT` reader - desc RTCOUT"]
pub type RTCOUT_R = crate::FieldReader;
#[doc = "Field `RTCOUT` writer - desc RTCOUT"]
pub type RTCOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TAMPEN` reader - desc TAMPEN"]
pub type TAMPEN_R = crate::BitReader;
#[doc = "Field `TAMPEN` writer - desc TAMPEN"]
pub type TAMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWTEN` reader - desc AWTEN"]
pub type AWTEN_R = crate::BitReader;
#[doc = "Field `AWTEN` writer - desc AWTEN"]
pub type AWTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALARMAEN` reader - desc ALARMAEN"]
pub type ALARMAEN_R = crate::BitReader;
#[doc = "Field `ALARMAEN` writer - desc ALARMAEN"]
pub type ALARMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALARMBEN` reader - desc ALARMBEN"]
pub type ALARMBEN_R = crate::BitReader;
#[doc = "Field `ALARMBEN` writer - desc ALARMBEN"]
pub type ALARMBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - desc AWTSRC"]
    #[inline(always)]
    pub fn awtsrc(&self) -> AWTSRC_R {
        AWTSRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc TAMPEDGE"]
    #[inline(always)]
    pub fn tampedge(&self) -> TAMPEDGE_R {
        TAMPEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc RTCOUT"]
    #[inline(always)]
    pub fn rtcout(&self) -> RTCOUT_R {
        RTCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc TAMPEN"]
    #[inline(always)]
    pub fn tampen(&self) -> TAMPEN_R {
        TAMPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc AWTEN"]
    #[inline(always)]
    pub fn awten(&self) -> AWTEN_R {
        AWTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ALARMAEN"]
    #[inline(always)]
    pub fn alarmaen(&self) -> ALARMAEN_R {
        ALARMAEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ALARMBEN"]
    #[inline(always)]
    pub fn alarmben(&self) -> ALARMBEN_R {
        ALARMBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc AWTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn awtsrc(&mut self) -> AWTSRC_W<CR2_SPEC, 0> {
        AWTSRC_W::new(self)
    }
    #[doc = "Bit 3 - desc TAMPEDGE"]
    #[inline(always)]
    #[must_use]
    pub fn tampedge(&mut self) -> TAMPEDGE_W<CR2_SPEC, 3> {
        TAMPEDGE_W::new(self)
    }
    #[doc = "Bits 4:5 - desc RTCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn rtcout(&mut self) -> RTCOUT_W<CR2_SPEC, 4> {
        RTCOUT_W::new(self)
    }
    #[doc = "Bit 6 - desc TAMPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tampen(&mut self) -> TAMPEN_W<CR2_SPEC, 6> {
        TAMPEN_W::new(self)
    }
    #[doc = "Bit 7 - desc AWTEN"]
    #[inline(always)]
    #[must_use]
    pub fn awten(&mut self) -> AWTEN_W<CR2_SPEC, 7> {
        AWTEN_W::new(self)
    }
    #[doc = "Bit 9 - desc ALARMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn alarmaen(&mut self) -> ALARMAEN_W<CR2_SPEC, 9> {
        ALARMAEN_W::new(self)
    }
    #[doc = "Bit 10 - desc ALARMBEN"]
    #[inline(always)]
    #[must_use]
    pub fn alarmben(&mut self) -> ALARMBEN_W<CR2_SPEC, 10> {
        ALARMBEN_W::new(self)
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
#[doc = "Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
