#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FILTER_SPEC>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FILTER_SPEC>;
#[doc = "Field `PIN13` reader - desc PIN13"]
pub type PIN13_R = crate::BitReader;
#[doc = "Field `PIN13` writer - desc PIN13"]
pub type PIN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN14` reader - desc PIN14"]
pub type PIN14_R = crate::BitReader;
#[doc = "Field `PIN14` writer - desc PIN14"]
pub type PIN14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN15` reader - desc PIN15"]
pub type PIN15_R = crate::BitReader;
#[doc = "Field `PIN15` writer - desc PIN15"]
pub type PIN15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTCLK` reader - desc FLTCLK"]
pub type FLTCLK_R = crate::FieldReader;
#[doc = "Field `FLTCLK` writer - desc FLTCLK"]
pub type FLTCLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&self) -> FLTCLK_R {
        FLTCLK_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> PIN13_W<FILTER_SPEC, 13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> PIN14_W<FILTER_SPEC, 14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> PIN15_W<FILTER_SPEC, 15> {
        PIN15_W::new(self)
    }
    #[doc = "Bits 16:18 - desc FLTCLK"]
    #[inline(always)]
    #[must_use]
    pub fn fltclk(&mut self) -> FLTCLK_W<FILTER_SPEC, 16> {
        FLTCLK_W::new(self)
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
#[doc = "desc FILTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_SPEC;
impl crate::RegisterSpec for FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FILTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
